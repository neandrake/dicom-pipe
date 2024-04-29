use crate::{app::CommandApplication, args::SvcProviderArgs, threadpool::ThreadPool};
use anyhow::{anyhow, Result};
use dcmpipe_lib::{
    core::{
        charset::DEFAULT_CHARACTER_SET,
        dcmobject::DicomRoot,
        defn::constants::ts::ImplicitVRLittleEndian,
        read::{ParserBuilder, ParserState},
        write::{builder::WriterBuilder, writer::WriterState},
    },
    dict::{stdlookup::STANDARD_DICOM_DICTIONARY, uids::VerificationSOPClass},
    dimse::{
        commands::{messages::CommandMessage, CommandStatus},
        pdus::{
            AssocAC, AssocACPresentationContext, AssocRQ, Pdu, PresentationDataItem, PresentationDataValue, ReleaseRP, ReleaseRQ, TransferSyntaxItem
        },
    },
};
use std::{
    io::{BufReader, BufWriter, Cursor, Write},
    net::{TcpListener, TcpStream},
};

pub struct SvcProviderApp {
    args: SvcProviderArgs,
}

impl SvcProviderApp {
    pub fn new(args: SvcProviderArgs) -> SvcProviderApp {
        SvcProviderApp { args }
    }
}

impl CommandApplication for SvcProviderApp {
    fn run(&mut self) -> Result<()> {
        // XXX: This enforces max associations with a thread pool, but will fail to send a response
        //      indicating that max assocations has been reached.
        let pool = ThreadPool::new(self.args.max_assoc);
        let listener = TcpListener::bind(&self.args.host)?;
        println!("[info <>]: Listening for associations on {}", self.args.host);

        for stream in listener.incoming() {
            let stream = stream?;
            let host_ae = self.args.aetitle.clone();
            pool.execute(|| {
                let mut assoc = Association { host_ae, stream };
                if let Err(e) = assoc.handle_association() {
                    eprintln!("[ err --]: {e:?}");
                }
            })?;
        }
        Ok(())
    }
}

struct Association {
    host_ae: String,
    stream: TcpStream,
}

impl Association {
    fn handle_association(&mut self) -> Result<()> {
        let mut bufread = BufReader::new(&self.stream);
        let mut bufwrite = BufWriter::new(&self.stream);

        let pdu = Pdu::read_pdu(&mut bufread)?;
        let Pdu::AssocRQ(rq) = pdu else {
            return Err(anyhow!("Unexpected PDU: {pdu:?}"));
        };

        self.handle_assoc_rq(&mut bufwrite, rq)?;

        let pdu = Pdu::read_pdu(&mut bufread)?;
        let Pdu::PresentationDataItem(pdi) = pdu else {
            return Err(anyhow!("Unexpected PDU: {pdu:?}"));
        };

        self.handle_pres_data_item(&mut bufwrite, pdi)?;

        let pdu = Pdu::read_pdu(&mut bufread)?;
        let Pdu::ReleaseRQ(rq) = pdu else {
            return Err(anyhow!("Unspected PDU: {pdu:?}"));
        };

        self.handle_release_rq(&mut bufwrite, rq)?;

        Ok(())
    }

    fn handle_assoc_rq(&self, mut bufwrite: &mut BufWriter<&TcpStream>, rq: AssocRQ) -> Result<()> {
        println!("[info <-]: ASSOCIATION-RQ");
        let cs = DEFAULT_CHARACTER_SET;
        let host_ae = self.host_ae.trim();

        let called_ae = cs.decode(rq.called_ae())?;
        let called_ae = called_ae.trim();
        if host_ae != called_ae {
            return Err(anyhow!(
                "Called AE ({called_ae}) is not host AE {}",
                host_ae
            ));
        }
        if rq.pres_ctxs().is_empty() {
            return Err(anyhow!("No presentation context items defined."));
        }
        let pres_ctx = &rq.pres_ctxs()[0];
        let abstract_syntax = cs.decode(pres_ctx.abstract_syntax().abstract_syntax())?;
        let abstract_syntax = abstract_syntax.trim();

        if abstract_syntax != VerificationSOPClass.uid() {
            return Err(anyhow!("Invalid abstract syntax: {abstract_syntax}"));
        }

        let ts = pres_ctx
            .transfer_syntaxes()
            .iter()
            .filter_map(|ts| cs.decode(ts.transfer_syntaxes()).ok())
            .filter(|ts| ts == ImplicitVRLittleEndian.uid().uid())
            .next();

        let Some(ts) = ts else {
            return Err(anyhow!(
                "Unsupported transfer syntax: {:?}",
                pres_ctx.transfer_syntaxes()
            ));
        };

        let rsp = AssocAC::new(
            rq.called_ae().to_owned(),
            rq.calling_ae().to_owned(),
            rq.reserved_3().to_owned(),
            rq.app_ctx().to_owned(),
            vec![AssocACPresentationContext::new(
                pres_ctx.ctx_id(),
                0u8, // Acceptance
                TransferSyntaxItem::new(ts.into_bytes()),
            )],
            rq.user_info().to_owned(),
        );

        println!("[info ->]: ASSOCIATION-AC");
        rsp.write(&mut bufwrite)?;
        bufwrite.flush()?;
        Ok(())
    }

    fn handle_pres_data_item(
        &self,
        bufwrite: &mut BufWriter<&TcpStream>,
        pdi: PresentationDataItem,
    ) -> Result<()> {
        println!("[info <-]: PDU-Presentation Data Item");
        let pb = ParserBuilder::default()
            .state(ParserState::Element)
            .dictionary(&STANDARD_DICOM_DICTIONARY)
            .dataset_ts(&ImplicitVRLittleEndian);

        for pdv in pdi.pres_data() {
            let mut parser = pb.build(Cursor::new(pdv.data()));
            let dicom_root = DicomRoot::parse(&mut parser)?;
            let Some(dicom_root) = dicom_root else {
                continue;
            };
            let command = CommandMessage::new(dicom_root);
            let rsp = CommandMessage::c_echo_rsp_from_req(command, &CommandStatus::Success(0))?;

            let mut writer = WriterBuilder::default()
                .state(WriterState::Element)
                .ts(&ImplicitVRLittleEndian)
                .build(Vec::<u8>::new());
            writer.write_dcmroot(rsp.message())?;
            let data = writer.into_dataset()?;

            let rsp = PresentationDataItem::new(vec![PresentationDataValue::new(
                pdv.ctx_id(),
                pdv.msg_header(),
                data,
            )]);

            rsp.write(bufwrite)?;
            bufwrite.flush()?;
            println!("[info ->]: PDU-Presentation Data Item");
        }

        Ok(())
    }

    fn handle_release_rq(&self, bufwrite: &mut BufWriter<&TcpStream>, _rq: ReleaseRQ) -> Result<()> {
        println!("[info <-]: A-RELEASE-RQ");

        let rsp = ReleaseRP::new();
        rsp.write(bufwrite)?;
        bufwrite.flush()?;
        println!("[info ->]: A-RELEASE-RP");

        Ok(())
    }
}
