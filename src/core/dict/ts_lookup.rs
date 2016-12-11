//! This is an auto-generated file. Do not make modifications here.

use core::dict::transfer_syntaxes as ts;
use core::dict::uids;
use core::ts::TransferSyntax;
use core::uid::UID;

use std::collections::hash_map::HashMap;

pub struct TransferSyntaxLookup {
	ident_to_ts: HashMap<&'static str, &'static TransferSyntax>,
	id_to_ts: HashMap<&'static str, &'static TransferSyntax>,
	uid_to_ts: HashMap<&'static UID, &'static TransferSyntax>,
}

impl TransferSyntaxLookup {
	pub fn by_ident(&self, ident: &str) -> Option<&'static TransferSyntax> {
		self.ident_to_ts.get(ident).map(|ts| *ts)
	}

	pub fn by_id(&self, id: &str) -> Option<&'static TransferSyntax> {
		self.id_to_ts.get(id).map(|ts| *ts)
	}

	pub fn by_uid(&self, uid: &UID) -> Option<&'static TransferSyntax> {
		self.uid_to_ts.get(uid).map(|ts| *ts)
	}

	pub fn new() -> TransferSyntaxLookup {
		let mut ident_to_ts: HashMap<&'static str, &'static TransferSyntax> = HashMap::new();
		let mut id_to_ts: HashMap<&'static str, &'static TransferSyntax> = HashMap::new();
		let mut uid_to_ts: HashMap<&'static UID, &'static TransferSyntax> = HashMap::new();

		ident_to_ts.insert("ImplicitVRLittleEndian", &ts::ImplicitVRLittleEndian);
		id_to_ts.insert("1.2.840.10008.1.2", &ts::ImplicitVRLittleEndian);
		uid_to_ts.insert(&uids::ImplicitVRLittleEndian, &ts::ImplicitVRLittleEndian);

		ident_to_ts.insert("ExplicitVRLittleEndian", &ts::ExplicitVRLittleEndian);
		id_to_ts.insert("1.2.840.10008.1.2.1", &ts::ExplicitVRLittleEndian);
		uid_to_ts.insert(&uids::ExplicitVRLittleEndian, &ts::ExplicitVRLittleEndian);

		ident_to_ts.insert("DeflatedExplicitVRLittleEndian", &ts::DeflatedExplicitVRLittleEndian);
		id_to_ts.insert("1.2.840.10008.1.2.1.99", &ts::DeflatedExplicitVRLittleEndian);
		uid_to_ts.insert(&uids::DeflatedExplicitVRLittleEndian, &ts::DeflatedExplicitVRLittleEndian);

		ident_to_ts.insert("ExplicitVRBigEndian", &ts::ExplicitVRBigEndian);
		id_to_ts.insert("1.2.840.10008.1.2.2", &ts::ExplicitVRBigEndian);
		uid_to_ts.insert(&uids::ExplicitVRBigEndian, &ts::ExplicitVRBigEndian);

		ident_to_ts.insert("JPEGBaselineProcess1", &ts::JPEGBaselineProcess1);
		id_to_ts.insert("1.2.840.10008.1.2.4.50", &ts::JPEGBaselineProcess1);
		uid_to_ts.insert(&uids::JPEGBaselineProcess1, &ts::JPEGBaselineProcess1);

		ident_to_ts.insert("JPEGExtendedProcess2_and_4", &ts::JPEGExtendedProcess2_and_4);
		id_to_ts.insert("1.2.840.10008.1.2.4.51", &ts::JPEGExtendedProcess2_and_4);
		uid_to_ts.insert(&uids::JPEGExtendedProcess2_and_4, &ts::JPEGExtendedProcess2_and_4);

		ident_to_ts.insert("JPEGExtendedProcess3_and_5", &ts::JPEGExtendedProcess3_and_5);
		id_to_ts.insert("1.2.840.10008.1.2.4.52", &ts::JPEGExtendedProcess3_and_5);
		uid_to_ts.insert(&uids::JPEGExtendedProcess3_and_5, &ts::JPEGExtendedProcess3_and_5);

		ident_to_ts.insert("JPEGSpectralSelectionNonHierarchicalProcess6_and_8", &ts::JPEGSpectralSelectionNonHierarchicalProcess6_and_8);
		id_to_ts.insert("1.2.840.10008.1.2.4.53", &ts::JPEGSpectralSelectionNonHierarchicalProcess6_and_8);
		uid_to_ts.insert(&uids::JPEGSpectralSelectionNonHierarchicalProcess6_and_8, &ts::JPEGSpectralSelectionNonHierarchicalProcess6_and_8);

		ident_to_ts.insert("JPEGSpectralSelectionNonHierarchicalProcess7_and_9", &ts::JPEGSpectralSelectionNonHierarchicalProcess7_and_9);
		id_to_ts.insert("1.2.840.10008.1.2.4.54", &ts::JPEGSpectralSelectionNonHierarchicalProcess7_and_9);
		uid_to_ts.insert(&uids::JPEGSpectralSelectionNonHierarchicalProcess7_and_9, &ts::JPEGSpectralSelectionNonHierarchicalProcess7_and_9);

		ident_to_ts.insert("JPEGFullProgressionNonHierarchicalProcess10_and_12", &ts::JPEGFullProgressionNonHierarchicalProcess10_and_12);
		id_to_ts.insert("1.2.840.10008.1.2.4.55", &ts::JPEGFullProgressionNonHierarchicalProcess10_and_12);
		uid_to_ts.insert(&uids::JPEGFullProgressionNonHierarchicalProcess10_and_12, &ts::JPEGFullProgressionNonHierarchicalProcess10_and_12);

		ident_to_ts.insert("JPEGFullProgressionNonHierarchicalProcess11_and_13", &ts::JPEGFullProgressionNonHierarchicalProcess11_and_13);
		id_to_ts.insert("1.2.840.10008.1.2.4.56", &ts::JPEGFullProgressionNonHierarchicalProcess11_and_13);
		uid_to_ts.insert(&uids::JPEGFullProgressionNonHierarchicalProcess11_and_13, &ts::JPEGFullProgressionNonHierarchicalProcess11_and_13);

		ident_to_ts.insert("JPEGLosslessNonHierarchicalProcess14", &ts::JPEGLosslessNonHierarchicalProcess14);
		id_to_ts.insert("1.2.840.10008.1.2.4.57", &ts::JPEGLosslessNonHierarchicalProcess14);
		uid_to_ts.insert(&uids::JPEGLosslessNonHierarchicalProcess14, &ts::JPEGLosslessNonHierarchicalProcess14);

		ident_to_ts.insert("JPEGLosslessNonHierarchicalProcess15", &ts::JPEGLosslessNonHierarchicalProcess15);
		id_to_ts.insert("1.2.840.10008.1.2.4.58", &ts::JPEGLosslessNonHierarchicalProcess15);
		uid_to_ts.insert(&uids::JPEGLosslessNonHierarchicalProcess15, &ts::JPEGLosslessNonHierarchicalProcess15);

		ident_to_ts.insert("JPEGExtendedHierarchicalProcess16_and_18", &ts::JPEGExtendedHierarchicalProcess16_and_18);
		id_to_ts.insert("1.2.840.10008.1.2.4.59", &ts::JPEGExtendedHierarchicalProcess16_and_18);
		uid_to_ts.insert(&uids::JPEGExtendedHierarchicalProcess16_and_18, &ts::JPEGExtendedHierarchicalProcess16_and_18);

		ident_to_ts.insert("JPEGExtendedHierarchicalProcess17_and_19", &ts::JPEGExtendedHierarchicalProcess17_and_19);
		id_to_ts.insert("1.2.840.10008.1.2.4.60", &ts::JPEGExtendedHierarchicalProcess17_and_19);
		uid_to_ts.insert(&uids::JPEGExtendedHierarchicalProcess17_and_19, &ts::JPEGExtendedHierarchicalProcess17_and_19);

		ident_to_ts.insert("JPEGSpectralSelectionHierarchicalProcess20_and_22", &ts::JPEGSpectralSelectionHierarchicalProcess20_and_22);
		id_to_ts.insert("1.2.840.10008.1.2.4.61", &ts::JPEGSpectralSelectionHierarchicalProcess20_and_22);
		uid_to_ts.insert(&uids::JPEGSpectralSelectionHierarchicalProcess20_and_22, &ts::JPEGSpectralSelectionHierarchicalProcess20_and_22);

		ident_to_ts.insert("JPEGSpectralSelectionHierarchicalProcess21_and_23", &ts::JPEGSpectralSelectionHierarchicalProcess21_and_23);
		id_to_ts.insert("1.2.840.10008.1.2.4.62", &ts::JPEGSpectralSelectionHierarchicalProcess21_and_23);
		uid_to_ts.insert(&uids::JPEGSpectralSelectionHierarchicalProcess21_and_23, &ts::JPEGSpectralSelectionHierarchicalProcess21_and_23);

		ident_to_ts.insert("JPEGFullProgressionHierarchicalProcess24_and_26", &ts::JPEGFullProgressionHierarchicalProcess24_and_26);
		id_to_ts.insert("1.2.840.10008.1.2.4.63", &ts::JPEGFullProgressionHierarchicalProcess24_and_26);
		uid_to_ts.insert(&uids::JPEGFullProgressionHierarchicalProcess24_and_26, &ts::JPEGFullProgressionHierarchicalProcess24_and_26);

		ident_to_ts.insert("JPEGFullProgressionHierarchicalProcess25_and_27", &ts::JPEGFullProgressionHierarchicalProcess25_and_27);
		id_to_ts.insert("1.2.840.10008.1.2.4.64", &ts::JPEGFullProgressionHierarchicalProcess25_and_27);
		uid_to_ts.insert(&uids::JPEGFullProgressionHierarchicalProcess25_and_27, &ts::JPEGFullProgressionHierarchicalProcess25_and_27);

		ident_to_ts.insert("JPEGLosslessHierarchicalProcess28", &ts::JPEGLosslessHierarchicalProcess28);
		id_to_ts.insert("1.2.840.10008.1.2.4.65", &ts::JPEGLosslessHierarchicalProcess28);
		uid_to_ts.insert(&uids::JPEGLosslessHierarchicalProcess28, &ts::JPEGLosslessHierarchicalProcess28);

		ident_to_ts.insert("JPEGLosslessHierarchicalProcess29", &ts::JPEGLosslessHierarchicalProcess29);
		id_to_ts.insert("1.2.840.10008.1.2.4.66", &ts::JPEGLosslessHierarchicalProcess29);
		uid_to_ts.insert(&uids::JPEGLosslessHierarchicalProcess29, &ts::JPEGLosslessHierarchicalProcess29);

		ident_to_ts.insert("JPEGLosslessNonHierarchicalFirstOrderPredictionProcess14SelectionValue1", &ts::JPEGLosslessNonHierarchicalFirstOrderPredictionProcess14SelectionValue1);
		id_to_ts.insert("1.2.840.10008.1.2.4.70", &ts::JPEGLosslessNonHierarchicalFirstOrderPredictionProcess14SelectionValue1);
		uid_to_ts.insert(&uids::JPEGLosslessNonHierarchicalFirstOrderPredictionProcess14SelectionValue1, &ts::JPEGLosslessNonHierarchicalFirstOrderPredictionProcess14SelectionValue1);

		ident_to_ts.insert("JPEGLSLosslessImageCompression", &ts::JPEGLSLosslessImageCompression);
		id_to_ts.insert("1.2.840.10008.1.2.4.80", &ts::JPEGLSLosslessImageCompression);
		uid_to_ts.insert(&uids::JPEGLSLosslessImageCompression, &ts::JPEGLSLosslessImageCompression);

		ident_to_ts.insert("JPEGLSLossyNearLosslessImageCompression", &ts::JPEGLSLossyNearLosslessImageCompression);
		id_to_ts.insert("1.2.840.10008.1.2.4.81", &ts::JPEGLSLossyNearLosslessImageCompression);
		uid_to_ts.insert(&uids::JPEGLSLossyNearLosslessImageCompression, &ts::JPEGLSLossyNearLosslessImageCompression);

		ident_to_ts.insert("JPEG2000ImageCompressionLosslessOnly", &ts::JPEG2000ImageCompressionLosslessOnly);
		id_to_ts.insert("1.2.840.10008.1.2.4.90", &ts::JPEG2000ImageCompressionLosslessOnly);
		uid_to_ts.insert(&uids::JPEG2000ImageCompressionLosslessOnly, &ts::JPEG2000ImageCompressionLosslessOnly);

		ident_to_ts.insert("JPEG2000ImageCompression", &ts::JPEG2000ImageCompression);
		id_to_ts.insert("1.2.840.10008.1.2.4.91", &ts::JPEG2000ImageCompression);
		uid_to_ts.insert(&uids::JPEG2000ImageCompression, &ts::JPEG2000ImageCompression);

		ident_to_ts.insert("JPEG2000Part2MulticomponentImageCompressionLosslessOnly", &ts::JPEG2000Part2MulticomponentImageCompressionLosslessOnly);
		id_to_ts.insert("1.2.840.10008.1.2.4.92", &ts::JPEG2000Part2MulticomponentImageCompressionLosslessOnly);
		uid_to_ts.insert(&uids::JPEG2000Part2MulticomponentImageCompressionLosslessOnly, &ts::JPEG2000Part2MulticomponentImageCompressionLosslessOnly);

		ident_to_ts.insert("JPEG2000Part2MulticomponentImageCompression", &ts::JPEG2000Part2MulticomponentImageCompression);
		id_to_ts.insert("1.2.840.10008.1.2.4.93", &ts::JPEG2000Part2MulticomponentImageCompression);
		uid_to_ts.insert(&uids::JPEG2000Part2MulticomponentImageCompression, &ts::JPEG2000Part2MulticomponentImageCompression);

		ident_to_ts.insert("JPIPReferenced", &ts::JPIPReferenced);
		id_to_ts.insert("1.2.840.10008.1.2.4.94", &ts::JPIPReferenced);
		uid_to_ts.insert(&uids::JPIPReferenced, &ts::JPIPReferenced);

		ident_to_ts.insert("JPIPReferencedDeflate", &ts::JPIPReferencedDeflate);
		id_to_ts.insert("1.2.840.10008.1.2.4.95", &ts::JPIPReferencedDeflate);
		uid_to_ts.insert(&uids::JPIPReferencedDeflate, &ts::JPIPReferencedDeflate);

		ident_to_ts.insert("MPEG2MainProfileMainLevel", &ts::MPEG2MainProfileMainLevel);
		id_to_ts.insert("1.2.840.10008.1.2.4.100", &ts::MPEG2MainProfileMainLevel);
		uid_to_ts.insert(&uids::MPEG2MainProfileMainLevel, &ts::MPEG2MainProfileMainLevel);

		ident_to_ts.insert("MPEG2MainProfileHighLevel", &ts::MPEG2MainProfileHighLevel);
		id_to_ts.insert("1.2.840.10008.1.2.4.101", &ts::MPEG2MainProfileHighLevel);
		uid_to_ts.insert(&uids::MPEG2MainProfileHighLevel, &ts::MPEG2MainProfileHighLevel);

		ident_to_ts.insert("MPEG4AVCH264HighProfileLevel41", &ts::MPEG4AVCH264HighProfileLevel41);
		id_to_ts.insert("1.2.840.10008.1.2.4.102", &ts::MPEG4AVCH264HighProfileLevel41);
		uid_to_ts.insert(&uids::MPEG4AVCH264HighProfileLevel41, &ts::MPEG4AVCH264HighProfileLevel41);

		ident_to_ts.insert("MPEG4AVCH264BDcompatibleHighProfileLevel41", &ts::MPEG4AVCH264BDcompatibleHighProfileLevel41);
		id_to_ts.insert("1.2.840.10008.1.2.4.103", &ts::MPEG4AVCH264BDcompatibleHighProfileLevel41);
		uid_to_ts.insert(&uids::MPEG4AVCH264BDcompatibleHighProfileLevel41, &ts::MPEG4AVCH264BDcompatibleHighProfileLevel41);

		ident_to_ts.insert("MPEG4AVCH264HighProfileLevel42For2DVideo", &ts::MPEG4AVCH264HighProfileLevel42For2DVideo);
		id_to_ts.insert("1.2.840.10008.1.2.4.104", &ts::MPEG4AVCH264HighProfileLevel42For2DVideo);
		uid_to_ts.insert(&uids::MPEG4AVCH264HighProfileLevel42For2DVideo, &ts::MPEG4AVCH264HighProfileLevel42For2DVideo);

		ident_to_ts.insert("MPEG4AVCH264HighProfileLevel42For3DVideo", &ts::MPEG4AVCH264HighProfileLevel42For3DVideo);
		id_to_ts.insert("1.2.840.10008.1.2.4.105", &ts::MPEG4AVCH264HighProfileLevel42For3DVideo);
		uid_to_ts.insert(&uids::MPEG4AVCH264HighProfileLevel42For3DVideo, &ts::MPEG4AVCH264HighProfileLevel42For3DVideo);

		ident_to_ts.insert("MPEG4AVCH264StereoHighProfileLevel42", &ts::MPEG4AVCH264StereoHighProfileLevel42);
		id_to_ts.insert("1.2.840.10008.1.2.4.106", &ts::MPEG4AVCH264StereoHighProfileLevel42);
		uid_to_ts.insert(&uids::MPEG4AVCH264StereoHighProfileLevel42, &ts::MPEG4AVCH264StereoHighProfileLevel42);

		ident_to_ts.insert("HEVCH265MainProfileLevel51", &ts::HEVCH265MainProfileLevel51);
		id_to_ts.insert("1.2.840.10008.1.2.4.107", &ts::HEVCH265MainProfileLevel51);
		uid_to_ts.insert(&uids::HEVCH265MainProfileLevel51, &ts::HEVCH265MainProfileLevel51);

		ident_to_ts.insert("HEVCH265Main10ProfileLevel51", &ts::HEVCH265Main10ProfileLevel51);
		id_to_ts.insert("1.2.840.10008.1.2.4.108", &ts::HEVCH265Main10ProfileLevel51);
		uid_to_ts.insert(&uids::HEVCH265Main10ProfileLevel51, &ts::HEVCH265Main10ProfileLevel51);

		ident_to_ts.insert("RLELossless", &ts::RLELossless);
		id_to_ts.insert("1.2.840.10008.1.2.5", &ts::RLELossless);
		uid_to_ts.insert(&uids::RLELossless, &ts::RLELossless);

		ident_to_ts.insert("RFC2557MIMEencapsulation", &ts::RFC2557MIMEencapsulation);
		id_to_ts.insert("1.2.840.10008.1.2.6.1", &ts::RFC2557MIMEencapsulation);
		uid_to_ts.insert(&uids::RFC2557MIMEencapsulation, &ts::RFC2557MIMEencapsulation);

		ident_to_ts.insert("XMLEncoding", &ts::XMLEncoding);
		id_to_ts.insert("1.2.840.10008.1.2.6.2", &ts::XMLEncoding);
		uid_to_ts.insert(&uids::XMLEncoding, &ts::XMLEncoding);

		ident_to_ts.insert("Papyrus3ImplicitVRLittleEndian", &ts::Papyrus3ImplicitVRLittleEndian);
		id_to_ts.insert("1.2.840.10008.1.20", &ts::Papyrus3ImplicitVRLittleEndian);
		uid_to_ts.insert(&uids::Papyrus3ImplicitVRLittleEndian, &ts::Papyrus3ImplicitVRLittleEndian);


		TransferSyntaxLookup {
			ident_to_ts: ident_to_ts,
			id_to_ts: id_to_ts,
			uid_to_ts: uid_to_ts,
		}
	}
}
