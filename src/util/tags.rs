#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use util::vr::VR;

pub struct Tag {
    ident: &'static str,
    vr: &'static VR,
    tag: u32,
}


/// (0000,0000) VR=UL, VM=1 Command Group Length
pub const CommandGroupLength: u32 = 0x00000000;

/// (0000,0001) VR=UL, VM=1 Command Length to End (retired)
pub const CommandLengthToEnd: u32 = 0x00000001;

/// (0000,0002) VR=UI, VM=1 Affected SOP Class UID
pub const AffectedSOPClassUID: u32 = 0x00000002;

/// (0000,0003) VR=UI, VM=1 Requested SOP Class UID
pub const RequestedSOPClassUID: u32 = 0x00000003;

/// (0000,0010) VR=SH, VM=1 Command Recognition Code (retired)
pub const CommandRecognitionCode: u32 = 0x00000010;

/// (0000,0100) VR=US, VM=1 Command Field
pub const CommandField: u32 = 0x00000100;

/// (0000,0110) VR=US, VM=1 Message ID
pub const MessageID: u32 = 0x00000110;

/// (0000,0120) VR=US, VM=1 Message ID Being Responded To
pub const MessageIDBeingRespondedTo: u32 = 0x00000120;

/// (0000,0200) VR=AE, VM=1 Initiator (retired)
pub const Initiator: u32 = 0x00000200;

/// (0000,0300) VR=AE, VM=1 Receiver (retired)
pub const Receiver: u32 = 0x00000300;

/// (0000,0400) VR=AE, VM=1 Find Location (retired)
pub const FindLocation: u32 = 0x00000400;

/// (0000,0600) VR=AE, VM=1 Move Destination
pub const MoveDestination: u32 = 0x00000600;

/// (0000,0700) VR=US, VM=1 Priority
pub const Priority: u32 = 0x00000700;

/// (0000,0800) VR=US, VM=1 Command Data Set Type
pub const CommandDataSetType: u32 = 0x00000800;

/// (0000,0850) VR=US, VM=1 Number of Matches (retired)
pub const NumberOfMatches: u32 = 0x00000850;

/// (0000,0860) VR=US, VM=1 Response Sequence Number (retired)
pub const ResponseSequenceNumber: u32 = 0x00000860;

/// (0000,0900) VR=US, VM=1 Status
pub const Status: u32 = 0x00000900;

/// (0000,0901) VR=AT, VM=1-n Offending Element
pub const OffendingElement: u32 = 0x00000901;

/// (0000,0902) VR=LO, VM=1 Error Comment
pub const ErrorComment: u32 = 0x00000902;

/// (0000,0903) VR=US, VM=1 Error ID
pub const ErrorID: u32 = 0x00000903;

/// (0000,1000) VR=UI, VM=1 Affected SOP Instance UID
pub const AffectedSOPInstanceUID: u32 = 0x00001000;

/// (0000,1001) VR=UI, VM=1 Requested SOP Instance UID
pub const RequestedSOPInstanceUID: u32 = 0x00001001;

/// (0000,1002) VR=US, VM=1 Event Type ID
pub const EventTypeID: u32 = 0x00001002;

/// (0000,1005) VR=AT, VM=1-n Attribute Identifier List
pub const AttributeIdentifierList: u32 = 0x00001005;

/// (0000,1008) VR=US, VM=1 Action Type ID
pub const ActionTypeID: u32 = 0x00001008;

/// (0000,1020) VR=US, VM=1 Number of Remaining Sub-operations
pub const NumberOfRemainingSuboperations: u32 = 0x00001020;

/// (0000,1021) VR=US, VM=1 Number of Completed Sub-operations
pub const NumberOfCompletedSuboperations: u32 = 0x00001021;

/// (0000,1022) VR=US, VM=1 Number of Failed Sub-operations
pub const NumberOfFailedSuboperations: u32 = 0x00001022;

/// (0000,1023) VR=US, VM=1 Number of Warning Sub-operations
pub const NumberOfWarningSuboperations: u32 = 0x00001023;

/// (0000,1030) VR=AE, VM=1 Move Originator Application Entity Title
pub const MoveOriginatorApplicationEntityTitle: u32 = 0x00001030;

/// (0000,1031) VR=US, VM=1 Move Originator Message ID
pub const MoveOriginatorMessageID: u32 = 0x00001031;

/// (0000,4000) VR=LT, VM=1 Dialog Receiver (retired)
pub const DialogReceiver: u32 = 0x00004000;

/// (0000,4010) VR=LT, VM=1 Terminal Type (retired)
pub const TerminalType: u32 = 0x00004010;

/// (0000,5010) VR=SH, VM=1 Message Set ID (retired)
pub const MessageSetID: u32 = 0x00005010;

/// (0000,5020) VR=SH, VM=1 End Message ID (retired)
pub const EndMessageID: u32 = 0x00005020;

/// (0000,5110) VR=LT, VM=1 Display Format (retired)
pub const DisplayFormat: u32 = 0x00005110;

/// (0000,5120) VR=LT, VM=1 Page Position ID (retired)
pub const PagePositionID: u32 = 0x00005120;

/// (0000,5130) VR=CS, VM=1 Text Format ID (retired)
pub const TextFormatID: u32 = 0x00005130;

/// (0000,5140) VR=CS, VM=1 Normal/Reverse (retired)
pub const NormalReverse: u32 = 0x00005140;

/// (0000,5150) VR=CS, VM=1 Add Gray Scale (retired)
pub const AddGrayScale: u32 = 0x00005150;

/// (0000,5160) VR=CS, VM=1 Borders (retired)
pub const Borders: u32 = 0x00005160;

/// (0000,5170) VR=IS, VM=1 Copies (retired)
pub const Copies: u32 = 0x00005170;

/// (0000,5180) VR=CS, VM=1 Command Magnification Type (retired)
pub const CommandMagnificationType: u32 = 0x00005180;

/// (0000,5190) VR=CS, VM=1 Erase (retired)
pub const Erase: u32 = 0x00005190;

/// (0000,51A0) VR=CS, VM=1 Print (retired)
pub const Print: u32 = 0x000051A0;

/// (0000,51B0) VR=US, VM=1-n Overlays (retired)
pub const Overlays: u32 = 0x000051B0;

/// (0002,0000) VR=UL, VM=1 File Meta Information Group Length
pub const FileMetaInformationGroupLength: u32 = 0x00020000;

/// (0002,0001) VR=OB, VM=1 File Meta Information Version
pub const FileMetaInformationVersion: u32 = 0x00020001;

/// (0002,0002) VR=UI, VM=1 Media Storage SOP Class UID
pub const MediaStorageSOPClassUID: u32 = 0x00020002;

/// (0002,0003) VR=UI, VM=1 Media Storage SOP Instance UID
pub const MediaStorageSOPInstanceUID: u32 = 0x00020003;

/// (0002,0010) VR=UI, VM=1 Transfer Syntax UID
pub const TransferSyntaxUID: u32 = 0x00020010;

/// (0002,0012) VR=UI, VM=1 Implementation Class UID
pub const ImplementationClassUID: u32 = 0x00020012;

/// (0002,0013) VR=SH, VM=1 Implementation Version Name
pub const ImplementationVersionName: u32 = 0x00020013;

/// (0002,0016) VR=AE, VM=1 Source Application Entity Title
pub const SourceApplicationEntityTitle: u32 = 0x00020016;

/// (0002,0100) VR=UI, VM=1 Private Information Creator UID
pub const PrivateInformationCreatorUID: u32 = 0x00020100;

/// (0002,0102) VR=OB, VM=1 Private Information
pub const PrivateInformation: u32 = 0x00020102;

/// (0004,1130) VR=CS, VM=1 File-set ID
pub const FileSetID: u32 = 0x00041130;

/// (0004,1141) VR=CS, VM=1-8 File-set Descriptor File ID
pub const FileSetDescriptorFileID: u32 = 0x00041141;

/// (0004,1142) VR=CS, VM=1 Specific Character Set of File-set Descriptor File
pub const SpecificCharacterSetOfFileSetDescriptorFile: u32 = 0x00041142;

/// (0004,1200) VR=UL, VM=1 Offset of the First Directory Record of the Root Directory Entity
pub const OffsetOfTheFirstDirectoryRecordOfTheRootDirectoryEntity: u32 = 0x00041200;

/// (0004,1202) VR=UL, VM=1 Offset of the Last Directory Record of the Root Directory Entity
pub const OffsetOfTheLastDirectoryRecordOfTheRootDirectoryEntity: u32 = 0x00041202;

/// (0004,1212) VR=US, VM=1 File-set Consistency Flag
pub const FileSetConsistencyFlag: u32 = 0x00041212;

/// (0004,1220) VR=SQ, VM=1 Directory Record Sequence
pub const DirectoryRecordSequence: u32 = 0x00041220;

/// (0004,1400) VR=UL, VM=1 Offset of the Next Directory Record
pub const OffsetOfTheNextDirectoryRecord: u32 = 0x00041400;

/// (0004,1410) VR=US, VM=1 Record In-use Flag
pub const RecordInUseFlag: u32 = 0x00041410;

/// (0004,1420) VR=UL, VM=1 Offset of Referenced Lower-Level Directory Entity
pub const OffsetOfReferencedLowerLevelDirectoryEntity: u32 = 0x00041420;

/// (0004,1430) VR=CS, VM=1 Directory Record Type
pub const DirectoryRecordType: u32 = 0x00041430;

/// (0004,1432) VR=UI, VM=1 Private Record UID
pub const PrivateRecordUID: u32 = 0x00041432;

/// (0004,1500) VR=CS, VM=1-8 Referenced File ID
pub const ReferencedFileID: u32 = 0x00041500;

/// (0004,1504) VR=UL, VM=1 MRDR Directory Record Offset (retired)
pub const MRDRDirectoryRecordOffset: u32 = 0x00041504;

/// (0004,1510) VR=UI, VM=1 Referenced SOP Class UID in File
pub const ReferencedSOPClassUIDInFile: u32 = 0x00041510;

/// (0004,1511) VR=UI, VM=1 Referenced SOP Instance UID in File
pub const ReferencedSOPInstanceUIDInFile: u32 = 0x00041511;

/// (0004,1512) VR=UI, VM=1 Referenced Transfer Syntax UID in File
pub const ReferencedTransferSyntaxUIDInFile: u32 = 0x00041512;

/// (0004,151A) VR=UI, VM=1-n Referenced Related General SOP Class UID in File
pub const ReferencedRelatedGeneralSOPClassUIDInFile: u32 = 0x0004151A;

/// (0004,1600) VR=UL, VM=1 Number of References (retired)
pub const NumberOfReferences: u32 = 0x00041600;

/// (0008,0001) VR=UL, VM=1 Length to End (retired)
pub const LengthToEnd: u32 = 0x00080001;

/// (0008,0005) VR=CS, VM=1-n Specific Character Set
pub const SpecificCharacterSet: u32 = 0x00080005;

/// (0008,0006) VR=SQ, VM=1 Language Code Sequence
pub const LanguageCodeSequence: u32 = 0x00080006;

/// (0008,0008) VR=CS, VM=2-n Image Type
pub const ImageType: u32 = 0x00080008;

/// (0008,0010) VR=SH, VM=1 Recognition Code (retired)
pub const RecognitionCode: u32 = 0x00080010;

/// (0008,0012) VR=DA, VM=1 Instance Creation Date
pub const InstanceCreationDate: u32 = 0x00080012;

/// (0008,0013) VR=TM, VM=1 Instance Creation Time
pub const InstanceCreationTime: u32 = 0x00080013;

/// (0008,0014) VR=UI, VM=1 Instance Creator UID
pub const InstanceCreatorUID: u32 = 0x00080014;

/// (0008,0016) VR=UI, VM=1 SOP Class UID
pub const SOPClassUID: u32 = 0x00080016;

/// (0008,0018) VR=UI, VM=1 SOP Instance UID
pub const SOPInstanceUID: u32 = 0x00080018;

/// (0008,001A) VR=UI, VM=1-n Related General SOP Class UID
pub const RelatedGeneralSOPClassUID: u32 = 0x0008001A;

/// (0008,001B) VR=UI, VM=1 Original Specialized SOP Class UID
pub const OriginalSpecializedSOPClassUID: u32 = 0x0008001B;

/// (0008,0020) VR=DA, VM=1 Study Date
pub const StudyDate: u32 = 0x00080020;

/// (0008,0021) VR=DA, VM=1 Series Date
pub const SeriesDate: u32 = 0x00080021;

/// (0008,0022) VR=DA, VM=1 Acquisition Date
pub const AcquisitionDate: u32 = 0x00080022;

/// (0008,0023) VR=DA, VM=1 Content Date
pub const ContentDate: u32 = 0x00080023;

/// (0008,0024) VR=DA, VM=1 Overlay Date (retired)
pub const OverlayDate: u32 = 0x00080024;

/// (0008,0025) VR=DA, VM=1 Curve Date (retired)
pub const CurveDate: u32 = 0x00080025;

/// (0008,002A) VR=DT, VM=1 Acquisition DateTime
pub const AcquisitionDateTime: u32 = 0x0008002A;

/// (0008,0030) VR=TM, VM=1 Study Time
pub const StudyTime: u32 = 0x00080030;

/// (0008,0031) VR=TM, VM=1 Series Time
pub const SeriesTime: u32 = 0x00080031;

/// (0008,0032) VR=TM, VM=1 Acquisition Time
pub const AcquisitionTime: u32 = 0x00080032;

/// (0008,0033) VR=TM, VM=1 Content Time
pub const ContentTime: u32 = 0x00080033;

/// (0008,0034) VR=TM, VM=1 Overlay Time (retired)
pub const OverlayTime: u32 = 0x00080034;

/// (0008,0035) VR=TM, VM=1 Curve Time (retired)
pub const CurveTime: u32 = 0x00080035;

/// (0008,0040) VR=US, VM=1 Data Set Type (retired)
pub const DataSetType: u32 = 0x00080040;

/// (0008,0041) VR=LO, VM=1 Data Set Subtype (retired)
pub const DataSetSubtype: u32 = 0x00080041;

/// (0008,0042) VR=CS, VM=1 Nuclear Medicine Series Type (retired)
pub const NuclearMedicineSeriesType: u32 = 0x00080042;

/// (0008,0050) VR=SH, VM=1 Accession Number
pub const AccessionNumber: u32 = 0x00080050;

/// (0008,0051) VR=SQ, VM=1 Issuer of Accession Number Sequence
pub const IssuerOfAccessionNumberSequence: u32 = 0x00080051;

/// (0008,0052) VR=CS, VM=1 Query/Retrieve Level
pub const QueryRetrieveLevel: u32 = 0x00080052;

/// (0008,0054) VR=AE, VM=1-n Retrieve AE Title
pub const RetrieveAETitle: u32 = 0x00080054;

/// (0008,0056) VR=CS, VM=1 Instance Availability
pub const InstanceAvailability: u32 = 0x00080056;

/// (0008,0058) VR=UI, VM=1-n Failed SOP Instance UID List
pub const FailedSOPInstanceUIDList: u32 = 0x00080058;

/// (0008,0060) VR=CS, VM=1 Modality
pub const Modality: u32 = 0x00080060;

/// (0008,0061) VR=CS, VM=1-n Modalities in Study
pub const ModalitiesInStudy: u32 = 0x00080061;

/// (0008,0062) VR=UI, VM=1-n SOP Classes in Study
pub const SOPClassesInStudy: u32 = 0x00080062;

/// (0008,0064) VR=CS, VM=1 Conversion Type
pub const ConversionType: u32 = 0x00080064;

/// (0008,0068) VR=CS, VM=1 Presentation Intent Type
pub const PresentationIntentType: u32 = 0x00080068;

/// (0008,0070) VR=LO, VM=1 Manufacturer
pub const Manufacturer: u32 = 0x00080070;

/// (0008,0080) VR=LO, VM=1 Institution Name
pub const InstitutionName: u32 = 0x00080080;

/// (0008,0081) VR=ST, VM=1 Institution Address
pub const InstitutionAddress: u32 = 0x00080081;

/// (0008,0082) VR=SQ, VM=1 Institution Code Sequence
pub const InstitutionCodeSequence: u32 = 0x00080082;

/// (0008,0090) VR=PN, VM=1 Referring Physician’s Name
pub const ReferringPhysicianName: u32 = 0x00080090;

/// (0008,0092) VR=ST, VM=1 Referring Physician’s Address
pub const ReferringPhysicianAddress: u32 = 0x00080092;

/// (0008,0094) VR=SH, VM=1-n Referring Physician’s Telephone Numbers
pub const ReferringPhysicianTelephoneNumbers: u32 = 0x00080094;

/// (0008,0096) VR=SQ, VM=1 Referring Physician Identification Sequence
pub const ReferringPhysicianIdentificationSequence: u32 = 0x00080096;

/// (0008,0100) VR=SH, VM=1 Code Value
pub const CodeValue: u32 = 0x00080100;

/// (0008,0102) VR=SH, VM=1 Coding Scheme Designator
pub const CodingSchemeDesignator: u32 = 0x00080102;

/// (0008,0103) VR=SH, VM=1 Coding Scheme Version
pub const CodingSchemeVersion: u32 = 0x00080103;

/// (0008,0104) VR=LO, VM=1 Code Meaning
pub const CodeMeaning: u32 = 0x00080104;

/// (0008,0105) VR=CS, VM=1 Mapping Resource
pub const MappingResource: u32 = 0x00080105;

/// (0008,0106) VR=DT, VM=1 Context Group Version
pub const ContextGroupVersion: u32 = 0x00080106;

/// (0008,0107) VR=DT, VM=1 Context Group Local Version
pub const ContextGroupLocalVersion: u32 = 0x00080107;

/// (0008,010B) VR=CS, VM=1 Context Group Extension Flag
pub const ContextGroupExtensionFlag: u32 = 0x0008010B;

/// (0008,010C) VR=UI, VM=1 Coding Scheme UID
pub const CodingSchemeUID: u32 = 0x0008010C;

/// (0008,010D) VR=UI, VM=1 Context Group Extension Creator UID
pub const ContextGroupExtensionCreatorUID: u32 = 0x0008010D;

/// (0008,010F) VR=CS, VM=1 Context Identifier
pub const ContextIdentifier: u32 = 0x0008010F;

/// (0008,0110) VR=SQ, VM=1 Coding Scheme Identification Sequence
pub const CodingSchemeIdentificationSequence: u32 = 0x00080110;

/// (0008,0112) VR=LO, VM=1 Coding Scheme Registry
pub const CodingSchemeRegistry: u32 = 0x00080112;

/// (0008,0114) VR=ST, VM=1 Coding Scheme External ID
pub const CodingSchemeExternalID: u32 = 0x00080114;

/// (0008,0115) VR=ST, VM=1 Coding Scheme Name
pub const CodingSchemeName: u32 = 0x00080115;

/// (0008,0116) VR=ST, VM=1 Coding Scheme Responsible Organization
pub const CodingSchemeResponsibleOrganization: u32 = 0x00080116;

/// (0008,0117) VR=UI, VM=1 Context UID
pub const ContextUID: u32 = 0x00080117;

/// (0008,0201) VR=SH, VM=1 Timezone Offset From UTC
pub const TimezoneOffsetFromUTC: u32 = 0x00080201;

/// (0008,1000) VR=AE, VM=1 Network ID (retired)
pub const NetworkID: u32 = 0x00081000;

/// (0008,1010) VR=SH, VM=1 Station Name
pub const StationName: u32 = 0x00081010;

/// (0008,1030) VR=LO, VM=1 Study Description
pub const StudyDescription: u32 = 0x00081030;

/// (0008,1032) VR=SQ, VM=1 Procedure Code Sequence
pub const ProcedureCodeSequence: u32 = 0x00081032;

/// (0008,103E) VR=LO, VM=1 Series Description
pub const SeriesDescription: u32 = 0x0008103E;

/// (0008,103F) VR=SQ, VM=1 Series Description Code Sequence
pub const SeriesDescriptionCodeSequence: u32 = 0x0008103F;

/// (0008,1040) VR=LO, VM=1 Institutional Department Name
pub const InstitutionalDepartmentName: u32 = 0x00081040;

/// (0008,1048) VR=PN, VM=1-n Physician(s) of Record
pub const PhysiciansOfRecord: u32 = 0x00081048;

/// (0008,1049) VR=SQ, VM=1 Physician(s) of Record Identification Sequence
pub const PhysiciansOfRecordIdentificationSequence: u32 = 0x00081049;

/// (0008,1050) VR=PN, VM=1-n Performing Physician’s Name
pub const PerformingPhysicianName: u32 = 0x00081050;

/// (0008,1052) VR=SQ, VM=1 Performing Physician Identification Sequence
pub const PerformingPhysicianIdentificationSequence: u32 = 0x00081052;

/// (0008,1060) VR=PN, VM=1-n Name of Physician(s) Reading Study
pub const NameOfPhysiciansReadingStudy: u32 = 0x00081060;

/// (0008,1062) VR=SQ, VM=1 Physician(s) Reading Study Identification Sequence
pub const PhysiciansReadingStudyIdentificationSequence: u32 = 0x00081062;

/// (0008,1070) VR=PN, VM=1-n Operators’ Name
pub const OperatorsName: u32 = 0x00081070;

/// (0008,1072) VR=SQ, VM=1 Operator Identification Sequence
pub const OperatorIdentificationSequence: u32 = 0x00081072;

/// (0008,1080) VR=LO, VM=1-n Admitting Diagnoses Description
pub const AdmittingDiagnosesDescription: u32 = 0x00081080;

/// (0008,1084) VR=SQ, VM=1 Admitting Diagnoses Code Sequence
pub const AdmittingDiagnosesCodeSequence: u32 = 0x00081084;

/// (0008,1090) VR=LO, VM=1 Manufacturer’s Model Name
pub const ManufacturerModelName: u32 = 0x00081090;

/// (0008,1100) VR=SQ, VM=1 Referenced Results Sequence (retired)
pub const ReferencedResultsSequence: u32 = 0x00081100;

/// (0008,1110) VR=SQ, VM=1 Referenced Study Sequence
pub const ReferencedStudySequence: u32 = 0x00081110;

/// (0008,1111) VR=SQ, VM=1 Referenced Performed Procedure Step Sequence
pub const ReferencedPerformedProcedureStepSequence: u32 = 0x00081111;

/// (0008,1115) VR=SQ, VM=1 Referenced Series Sequence
pub const ReferencedSeriesSequence: u32 = 0x00081115;

/// (0008,1120) VR=SQ, VM=1 Referenced Patient Sequence
pub const ReferencedPatientSequence: u32 = 0x00081120;

/// (0008,1125) VR=SQ, VM=1 Referenced Visit Sequence
pub const ReferencedVisitSequence: u32 = 0x00081125;

/// (0008,1130) VR=SQ, VM=1 Referenced Overlay Sequence (retired)
pub const ReferencedOverlaySequence: u32 = 0x00081130;

/// (0008,1134) VR=SQ, VM=1 Referenced Stereometric Instance Sequence
pub const ReferencedStereometricInstanceSequence: u32 = 0x00081134;

/// (0008,113A) VR=SQ, VM=1 Referenced Waveform Sequence
pub const ReferencedWaveformSequence: u32 = 0x0008113A;

/// (0008,1140) VR=SQ, VM=1 Referenced Image Sequence
pub const ReferencedImageSequence: u32 = 0x00081140;

/// (0008,1145) VR=SQ, VM=1 Referenced Curve Sequence (retired)
pub const ReferencedCurveSequence: u32 = 0x00081145;

/// (0008,114A) VR=SQ, VM=1 Referenced Instance Sequence
pub const ReferencedInstanceSequence: u32 = 0x0008114A;

/// (0008,114B) VR=SQ, VM=1 Referenced Real World Value Mapping Instance Sequence
pub const ReferencedRealWorldValueMappingInstanceSequence: u32 = 0x0008114B;

/// (0008,1150) VR=UI, VM=1 Referenced SOP Class UID
pub const ReferencedSOPClassUID: u32 = 0x00081150;

/// (0008,1155) VR=UI, VM=1 Referenced SOP Instance UID
pub const ReferencedSOPInstanceUID: u32 = 0x00081155;

/// (0008,115A) VR=UI, VM=1-n SOP Classes Supported
pub const SOPClassesSupported: u32 = 0x0008115A;

/// (0008,1160) VR=IS, VM=1-n Referenced Frame Number
pub const ReferencedFrameNumber: u32 = 0x00081160;

/// (0008,1161) VR=UL, VM=1-n Simple Frame List
pub const SimpleFrameList: u32 = 0x00081161;

/// (0008,1162) VR=UL, VM=3-3n Calculated Frame List
pub const CalculatedFrameList: u32 = 0x00081162;

/// (0008,1163) VR=FD, VM=2 Time Range
pub const TimeRange: u32 = 0x00081163;

/// (0008,1164) VR=SQ, VM=1 Frame Extraction Sequence
pub const FrameExtractionSequence: u32 = 0x00081164;

/// (0008,1167) VR=UI, VM=1 Multi-Frame Source SOP Instance UID
pub const MultiFrameSourceSOPInstanceUID: u32 = 0x00081167;

/// (0008,1195) VR=UI, VM=1 Transaction UID
pub const TransactionUID: u32 = 0x00081195;

/// (0008,1197) VR=US, VM=1 Failure Reason
pub const FailureReason: u32 = 0x00081197;

/// (0008,1198) VR=SQ, VM=1 Failed SOP Sequence
pub const FailedSOPSequence: u32 = 0x00081198;

/// (0008,1199) VR=SQ, VM=1 Referenced SOP Sequence
pub const ReferencedSOPSequence: u32 = 0x00081199;

/// (0008,1200) VR=SQ, VM=1 Studies Containing Other Referenced Instances Sequence
pub const StudiesContainingOtherReferencedInstancesSequence: u32 = 0x00081200;

/// (0008,1250) VR=SQ, VM=1 Related Series Sequence
pub const RelatedSeriesSequence: u32 = 0x00081250;

/// (0008,2110) VR=CS, VM=1 Lossy Image Compression (Retired) (retired)
pub const LossyImageCompressionRetired: u32 = 0x00082110;

/// (0008,2111) VR=ST, VM=1 Derivation Description
pub const DerivationDescription: u32 = 0x00082111;

/// (0008,2112) VR=SQ, VM=1 Source Image Sequence
pub const SourceImageSequence: u32 = 0x00082112;

/// (0008,2120) VR=SH, VM=1 Stage Name
pub const StageName: u32 = 0x00082120;

/// (0008,2122) VR=IS, VM=1 Stage Number
pub const StageNumber: u32 = 0x00082122;

/// (0008,2124) VR=IS, VM=1 Number of Stages
pub const NumberOfStages: u32 = 0x00082124;

/// (0008,2127) VR=SH, VM=1 View Name
pub const ViewName: u32 = 0x00082127;

/// (0008,2128) VR=IS, VM=1 View Number
pub const ViewNumber: u32 = 0x00082128;

/// (0008,2129) VR=IS, VM=1 Number of Event Timers
pub const NumberOfEventTimers: u32 = 0x00082129;

/// (0008,212A) VR=IS, VM=1 Number of Views in Stage
pub const NumberOfViewsInStage: u32 = 0x0008212A;

/// (0008,2130) VR=DS, VM=1-n Event Elapsed Time(s)
pub const EventElapsedTimes: u32 = 0x00082130;

/// (0008,2132) VR=LO, VM=1-n Event Timer Name(s)
pub const EventTimerNames: u32 = 0x00082132;

/// (0008,2133) VR=SQ, VM=1 Event Timer Sequence
pub const EventTimerSequence: u32 = 0x00082133;

/// (0008,2134) VR=FD, VM=1 Event Time Offset
pub const EventTimeOffset: u32 = 0x00082134;

/// (0008,2135) VR=SQ, VM=1 Event Code Sequence
pub const EventCodeSequence: u32 = 0x00082135;

/// (0008,2142) VR=IS, VM=1 Start Trim
pub const StartTrim: u32 = 0x00082142;

/// (0008,2143) VR=IS, VM=1 Stop Trim
pub const StopTrim: u32 = 0x00082143;

/// (0008,2144) VR=IS, VM=1 Recommended Display Frame Rate
pub const RecommendedDisplayFrameRate: u32 = 0x00082144;

/// (0008,2200) VR=CS, VM=1 Transducer Position (retired)
pub const TransducerPosition: u32 = 0x00082200;

/// (0008,2204) VR=CS, VM=1 Transducer Orientation (retired)
pub const TransducerOrientation: u32 = 0x00082204;

/// (0008,2208) VR=CS, VM=1 Anatomic Structure (retired)
pub const AnatomicStructure: u32 = 0x00082208;

/// (0008,2218) VR=SQ, VM=1 Anatomic Region Sequence
pub const AnatomicRegionSequence: u32 = 0x00082218;

/// (0008,2220) VR=SQ, VM=1 Anatomic Region Modifier Sequence
pub const AnatomicRegionModifierSequence: u32 = 0x00082220;

/// (0008,2228) VR=SQ, VM=1 Primary Anatomic Structure Sequence
pub const PrimaryAnatomicStructureSequence: u32 = 0x00082228;

/// (0008,2229) VR=SQ, VM=1 Anatomic Structure, Space or Region Sequence
pub const AnatomicStructureSpaceOrRegionSequence: u32 = 0x00082229;

/// (0008,2230) VR=SQ, VM=1 Primary Anatomic Structure Modifier Sequence
pub const PrimaryAnatomicStructureModifierSequence: u32 = 0x00082230;

/// (0008,2240) VR=SQ, VM=1 Transducer Position Sequence (retired)
pub const TransducerPositionSequence: u32 = 0x00082240;

/// (0008,2242) VR=SQ, VM=1 Transducer Position Modifier Sequence (retired)
pub const TransducerPositionModifierSequence: u32 = 0x00082242;

/// (0008,2244) VR=SQ, VM=1 Transducer Orientation Sequence (retired)
pub const TransducerOrientationSequence: u32 = 0x00082244;

/// (0008,2246) VR=SQ, VM=1 Transducer Orientation Modifier Sequence (retired)
pub const TransducerOrientationModifierSequence: u32 = 0x00082246;

/// (0008,2251) VR=SQ, VM=1 Anatomic Structure Space Or Region Code Sequence (Trial) (retired)
pub const AnatomicStructureSpaceOrRegionCodeSequenceTrial: u32 = 0x00082251;

/// (0008,2253) VR=SQ, VM=1 Anatomic Portal Of Entrance Code Sequence (Trial) (retired)
pub const AnatomicPortalOfEntranceCodeSequenceTrial: u32 = 0x00082253;

/// (0008,2255) VR=SQ, VM=1 Anatomic Approach Direction Code Sequence (Trial) (retired)
pub const AnatomicApproachDirectionCodeSequenceTrial: u32 = 0x00082255;

/// (0008,2256) VR=ST, VM=1 Anatomic Perspective Description (Trial) (retired)
pub const AnatomicPerspectiveDescriptionTrial: u32 = 0x00082256;

/// (0008,2257) VR=SQ, VM=1 Anatomic Perspective Code Sequence (Trial) (retired)
pub const AnatomicPerspectiveCodeSequenceTrial: u32 = 0x00082257;

/// (0008,2258) VR=ST, VM=1 Anatomic Location Of Examining Instrument Description (Trial) (retired)
pub const AnatomicLocationOfExaminingInstrumentDescriptionTrial: u32 = 0x00082258;

/// (0008,2259) VR=SQ, VM=1 Anatomic Location Of Examining Instrument Code Sequence (Trial) (retired)
pub const AnatomicLocationOfExaminingInstrumentCodeSequenceTrial: u32 = 0x00082259;

/// (0008,225A) VR=SQ, VM=1 Anatomic Structure Space Or Region Modifier Code Sequence (Trial) (retired)
pub const AnatomicStructureSpaceOrRegionModifierCodeSequenceTrial: u32 = 0x0008225A;

/// (0008,225C) VR=SQ, VM=1 OnAxis Background Anatomic Structure Code Sequence (Trial) (retired)
pub const OnAxisBackgroundAnatomicStructureCodeSequenceTrial: u32 = 0x0008225C;

/// (0008,3001) VR=SQ, VM=1 Alternate Representation Sequence
pub const AlternateRepresentationSequence: u32 = 0x00083001;

/// (0008,3010) VR=UI, VM=1 Irradiation Event UID
pub const IrradiationEventUID: u32 = 0x00083010;

/// (0008,4000) VR=LT, VM=1 Identifying Comments (retired)
pub const IdentifyingComments: u32 = 0x00084000;

/// (0008,9007) VR=CS, VM=4 Frame Type
pub const FrameType: u32 = 0x00089007;

/// (0008,9092) VR=SQ, VM=1 Referenced Image Evidence Sequence
pub const ReferencedImageEvidenceSequence: u32 = 0x00089092;

/// (0008,9121) VR=SQ, VM=1 Referenced Raw Data Sequence
pub const ReferencedRawDataSequence: u32 = 0x00089121;

/// (0008,9123) VR=UI, VM=1 Creator-Version UID
pub const CreatorVersionUID: u32 = 0x00089123;

/// (0008,9124) VR=SQ, VM=1 Derivation Image Sequence
pub const DerivationImageSequence: u32 = 0x00089124;

/// (0008,9154) VR=SQ, VM=1 Source Image Evidence Sequence
pub const SourceImageEvidenceSequence: u32 = 0x00089154;

/// (0008,9205) VR=CS, VM=1 Pixel Presentation
pub const PixelPresentation: u32 = 0x00089205;

/// (0008,9206) VR=CS, VM=1 Volumetric Properties
pub const VolumetricProperties: u32 = 0x00089206;

/// (0008,9207) VR=CS, VM=1 Volume Based Calculation Technique
pub const VolumeBasedCalculationTechnique: u32 = 0x00089207;

/// (0008,9208) VR=CS, VM=1 Complex Image Component
pub const ComplexImageComponent: u32 = 0x00089208;

/// (0008,9209) VR=CS, VM=1 Acquisition Contrast
pub const AcquisitionContrast: u32 = 0x00089209;

/// (0008,9215) VR=SQ, VM=1 Derivation Code Sequence
pub const DerivationCodeSequence: u32 = 0x00089215;

/// (0008,9237) VR=SQ, VM=1 Referenced Presentation State Sequence
pub const ReferencedPresentationStateSequence: u32 = 0x00089237;

/// (0008,9410) VR=SQ, VM=1 Referenced Other Plane Sequence
pub const ReferencedOtherPlaneSequence: u32 = 0x00089410;

/// (0008,9458) VR=SQ, VM=1 Frame Display Sequence
pub const FrameDisplaySequence: u32 = 0x00089458;

/// (0008,9459) VR=FL, VM=1 Recommended Display Frame Rate in Float
pub const RecommendedDisplayFrameRateInFloat: u32 = 0x00089459;

/// (0008,9460) VR=CS, VM=1 Skip Frame Range Flag
pub const SkipFrameRangeFlag: u32 = 0x00089460;

/// (0010,0010) VR=PN, VM=1 Patient’s Name
pub const PatientName: u32 = 0x00100010;

/// (0010,0020) VR=LO, VM=1 Patient ID
pub const PatientID: u32 = 0x00100020;

/// (0010,0021) VR=LO, VM=1 Issuer of Patient ID
pub const IssuerOfPatientID: u32 = 0x00100021;

/// (0010,0022) VR=CS, VM=1 Type of Patient ID
pub const TypeOfPatientID: u32 = 0x00100022;

/// (0010,0024) VR=SQ, VM=1 Issuer of Patient ID Qualifiers Sequence
pub const IssuerOfPatientIDQualifiersSequence: u32 = 0x00100024;

/// (0010,0030) VR=DA, VM=1 Patient’s Birth Date
pub const PatientBirthDate: u32 = 0x00100030;

/// (0010,0032) VR=TM, VM=1 Patient’s Birth Time
pub const PatientBirthTime: u32 = 0x00100032;

/// (0010,0040) VR=CS, VM=1 Patient’s Sex
pub const PatientSex: u32 = 0x00100040;

/// (0010,0050) VR=SQ, VM=1 Patient’s Insurance Plan Code Sequence
pub const PatientInsurancePlanCodeSequence: u32 = 0x00100050;

/// (0010,0101) VR=SQ, VM=1 Patient’s Primary Language Code Sequence
pub const PatientPrimaryLanguageCodeSequence: u32 = 0x00100101;

/// (0010,0102) VR=SQ, VM=1 Patient’s Primary Language Modifier Code Sequence
pub const PatientPrimaryLanguageModifierCodeSequence: u32 = 0x00100102;

/// (0010,1000) VR=LO, VM=1-n Other Patient IDs
pub const OtherPatientIDs: u32 = 0x00101000;

/// (0010,1001) VR=PN, VM=1-n Other Patient Names
pub const OtherPatientNames: u32 = 0x00101001;

/// (0010,1002) VR=SQ, VM=1 Other Patient IDs Sequence
pub const OtherPatientIDsSequence: u32 = 0x00101002;

/// (0010,1005) VR=PN, VM=1 Patient’s Birth Name
pub const PatientBirthName: u32 = 0x00101005;

/// (0010,1010) VR=AS, VM=1 Patient’s Age
pub const PatientAge: u32 = 0x00101010;

/// (0010,1020) VR=DS, VM=1 Patient’s Size
pub const PatientSize: u32 = 0x00101020;

/// (0010,1021) VR=SQ, VM=1 Patient’s Size Code Sequence
pub const PatientSizeCodeSequence: u32 = 0x00101021;

/// (0010,1030) VR=DS, VM=1 Patient’s Weight
pub const PatientWeight: u32 = 0x00101030;

/// (0010,1040) VR=LO, VM=1 Patient’s Address
pub const PatientAddress: u32 = 0x00101040;

/// (0010,1050) VR=LO, VM=1-n Insurance Plan Identification (retired)
pub const InsurancePlanIdentification: u32 = 0x00101050;

/// (0010,1060) VR=PN, VM=1 Patient’s Mother’s Birth Name
pub const PatientMotherBirthName: u32 = 0x00101060;

/// (0010,1080) VR=LO, VM=1 Military Rank
pub const MilitaryRank: u32 = 0x00101080;

/// (0010,1081) VR=LO, VM=1 Branch of Service
pub const BranchOfService: u32 = 0x00101081;

/// (0010,1090) VR=LO, VM=1 Medical Record Locator
pub const MedicalRecordLocator: u32 = 0x00101090;

/// (0010,2000) VR=LO, VM=1-n Medical Alerts
pub const MedicalAlerts: u32 = 0x00102000;

/// (0010,2110) VR=LO, VM=1-n Allergies
pub const Allergies: u32 = 0x00102110;

/// (0010,2150) VR=LO, VM=1 Country of Residence
pub const CountryOfResidence: u32 = 0x00102150;

/// (0010,2152) VR=LO, VM=1 Region of Residence
pub const RegionOfResidence: u32 = 0x00102152;

/// (0010,2154) VR=SH, VM=1-n Patient’s Telephone Numbers
pub const PatientTelephoneNumbers: u32 = 0x00102154;

/// (0010,2160) VR=SH, VM=1 Ethnic Group
pub const EthnicGroup: u32 = 0x00102160;

/// (0010,2180) VR=SH, VM=1 Occupation
pub const Occupation: u32 = 0x00102180;

/// (0010,21A0) VR=CS, VM=1 Smoking Status
pub const SmokingStatus: u32 = 0x001021A0;

/// (0010,21B0) VR=LT, VM=1 Additional Patient History
pub const AdditionalPatientHistory: u32 = 0x001021B0;

/// (0010,21C0) VR=US, VM=1 Pregnancy Status
pub const PregnancyStatus: u32 = 0x001021C0;

/// (0010,21D0) VR=DA, VM=1 Last Menstrual Date
pub const LastMenstrualDate: u32 = 0x001021D0;

/// (0010,21F0) VR=LO, VM=1 Patient’s Religious Preference
pub const PatientReligiousPreference: u32 = 0x001021F0;

/// (0010,2201) VR=LO, VM=1 Patient Species Description
pub const PatientSpeciesDescription: u32 = 0x00102201;

/// (0010,2202) VR=SQ, VM=1 Patient Species Code Sequence
pub const PatientSpeciesCodeSequence: u32 = 0x00102202;

/// (0010,2203) VR=CS, VM=1 Patient’s Sex Neutered
pub const PatientSexNeutered: u32 = 0x00102203;

/// (0010,2210) VR=CS, VM=1 Anatomical Orientation Type
pub const AnatomicalOrientationType: u32 = 0x00102210;

/// (0010,2292) VR=LO, VM=1 Patient Breed Description
pub const PatientBreedDescription: u32 = 0x00102292;

/// (0010,2293) VR=SQ, VM=1 Patient Breed Code Sequence
pub const PatientBreedCodeSequence: u32 = 0x00102293;

/// (0010,2294) VR=SQ, VM=1 Breed Registration Sequence
pub const BreedRegistrationSequence: u32 = 0x00102294;

/// (0010,2295) VR=LO, VM=1 Breed Registration Number
pub const BreedRegistrationNumber: u32 = 0x00102295;

/// (0010,2296) VR=SQ, VM=1 Breed Registry Code Sequence
pub const BreedRegistryCodeSequence: u32 = 0x00102296;

/// (0010,2297) VR=PN, VM=1 Responsible Person
pub const ResponsiblePerson: u32 = 0x00102297;

/// (0010,2298) VR=CS, VM=1 Responsible Person Role
pub const ResponsiblePersonRole: u32 = 0x00102298;

/// (0010,2299) VR=LO, VM=1 Responsible Organization
pub const ResponsibleOrganization: u32 = 0x00102299;

/// (0010,4000) VR=LT, VM=1 Patient Comments
pub const PatientComments: u32 = 0x00104000;

/// (0010,9431) VR=FL, VM=1 Examined Body Thickness
pub const ExaminedBodyThickness: u32 = 0x00109431;

/// (0012,0010) VR=LO, VM=1 Clinical Trial Sponsor Name
pub const ClinicalTrialSponsorName: u32 = 0x00120010;

/// (0012,0020) VR=LO, VM=1 Clinical Trial Protocol ID
pub const ClinicalTrialProtocolID: u32 = 0x00120020;

/// (0012,0021) VR=LO, VM=1 Clinical Trial Protocol Name
pub const ClinicalTrialProtocolName: u32 = 0x00120021;

/// (0012,0030) VR=LO, VM=1 Clinical Trial Site ID
pub const ClinicalTrialSiteID: u32 = 0x00120030;

/// (0012,0031) VR=LO, VM=1 Clinical Trial Site Name
pub const ClinicalTrialSiteName: u32 = 0x00120031;

/// (0012,0040) VR=LO, VM=1 Clinical Trial Subject ID
pub const ClinicalTrialSubjectID: u32 = 0x00120040;

/// (0012,0042) VR=LO, VM=1 Clinical Trial Subject Reading ID
pub const ClinicalTrialSubjectReadingID: u32 = 0x00120042;

/// (0012,0050) VR=LO, VM=1 Clinical Trial Time Point ID
pub const ClinicalTrialTimePointID: u32 = 0x00120050;

/// (0012,0051) VR=ST, VM=1 Clinical Trial Time Point Description
pub const ClinicalTrialTimePointDescription: u32 = 0x00120051;

/// (0012,0060) VR=LO, VM=1 Clinical Trial Coordinating Center Name
pub const ClinicalTrialCoordinatingCenterName: u32 = 0x00120060;

/// (0012,0062) VR=CS, VM=1 Patient Identity Removed
pub const PatientIdentityRemoved: u32 = 0x00120062;

/// (0012,0063) VR=LO, VM=1-n De-identification Method
pub const DeidentificationMethod: u32 = 0x00120063;

/// (0012,0064) VR=SQ, VM=1 De-identification Method Code Sequence
pub const DeidentificationMethodCodeSequence: u32 = 0x00120064;

/// (0012,0071) VR=LO, VM=1 Clinical Trial Series ID
pub const ClinicalTrialSeriesID: u32 = 0x00120071;

/// (0012,0072) VR=LO, VM=1 Clinical Trial Series Description
pub const ClinicalTrialSeriesDescription: u32 = 0x00120072;

/// (0012,0081) VR=LO, VM=1 Clinical Trial Protocol Ethics Committee Name
pub const ClinicalTrialProtocolEthicsCommitteeName: u32 = 0x00120081;

/// (0012,0082) VR=LO, VM=1 Clinical Trial Protocol Ethics Committee Approval Number
pub const ClinicalTrialProtocolEthicsCommitteeApprovalNumber: u32 = 0x00120082;

/// (0012,0083) VR=SQ, VM=1 Consent for Clinical Trial Use Sequence
pub const ConsentForClinicalTrialUseSequence: u32 = 0x00120083;

/// (0012,0084) VR=CS, VM=1 Distribution Type
pub const DistributionType: u32 = 0x00120084;

/// (0012,0085) VR=CS, VM=1 Consent for Distribution Flag
pub const ConsentForDistributionFlag: u32 = 0x00120085;

/// (0014,0023) VR=ST, VM=1-n CAD File Format
pub const CADFileFormat: u32 = 0x00140023;

/// (0014,0024) VR=ST, VM=1-n Component Reference System
pub const ComponentReferenceSystem: u32 = 0x00140024;

/// (0014,0025) VR=ST, VM=1-n Component Manufacturing Procedure
pub const ComponentManufacturingProcedure: u32 = 0x00140025;

/// (0014,0028) VR=ST, VM=1-n Component Manufacturer
pub const ComponentManufacturer: u32 = 0x00140028;

/// (0014,0030) VR=DS, VM=1-n Material Thickness
pub const MaterialThickness: u32 = 0x00140030;

/// (0014,0032) VR=DS, VM=1-n Material Pipe Diameter
pub const MaterialPipeDiameter: u32 = 0x00140032;

/// (0014,0034) VR=DS, VM=1-n Material Isolation Diameter
pub const MaterialIsolationDiameter: u32 = 0x00140034;

/// (0014,0042) VR=ST, VM=1-n Material Grade
pub const MaterialGrade: u32 = 0x00140042;

/// (0014,0044) VR=ST, VM=1-n Material Properties File ID
pub const MaterialPropertiesFileID: u32 = 0x00140044;

/// (0014,0045) VR=ST, VM=1-n Material Properties File Format
pub const MaterialPropertiesFileFormat: u32 = 0x00140045;

/// (0014,0046) VR=LT, VM=1 Material Notes
pub const MaterialNotes: u32 = 0x00140046;

/// (0014,0050) VR=CS, VM=1 Component Shape
pub const ComponentShape: u32 = 0x00140050;

/// (0014,0052) VR=CS, VM=1 Curvature Type
pub const CurvatureType: u32 = 0x00140052;

/// (0014,0054) VR=DS, VM=1 Outer Diameter
pub const OuterDiameter: u32 = 0x00140054;

/// (0014,0056) VR=DS, VM=1 Inner Diameter
pub const InnerDiameter: u32 = 0x00140056;

/// (0014,1010) VR=ST, VM=1 Actual Environmental Conditions
pub const ActualEnvironmentalConditions: u32 = 0x00141010;

/// (0014,1020) VR=DA, VM=1 Expiry Date
pub const ExpiryDate: u32 = 0x00141020;

/// (0014,1040) VR=ST, VM=1 Environmental Conditions
pub const EnvironmentalConditions: u32 = 0x00141040;

/// (0014,2002) VR=SQ, VM=1 Evaluator Sequence
pub const EvaluatorSequence: u32 = 0x00142002;

/// (0014,2004) VR=IS, VM=1 Evaluator Number
pub const EvaluatorNumber: u32 = 0x00142004;

/// (0014,2006) VR=PN, VM=1 Evaluator Name
pub const EvaluatorName: u32 = 0x00142006;

/// (0014,2008) VR=IS, VM=1 Evaluation Attempt
pub const EvaluationAttempt: u32 = 0x00142008;

/// (0014,2012) VR=SQ, VM=1 Indication Sequence
pub const IndicationSequence: u32 = 0x00142012;

/// (0014,2014) VR=IS, VM=1 Indication Number
pub const IndicationNumber: u32  = 0x00142014;

/// (0014,2016) VR=SH, VM=1 Indication Label
pub const IndicationLabel: u32 = 0x00142016;

/// (0014,2018) VR=ST, VM=1 Indication Description
pub const IndicationDescription: u32 = 0x00142018;

/// (0014,201A) VR=CS, VM=1-n Indication Type
pub const IndicationType: u32 = 0x0014201A;

/// (0014,201C) VR=CS, VM=1 Indication Disposition
pub const IndicationDisposition: u32 = 0x0014201C;

/// (0014,201E) VR=SQ, VM=1 Indication ROI Sequence
pub const IndicationROISequence: u32 = 0x0014201E;

/// (0014,2030) VR=SQ, VM=1 Indication Physical Property Sequence
pub const IndicationPhysicalPropertySequence: u32 = 0x00142030;

/// (0014,2032) VR=SH, VM=1 Property Label
pub const PropertyLabel: u32 = 0x00142032;

/// (0014,2202) VR=IS, VM=1 Coordinate System Number of Axes
pub const CoordinateSystemNumberOfAxes: u32  = 0x00142202;

/// (0014,2204) VR=SQ, VM=1 Coordinate System Axes Sequence
pub const CoordinateSystemAxesSequence: u32 = 0x00142204;

/// (0014,2206) VR=ST, VM=1 Coordinate System Axis Description
pub const CoordinateSystemAxisDescription: u32 = 0x00142206;

/// (0014,2208) VR=CS, VM=1 Coordinate System Data Set Mapping
pub const CoordinateSystemDataSetMapping: u32 = 0x00142208;

/// (0014,220A) VR=IS, VM=1 Coordinate System Axis Number
pub const CoordinateSystemAxisNumber: u32 = 0x0014220A;

/// (0014,220C) VR=CS, VM=1 Coordinate System Axis Type
pub const CoordinateSystemAxisType: u32 = 0x0014220C;

/// (0014,220E) VR=CS, VM=1 Coordinate System Axis Units
pub const CoordinateSystemAxisUnits: u32 = 0x0014220E;

/// (0014,2210) VR=OB, VM=1 Coordinate System Axis Values
pub const CoordinateSystemAxisValues: u32 = 0x00142210;

/// (0014,2220) VR=SQ, VM=1 Coordinate System Transform Sequence
pub const CoordinateSystemTransformSequence: u32 = 0x00142220;

/// (0014,2222) VR=ST, VM=1 Transform Description
pub const TransformDescription: u32 = 0x00142222;

/// (0014,2224) VR=IS, VM=1 Transform Number of Axes
pub const TransformNumberOfAxes: u32 = 0x00142224;

/// (0014,2226) VR=IS, VM=1-n Transform Order of Axes
pub const TransformOrderOfAxes: u32 = 0x00142226;

/// (0014,2228) VR=CS, VM=1 Transformed Axis Units
pub const TransformedAxisUnits: u32 = 0x00142228;

/// (0014,222A) VR=DS, VM=1-n Coordinate System Transform Rotation and Scale Matrix
pub const CoordinateSystemTransformRotationAndScaleMatrix: u32 = 0x0014222A;

/// (0014,222C) VR=DS, VM=1-n Coordinate System Transform Translation Matrix
pub const CoordinateSystemTransformTranslationMatrix: u32 = 0x0014222C;

/// (0014,3011) VR=DS, VM=1 Internal Detector Frame Time
pub const InternalDetectorFrameTime: u32 = 0x00143011;

/// (0014,3012) VR=DS, VM=1 Number of Frames Integrated
pub const NumberOfFramesIntegrated: u32 = 0x00143012;

/// (0014,3020) VR=SQ, VM=1 Detector Temperature Sequence
pub const DetectorTemperatureSequence: u32 = 0x00143020;

/// (0014,3022) VR=DS, VM=1 Sensor Name
pub const SensorName: u32 = 0x00143022;

/// (0014,3024) VR=DS, VM=1 Horizontal Offset of Sensor
pub const HorizontalOffsetOfSensor: u32 = 0x00143024;

/// (0014,3026) VR=DS, VM=1 Vertical Offset of Sensor
pub const VerticalOffsetOfSensor: u32 = 0x00143026;

/// (0014,3028) VR=DS, VM=1 Sensor Temperature
pub const SensorTemperature: u32 = 0x00143028;

/// (0014,3040) VR=SQ, VM=1 Dark Current Sequence
pub const DarkCurrentSequence: u32 = 0x00143040;

/// (0014,3050) VR=OB|OW, VM=1 Dark Current Counts
pub const DarkCurrentCounts: u32 = 0x00143050;

/// (0014,3060) VR=SQ, VM=1 Gain Correction Reference Sequence
pub const GainCorrectionReferenceSequence: u32 = 0x00143060;

/// (0014,3070) VR=OB|OW, VM=1 Air Counts
pub const AirCounts: u32 = 0x00143070;

/// (0014,3071) VR=DS, VM=1 KV Used in Gain Calibration
pub const KVUsedInGainCalibration: u32 = 0x00143071;

/// (0014,3072) VR=DS, VM=1 MA Used in Gain Calibration
pub const MAUsedInGainCalibration: u32 = 0x00143072;

/// (0014,3073) VR=DS, VM=1 Number of Frames Used for Integration
pub const NumberOfFramesUsedForIntegration: u32 = 0x00143073;

/// (0014,3074) VR=LO, VM=1 Filter Material Used in Gain Calibration
pub const FilterMaterialUsedInGainCalibration: u32 = 0x00143074;

/// (0014,3075) VR=DS, VM=1 Filter Thickness Used in Gain Calibration
pub const FilterThicknessUsedInGainCalibration: u32 = 0x00143075;

/// (0014,3076) VR=DA, VM=1 Date of Gain Calibration
pub const DateOfGainCalibration: u32 = 0x00143076;

/// (0014,3077) VR=TM, VM=1 Time of Gain Calibration
pub const TimeOfGainCalibration: u32 = 0x00143077;

/// (0014,3080) VR=OB, VM=1 Bad Pixel Image
pub const BadPixelImage: u32 = 0x00143080;

/// (0014,3099) VR=LT, VM=1 Calibration Notes
pub const CalibrationNotes: u32 = 0x00143099;

/// (0014,4002) VR=SQ, VM=1 Pulser Equipment Sequence
pub const PulserEquipmentSequence: u32 = 0x00144002;

/// (0014,4004) VR=CS, VM=1 Pulser Type
pub const PulserType: u32 = 0x00144004;

/// (0014,4006) VR=LT, VM=1 Pulser Notes
pub const PulserNotes: u32 = 0x00144006;

/// (0014,4008) VR=SQ, VM=1 Receiver Equipment Sequence
pub const ReceiverEquipmentSequence: u32 = 0x00144008;

/// (0014,400A) VR=CS, VM=1 Amplifier Type
pub const AmplifierType: u32 = 0x0014400A;

/// (0014,400C) VR=LT, VM=1 Receiver Notes
pub const ReceiverNotes: u32 = 0x0014400C;

/// (0014,400E) VR=SQ, VM=1 Pre-Amplifier Equipment Sequence
pub const PreAmplifierEquipmentSequence: u32 = 0x0014400E;

/// (0014,400F) VR=LT, VM=1 Pre-Amplifier Notes
pub const PreAmplifierNotes: u32 = 0x0014400F;

/// (0014,4010) VR=SQ, VM=1 Transmit Transducer Sequence
pub const TransmitTransducerSequence: u32 = 0x00144010;

/// (0014,4011) VR=SQ, VM=1 Receive Transducer Sequence
pub const ReceiveTransducerSequence: u32 = 0x00144011;

/// (0014,4012) VR=US, VM=1 Number of Elements
pub const NumberOfElements: u32 = 0x00144012;

/// (0014,4013) VR=CS, VM=1 Element Shape
pub const ElementShape: u32 = 0x00144013;

/// (0014,4014) VR=DS, VM=1 Element Dimension A
pub const ElementDimensionA: u32 = 0x00144014;

/// (0014,4015) VR=DS, VM=1 Element Dimension B
pub const ElementDimensionB: u32 = 0x00144015;

/// (0014,4016) VR=DS, VM=1 Element Pitch
pub const ElementPitch: u32 = 0x00144016;

/// (0014,4017) VR=DS, VM=1 Measured Beam Dimension A
pub const MeasuredBeamDimensionA: u32 = 0x00144017;

/// (0014,4018) VR=DS, VM=1 Measured Beam Dimension B
pub const MeasuredBeamDimensionB: u32 = 0x00144018;

/// (0014,4019) VR=DS, VM=1 Location of Measured Beam Diameter
pub const LocationOfMeasuredBeamDiameter: u32 = 0x00144019;

/// (0014,401A) VR=DS, VM=1 Nominal Frequency
pub const NominalFrequency: u32 = 0x0014401A;

/// (0014,401B) VR=DS, VM=1 Measured Center Frequency
pub const MeasuredCenterFrequency: u32 = 0x0014401B;

/// (0014,401C) VR=DS, VM=1 Measured Bandwidth
pub const MeasuredBandwidth: u32 = 0x0014401C;

/// (0014,4020) VR=SQ, VM=1 Pulser Settings Sequence
pub const PulserSettingsSequence: u32 = 0x00144020;

/// (0014,4022) VR=DS, VM=1 Pulse Width
pub const PulseWidth: u32 = 0x00144022;

/// (0014,4024) VR=DS, VM=1 Excitation Frequency
pub const ExcitationFrequency: u32 = 0x00144024;

/// (0014,4026) VR=CS, VM=1 Modulation Type
pub const ModulationType: u32 = 0x00144026;

/// (0014,4028) VR=DS, VM=1 Damping
pub const Damping: u32 = 0x00144028;

/// (0014,4030) VR=SQ, VM=1 Receiver Settings Sequence
pub const ReceiverSettingsSequence: u32 = 0x00144030;

/// (0014,4031) VR=DS, VM=1 Acquired Soundpath Length
pub const AcquiredSoundpathLength: u32 = 0x00144031;

/// (0014,4032) VR=CS, VM=1 Acquisition Compression Type
pub const AcquisitionCompressionType: u32 = 0x00144032;

/// (0014,4033) VR=IS, VM=1 Acquisition Sample Size
pub const AcquisitionSampleSize: u32 = 0x00144033;

/// (0014,4034) VR=DS, VM=1 Rectifier Smoothing
pub const RectifierSmoothing: u32 = 0x00144034;

/// (0014,4035) VR=SQ, VM=1 DAC Sequence
pub const DACSequence: u32 = 0x00144035;

/// (0014,4036) VR=CS, VM=1 DAC Type
pub const DACType: u32 = 0x00144036;

/// (0014,4038) VR=DS, VM=1-n DAC Gain Points
pub const DACGainPoints: u32 = 0x00144038;

/// (0014,403A) VR=DS, VM=1-n DAC Time Points
pub const DACTimePoints: u32 = 0x0014403A;

/// (0014,403C) VR=DS, VM=1-n DAC Amplitude
pub const DACAmplitude: u32 = 0x0014403C;

/// (0014,4040) VR=SQ, VM=1 Pre-Amplifier Settings Sequence
pub const PreAmplifierSettingsSequence: u32 = 0x00144040;

/// (0014,4050) VR=SQ, VM=1 Transmit Transducer Settings Sequence
pub const TransmitTransducerSettingsSequence: u32 = 0x00144050;

/// (0014,4051) VR=SQ, VM=1 Receive Transducer Settings Sequence
pub const ReceiveTransducerSettingsSequence: u32 = 0x00144051;

/// (0014,4052) VR=DS, VM=1 Incident Angle
pub const IncidentAngle: u32 = 0x00144052;

/// (0014,4054) VR=ST, VM=1 Coupling Technique
pub const CouplingTechnique: u32 = 0x00144054;

/// (0014,4056) VR=ST, VM=1 Coupling Medium
pub const CouplingMedium: u32 = 0x00144056;

/// (0014,4057) VR=DS, VM=1 Coupling Velocity
pub const CouplingVelocity: u32 = 0x00144057;

/// (0014,4058) VR=DS, VM=1 Crystal Center Location X
pub const CrystalCenterLocationX: u32 = 0x00144058;

/// (0014,4059) VR=DS, VM=1 Crystal Center Location Z
pub const CrystalCenterLocationZ: u32 = 0x00144059;

/// (0014,405A) VR=DS, VM=1 Sound Path Length
pub const SoundPathLength: u32 = 0x0014405A;

/// (0014,405C) VR=ST, VM=1 Delay Law Identifier
pub const DelayLawIdentifier: u32 = 0x0014405C;

/// (0014,4060) VR=SQ, VM=1 Gate Settings Sequence
pub const GateSettingsSequence: u32 = 0x00144060;

/// (0014,4062) VR=DS, VM=1 Gate Threshold
pub const GateThreshold: u32 = 0x00144062;

/// (0014,4064) VR=DS, VM=1 Velocity of Sound
pub const VelocityOfSound: u32 = 0x00144064;

/// (0014,4070) VR=SQ, VM=1 Calibration Settings Sequence
pub const CalibrationSettingsSequence: u32 = 0x00144070;

/// (0014,4072) VR=ST, VM=1 Calibration Procedure
pub const CalibrationProcedure: u32 = 0x00144072;

/// (0014,4074) VR=SH, VM=1 Procedure Version
pub const ProcedureVersion: u32 = 0x00144074;

/// (0014,4076) VR=DA, VM=1 Procedure Creation Date
pub const ProcedureCreationDate: u32 = 0x00144076;

/// (0014,4078) VR=DA, VM=1 Procedure Expiration Date
pub const ProcedureExpirationDate: u32 = 0x00144078;

/// (0014,407A) VR=DA, VM=1 Procedure Last Modified Date
pub const ProcedureLastModifiedDate: u32 = 0x0014407A;

/// (0014,407C) VR=TM, VM=1-n Calibration Time
pub const CalibrationTime: u32 = 0x0014407C;

/// (0014,407E) VR=DA, VM=1-n Calibration Date
pub const CalibrationDate: u32 = 0x0014407E;

/// (0014,5002) VR=IS, VM=1 LINAC Energy
pub const LINACEnergy: u32 = 0x00145002;

/// (0014,5004) VR=IS, VM=1 LINAC Output
pub const LINACOutput: u32 = 0x00145004;

/// (0018,0010) VR=LO, VM=1 Contrast/Bolus Agent
pub const ContrastBolusAgent: u32 = 0x00180010;

/// (0018,0012) VR=SQ, VM=1 Contrast/Bolus Agent Sequence
pub const ContrastBolusAgentSequence: u32 = 0x00180012;

/// (0018,0014) VR=SQ, VM=1 Contrast/Bolus Administration Route Sequence
pub const ContrastBolusAdministrationRouteSequence: u32 = 0x00180014;

/// (0018,0015) VR=CS, VM=1 Body Part Examined
pub const BodyPartExamined: u32 = 0x00180015;

/// (0018,0020) VR=CS, VM=1-n Scanning Sequence
pub const ScanningSequence: u32 = 0x00180020;

/// (0018,0021) VR=CS, VM=1-n Sequence Variant
pub const SequenceVariant: u32 = 0x00180021;

/// (0018,0022) VR=CS, VM=1-n Scan Options
pub const ScanOptions: u32 = 0x00180022;

/// (0018,0023) VR=CS, VM=1 MR Acquisition Type
pub const MRAcquisitionType: u32 = 0x00180023;

/// (0018,0024) VR=SH, VM=1 Sequence Name
pub const SequenceName: u32 = 0x00180024;

/// (0018,0025) VR=CS, VM=1 Angio Flag
pub const AngioFlag: u32 = 0x00180025;

/// (0018,0026) VR=SQ, VM=1 Intervention Drug Information Sequence
pub const InterventionDrugInformationSequence: u32 = 0x00180026;

/// (0018,0027) VR=TM, VM=1 Intervention Drug Stop Time
pub const InterventionDrugStopTime: u32 = 0x00180027;

/// (0018,0028) VR=DS, VM=1 Intervention Drug Dose
pub const InterventionDrugDose: u32 = 0x00180028;

/// (0018,0029) VR=SQ, VM=1 Intervention Drug Code Sequence
pub const InterventionDrugCodeSequence: u32 = 0x00180029;

/// (0018,002A) VR=SQ, VM=1 Additional Drug Sequence
pub const AdditionalDrugSequence: u32 = 0x0018002A;

/// (0018,0030) VR=LO, VM=1-n Radionuclide (retired)
pub const Radionuclide: u32 = 0x00180030;

/// (0018,0031) VR=LO, VM=1 Radiopharmaceutical
pub const Radiopharmaceutical: u32 = 0x00180031;

/// (0018,0032) VR=DS, VM=1 Energy Window Centerline (retired)
pub const EnergyWindowCenterline: u32 = 0x00180032;

/// (0018,0033) VR=DS, VM=1-n Energy Window Total Width (retired)
pub const EnergyWindowTotalWidth: u32 = 0x00180033;

/// (0018,0034) VR=LO, VM=1 Intervention Drug Name
pub const InterventionDrugName: u32 = 0x00180034;

/// (0018,0035) VR=TM, VM=1 Intervention Drug Start Time
pub const InterventionDrugStartTime: u32 = 0x00180035;

/// (0018,0036) VR=SQ, VM=1 Intervention Sequence
pub const InterventionSequence: u32 = 0x00180036;

/// (0018,0037) VR=CS, VM=1 Therapy Type (retired)
pub const TherapyType: u32 = 0x00180037;

/// (0018,0038) VR=CS, VM=1 Intervention Status
pub const InterventionStatus: u32 = 0x00180038;

/// (0018,0039) VR=CS, VM=1 Therapy Description (retired)
pub const TherapyDescription: u32 = 0x00180039;

/// (0018,003A) VR=ST, VM=1 Intervention Description
pub const InterventionDescription: u32 = 0x0018003A;

/// (0018,0040) VR=IS, VM=1 Cine Rate
pub const CineRate: u32 = 0x00180040;

/// (0018,0042) VR=CS, VM=1 Initial Cine Run State
pub const InitialCineRunState: u32 = 0x00180042;

/// (0018,0050) VR=DS, VM=1 Slice Thickness
pub const SliceThickness: u32 = 0x00180050;

/// (0018,0060) VR=DS, VM=1 KVP
pub const KVP: u32 = 0x00180060;

/// (0018,0070) VR=IS, VM=1 Counts Accumulated
pub const CountsAccumulated: u32 = 0x00180070;

/// (0018,0071) VR=CS, VM=1 Acquisition Termination Condition
pub const AcquisitionTerminationCondition: u32 = 0x00180071;

/// (0018,0072) VR=DS, VM=1 Effective Duration
pub const EffectiveDuration: u32 = 0x00180072;

/// (0018,0073) VR=CS, VM=1 Acquisition Start Condition
pub const AcquisitionStartCondition: u32 = 0x00180073;

/// (0018,0074) VR=IS, VM=1 Acquisition Start Condition Data
pub const AcquisitionStartConditionData: u32 = 0x00180074;

/// (0018,0075) VR=IS, VM=1 Acquisition Termination Condition Data
pub const AcquisitionTerminationConditionData: u32 = 0x00180075;

/// (0018,0080) VR=DS, VM=1 Repetition Time
pub const RepetitionTime: u32 = 0x00180080;

/// (0018,0081) VR=DS, VM=1 Echo Time
pub const EchoTime: u32 = 0x00180081;

/// (0018,0082) VR=DS, VM=1 Inversion Time
pub const InversionTime: u32 = 0x00180082;

/// (0018,0083) VR=DS, VM=1 Number of Averages
pub const NumberOfAverages: u32 = 0x00180083;

/// (0018,0084) VR=DS, VM=1 Imaging Frequency
pub const ImagingFrequency: u32 = 0x00180084;

/// (0018,0085) VR=SH, VM=1 Imaged Nucleus
pub const ImagedNucleus: u32 = 0x00180085;

/// (0018,0086) VR=IS, VM=1-n Echo Number(s)
pub const EchoNumbers: u32 = 0x00180086;

/// (0018,0087) VR=DS, VM=1 Magnetic Field Strength
pub const MagneticFieldStrength: u32 = 0x00180087;

/// (0018,0088) VR=DS, VM=1 Spacing Between Slices
pub const SpacingBetweenSlices: u32 = 0x00180088;

/// (0018,0089) VR=IS, VM=1 Number of Phase Encoding Steps
pub const NumberOfPhaseEncodingSteps: u32 = 0x00180089;

/// (0018,0090) VR=DS, VM=1 Data Collection Diameter
pub const DataCollectionDiameter: u32 = 0x00180090;

/// (0018,0091) VR=IS, VM=1 Echo Train Length
pub const EchoTrainLength: u32 = 0x00180091;

/// (0018,0093) VR=DS, VM=1 Percent Sampling
pub const PercentSampling: u32 = 0x00180093;

/// (0018,0094) VR=DS, VM=1 Percent Phase Field of View
pub const PercentPhaseFieldOfView: u32 = 0x00180094;

/// (0018,0095) VR=DS, VM=1 Pixel Bandwidth
pub const PixelBandwidth: u32 = 0x00180095;

/// (0018,1000) VR=LO, VM=1 Device Serial Number
pub const DeviceSerialNumber: u32 = 0x00181000;

/// (0018,1002) VR=UI, VM=1 Device UID
pub const DeviceUID: u32 = 0x00181002;

/// (0018,1003) VR=LO, VM=1 Device ID
pub const DeviceID: u32 = 0x00181003;

/// (0018,1004) VR=LO, VM=1 Plate ID
pub const PlateID: u32 = 0x00181004;

/// (0018,1005) VR=LO, VM=1 Generator ID
pub const GeneratorID: u32 = 0x00181005;

/// (0018,1006) VR=LO, VM=1 Grid ID
pub const GridID: u32 = 0x00181006;

/// (0018,1007) VR=LO, VM=1 Cassette ID
pub const CassetteID: u32 = 0x00181007;

/// (0018,1008) VR=LO, VM=1 Gantry ID
pub const GantryID: u32 = 0x00181008;

/// (0018,1010) VR=LO, VM=1 Secondary Capture Device ID
pub const SecondaryCaptureDeviceID: u32 = 0x00181010;

/// (0018,1011) VR=LO, VM=1 Hardcopy Creation Device ID (retired)
pub const HardcopyCreationDeviceID: u32 = 0x00181011;

/// (0018,1012) VR=DA, VM=1 Date of Secondary Capture
pub const DateOfSecondaryCapture: u32 = 0x00181012;

/// (0018,1014) VR=TM, VM=1 Time of Secondary Capture
pub const TimeOfSecondaryCapture: u32 = 0x00181014;

/// (0018,1016) VR=LO, VM=1 Secondary Capture Device Manufacturer
pub const SecondaryCaptureDeviceManufacturer: u32 = 0x00181016;

/// (0018,1017) VR=LO, VM=1 Hardcopy Device Manufacturer (retired)
pub const HardcopyDeviceManufacturer: u32 = 0x00181017;

/// (0018,1018) VR=LO, VM=1 Secondary Capture Device Manufacturer’s Model Name
pub const SecondaryCaptureDeviceManufacturerModelName: u32 = 0x00181018;

/// (0018,1019) VR=LO, VM=1-n Secondary Capture Device Software Versions
pub const SecondaryCaptureDeviceSoftwareVersions: u32 = 0x00181019;

/// (0018,101A) VR=LO, VM=1-n Hardcopy Device Software Version (retired)
pub const HardcopyDeviceSoftwareVersion: u32 = 0x0018101A;

/// (0018,101B) VR=LO, VM=1 Hardcopy Device Manufacturer’s Model Name (retired)
pub const HardcopyDeviceManufacturerModelName: u32 = 0x0018101B;

/// (0018,1020) VR=LO, VM=1-n Software Version(s)
pub const SoftwareVersions: u32 = 0x00181020;

/// (0018,1022) VR=SH, VM=1 Video Image Format Acquired
pub const VideoImageFormatAcquired: u32 = 0x00181022;

/// (0018,1023) VR=LO, VM=1 Digital Image Format Acquired
pub const DigitalImageFormatAcquired: u32 = 0x00181023;

/// (0018,1030) VR=LO, VM=1 Protocol Name
pub const ProtocolName: u32 = 0x00181030;

/// (0018,1040) VR=LO, VM=1 Contrast/Bolus Route
pub const ContrastBolusRoute: u32 = 0x00181040;

/// (0018,1041) VR=DS, VM=1 Contrast/Bolus Volume
pub const ContrastBolusVolume: u32 = 0x00181041;

/// (0018,1042) VR=TM, VM=1 Contrast/Bolus Start Time
pub const ContrastBolusStartTime: u32 = 0x00181042;

/// (0018,1043) VR=TM, VM=1 Contrast/Bolus Stop Time
pub const ContrastBolusStopTime: u32 = 0x00181043;

/// (0018,1044) VR=DS, VM=1 Contrast/Bolus Total Dose
pub const ContrastBolusTotalDose: u32 = 0x00181044;

/// (0018,1045) VR=IS, VM=1 Syringe Counts
pub const SyringeCounts: u32 = 0x00181045;

/// (0018,1046) VR=DS, VM=1-n Contrast Flow Rate
pub const ContrastFlowRate: u32 = 0x00181046;

/// (0018,1047) VR=DS, VM=1-n Contrast Flow Duration
pub const ContrastFlowDuration: u32 = 0x00181047;

/// (0018,1048) VR=CS, VM=1 Contrast/Bolus Ingredient
pub const ContrastBolusIngredient: u32 = 0x00181048;

/// (0018,1049) VR=DS, VM=1 Contrast/Bolus Ingredient Concentration
pub const ContrastBolusIngredientConcentration: u32 = 0x00181049;

/// (0018,1050) VR=DS, VM=1 Spatial Resolution
pub const SpatialResolution: u32 = 0x00181050;

/// (0018,1060) VR=DS, VM=1 Trigger Time
pub const TriggerTime: u32 = 0x00181060;

/// (0018,1061) VR=LO, VM=1 Trigger Source or Type
pub const TriggerSourceOrType: u32 = 0x00181061;

/// (0018,1062) VR=IS, VM=1 Nominal Interval
pub const NominalInterval: u32 = 0x00181062;

/// (0018,1063) VR=DS, VM=1 Frame Time
pub const FrameTime: u32 = 0x00181063;

/// (0018,1064) VR=LO, VM=1 Cardiac Framing Type
pub const CardiacFramingType: u32 = 0x00181064;

/// (0018,1065) VR=DS, VM=1-n Frame Time Vector
pub const FrameTimeVector: u32 = 0x00181065;

/// (0018,1066) VR=DS, VM=1 Frame Delay
pub const FrameDelay: u32 = 0x00181066;

/// (0018,1067) VR=DS, VM=1 Image Trigger Delay
pub const ImageTriggerDelay: u32 = 0x00181067;

/// (0018,1068) VR=DS, VM=1 Multiplex Group Time Offset
pub const MultiplexGroupTimeOffset: u32 = 0x00181068;

/// (0018,1069) VR=DS, VM=1 Trigger Time Offset
pub const TriggerTimeOffset: u32 = 0x00181069;

/// (0018,106A) VR=CS, VM=1 Synchronization Trigger
pub const SynchronizationTrigger: u32 = 0x0018106A;

/// (0018,106C) VR=US, VM=2 Synchronization Channel
pub const SynchronizationChannel: u32 = 0x0018106C;

/// (0018,106E) VR=UL, VM=1 Trigger Sample Position
pub const TriggerSamplePosition: u32 = 0x0018106E;

/// (0018,1070) VR=LO, VM=1 Radiopharmaceutical Route
pub const RadiopharmaceuticalRoute: u32 = 0x00181070;

/// (0018,1071) VR=DS, VM=1 Radiopharmaceutical Volume
pub const RadiopharmaceuticalVolume: u32 = 0x00181071;

/// (0018,1072) VR=TM, VM=1 Radiopharmaceutical Start Time
pub const RadiopharmaceuticalStartTime: u32 = 0x00181072;

/// (0018,1073) VR=TM, VM=1 Radiopharmaceutical Stop Time
pub const RadiopharmaceuticalStopTime: u32 = 0x00181073;

/// (0018,1074) VR=DS, VM=1 Radionuclide Total Dose
pub const RadionuclideTotalDose: u32 = 0x00181074;

/// (0018,1075) VR=DS, VM=1 Radionuclide Half Life
pub const RadionuclideHalfLife: u32 = 0x00181075;

/// (0018,1076) VR=DS, VM=1 Radionuclide Positron Fraction
pub const RadionuclidePositronFraction: u32 = 0x00181076;

/// (0018,1077) VR=DS, VM=1 Radiopharmaceutical Specific Activity
pub const RadiopharmaceuticalSpecificActivity: u32 = 0x00181077;

/// (0018,1078) VR=DT, VM=1 Radiopharmaceutical Start DateTime
pub const RadiopharmaceuticalStartDateTime: u32 = 0x00181078;

/// (0018,1079) VR=DT, VM=1 Radiopharmaceutical Stop DateTime
pub const RadiopharmaceuticalStopDateTime: u32 = 0x00181079;

/// (0018,1080) VR=CS, VM=1 Beat Rejection Flag
pub const BeatRejectionFlag: u32 = 0x00181080;

/// (0018,1081) VR=IS, VM=1 Low R-R Value
pub const LowRRValue: u32 = 0x00181081;

/// (0018,1082) VR=IS, VM=1 High R-R Value
pub const HighRRValue: u32 = 0x00181082;

/// (0018,1083) VR=IS, VM=1 Intervals Acquired
pub const IntervalsAcquired: u32 = 0x00181083;

/// (0018,1084) VR=IS, VM=1 Intervals Rejected
pub const IntervalsRejected: u32 = 0x00181084;

/// (0018,1085) VR=LO, VM=1 PVC Rejection
pub const PVCRejection: u32 = 0x00181085;

/// (0018,1086) VR=IS, VM=1 Skip Beats
pub const SkipBeats: u32 = 0x00181086;

/// (0018,1088) VR=IS, VM=1 Heart Rate
pub const HeartRate: u32 = 0x00181088;

/// (0018,1090) VR=IS, VM=1 Cardiac Number of Images
pub const CardiacNumberOfImages: u32 = 0x00181090;

/// (0018,1094) VR=IS, VM=1 Trigger Window
pub const TriggerWindow: u32 = 0x00181094;

/// (0018,1100) VR=DS, VM=1 Reconstruction Diameter
pub const ReconstructionDiameter: u32 = 0x00181100;

/// (0018,1110) VR=DS, VM=1 Distance Source to Detector
pub const DistanceSourceToDetector: u32 = 0x00181110;

/// (0018,1111) VR=DS, VM=1 Distance Source to Patient
pub const DistanceSourceToPatient: u32 = 0x00181111;

/// (0018,1114) VR=DS, VM=1 Estimated Radiographic Magnification Factor
pub const EstimatedRadiographicMagnificationFactor: u32 = 0x00181114;

/// (0018,1120) VR=DS, VM=1 Gantry/Detector Tilt
pub const GantryDetectorTilt: u32 = 0x00181120;

/// (0018,1121) VR=DS, VM=1 Gantry/Detector Slew
pub const GantryDetectorSlew: u32 = 0x00181121;

/// (0018,1130) VR=DS, VM=1 Table Height
pub const TableHeight: u32 = 0x00181130;

/// (0018,1131) VR=DS, VM=1 Table Traverse
pub const TableTraverse: u32 = 0x00181131;

/// (0018,1134) VR=CS, VM=1 Table Motion
pub const TableMotion: u32 = 0x00181134;

/// (0018,1135) VR=DS, VM=1-n Table Vertical Increment
pub const TableVerticalIncrement: u32 = 0x00181135;

/// (0018,1136) VR=DS, VM=1-n Table Lateral Increment
pub const TableLateralIncrement: u32 = 0x00181136;

/// (0018,1137) VR=DS, VM=1-n Table Longitudinal Increment
pub const TableLongitudinalIncrement: u32 = 0x00181137;

/// (0018,1138) VR=DS, VM=1 Table Angle
pub const TableAngle: u32 = 0x00181138;

/// (0018,113A) VR=CS, VM=1 Table Type
pub const TableType: u32 = 0x0018113A;

/// (0018,1140) VR=CS, VM=1 Rotation Direction
pub const RotationDirection: u32 = 0x00181140;

/// (0018,1141) VR=DS, VM=1 Angular Position (retired)
pub const AngularPosition: u32 = 0x00181141;

/// (0018,1142) VR=DS, VM=1-n Radial Position
pub const RadialPosition: u32 = 0x00181142;

/// (0018,1143) VR=DS, VM=1 Scan Arc
pub const ScanArc: u32 = 0x00181143;

/// (0018,1144) VR=DS, VM=1 Angular Step
pub const AngularStep: u32 = 0x00181144;

/// (0018,1145) VR=DS, VM=1 Center of Rotation Offset
pub const CenterOfRotationOffset: u32 = 0x00181145;

/// (0018,1146) VR=DS, VM=1-n Rotation Offset (retired)
pub const RotationOffset: u32 = 0x00181146;

/// (0018,1147) VR=CS, VM=1 Field of View Shape
pub const FieldOfViewShape: u32 = 0x00181147;

/// (0018,1149) VR=IS, VM=1-2 Field of View Dimension(s)
pub const FieldOfViewDimensions: u32 = 0x00181149;

/// (0018,1150) VR=IS, VM=1 Exposure Time
pub const ExposureTime: u32 = 0x00181150;

/// (0018,1151) VR=IS, VM=1 X-Ray Tube Current
pub const XRayTubeCurrent: u32 = 0x00181151;

/// (0018,1152) VR=IS, VM=1 Exposure
pub const Exposure: u32 = 0x00181152;

/// (0018,1153) VR=IS, VM=1 Exposure in µAs
pub const ExposureInuAs: u32 = 0x00181153;

/// (0018,1154) VR=DS, VM=1 Average Pulse Width
pub const AveragePulseWidth: u32 = 0x00181154;

/// (0018,1155) VR=CS, VM=1 Radiation Setting
pub const RadiationSetting: u32 = 0x00181155;

/// (0018,1156) VR=CS, VM=1 Rectification Type
pub const RectificationType: u32 = 0x00181156;

/// (0018,115A) VR=CS, VM=1 Radiation Mode
pub const RadiationMode: u32 = 0x0018115A;

/// (0018,115E) VR=DS, VM=1 Image and Fluoroscopy Area Dose Product
pub const ImageAndFluoroscopyAreaDoseProduct: u32 = 0x0018115E;

/// (0018,1160) VR=SH, VM=1 Filter Type
pub const FilterType: u32 = 0x00181160;

/// (0018,1161) VR=LO, VM=1-n Type of Filters
pub const TypeOfFilters: u32 = 0x00181161;

/// (0018,1162) VR=DS, VM=1 Intensifier Size
pub const IntensifierSize: u32 = 0x00181162;

/// (0018,1164) VR=DS, VM=2 Imager Pixel Spacing
pub const ImagerPixelSpacing: u32 = 0x00181164;

/// (0018,1166) VR=CS, VM=1-n Grid
pub const Grid: u32 = 0x00181166;

/// (0018,1170) VR=IS, VM=1 Generator Power
pub const GeneratorPower: u32 = 0x00181170;

/// (0018,1180) VR=SH, VM=1 Collimator/grid Name
pub const CollimatorGridName: u32 = 0x00181180;

/// (0018,1181) VR=CS, VM=1 Collimator Type
pub const CollimatorType: u32 = 0x00181181;

/// (0018,1182) VR=IS, VM=1-2 Focal Distance
pub const FocalDistance: u32 = 0x00181182;

/// (0018,1183) VR=DS, VM=1-2 X Focus Center
pub const XFocusCenter: u32 = 0x00181183;

/// (0018,1184) VR=DS, VM=1-2 Y Focus Center
pub const YFocusCenter: u32 = 0x00181184;

/// (0018,1190) VR=DS, VM=1-n Focal Spot(s)
pub const FocalSpots: u32 = 0x00181190;

/// (0018,1191) VR=CS, VM=1 Anode Target Material
pub const AnodeTargetMaterial: u32 = 0x00181191;

/// (0018,11A0) VR=DS, VM=1 Body Part Thickness
pub const BodyPartThickness: u32 = 0x001811A0;

/// (0018,11A2) VR=DS, VM=1 Compression Force
pub const CompressionForce: u32 = 0x001811A2;

/// (0018,1200) VR=DA, VM=1-n Date of Last Calibration
pub const DateOfLastCalibration: u32 = 0x00181200;

/// (0018,1201) VR=TM, VM=1-n Time of Last Calibration
pub const TimeOfLastCalibration: u32 = 0x00181201;

/// (0018,1210) VR=SH, VM=1-n Convolution Kernel
pub const ConvolutionKernel: u32 = 0x00181210;

/// (0018,1240) VR=IS, VM=1-n Upper/Lower Pixel Values (retired)
pub const UpperLowerPixelValues: u32 = 0x00181240;

/// (0018,1242) VR=IS, VM=1 Actual Frame Duration
pub const ActualFrameDuration: u32 = 0x00181242;

/// (0018,1243) VR=IS, VM=1 Count Rate
pub const CountRate: u32 = 0x00181243;

/// (0018,1244) VR=US, VM=1 Preferred Playback Sequencing
pub const PreferredPlaybackSequencing: u32 = 0x00181244;

/// (0018,1250) VR=SH, VM=1 Receive Coil Name
pub const ReceiveCoilName: u32 = 0x00181250;

/// (0018,1251) VR=SH, VM=1 Transmit Coil Name
pub const TransmitCoilName: u32 = 0x00181251;

/// (0018,1260) VR=SH, VM=1 Plate Type
pub const PlateType: u32 = 0x00181260;

/// (0018,1261) VR=LO, VM=1 Phosphor Type
pub const PhosphorType: u32 = 0x00181261;

/// (0018,1300) VR=DS, VM=1 Scan Velocity
pub const ScanVelocity: u32 = 0x00181300;

/// (0018,1301) VR=CS, VM=1-n Whole Body Technique
pub const WholeBodyTechnique: u32 = 0x00181301;

/// (0018,1302) VR=IS, VM=1 Scan Length
pub const ScanLength: u32 = 0x00181302;

/// (0018,1310) VR=US, VM=4 Acquisition Matrix
pub const AcquisitionMatrix: u32 = 0x00181310;

/// (0018,1312) VR=CS, VM=1 In-plane Phase Encoding Direction
pub const InPlanePhaseEncodingDirection: u32 = 0x00181312;

/// (0018,1314) VR=DS, VM=1 Flip Angle
pub const FlipAngle: u32 = 0x00181314;

/// (0018,1315) VR=CS, VM=1 Variable Flip Angle Flag
pub const VariableFlipAngleFlag: u32 = 0x00181315;

/// (0018,1316) VR=DS, VM=1 SAR
pub const SAR: u32 = 0x00181316;

/// (0018,1318) VR=DS, VM=1 dB/dt
pub const dBdt: u32 = 0x00181318;

/// (0018,1400) VR=LO, VM=1 Acquisition Device Processing Description
pub const AcquisitionDeviceProcessingDescription: u32 = 0x00181400;

/// (0018,1401) VR=LO, VM=1 Acquisition Device Processing Code
pub const AcquisitionDeviceProcessingCode: u32 = 0x00181401;

/// (0018,1402) VR=CS, VM=1 Cassette Orientation
pub const CassetteOrientation: u32 = 0x00181402;

/// (0018,1403) VR=CS, VM=1 Cassette Size
pub const CassetteSize: u32 = 0x00181403;

/// (0018,1404) VR=US, VM=1 Exposures on Plate
pub const ExposuresOnPlate: u32 = 0x00181404;

/// (0018,1405) VR=IS, VM=1 Relative X-Ray Exposure
pub const RelativeXRayExposure: u32 = 0x00181405;

/// (0018,1411) VR=DS, VM=1 Exposure Index
pub const ExposureIndex: u32 = 0x00181411;

/// (0018,1412) VR=DS, VM=1 Target Exposure Index
pub const TargetExposureIndex: u32 = 0x00181412;

/// (0018,1413) VR=DS, VM=1 Deviation Index
pub const DeviationIndex: u32 = 0x00181413;

/// (0018,1450) VR=DS, VM=1 Column Angulation
pub const ColumnAngulation: u32 = 0x00181450;

/// (0018,1460) VR=DS, VM=1 Tomo Layer Height
pub const TomoLayerHeight: u32 = 0x00181460;

/// (0018,1470) VR=DS, VM=1 Tomo Angle
pub const TomoAngle: u32 = 0x00181470;

/// (0018,1480) VR=DS, VM=1 Tomo Time
pub const TomoTime: u32 = 0x00181480;

/// (0018,1490) VR=CS, VM=1 Tomo Type
pub const TomoType: u32 = 0x00181490;

/// (0018,1491) VR=CS, VM=1 Tomo Class
pub const TomoClass: u32 = 0x00181491;

/// (0018,1495) VR=IS, VM=1 Number of Tomosynthesis Source Images
pub const NumberOfTomosynthesisSourceImages: u32 = 0x00181495;

/// (0018,1500) VR=CS, VM=1 Positioner Motion
pub const PositionerMotion: u32 = 0x00181500;

/// (0018,1508) VR=CS, VM=1 Positioner Type
pub const PositionerType: u32 = 0x00181508;

/// (0018,1510) VR=DS, VM=1 Positioner Primary Angle
pub const PositionerPrimaryAngle: u32 = 0x00181510;

/// (0018,1511) VR=DS, VM=1 Positioner Secondary Angle
pub const PositionerSecondaryAngle: u32 = 0x00181511;

/// (0018,1520) VR=DS, VM=1-n Positioner Primary Angle Increment
pub const PositionerPrimaryAngleIncrement: u32 = 0x00181520;

/// (0018,1521) VR=DS, VM=1-n Positioner Secondary Angle Increment
pub const PositionerSecondaryAngleIncrement: u32 = 0x00181521;

/// (0018,1530) VR=DS, VM=1 Detector Primary Angle
pub const DetectorPrimaryAngle: u32 = 0x00181530;

/// (0018,1531) VR=DS, VM=1 Detector Secondary Angle
pub const DetectorSecondaryAngle: u32 = 0x00181531;

/// (0018,1600) VR=CS, VM=1-3 Shutter Shape
pub const ShutterShape: u32 = 0x00181600;

/// (0018,1602) VR=IS, VM=1 Shutter Left Vertical Edge
pub const ShutterLeftVerticalEdge: u32 = 0x00181602;

/// (0018,1604) VR=IS, VM=1 Shutter Right Vertical Edge
pub const ShutterRightVerticalEdge: u32 = 0x00181604;

/// (0018,1606) VR=IS, VM=1 Shutter Upper Horizontal Edge
pub const ShutterUpperHorizontalEdge: u32 = 0x00181606;

/// (0018,1608) VR=IS, VM=1 Shutter Lower Horizontal Edge
pub const ShutterLowerHorizontalEdge: u32 = 0x00181608;

/// (0018,1610) VR=IS, VM=2 Center of Circular Shutter
pub const CenterOfCircularShutter: u32 = 0x00181610;

/// (0018,1612) VR=IS, VM=1 Radius of Circular Shutter
pub const RadiusOfCircularShutter: u32 = 0x00181612;

/// (0018,1620) VR=IS, VM=2-2n Vertices of the Polygonal Shutter
pub const VerticesOfThePolygonalShutter: u32 = 0x00181620;

/// (0018,1622) VR=US, VM=1 Shutter Presentation Value
pub const ShutterPresentationValue: u32 = 0x00181622;

/// (0018,1623) VR=US, VM=1 Shutter Overlay Group
pub const ShutterOverlayGroup: u32 = 0x00181623;

/// (0018,1624) VR=US, VM=3 Shutter Presentation Color CIELab Value
pub const ShutterPresentationColorCIELabValue: u32 = 0x00181624;

/// (0018,1700) VR=CS, VM=1-3 Collimator Shape
pub const CollimatorShape: u32 = 0x00181700;

/// (0018,1702) VR=IS, VM=1 Collimator Left Vertical Edge
pub const CollimatorLeftVerticalEdge: u32 = 0x00181702;

/// (0018,1704) VR=IS, VM=1 Collimator Right Vertical Edge
pub const CollimatorRightVerticalEdge: u32 = 0x00181704;

/// (0018,1706) VR=IS, VM=1 Collimator Upper Horizontal Edge
pub const CollimatorUpperHorizontalEdge: u32 = 0x00181706;

/// (0018,1708) VR=IS, VM=1 Collimator Lower Horizontal Edge
pub const CollimatorLowerHorizontalEdge: u32 = 0x00181708;

/// (0018,1710) VR=IS, VM=2 Center of Circular Collimator
pub const CenterOfCircularCollimator: u32 = 0x00181710;

/// (0018,1712) VR=IS, VM=1 Radius of Circular Collimator
pub const RadiusOfCircularCollimator: u32 = 0x00181712;

/// (0018,1720) VR=IS, VM=2-2n Vertices of the Polygonal Collimator
pub const VerticesOfThePolygonalCollimator: u32 = 0x00181720;

/// (0018,1800) VR=CS, VM=1 Acquisition Time Synchronized
pub const AcquisitionTimeSynchronized: u32 = 0x00181800;

/// (0018,1801) VR=SH, VM=1 Time Source
pub const TimeSource: u32 = 0x00181801;

/// (0018,1802) VR=CS, VM=1 Time Distribution Protocol
pub const TimeDistributionProtocol: u32 = 0x00181802;

/// (0018,1803) VR=LO, VM=1 NTP Source Address
pub const NTPSourceAddress: u32 = 0x00181803;

/// (0018,2001) VR=IS, VM=1-n Page Number Vector
pub const PageNumberVector: u32 = 0x00182001;

/// (0018,2002) VR=SH, VM=1-n Frame Label Vector
pub const FrameLabelVector: u32 = 0x00182002;

/// (0018,2003) VR=DS, VM=1-n Frame Primary Angle Vector
pub const FramePrimaryAngleVector: u32 = 0x00182003;

/// (0018,2004) VR=DS, VM=1-n Frame Secondary Angle Vector
pub const FrameSecondaryAngleVector: u32 = 0x00182004;

/// (0018,2005) VR=DS, VM=1-n Slice Location Vector
pub const SliceLocationVector: u32 = 0x00182005;

/// (0018,2006) VR=SH, VM=1-n Display Window Label Vector
pub const DisplayWindowLabelVector: u32 = 0x00182006;

/// (0018,2010) VR=DS, VM=2 Nominal Scanned Pixel Spacing
pub const NominalScannedPixelSpacing: u32 = 0x00182010;

/// (0018,2020) VR=CS, VM=1 Digitizing Device Transport Direction
pub const DigitizingDeviceTransportDirection: u32 = 0x00182020;

/// (0018,2030) VR=DS, VM=1 Rotation of Scanned Film
pub const RotationOfScannedFilm: u32 = 0x00182030;

/// (0018,3100) VR=CS, VM=1 IVUS Acquisition
pub const IVUSAcquisition: u32 = 0x00183100;

/// (0018,3101) VR=DS, VM=1 IVUS Pullback Rate
pub const IVUSPullbackRate: u32 = 0x00183101;

/// (0018,3102) VR=DS, VM=1 IVUS Gated Rate
pub const IVUSGatedRate: u32 = 0x00183102;

/// (0018,3103) VR=IS, VM=1 IVUS Pullback Start Frame Number
pub const IVUSPullbackStartFrameNumber: u32 = 0x00183103;

/// (0018,3104) VR=IS, VM=1 IVUS Pullback Stop Frame Number
pub const IVUSPullbackStopFrameNumber: u32 = 0x00183104;

/// (0018,3105) VR=IS, VM=1-n Lesion Number
pub const LesionNumber: u32 = 0x00183105;

/// (0018,4000) VR=LT, VM=1 Acquisition Comments (retired)
pub const AcquisitionComments: u32 = 0x00184000;

/// (0018,5000) VR=SH, VM=1-n Output Power
pub const OutputPower: u32 = 0x00185000;

/// (0018,5010) VR=LO, VM=1-n Transducer Data
pub const TransducerData: u32 = 0x00185010;

/// (0018,5012) VR=DS, VM=1 Focus Depth
pub const FocusDepth: u32 = 0x00185012;

/// (0018,5020) VR=LO, VM=1 Processing Function
pub const ProcessingFunction: u32 = 0x00185020;

/// (0018,5021) VR=LO, VM=1 Postprocessing Function (retired)
pub const PostprocessingFunction: u32 = 0x00185021;

/// (0018,5022) VR=DS, VM=1 Mechanical Index
pub const MechanicalIndex: u32 = 0x00185022;

/// (0018,5024) VR=DS, VM=1 Bone Thermal Index
pub const BoneThermalIndex: u32 = 0x00185024;

/// (0018,5026) VR=DS, VM=1 Cranial Thermal Index
pub const CranialThermalIndex: u32 = 0x00185026;

/// (0018,5027) VR=DS, VM=1 Soft Tissue Thermal Index
pub const SoftTissueThermalIndex: u32 = 0x00185027;

/// (0018,5028) VR=DS, VM=1 Soft Tissue-focus Thermal Index
pub const SoftTissueFocusThermalIndex: u32 = 0x00185028;

/// (0018,5029) VR=DS, VM=1 Soft Tissue-surface Thermal Index
pub const SoftTissueSurfaceThermalIndex: u32 = 0x00185029;

/// (0018,5030) VR=DS, VM=1 Dynamic Range (retired)
pub const DynamicRange: u32 = 0x00185030;

/// (0018,5040) VR=DS, VM=1 Total Gain (retired)
pub const TotalGain: u32 = 0x00185040;

/// (0018,5050) VR=IS, VM=1 Depth of Scan Field
pub const DepthOfScanField: u32 = 0x00185050;

/// (0018,5100) VR=CS, VM=1 Patient Position
pub const PatientPosition: u32 = 0x00185100;

/// (0018,5101) VR=CS, VM=1 View Position
pub const ViewPosition: u32 = 0x00185101;

/// (0018,5104) VR=SQ, VM=1 Projection Eponymous Name Code Sequence
pub const ProjectionEponymousNameCodeSequence: u32 = 0x00185104;

/// (0018,5210) VR=DS, VM=6 Image Transformation Matrix (retired)
pub const ImageTransformationMatrix: u32 = 0x00185210;

/// (0018,5212) VR=DS, VM=3 Image Translation Vector (retired)
pub const ImageTranslationVector: u32 = 0x00185212;

/// (0018,6000) VR=DS, VM=1 Sensitivity
pub const Sensitivity: u32 = 0x00186000;

/// (0018,6011) VR=SQ, VM=1 Sequence of Ultrasound Regions
pub const SequenceOfUltrasoundRegions: u32 = 0x00186011;

/// (0018,6012) VR=US, VM=1 Region Spatial Format
pub const RegionSpatialFormat: u32 = 0x00186012;

/// (0018,6014) VR=US, VM=1 Region Data Type
pub const RegionDataType: u32 = 0x00186014;

/// (0018,6016) VR=UL, VM=1 Region Flags
pub const RegionFlags: u32 = 0x00186016;

/// (0018,6018) VR=UL, VM=1 Region Location Min X0
pub const RegionLocationMinX0: u32 = 0x00186018;

/// (0018,601A) VR=UL, VM=1 Region Location Min Y0
pub const RegionLocationMinY0: u32 = 0x0018601A;

/// (0018,601C) VR=UL, VM=1 Region Location Max X1
pub const RegionLocationMaxX1: u32 = 0x0018601C;

/// (0018,601E) VR=UL, VM=1 Region Location Max Y1
pub const RegionLocationMaxY1: u32 = 0x0018601E;

/// (0018,6020) VR=SL, VM=1 Reference Pixel X0
pub const ReferencePixelX0: u32 = 0x00186020;

/// (0018,6022) VR=SL, VM=1 Reference Pixel Y0
pub const ReferencePixelY0: u32 = 0x00186022;

/// (0018,6024) VR=US, VM=1 Physical Units X Direction
pub const PhysicalUnitsXDirection: u32 = 0x00186024;

/// (0018,6026) VR=US, VM=1 Physical Units Y Direction
pub const PhysicalUnitsYDirection: u32 = 0x00186026;

/// (0018,6028) VR=FD, VM=1 Reference Pixel Physical Value X
pub const ReferencePixelPhysicalValueX: u32 = 0x00186028;

/// (0018,602A) VR=FD, VM=1 Reference Pixel Physical Value Y
pub const ReferencePixelPhysicalValueY: u32 = 0x0018602A;

/// (0018,602C) VR=FD, VM=1 Physical Delta X
pub const PhysicalDeltaX: u32 = 0x0018602C;

/// (0018,602E) VR=FD, VM=1 Physical Delta Y
pub const PhysicalDeltaY: u32 = 0x0018602E;

/// (0018,6030) VR=UL, VM=1 Transducer Frequency
pub const TransducerFrequency: u32 = 0x00186030;

/// (0018,6031) VR=CS, VM=1 Transducer Type
pub const TransducerType: u32 = 0x00186031;

/// (0018,6032) VR=UL, VM=1 Pulse Repetition Frequency
pub const PulseRepetitionFrequency: u32 = 0x00186032;

/// (0018,6034) VR=FD, VM=1 Doppler Correction Angle
pub const DopplerCorrectionAngle: u32 = 0x00186034;

/// (0018,6036) VR=FD, VM=1 Steering Angle
pub const SteeringAngle: u32 = 0x00186036;

/// (0018,6038) VR=UL, VM=1 Doppler Sample Volume X Position (Retired) (retired)
pub const DopplerSampleVolumeXPositionRetired: u32 = 0x00186038;

/// (0018,6039) VR=SL, VM=1 Doppler Sample Volume X Position
pub const DopplerSampleVolumeXPosition: u32 = 0x00186039;

/// (0018,603A) VR=UL, VM=1 Doppler Sample Volume Y Position (Retired) (retired)
pub const DopplerSampleVolumeYPositionRetired: u32 = 0x0018603A;

/// (0018,603B) VR=SL, VM=1 Doppler Sample Volume Y Position
pub const DopplerSampleVolumeYPosition: u32 = 0x0018603B;

/// (0018,603C) VR=UL, VM=1 TM-Line Position X0 (Retired) (retired)
pub const TMLinePositionX0Retired: u32 = 0x0018603C;

/// (0018,603D) VR=SL, VM=1 TM-Line Position X0
pub const TMLinePositionX0: u32 = 0x0018603D;

/// (0018,603E) VR=UL, VM=1 TM-Line Position Y0 (Retired) (retired)
pub const TMLinePositionY0Retired: u32 = 0x0018603E;

/// (0018,603F) VR=SL, VM=1 TM-Line Position Y0
pub const TMLinePositionY0: u32 = 0x0018603F;

/// (0018,6040) VR=UL, VM=1 TM-Line Position X1 (Retired) (retired)
pub const TMLinePositionX1Retired: u32 = 0x00186040;

/// (0018,6041) VR=SL, VM=1 TM-Line Position X1
pub const TMLinePositionX1: u32 = 0x00186041;

/// (0018,6042) VR=UL, VM=1 TM-Line Position Y1 (Retired) (retired)
pub const TMLinePositionY1Retired: u32 = 0x00186042;

/// (0018,6043) VR=SL, VM=1 TM-Line Position Y1
pub const TMLinePositionY1: u32 = 0x00186043;

/// (0018,6044) VR=US, VM=1 Pixel Component Organization
pub const PixelComponentOrganization: u32 = 0x00186044;

/// (0018,6046) VR=UL, VM=1 Pixel Component Mask
pub const PixelComponentMask: u32 = 0x00186046;

/// (0018,6048) VR=UL, VM=1 Pixel Component Range Start
pub const PixelComponentRangeStart: u32 = 0x00186048;

/// (0018,604A) VR=UL, VM=1 Pixel Component Range Stop
pub const PixelComponentRangeStop: u32 = 0x0018604A;

/// (0018,604C) VR=US, VM=1 Pixel Component Physical Units
pub const PixelComponentPhysicalUnits: u32 = 0x0018604C;

/// (0018,604E) VR=US, VM=1 Pixel Component Data Type
pub const PixelComponentDataType: u32 = 0x0018604E;

/// (0018,6050) VR=UL, VM=1 Number of Table Break Points
pub const NumberOfTableBreakPoints: u32 = 0x00186050;

/// (0018,6052) VR=UL, VM=1-n Table of X Break Points
pub const TableOfXBreakPoints: u32 = 0x00186052;

/// (0018,6054) VR=FD, VM=1-n Table of Y Break Points
pub const TableOfYBreakPoints: u32 = 0x00186054;

/// (0018,6056) VR=UL, VM=1 Number of Table Entries
pub const NumberOfTableEntries: u32 = 0x00186056;

/// (0018,6058) VR=UL, VM=1-n Table of Pixel Values
pub const TableOfPixelValues: u32 = 0x00186058;

/// (0018,605A) VR=FL, VM=1-n Table of Parameter Values
pub const TableOfParameterValues: u32 = 0x0018605A;

/// (0018,6060) VR=FL, VM=1-n R Wave Time Vector
pub const RWaveTimeVector: u32 = 0x00186060;

/// (0018,7000) VR=CS, VM=1 Detector Conditions Nominal Flag
pub const DetectorConditionsNominalFlag: u32 = 0x00187000;

/// (0018,7001) VR=DS, VM=1 Detector Temperature
pub const DetectorTemperature: u32 = 0x00187001;

/// (0018,7004) VR=CS, VM=1 Detector Type
pub const DetectorType: u32 = 0x00187004;

/// (0018,7005) VR=CS, VM=1 Detector Configuration
pub const DetectorConfiguration: u32 = 0x00187005;

/// (0018,7006) VR=LT, VM=1 Detector Description
pub const DetectorDescription: u32 = 0x00187006;

/// (0018,7008) VR=LT, VM=1 Detector Mode
pub const DetectorMode: u32 = 0x00187008;

/// (0018,700A) VR=SH, VM=1 Detector ID
pub const DetectorID: u32 = 0x0018700A;

/// (0018,700C) VR=DA, VM=1 Date of Last Detector Calibration
pub const DateOfLastDetectorCalibration: u32 = 0x0018700C;

/// (0018,700E) VR=TM, VM=1 Time of Last Detector Calibration
pub const TimeOfLastDetectorCalibration: u32 = 0x0018700E;

/// (0018,7010) VR=IS, VM=1 Exposures on Detector Since Last Calibration
pub const ExposuresOnDetectorSinceLastCalibration: u32 = 0x00187010;

/// (0018,7011) VR=IS, VM=1 Exposures on Detector Since Manufactured
pub const ExposuresOnDetectorSinceManufactured: u32 = 0x00187011;

/// (0018,7012) VR=DS, VM=1 Detector Time Since Last Exposure
pub const DetectorTimeSinceLastExposure: u32 = 0x00187012;

/// (0018,7014) VR=DS, VM=1 Detector Active Time
pub const DetectorActiveTime: u32 = 0x00187014;

/// (0018,7016) VR=DS, VM=1 Detector Activation Offset From Exposure
pub const DetectorActivationOffsetFromExposure: u32 = 0x00187016;

/// (0018,701A) VR=DS, VM=2 Detector Binning
pub const DetectorBinning: u32 = 0x0018701A;

/// (0018,7020) VR=DS, VM=2 Detector Element Physical Size
pub const DetectorElementPhysicalSize: u32 = 0x00187020;

/// (0018,7022) VR=DS, VM=2 Detector Element Spacing
pub const DetectorElementSpacing: u32 = 0x00187022;

/// (0018,7024) VR=CS, VM=1 Detector Active Shape
pub const DetectorActiveShape: u32 = 0x00187024;

/// (0018,7026) VR=DS, VM=1-2 Detector Active Dimension(s)
pub const DetectorActiveDimensions: u32 = 0x00187026;

/// (0018,7028) VR=DS, VM=2 Detector Active Origin
pub const DetectorActiveOrigin: u32 = 0x00187028;

/// (0018,702A) VR=LO, VM=1 Detector Manufacturer Name
pub const DetectorManufacturerName: u32 = 0x0018702A;

/// (0018,702B) VR=LO, VM=1 Detector Manufacturer’s Model Name
pub const DetectorManufacturerModelName: u32 = 0x0018702B;

/// (0018,7030) VR=DS, VM=2 Field of View Origin
pub const FieldOfViewOrigin: u32 = 0x00187030;

/// (0018,7032) VR=DS, VM=1 Field of View Rotation
pub const FieldOfViewRotation: u32 = 0x00187032;

/// (0018,7034) VR=CS, VM=1 Field of View Horizontal Flip
pub const FieldOfViewHorizontalFlip: u32 = 0x00187034;

/// (0018,7036) VR=FL, VM=2 Pixel Data Area Origin Relative To FOV
pub const PixelDataAreaOriginRelativeToFOV: u32 = 0x00187036;

/// (0018,7038) VR=FL, VM=1 Pixel Data Area Rotation Angle Relative To FOV
pub const PixelDataAreaRotationAngleRelativeToFOV: u32 = 0x00187038;

/// (0018,7040) VR=LT, VM=1 Grid Absorbing Material
pub const GridAbsorbingMaterial: u32 = 0x00187040;

/// (0018,7041) VR=LT, VM=1 Grid Spacing Material
pub const GridSpacingMaterial: u32 = 0x00187041;

/// (0018,7042) VR=DS, VM=1 Grid Thickness
pub const GridThickness: u32 = 0x00187042;

/// (0018,7044) VR=DS, VM=1 Grid Pitch
pub const GridPitch: u32 = 0x00187044;

/// (0018,7046) VR=IS, VM=2 Grid Aspect Ratio
pub const GridAspectRatio: u32 = 0x00187046;

/// (0018,7048) VR=DS, VM=1 Grid Period
pub const GridPeriod: u32 = 0x00187048;

/// (0018,704C) VR=DS, VM=1 Grid Focal Distance
pub const GridFocalDistance: u32 = 0x0018704C;

/// (0018,7050) VR=CS, VM=1-n Filter Material
pub const FilterMaterial: u32 = 0x00187050;

/// (0018,7052) VR=DS, VM=1-n Filter Thickness Minimum
pub const FilterThicknessMinimum: u32 = 0x00187052;

/// (0018,7054) VR=DS, VM=1-n Filter Thickness Maximum
pub const FilterThicknessMaximum: u32 = 0x00187054;

/// (0018,7056) VR=FL, VM=1-n Filter Beam Path Length Minimum
pub const FilterBeamPathLengthMinimum: u32 = 0x00187056;

/// (0018,7058) VR=FL, VM=1-n Filter Beam Path Length Maximum
pub const FilterBeamPathLengthMaximum: u32 = 0x00187058;

/// (0018,7060) VR=CS, VM=1 Exposure Control Mode
pub const ExposureControlMode: u32 = 0x00187060;

/// (0018,7062) VR=LT, VM=1 Exposure Control Mode Description
pub const ExposureControlModeDescription: u32 = 0x00187062;

/// (0018,7064) VR=CS, VM=1 Exposure Status
pub const ExposureStatus: u32 = 0x00187064;

/// (0018,7065) VR=DS, VM=1 Phototimer Setting
pub const PhototimerSetting: u32 = 0x00187065;

/// (0018,8150) VR=DS, VM=1 Exposure Time in µS
pub const ExposureTimeInuS: u32 = 0x00188150;

/// (0018,8151) VR=DS, VM=1 X-Ray Tube Current in µA
pub const XRayTubeCurrentInuA: u32 = 0x00188151;

/// (0018,9004) VR=CS, VM=1 Content Qualification
pub const ContentQualification: u32 = 0x00189004;

/// (0018,9005) VR=SH, VM=1 Pulse Sequence Name
pub const PulseSequenceName: u32 = 0x00189005;

/// (0018,9006) VR=SQ, VM=1 MR Imaging Modifier Sequence
pub const MRImagingModifierSequence: u32 = 0x00189006;

/// (0018,9008) VR=CS, VM=1 Echo Pulse Sequence
pub const EchoPulseSequence: u32 = 0x00189008;

/// (0018,9009) VR=CS, VM=1 Inversion Recovery
pub const InversionRecovery: u32 = 0x00189009;

/// (0018,9010) VR=CS, VM=1 Flow Compensation
pub const FlowCompensation: u32 = 0x00189010;

/// (0018,9011) VR=CS, VM=1 Multiple Spin Echo
pub const MultipleSpinEcho: u32 = 0x00189011;

/// (0018,9012) VR=CS, VM=1 Multi-planar Excitation
pub const MultiPlanarExcitation: u32 = 0x00189012;

/// (0018,9014) VR=CS, VM=1 Phase Contrast
pub const PhaseContrast: u32 = 0x00189014;

/// (0018,9015) VR=CS, VM=1 Time of Flight Contrast
pub const TimeOfFlightContrast: u32 = 0x00189015;

/// (0018,9016) VR=CS, VM=1 Spoiling
pub const Spoiling: u32 = 0x00189016;

/// (0018,9017) VR=CS, VM=1 Steady State Pulse Sequence
pub const SteadyStatePulseSequence: u32 = 0x00189017;

/// (0018,9018) VR=CS, VM=1 Echo Planar Pulse Sequence
pub const EchoPlanarPulseSequence: u32 = 0x00189018;

/// (0018,9019) VR=FD, VM=1 Tag Angle First Axis
pub const TagAngleFirstAxis: u32 = 0x00189019;

/// (0018,9020) VR=CS, VM=1 Magnetization Transfer
pub const MagnetizationTransfer: u32 = 0x00189020;

/// (0018,9021) VR=CS, VM=1 T2 Preparation
pub const T2Preparation: u32 = 0x00189021;

/// (0018,9022) VR=CS, VM=1 Blood Signal Nulling
pub const BloodSignalNulling: u32 = 0x00189022;

/// (0018,9024) VR=CS, VM=1 Saturation Recovery
pub const SaturationRecovery: u32 = 0x00189024;

/// (0018,9025) VR=CS, VM=1 Spectrally Selected Suppression
pub const SpectrallySelectedSuppression: u32 = 0x00189025;

/// (0018,9026) VR=CS, VM=1 Spectrally Selected Excitation
pub const SpectrallySelectedExcitation: u32 = 0x00189026;

/// (0018,9027) VR=CS, VM=1 Spatial Pre-saturation
pub const SpatialPresaturation: u32 = 0x00189027;

/// (0018,9028) VR=CS, VM=1 Tagging
pub const Tagging: u32 = 0x00189028;

/// (0018,9029) VR=CS, VM=1 Oversampling Phase
pub const OversamplingPhase: u32 = 0x00189029;

/// (0018,9030) VR=FD, VM=1 Tag Spacing First Dimension
pub const TagSpacingFirstDimension: u32 = 0x00189030;

/// (0018,9032) VR=CS, VM=1 Geometry of k-Space Traversal
pub const GeometryOfKSpaceTraversal: u32 = 0x00189032;

/// (0018,9033) VR=CS, VM=1 Segmented k-Space Traversal
pub const SegmentedKSpaceTraversal: u32 = 0x00189033;

/// (0018,9034) VR=CS, VM=1 Rectilinear Phase Encode Reordering
pub const RectilinearPhaseEncodeReordering: u32 = 0x00189034;

/// (0018,9035) VR=FD, VM=1 Tag Thickness
pub const TagThickness: u32 = 0x00189035;

/// (0018,9036) VR=CS, VM=1 Partial Fourier Direction
pub const PartialFourierDirection: u32 = 0x00189036;

/// (0018,9037) VR=CS, VM=1 Cardiac Synchronization Technique
pub const CardiacSynchronizationTechnique: u32 = 0x00189037;

/// (0018,9041) VR=LO, VM=1 Receive Coil Manufacturer Name
pub const ReceiveCoilManufacturerName: u32 = 0x00189041;

/// (0018,9042) VR=SQ, VM=1 MR Receive Coil Sequence
pub const MRReceiveCoilSequence: u32 = 0x00189042;

/// (0018,9043) VR=CS, VM=1 Receive Coil Type
pub const ReceiveCoilType: u32 = 0x00189043;

/// (0018,9044) VR=CS, VM=1 Quadrature Receive Coil
pub const QuadratureReceiveCoil: u32 = 0x00189044;

/// (0018,9045) VR=SQ, VM=1 Multi-Coil Definition Sequence
pub const MultiCoilDefinitionSequence: u32 = 0x00189045;

/// (0018,9046) VR=LO, VM=1 Multi-Coil Configuration
pub const MultiCoilConfiguration: u32 = 0x00189046;

/// (0018,9047) VR=SH, VM=1 Multi-Coil Element Name
pub const MultiCoilElementName: u32 = 0x00189047;

/// (0018,9048) VR=CS, VM=1 Multi-Coil Element Used
pub const MultiCoilElementUsed: u32 = 0x00189048;

/// (0018,9049) VR=SQ, VM=1 MR Transmit Coil Sequence
pub const MRTransmitCoilSequence: u32 = 0x00189049;

/// (0018,9050) VR=LO, VM=1 Transmit Coil Manufacturer Name
pub const TransmitCoilManufacturerName: u32 = 0x00189050;

/// (0018,9051) VR=CS, VM=1 Transmit Coil Type
pub const TransmitCoilType: u32 = 0x00189051;

/// (0018,9052) VR=FD, VM=1-2 Spectral Width
pub const SpectralWidth: u32 = 0x00189052;

/// (0018,9053) VR=FD, VM=1-2 Chemical Shift Reference
pub const ChemicalShiftReference: u32 = 0x00189053;

/// (0018,9054) VR=CS, VM=1 Volume Localization Technique
pub const VolumeLocalizationTechnique: u32 = 0x00189054;

/// (0018,9058) VR=US, VM=1 MR Acquisition Frequency Encoding Steps
pub const MRAcquisitionFrequencyEncodingSteps: u32 = 0x00189058;

/// (0018,9059) VR=CS, VM=1 De-coupling
pub const Decoupling: u32 = 0x00189059;

/// (0018,9060) VR=CS, VM=1-2 De-coupled Nucleus
pub const DecoupledNucleus: u32 = 0x00189060;

/// (0018,9061) VR=FD, VM=1-2 De-coupling Frequency
pub const DecouplingFrequency: u32 = 0x00189061;

/// (0018,9062) VR=CS, VM=1 De-coupling Method
pub const DecouplingMethod: u32 = 0x00189062;

/// (0018,9063) VR=FD, VM=1-2 De-coupling Chemical Shift Reference
pub const DecouplingChemicalShiftReference: u32 = 0x00189063;

/// (0018,9064) VR=CS, VM=1 k-space Filtering
pub const KSpaceFiltering: u32 = 0x00189064;

/// (0018,9065) VR=CS, VM=1-2 Time Domain Filtering
pub const TimeDomainFiltering: u32 = 0x00189065;

/// (0018,9066) VR=US, VM=1-2 Number of Zero Fills
pub const NumberOfZeroFills: u32 = 0x00189066;

/// (0018,9067) VR=CS, VM=1 Baseline Correction
pub const BaselineCorrection: u32 = 0x00189067;

/// (0018,9069) VR=FD, VM=1 Parallel Reduction Factor In-plane
pub const ParallelReductionFactorInPlane: u32 = 0x00189069;

/// (0018,9070) VR=FD, VM=1 Cardiac R-R Interval Specified
pub const CardiacRRIntervalSpecified: u32 = 0x00189070;

/// (0018,9073) VR=FD, VM=1 Acquisition Duration
pub const AcquisitionDuration: u32 = 0x00189073;

/// (0018,9074) VR=DT, VM=1 Frame Acquisition DateTime
pub const FrameAcquisitionDateTime: u32 = 0x00189074;

/// (0018,9075) VR=CS, VM=1 Diffusion Directionality
pub const DiffusionDirectionality: u32 = 0x00189075;

/// (0018,9076) VR=SQ, VM=1 Diffusion Gradient Direction Sequence
pub const DiffusionGradientDirectionSequence: u32 = 0x00189076;

/// (0018,9077) VR=CS, VM=1 Parallel Acquisition
pub const ParallelAcquisition: u32 = 0x00189077;

/// (0018,9078) VR=CS, VM=1 Parallel Acquisition Technique
pub const ParallelAcquisitionTechnique: u32 = 0x00189078;

/// (0018,9079) VR=FD, VM=1-n Inversion Times
pub const InversionTimes: u32 = 0x00189079;

/// (0018,9080) VR=ST, VM=1 Metabolite Map Description
pub const MetaboliteMapDescription: u32 = 0x00189080;

/// (0018,9081) VR=CS, VM=1 Partial Fourier
pub const PartialFourier: u32 = 0x00189081;

/// (0018,9082) VR=FD, VM=1 Effective Echo Time
pub const EffectiveEchoTime: u32 = 0x00189082;

/// (0018,9083) VR=SQ, VM=1 Metabolite Map Code Sequence
pub const MetaboliteMapCodeSequence: u32 = 0x00189083;

/// (0018,9084) VR=SQ, VM=1 Chemical Shift Sequence
pub const ChemicalShiftSequence: u32 = 0x00189084;

/// (0018,9085) VR=CS, VM=1 Cardiac Signal Source
pub const CardiacSignalSource: u32 = 0x00189085;

/// (0018,9087) VR=FD, VM=1 Diffusion b-value
pub const DiffusionBValue: u32 = 0x00189087;

/// (0018,9089) VR=FD, VM=3 Diffusion Gradient Orientation
pub const DiffusionGradientOrientation: u32 = 0x00189089;

/// (0018,9090) VR=FD, VM=3 Velocity Encoding Direction
pub const VelocityEncodingDirection: u32 = 0x00189090;

/// (0018,9091) VR=FD, VM=1 Velocity Encoding Minimum Value
pub const VelocityEncodingMinimumValue: u32 = 0x00189091;

/// (0018,9092) VR=SQ, VM=1 Velocity Encoding Acquisition Sequence
pub const VelocityEncodingAcquisitionSequence: u32 = 0x00189092;

/// (0018,9093) VR=US, VM=1 Number of k-Space Trajectories
pub const NumberOfKSpaceTrajectories: u32 = 0x00189093;

/// (0018,9094) VR=CS, VM=1 Coverage of k-Space
pub const CoverageOfKSpace: u32 = 0x00189094;

/// (0018,9095) VR=UL, VM=1 Spectroscopy Acquisition Phase Rows
pub const SpectroscopyAcquisitionPhaseRows: u32 = 0x00189095;

/// (0018,9096) VR=FD, VM=1 Parallel Reduction Factor In-plane (Retired) (retired)
pub const ParallelReductionFactorInPlaneRetired: u32 = 0x00189096;

/// (0018,9098) VR=FD, VM=1-2 Transmitter Frequency
pub const TransmitterFrequency: u32 = 0x00189098;

/// (0018,9100) VR=CS, VM=1-2 Resonant Nucleus
pub const ResonantNucleus: u32 = 0x00189100;

/// (0018,9101) VR=CS, VM=1 Frequency Correction
pub const FrequencyCorrection: u32 = 0x00189101;

/// (0018,9103) VR=SQ, VM=1 MR Spectroscopy FOV/Geometry Sequence
pub const MRSpectroscopyFOVGeometrySequence: u32 = 0x00189103;

/// (0018,9104) VR=FD, VM=1 Slab Thickness
pub const SlabThickness: u32 = 0x00189104;

/// (0018,9105) VR=FD, VM=3 Slab Orientation
pub const SlabOrientation: u32 = 0x00189105;

/// (0018,9106) VR=FD, VM=3 Mid Slab Position
pub const MidSlabPosition: u32 = 0x00189106;

/// (0018,9107) VR=SQ, VM=1 MR Spatial Saturation Sequence
pub const MRSpatialSaturationSequence: u32 = 0x00189107;

/// (0018,9112) VR=SQ, VM=1 MR Timing and Related Parameters Sequence
pub const MRTimingAndRelatedParametersSequence: u32 = 0x00189112;

/// (0018,9114) VR=SQ, VM=1 MR Echo Sequence
pub const MREchoSequence: u32 = 0x00189114;

/// (0018,9115) VR=SQ, VM=1 MR Modifier Sequence
pub const MRModifierSequence: u32 = 0x00189115;

/// (0018,9117) VR=SQ, VM=1 MR Diffusion Sequence
pub const MRDiffusionSequence: u32 = 0x00189117;

/// (0018,9118) VR=SQ, VM=1 Cardiac Synchronization Sequence
pub const CardiacSynchronizationSequence: u32 = 0x00189118;

/// (0018,9119) VR=SQ, VM=1 MR Averages Sequence
pub const MRAveragesSequence: u32 = 0x00189119;

/// (0018,9125) VR=SQ, VM=1 MR FOV/Geometry Sequence
pub const MRFOVGeometrySequence: u32 = 0x00189125;

/// (0018,9126) VR=SQ, VM=1 Volume Localization Sequence
pub const VolumeLocalizationSequence: u32 = 0x00189126;

/// (0018,9127) VR=UL, VM=1 Spectroscopy Acquisition Data Columns
pub const SpectroscopyAcquisitionDataColumns: u32 = 0x00189127;

/// (0018,9147) VR=CS, VM=1 Diffusion Anisotropy Type
pub const DiffusionAnisotropyType: u32 = 0x00189147;

/// (0018,9151) VR=DT, VM=1 Frame Reference DateTime
pub const FrameReferenceDateTime: u32 = 0x00189151;

/// (0018,9152) VR=SQ, VM=1 MR Metabolite Map Sequence
pub const MRMetaboliteMapSequence: u32 = 0x00189152;

/// (0018,9155) VR=FD, VM=1 Parallel Reduction Factor out-of-plane
pub const ParallelReductionFactorOutOfPlane: u32 = 0x00189155;

/// (0018,9159) VR=UL, VM=1 Spectroscopy Acquisition Out-of-plane Phase Steps
pub const SpectroscopyAcquisitionOutOfPlanePhaseSteps: u32 = 0x00189159;

/// (0018,9166) VR=CS, VM=1 Bulk Motion Status (retired)
pub const BulkMotionStatus: u32 = 0x00189166;

/// (0018,9168) VR=FD, VM=1 Parallel Reduction Factor Second In-plane
pub const ParallelReductionFactorSecondInPlane: u32 = 0x00189168;

/// (0018,9169) VR=CS, VM=1 Cardiac Beat Rejection Technique
pub const CardiacBeatRejectionTechnique: u32 = 0x00189169;

/// (0018,9170) VR=CS, VM=1 Respiratory Motion Compensation Technique
pub const RespiratoryMotionCompensationTechnique: u32 = 0x00189170;

/// (0018,9171) VR=CS, VM=1 Respiratory Signal Source
pub const RespiratorySignalSource: u32 = 0x00189171;

/// (0018,9172) VR=CS, VM=1 Bulk Motion Compensation Technique
pub const BulkMotionCompensationTechnique: u32 = 0x00189172;

/// (0018,9173) VR=CS, VM=1 Bulk Motion Signal Source
pub const BulkMotionSignalSource: u32 = 0x00189173;

/// (0018,9174) VR=CS, VM=1 Applicable Safety Standard Agency
pub const ApplicableSafetyStandardAgency: u32 = 0x00189174;

/// (0018,9175) VR=LO, VM=1 Applicable Safety Standard Description
pub const ApplicableSafetyStandardDescription: u32 = 0x00189175;

/// (0018,9176) VR=SQ, VM=1 Operating Mode Sequence
pub const OperatingModeSequence: u32 = 0x00189176;

/// (0018,9177) VR=CS, VM=1 Operating Mode Type
pub const OperatingModeType: u32 = 0x00189177;

/// (0018,9178) VR=CS, VM=1 Operating Mode
pub const OperatingMode: u32 = 0x00189178;

/// (0018,9179) VR=CS, VM=1 Specific Absorption Rate Definition
pub const SpecificAbsorptionRateDefinition: u32 = 0x00189179;

/// (0018,9180) VR=CS, VM=1 Gradient Output Type
pub const GradientOutputType: u32 = 0x00189180;

/// (0018,9181) VR=FD, VM=1 Specific Absorption Rate Value
pub const SpecificAbsorptionRateValue: u32 = 0x00189181;

/// (0018,9182) VR=FD, VM=1 Gradient Output
pub const GradientOutput: u32 = 0x00189182;

/// (0018,9183) VR=CS, VM=1 Flow Compensation Direction
pub const FlowCompensationDirection: u32 = 0x00189183;

/// (0018,9184) VR=FD, VM=1 Tagging Delay
pub const TaggingDelay: u32 = 0x00189184;

/// (0018,9185) VR=ST, VM=1 Respiratory Motion Compensation Technique Description
pub const RespiratoryMotionCompensationTechniqueDescription: u32 = 0x00189185;

/// (0018,9186) VR=SH, VM=1 Respiratory Signal Source ID
pub const RespiratorySignalSourceID: u32 = 0x00189186;

/// (0018,9195) VR=FD, VM=1 Chemical Shift Minimum Integration Limit in Hz (retired)
pub const ChemicalShiftMinimumIntegrationLimitInHz: u32 = 0x00189195;

/// (0018,9196) VR=FD, VM=1 Chemical Shift Maximum Integration Limit in Hz (retired)
pub const ChemicalShiftMaximumIntegrationLimitInHz: u32 = 0x00189196;

/// (0018,9197) VR=SQ, VM=1 MR Velocity Encoding Sequence
pub const MRVelocityEncodingSequence: u32 = 0x00189197;

/// (0018,9198) VR=CS, VM=1 First Order Phase Correction
pub const FirstOrderPhaseCorrection: u32 = 0x00189198;

/// (0018,9199) VR=CS, VM=1 Water Referenced Phase Correction
pub const WaterReferencedPhaseCorrection: u32 = 0x00189199;

/// (0018,9200) VR=CS, VM=1 MR Spectroscopy Acquisition Type
pub const MRSpectroscopyAcquisitionType: u32 = 0x00189200;

/// (0018,9214) VR=CS, VM=1 Respiratory Cycle Position
pub const RespiratoryCyclePosition: u32 = 0x00189214;

/// (0018,9217) VR=FD, VM=1 Velocity Encoding Maximum Value
pub const VelocityEncodingMaximumValue: u32 = 0x00189217;

/// (0018,9218) VR=FD, VM=1 Tag Spacing Second Dimension
pub const TagSpacingSecondDimension: u32 = 0x00189218;

/// (0018,9219) VR=SS, VM=1 Tag Angle Second Axis
pub const TagAngleSecondAxis: u32 = 0x00189219;

/// (0018,9220) VR=FD, VM=1 Frame Acquisition Duration
pub const FrameAcquisitionDuration: u32 = 0x00189220;

/// (0018,9226) VR=SQ, VM=1 MR Image Frame Type Sequence
pub const MRImageFrameTypeSequence: u32 = 0x00189226;

/// (0018,9227) VR=SQ, VM=1 MR Spectroscopy Frame Type Sequence
pub const MRSpectroscopyFrameTypeSequence: u32 = 0x00189227;

/// (0018,9231) VR=US, VM=1 MR Acquisition Phase Encoding Steps in-plane
pub const MRAcquisitionPhaseEncodingStepsInPlane: u32 = 0x00189231;

/// (0018,9232) VR=US, VM=1 MR Acquisition Phase Encoding Steps out-of-plane
pub const MRAcquisitionPhaseEncodingStepsOutOfPlane: u32 = 0x00189232;

/// (0018,9234) VR=UL, VM=1 Spectroscopy Acquisition Phase Columns
pub const SpectroscopyAcquisitionPhaseColumns: u32 = 0x00189234;

/// (0018,9236) VR=CS, VM=1 Cardiac Cycle Position
pub const CardiacCyclePosition: u32 = 0x00189236;

/// (0018,9239) VR=SQ, VM=1 Specific Absorption Rate Sequence
pub const SpecificAbsorptionRateSequence: u32 = 0x00189239;

/// (0018,9240) VR=US, VM=1 RF Echo Train Length
pub const RFEchoTrainLength: u32 = 0x00189240;

/// (0018,9241) VR=US, VM=1 Gradient Echo Train Length
pub const GradientEchoTrainLength: u32 = 0x00189241;

/// (0018,9250) VR=CS, VM=1 Arterial Spin Labeling Contrast
pub const ArterialSpinLabelingContrast: u32 = 0x00189250;

/// (0018,9251) VR=SQ, VM=1 MR Arterial Spin Labeling Sequence
pub const MRArterialSpinLabelingSequence: u32 = 0x00189251;

/// (0018,9252) VR=LO, VM=1 ASL Technique Description
pub const ASLTechniqueDescription: u32 = 0x00189252;

/// (0018,9253) VR=US, VM=1 ASL Slab Number
pub const ASLSlabNumber: u32 = 0x00189253;

/// (0018,9254) VR=FD, VM=1  ASL Slab Thickness
pub const ASLSlabThickness: u32 = 0x00189254;

/// (0018,9255) VR=FD, VM=3  ASL Slab Orientation
pub const ASLSlabOrientation: u32 = 0x00189255;

/// (0018,9256) VR=FD, VM=3 ASL Mid Slab Position
pub const ASLMidSlabPosition: u32 = 0x00189256;

/// (0018,9257) VR=CS, VM=1  ASL Context
pub const ASLContext: u32 = 0x00189257;

/// (0018,9258) VR=UL, VM=1 ASL Pulse Train Duration
pub const ASLPulseTrainDuration: u32 = 0x00189258;

/// (0018,9259) VR=CS, VM=1  ASL Crusher Flag
pub const ASLCrusherFlag: u32 = 0x00189259;

/// (0018,925A) VR=FD, VM=1 ASL Crusher Flow
pub const ASLCrusherFlow: u32 = 0x0018925A;

/// (0018,925B) VR=LO, VM=1 ASL Crusher Description
pub const ASLCrusherDescription: u32 = 0x0018925B;

/// (0018,925C) VR=CS, VM=1  ASL Bolus Cut-off Flag
pub const ASLBolusCutoffFlag: u32 = 0x0018925C;

/// (0018,925D) VR=SQ, VM=1 ASL Bolus Cut-off Timing Sequence
pub const ASLBolusCutoffTimingSequence: u32 = 0x0018925D;

/// (0018,925E) VR=LO, VM=1 ASL Bolus Cut-off Technique
pub const ASLBolusCutoffTechnique: u32 = 0x0018925E;

/// (0018,925F) VR=UL, VM=1 ASL Bolus Cut-off Delay Time
pub const ASLBolusCutoffDelayTime: u32 = 0x0018925F;

/// (0018,9260) VR=SQ, VM=1 ASL Slab Sequence
pub const ASLSlabSequence: u32 = 0x00189260;

/// (0018,9295) VR=FD, VM=1 Chemical Shift Minimum Integration Limit in ppm
pub const ChemicalShiftMinimumIntegrationLimitInppm: u32 = 0x00189295;

/// (0018,9296) VR=FD, VM=1 Chemical Shift Maximum Integration Limit in ppm
pub const ChemicalShiftMaximumIntegrationLimitInppm: u32 = 0x00189296;

/// (0018,9301) VR=SQ, VM=1 CT Acquisition Type Sequence
pub const CTAcquisitionTypeSequence: u32 = 0x00189301;

/// (0018,9302) VR=CS, VM=1 Acquisition Type
pub const AcquisitionType: u32 = 0x00189302;

/// (0018,9303) VR=FD, VM=1 Tube Angle
pub const TubeAngle: u32 = 0x00189303;

/// (0018,9304) VR=SQ, VM=1 CT Acquisition Details Sequence
pub const CTAcquisitionDetailsSequence: u32 = 0x00189304;

/// (0018,9305) VR=FD, VM=1 Revolution Time
pub const RevolutionTime: u32 = 0x00189305;

/// (0018,9306) VR=FD, VM=1 Single Collimation Width
pub const SingleCollimationWidth: u32 = 0x00189306;

/// (0018,9307) VR=FD, VM=1 Total Collimation Width
pub const TotalCollimationWidth: u32 = 0x00189307;

/// (0018,9308) VR=SQ, VM=1 CT Table Dynamics Sequence
pub const CTTableDynamicsSequence: u32 = 0x00189308;

/// (0018,9309) VR=FD, VM=1 Table Speed
pub const TableSpeed: u32 = 0x00189309;

/// (0018,9310) VR=FD, VM=1 Table Feed per Rotation
pub const TableFeedPerRotation: u32 = 0x00189310;

/// (0018,9311) VR=FD, VM=1 Spiral Pitch Factor
pub const SpiralPitchFactor: u32 = 0x00189311;

/// (0018,9312) VR=SQ, VM=1 CT Geometry Sequence
pub const CTGeometrySequence: u32 = 0x00189312;

/// (0018,9313) VR=FD, VM=3 Data Collection Center (Patient)
pub const DataCollectionCenterPatient: u32 = 0x00189313;

/// (0018,9314) VR=SQ, VM=1 CT Reconstruction Sequence
pub const CTReconstructionSequence: u32 = 0x00189314;

/// (0018,9315) VR=CS, VM=1 Reconstruction Algorithm
pub const ReconstructionAlgorithm: u32 = 0x00189315;

/// (0018,9316) VR=CS, VM=1 Convolution Kernel Group
pub const ConvolutionKernelGroup: u32 = 0x00189316;

/// (0018,9317) VR=FD, VM=2 Reconstruction Field of View
pub const ReconstructionFieldOfView: u32 = 0x00189317;

/// (0018,9318) VR=FD, VM=3 Reconstruction Target Center (Patient)
pub const ReconstructionTargetCenterPatient: u32 = 0x00189318;

/// (0018,9319) VR=FD, VM=1 Reconstruction Angle
pub const ReconstructionAngle: u32 = 0x00189319;

/// (0018,9320) VR=SH, VM=1 Image Filter
pub const ImageFilter: u32 = 0x00189320;

/// (0018,9321) VR=SQ, VM=1 CT Exposure Sequence
pub const CTExposureSequence: u32 = 0x00189321;

/// (0018,9322) VR=FD, VM=2 Reconstruction Pixel Spacing
pub const ReconstructionPixelSpacing: u32 = 0x00189322;

/// (0018,9323) VR=CS, VM=1 Exposure Modulation Type
pub const ExposureModulationType: u32 = 0x00189323;

/// (0018,9324) VR=FD, VM=1 Estimated Dose Saving
pub const EstimatedDoseSaving: u32 = 0x00189324;

/// (0018,9325) VR=SQ, VM=1 CT X-Ray Details Sequence
pub const CTXRayDetailsSequence: u32 = 0x00189325;

/// (0018,9326) VR=SQ, VM=1 CT Position Sequence
pub const CTPositionSequence: u32 = 0x00189326;

/// (0018,9327) VR=FD, VM=1 Table Position
pub const TablePosition: u32 = 0x00189327;

/// (0018,9328) VR=FD, VM=1 Exposure Time in ms
pub const ExposureTimeInms: u32 = 0x00189328;

/// (0018,9329) VR=SQ, VM=1 CT Image Frame Type Sequence
pub const CTImageFrameTypeSequence: u32 = 0x00189329;

/// (0018,9330) VR=FD, VM=1 X-Ray Tube Current in mA
pub const XRayTubeCurrentInmA: u32 = 0x00189330;

/// (0018,9332) VR=FD, VM=1 Exposure in mAs
pub const ExposureInmAs: u32 = 0x00189332;

/// (0018,9333) VR=CS, VM=1 Constant Volume Flag
pub const ConstantVolumeFlag: u32 = 0x00189333;

/// (0018,9334) VR=CS, VM=1 Fluoroscopy Flag
pub const FluoroscopyFlag: u32 = 0x00189334;

/// (0018,9335) VR=FD, VM=1 Distance Source to Data Collection Center
pub const DistanceSourceToDataCollectionCenter: u32 = 0x00189335;

/// (0018,9337) VR=US, VM=1 Contrast/Bolus Agent Number
pub const ContrastBolusAgentNumber: u32 = 0x00189337;

/// (0018,9338) VR=SQ, VM=1 Contrast/Bolus Ingredient Code Sequence
pub const ContrastBolusIngredientCodeSequence: u32 = 0x00189338;

/// (0018,9340) VR=SQ, VM=1 Contrast Administration Profile Sequence
pub const ContrastAdministrationProfileSequence: u32 = 0x00189340;

/// (0018,9341) VR=SQ, VM=1 Contrast/Bolus Usage Sequence
pub const ContrastBolusUsageSequence: u32 = 0x00189341;

/// (0018,9342) VR=CS, VM=1 Contrast/Bolus Agent Administered
pub const ContrastBolusAgentAdministered: u32 = 0x00189342;

/// (0018,9343) VR=CS, VM=1 Contrast/Bolus Agent Detected
pub const ContrastBolusAgentDetected: u32 = 0x00189343;

/// (0018,9344) VR=CS, VM=1 Contrast/Bolus Agent Phase
pub const ContrastBolusAgentPhase: u32 = 0x00189344;

/// (0018,9345) VR=FD, VM=1 CTDIvol
pub const CTDIvol: u32 = 0x00189345;

/// (0018,9346) VR=SQ, VM=1 CTDI Phantom Type Code Sequence
pub const CTDIPhantomTypeCodeSequence: u32 = 0x00189346;

/// (0018,9351) VR=FL, VM=1 Calcium Scoring Mass Factor Patient
pub const CalciumScoringMassFactorPatient: u32 = 0x00189351;

/// (0018,9352) VR=FL, VM=3 Calcium Scoring Mass Factor Device
pub const CalciumScoringMassFactorDevice: u32 = 0x00189352;

/// (0018,9353) VR=FL, VM=1 Energy Weighting Factor
pub const EnergyWeightingFactor: u32 = 0x00189353;

/// (0018,9360) VR=SQ, VM=1 CT Additional X-Ray Source Sequence
pub const CTAdditionalXRaySourceSequence: u32 = 0x00189360;

/// (0018,9401) VR=SQ, VM=1 Projection Pixel Calibration Sequence
pub const ProjectionPixelCalibrationSequence: u32 = 0x00189401;

/// (0018,9402) VR=FL, VM=1 Distance Source to Isocenter
pub const DistanceSourceToIsocenter: u32 = 0x00189402;

/// (0018,9403) VR=FL, VM=1 Distance Object to Table Top
pub const DistanceObjectToTableTop: u32 = 0x00189403;

/// (0018,9404) VR=FL, VM=2 Object Pixel Spacing in Center of Beam
pub const ObjectPixelSpacingInCenterOfBeam: u32 = 0x00189404;

/// (0018,9405) VR=SQ, VM=1 Positioner Position Sequence
pub const PositionerPositionSequence: u32 = 0x00189405;

/// (0018,9406) VR=SQ, VM=1 Table Position Sequence
pub const TablePositionSequence: u32 = 0x00189406;

/// (0018,9407) VR=SQ, VM=1 Collimator Shape Sequence
pub const CollimatorShapeSequence: u32 = 0x00189407;

/// (0018,9410) VR=CS, VM=1 Planes in Acquisition
pub const PlanesInAcquisition: u32 = 0x00189410;

/// (0018,9412) VR=SQ, VM=1 XA/XRF Frame Characteristics Sequence
pub const XAXRFFrameCharacteristicsSequence: u32 = 0x00189412;

/// (0018,9417) VR=SQ, VM=1 Frame Acquisition Sequence
pub const FrameAcquisitionSequence: u32 = 0x00189417;

/// (0018,9420) VR=CS, VM=1 X-Ray Receptor Type
pub const XRayReceptorType: u32 = 0x00189420;

/// (0018,9423) VR=LO, VM=1 Acquisition Protocol Name
pub const AcquisitionProtocolName: u32 = 0x00189423;

/// (0018,9424) VR=LT, VM=1 Acquisition Protocol Description
pub const AcquisitionProtocolDescription: u32 = 0x00189424;

/// (0018,9425) VR=CS, VM=1 Contrast/Bolus Ingredient Opaque
pub const ContrastBolusIngredientOpaque: u32 = 0x00189425;

/// (0018,9426) VR=FL, VM=1 Distance Receptor Plane to Detector Housing
pub const DistanceReceptorPlaneToDetectorHousing: u32 = 0x00189426;

/// (0018,9427) VR=CS, VM=1 Intensifier Active Shape
pub const IntensifierActiveShape: u32 = 0x00189427;

/// (0018,9428) VR=FL, VM=1-2 Intensifier Active Dimension(s)
pub const IntensifierActiveDimensions: u32 = 0x00189428;

/// (0018,9429) VR=FL, VM=2 Physical Detector Size
pub const PhysicalDetectorSize: u32 = 0x00189429;

/// (0018,9430) VR=FL, VM=2 Position of Isocenter Projection
pub const PositionOfIsocenterProjection: u32 = 0x00189430;

/// (0018,9432) VR=SQ, VM=1 Field of View Sequence
pub const FieldOfViewSequence: u32 = 0x00189432;

/// (0018,9433) VR=LO, VM=1 Field of View Description
pub const FieldOfViewDescription: u32 = 0x00189433;

/// (0018,9434) VR=SQ, VM=1 Exposure Control Sensing Regions Sequence
pub const ExposureControlSensingRegionsSequence: u32 = 0x00189434;

/// (0018,9435) VR=CS, VM=1 Exposure Control Sensing Region Shape
pub const ExposureControlSensingRegionShape: u32 = 0x00189435;

/// (0018,9436) VR=SS, VM=1 Exposure Control Sensing Region Left Vertical Edge
pub const ExposureControlSensingRegionLeftVerticalEdge: u32 = 0x00189436;

/// (0018,9437) VR=SS, VM=1 Exposure Control Sensing Region Right Vertical Edge
pub const ExposureControlSensingRegionRightVerticalEdge: u32 = 0x00189437;

/// (0018,9438) VR=SS, VM=1 Exposure Control Sensing Region Upper Horizontal Edge
pub const ExposureControlSensingRegionUpperHorizontalEdge: u32 = 0x00189438;

/// (0018,9439) VR=SS, VM=1 Exposure Control Sensing Region Lower Horizontal Edge
pub const ExposureControlSensingRegionLowerHorizontalEdge: u32 = 0x00189439;

/// (0018,9440) VR=SS, VM=2 Center of Circular Exposure Control Sensing Region
pub const CenterOfCircularExposureControlSensingRegion: u32 = 0x00189440;

/// (0018,9441) VR=US, VM=1 Radius of Circular Exposure Control Sensing Region
pub const RadiusOfCircularExposureControlSensingRegion: u32 = 0x00189441;

/// (0018,9442) VR=SS, VM=2-n Vertices of the Polygonal Exposure Control Sensing Region
pub const VerticesOfThePolygonalExposureControlSensingRegion: u32 = 0x00189442;

/// (0018,9447) VR=FL, VM=1 Column Angulation (Patient)
pub const ColumnAngulationPatient: u32 = 0x00189447;

/// (0018,9449) VR=FL, VM=1 Beam Angle
pub const BeamAngle: u32 = 0x00189449;

/// (0018,9451) VR=SQ, VM=1 Frame Detector Parameters Sequence
pub const FrameDetectorParametersSequence: u32 = 0x00189451;

/// (0018,9452) VR=FL, VM=1 Calculated Anatomy Thickness
pub const CalculatedAnatomyThickness: u32 = 0x00189452;

/// (0018,9455) VR=SQ, VM=1 Calibration Sequence
pub const CalibrationSequence: u32 = 0x00189455;

/// (0018,9456) VR=SQ, VM=1 Object Thickness Sequence
pub const ObjectThicknessSequence: u32 = 0x00189456;

/// (0018,9457) VR=CS, VM=1 Plane Identification
pub const PlaneIdentification: u32 = 0x00189457;

/// (0018,9461) VR=FL, VM=1-2 Field of View Dimension(s) in Float
pub const FieldOfViewDimensionsInFloat: u32 = 0x00189461;

/// (0018,9462) VR=SQ, VM=1 Isocenter Reference System Sequence
pub const IsocenterReferenceSystemSequence: u32 = 0x00189462;

/// (0018,9463) VR=FL, VM=1 Positioner Isocenter Primary Angle
pub const PositionerIsocenterPrimaryAngle: u32 = 0x00189463;

/// (0018,9464) VR=FL, VM=1 Positioner Isocenter Secondary Angle
pub const PositionerIsocenterSecondaryAngle: u32 = 0x00189464;

/// (0018,9465) VR=FL, VM=1 Positioner Isocenter Detector Rotation Angle
pub const PositionerIsocenterDetectorRotationAngle: u32 = 0x00189465;

/// (0018,9466) VR=FL, VM=1 Table X Position to Isocenter
pub const TableXPositionToIsocenter: u32 = 0x00189466;

/// (0018,9467) VR=FL, VM=1 Table Y Position to Isocenter
pub const TableYPositionToIsocenter: u32 = 0x00189467;

/// (0018,9468) VR=FL, VM=1 Table Z Position to Isocenter
pub const TableZPositionToIsocenter: u32 = 0x00189468;

/// (0018,9469) VR=FL, VM=1 Table Horizontal Rotation Angle
pub const TableHorizontalRotationAngle: u32 = 0x00189469;

/// (0018,9470) VR=FL, VM=1 Table Head Tilt Angle
pub const TableHeadTiltAngle: u32 = 0x00189470;

/// (0018,9471) VR=FL, VM=1 Table Cradle Tilt Angle
pub const TableCradleTiltAngle: u32 = 0x00189471;

/// (0018,9472) VR=SQ, VM=1 Frame Display Shutter Sequence
pub const FrameDisplayShutterSequence: u32 = 0x00189472;

/// (0018,9473) VR=FL, VM=1 Acquired Image Area Dose Product
pub const AcquiredImageAreaDoseProduct: u32 = 0x00189473;

/// (0018,9474) VR=CS, VM=1 C-arm Positioner Tabletop Relationship
pub const CArmPositionerTabletopRelationship: u32 = 0x00189474;

/// (0018,9476) VR=SQ, VM=1 X-Ray Geometry Sequence
pub const XRayGeometrySequence: u32 = 0x00189476;

/// (0018,9477) VR=SQ, VM=1 Irradiation Event Identification Sequence
pub const IrradiationEventIdentificationSequence: u32 = 0x00189477;

/// (0018,9504) VR=SQ, VM=1 X-Ray 3D Frame Type Sequence
pub const XRay3DFrameTypeSequence: u32 = 0x00189504;

/// (0018,9506) VR=SQ, VM=1 Contributing Sources Sequence
pub const ContributingSourcesSequence: u32 = 0x00189506;

/// (0018,9507) VR=SQ, VM=1 X-Ray 3D Acquisition Sequence
pub const XRay3DAcquisitionSequence: u32 = 0x00189507;

/// (0018,9508) VR=FL, VM=1 Primary Positioner Scan Arc
pub const PrimaryPositionerScanArc: u32 = 0x00189508;

/// (0018,9509) VR=FL, VM=1 Secondary Positioner Scan Arc
pub const SecondaryPositionerScanArc: u32 = 0x00189509;

/// (0018,9510) VR=FL, VM=1 Primary Positioner Scan Start Angle
pub const PrimaryPositionerScanStartAngle: u32 = 0x00189510;

/// (0018,9511) VR=FL, VM=1 Secondary Positioner Scan Start Angle
pub const SecondaryPositionerScanStartAngle: u32 = 0x00189511;

/// (0018,9514) VR=FL, VM=1 Primary Positioner Increment
pub const PrimaryPositionerIncrement: u32 = 0x00189514;

/// (0018,9515) VR=FL, VM=1 Secondary Positioner Increment
pub const SecondaryPositionerIncrement: u32 = 0x00189515;

/// (0018,9516) VR=DT, VM=1 Start Acquisition DateTime
pub const StartAcquisitionDateTime: u32 = 0x00189516;

/// (0018,9517) VR=DT, VM=1 End Acquisition DateTime
pub const EndAcquisitionDateTime: u32 = 0x00189517;

/// (0018,9524) VR=LO, VM=1 Application Name
pub const ApplicationName: u32 = 0x00189524;

/// (0018,9525) VR=LO, VM=1 Application Version
pub const ApplicationVersion: u32 = 0x00189525;

/// (0018,9526) VR=LO, VM=1 Application Manufacturer
pub const ApplicationManufacturer: u32 = 0x00189526;

/// (0018,9527) VR=CS, VM=1 Algorithm Type
pub const AlgorithmType: u32 = 0x00189527;

/// (0018,9528) VR=LO, VM=1 Algorithm Description
pub const AlgorithmDescription: u32 = 0x00189528;

/// (0018,9530) VR=SQ, VM=1 X-Ray 3D Reconstruction Sequence
pub const XRay3DReconstructionSequence: u32 = 0x00189530;

/// (0018,9531) VR=LO, VM=1 Reconstruction Description
pub const ReconstructionDescription: u32 = 0x00189531;

/// (0018,9538) VR=SQ, VM=1 Per Projection Acquisition Sequence
pub const PerProjectionAcquisitionSequence: u32 = 0x00189538;

/// (0018,9601) VR=SQ, VM=1 Diffusion b-matrix Sequence
pub const DiffusionBMatrixSequence: u32 = 0x00189601;

/// (0018,9602) VR=FD, VM=1 Diffusion b-value XX
pub const DiffusionBValueXX: u32 = 0x00189602;

/// (0018,9603) VR=FD, VM=1 Diffusion b-value XY
pub const DiffusionBValueXY: u32 = 0x00189603;

/// (0018,9604) VR=FD, VM=1 Diffusion b-value XZ
pub const DiffusionBValueXZ: u32 = 0x00189604;

/// (0018,9605) VR=FD, VM=1 Diffusion b-value YY
pub const DiffusionBValueYY: u32 = 0x00189605;

/// (0018,9606) VR=FD, VM=1 Diffusion b-value YZ
pub const DiffusionBValueYZ: u32 = 0x00189606;

/// (0018,9607) VR=FD, VM=1 Diffusion b-value ZZ
pub const DiffusionBValueZZ: u32 = 0x00189607;

/// (0018,9701) VR=DT, VM=1 Decay Correction DateTime
pub const DecayCorrectionDateTime: u32 = 0x00189701;

/// (0018,9715) VR=FD, VM=1 Start Density Threshold
pub const StartDensityThreshold: u32 = 0x00189715;

/// (0018,9716) VR=FD, VM=1 Start Relative Density Difference Threshold
pub const StartRelativeDensityDifferenceThreshold: u32 = 0x00189716;

/// (0018,9717) VR=FD, VM=1 Start Cardiac Trigger Count Threshold
pub const StartCardiacTriggerCountThreshold: u32 = 0x00189717;

/// (0018,9718) VR=FD, VM=1 Start Respiratory Trigger Count Threshold
pub const StartRespiratoryTriggerCountThreshold: u32 = 0x00189718;

/// (0018,9719) VR=FD, VM=1 Termination Counts Threshold
pub const TerminationCountsThreshold: u32 = 0x00189719;

/// (0018,9720) VR=FD, VM=1 Termination Density Threshold
pub const TerminationDensityThreshold: u32 = 0x00189720;

/// (0018,9721) VR=FD, VM=1 Termination Relative Density Threshold
pub const TerminationRelativeDensityThreshold: u32 = 0x00189721;

/// (0018,9722) VR=FD, VM=1 Termination Time Threshold
pub const TerminationTimeThreshold: u32 = 0x00189722;

/// (0018,9723) VR=FD, VM=1 Termination Cardiac Trigger Count Threshold
pub const TerminationCardiacTriggerCountThreshold: u32 = 0x00189723;

/// (0018,9724) VR=FD, VM=1 Termination Respiratory Trigger Count Threshold
pub const TerminationRespiratoryTriggerCountThreshold: u32 = 0x00189724;

/// (0018,9725) VR=CS, VM=1 Detector Geometry
pub const DetectorGeometry: u32 = 0x00189725;

/// (0018,9726) VR=FD, VM=1 Transverse Detector Separation
pub const TransverseDetectorSeparation: u32 = 0x00189726;

/// (0018,9727) VR=FD, VM=1 Axial Detector Dimension
pub const AxialDetectorDimension: u32 = 0x00189727;

/// (0018,9729) VR=US, VM=1 Radiopharmaceutical Agent Number
pub const RadiopharmaceuticalAgentNumber: u32 = 0x00189729;

/// (0018,9732) VR=SQ, VM=1 PET Frame Acquisition Sequence
pub const PETFrameAcquisitionSequence: u32 = 0x00189732;

/// (0018,9733) VR=SQ, VM=1 PET Detector Motion Details Sequence
pub const PETDetectorMotionDetailsSequence: u32 = 0x00189733;

/// (0018,9734) VR=SQ, VM=1 PET Table Dynamics Sequence
pub const PETTableDynamicsSequence: u32 = 0x00189734;

/// (0018,9735) VR=SQ, VM=1 PET Position Sequence
pub const PETPositionSequence: u32 = 0x00189735;

/// (0018,9736) VR=SQ, VM=1 PET Frame Correction Factors Sequence
pub const PETFrameCorrectionFactorsSequence: u32 = 0x00189736;

/// (0018,9737) VR=SQ, VM=1 Radiopharmaceutical Usage Sequence
pub const RadiopharmaceuticalUsageSequence: u32 = 0x00189737;

/// (0018,9738) VR=CS, VM=1 Attenuation Correction Source
pub const AttenuationCorrectionSource: u32 = 0x00189738;

/// (0018,9739) VR=US, VM=1 Number of Iterations
pub const NumberOfIterations: u32 = 0x00189739;

/// (0018,9740) VR=US, VM=1 Number of Subsets
pub const NumberOfSubsets: u32 = 0x00189740;

/// (0018,9749) VR=SQ, VM=1 PET Reconstruction Sequence
pub const PETReconstructionSequence: u32 = 0x00189749;

/// (0018,9751) VR=SQ, VM=1 PET Frame Type Sequence
pub const PETFrameTypeSequence: u32 = 0x00189751;

/// (0018,9755) VR=CS, VM=1 Time of Flight Information Used
pub const TimeOfFlightInformationUsed: u32 = 0x00189755;

/// (0018,9756) VR=CS, VM=1 Reconstruction Type
pub const ReconstructionType: u32 = 0x00189756;

/// (0018,9758) VR=CS, VM=1 Decay Corrected
pub const DecayCorrected: u32 = 0x00189758;

/// (0018,9759) VR=CS, VM=1 Attenuation Corrected
pub const AttenuationCorrected: u32 = 0x00189759;

/// (0018,9760) VR=CS, VM=1 Scatter Corrected
pub const ScatterCorrected: u32 = 0x00189760;

/// (0018,9761) VR=CS, VM=1 Dead Time Corrected
pub const DeadTimeCorrected: u32 = 0x00189761;

/// (0018,9762) VR=CS, VM=1 Gantry Motion Corrected
pub const GantryMotionCorrected: u32 = 0x00189762;

/// (0018,9763) VR=CS, VM=1 Patient Motion Corrected
pub const PatientMotionCorrected: u32 = 0x00189763;

/// (0018,9764) VR=CS, VM=1 Count Loss Normalization Corrected
pub const CountLossNormalizationCorrected: u32 = 0x00189764;

/// (0018,9765) VR=CS, VM=1 Randoms Corrected
pub const RandomsCorrected: u32 = 0x00189765;

/// (0018,9766) VR=CS, VM=1 Non-uniform Radial Sampling Corrected
pub const NonUniformRadialSamplingCorrected: u32 = 0x00189766;

/// (0018,9767) VR=CS, VM=1 Sensitivity Calibrated
pub const SensitivityCalibrated: u32 = 0x00189767;

/// (0018,9768) VR=CS, VM=1 Detector Normalization Correction
pub const DetectorNormalizationCorrection: u32 = 0x00189768;

/// (0018,9769) VR=CS, VM=1 Iterative Reconstruction Method
pub const IterativeReconstructionMethod: u32 = 0x00189769;

/// (0018,9770) VR=CS, VM=1 Attenuation Correction Temporal Relationship
pub const AttenuationCorrectionTemporalRelationship: u32 = 0x00189770;

/// (0018,9771) VR=SQ, VM=1 Patient Physiological State Sequence
pub const PatientPhysiologicalStateSequence: u32 = 0x00189771;

/// (0018,9772) VR=SQ, VM=1 Patient Physiological State Code Sequence
pub const PatientPhysiologicalStateCodeSequence: u32 = 0x00189772;

/// (0018,9801) VR=FD, VM=1-n Depth(s) of Focus
pub const DepthsOfFocus: u32 = 0x00189801;

/// (0018,9803) VR=SQ, VM=1 Excluded Intervals Sequence
pub const ExcludedIntervalsSequence: u32 = 0x00189803;

/// (0018,9804) VR=DT, VM=1 Exclusion Start Datetime
pub const ExclusionStartDatetime: u32 = 0x00189804;

/// (0018,9805) VR=FD, VM=1 Exclusion Duration
pub const ExclusionDuration: u32 = 0x00189805;

/// (0018,9806) VR=SQ, VM=1 US Image Description Sequence
pub const USImageDescriptionSequence: u32 = 0x00189806;

/// (0018,9807) VR=SQ, VM=1 Image Data Type Sequence
pub const ImageDataTypeSequence: u32 = 0x00189807;

/// (0018,9808) VR=CS, VM=1 Data Type
pub const DataType: u32 = 0x00189808;

/// (0018,9809) VR=SQ, VM=1 Transducer Scan Pattern Code Sequence
pub const TransducerScanPatternCodeSequence: u32 = 0x00189809;

/// (0018,980B) VR=CS, VM=1 Aliased Data Type
pub const AliasedDataType: u32 = 0x0018980B;

/// (0018,980C) VR=CS, VM=1 Position Measuring Device Used
pub const PositionMeasuringDeviceUsed: u32 = 0x0018980C;

/// (0018,980D) VR=SQ, VM=1 Transducer Geometry Code Sequence
pub const TransducerGeometryCodeSequence: u32 = 0x0018980D;

/// (0018,980E) VR=SQ, VM=1 Transducer Beam Steering Code Sequence
pub const TransducerBeamSteeringCodeSequence: u32 = 0x0018980E;

/// (0018,980F) VR=SQ, VM=1 Transducer Application Code Sequence
pub const TransducerApplicationCodeSequence: u32 = 0x0018980F;

/// (0018,A001) VR=SQ, VM=1 Contributing Equipment Sequence
pub const ContributingEquipmentSequence: u32 = 0x0018A001;

/// (0018,A002) VR=DT, VM=1 Contribution Date Time
pub const ContributionDateTime: u32 = 0x0018A002;

/// (0018,A003) VR=ST, VM=1 Contribution Description
pub const ContributionDescription: u32 = 0x0018A003;

/// (0020,000D) VR=UI, VM=1 Study Instance UID
pub const StudyInstanceUID: u32 = 0x0020000D;

/// (0020,000E) VR=UI, VM=1 Series Instance UID
pub const SeriesInstanceUID: u32 = 0x0020000E;

/// (0020,0010) VR=SH, VM=1 Study ID
pub const StudyID: u32 = 0x00200010;

/// (0020,0011) VR=IS, VM=1 Series Number
pub const SeriesNumber: u32 = 0x00200011;

/// (0020,0012) VR=IS, VM=1 Acquisition Number
pub const AcquisitionNumber: u32 = 0x00200012;

/// (0020,0013) VR=IS, VM=1 Instance Number
pub const InstanceNumber: u32 = 0x00200013;

/// (0020,0014) VR=IS, VM=1 Isotope Number (retired)
pub const IsotopeNumber: u32 = 0x00200014;

/// (0020,0015) VR=IS, VM=1 Phase Number (retired)
pub const PhaseNumber: u32 = 0x00200015;

/// (0020,0016) VR=IS, VM=1 Interval Number (retired)
pub const IntervalNumber: u32 = 0x00200016;

/// (0020,0017) VR=IS, VM=1 Time Slot Number (retired)
pub const TimeSlotNumber: u32 = 0x00200017;

/// (0020,0018) VR=IS, VM=1 Angle Number (retired)
pub const AngleNumber: u32 = 0x00200018;

/// (0020,0019) VR=IS, VM=1 Item Number
pub const ItemNumber: u32 = 0x00200019;

/// (0020,0020) VR=CS, VM=2 Patient Orientation
pub const PatientOrientation: u32 = 0x00200020;

/// (0020,0022) VR=IS, VM=1 Overlay Number (retired)
pub const OverlayNumber: u32 = 0x00200022;

/// (0020,0024) VR=IS, VM=1 Curve Number (retired)
pub const CurveNumber: u32 = 0x00200024;

/// (0020,0026) VR=IS, VM=1 LUT Number (retired)
pub const LUTNumber: u32 = 0x00200026;

/// (0020,0030) VR=DS, VM=3 Image Position (retired)
pub const ImagePosition: u32 = 0x00200030;

/// (0020,0032) VR=DS, VM=3 Image Position (Patient)
pub const ImagePositionPatient: u32 = 0x00200032;

/// (0020,0035) VR=DS, VM=6 Image Orientation (retired)
pub const ImageOrientation: u32 = 0x00200035;

/// (0020,0037) VR=DS, VM=6 Image Orientation (Patient)
pub const ImageOrientationPatient: u32 = 0x00200037;

/// (0020,0050) VR=DS, VM=1 Location (retired)
pub const Location: u32 = 0x00200050;

/// (0020,0052) VR=UI, VM=1 Frame of Reference UID
pub const FrameOfReferenceUID: u32 = 0x00200052;

/// (0020,0060) VR=CS, VM=1 Laterality
pub const Laterality: u32 = 0x00200060;

/// (0020,0062) VR=CS, VM=1 Image Laterality
pub const ImageLaterality: u32 = 0x00200062;

/// (0020,0070) VR=LO, VM=1 Image Geometry Type (retired)
pub const ImageGeometryType: u32 = 0x00200070;

/// (0020,0080) VR=CS, VM=1-n Masking Image (retired)
pub const MaskingImage: u32 = 0x00200080;

/// (0020,00AA) VR=IS, VM=1 Report Number (retired)
pub const ReportNumber: u32 = 0x002000AA;

/// (0020,0100) VR=IS, VM=1 Temporal Position Identifier
pub const TemporalPositionIdentifier: u32 = 0x00200100;

/// (0020,0105) VR=IS, VM=1 Number of Temporal Positions
pub const NumberOfTemporalPositions: u32 = 0x00200105;

/// (0020,0110) VR=DS, VM=1 Temporal Resolution
pub const TemporalResolution: u32 = 0x00200110;

/// (0020,0200) VR=UI, VM=1 Synchronization Frame of Reference UID
pub const SynchronizationFrameOfReferenceUID: u32 = 0x00200200;

/// (0020,0242) VR=UI, VM=1 SOP Instance UID of Concatenation Source
pub const SOPInstanceUIDOfConcatenationSource: u32 = 0x00200242;

/// (0020,1000) VR=IS, VM=1 Series in Study (retired)
pub const SeriesInStudy: u32 = 0x00201000;

/// (0020,1001) VR=IS, VM=1 Acquisitions in Series (retired)
pub const AcquisitionsInSeries: u32 = 0x00201001;

/// (0020,1002) VR=IS, VM=1 Images in Acquisition
pub const ImagesInAcquisition: u32 = 0x00201002;

/// (0020,1003) VR=IS, VM=1 Images in Series (retired)
pub const ImagesInSeries: u32 = 0x00201003;

/// (0020,1004) VR=IS, VM=1 Acquisitions in Study (retired)
pub const AcquisitionsInStudy: u32 = 0x00201004;

/// (0020,1005) VR=IS, VM=1 Images in Study (retired)
pub const ImagesInStudy: u32 = 0x00201005;

/// (0020,1020) VR=LO, VM=1-n Reference (retired)
pub const Reference: u32 = 0x00201020;

/// (0020,1040) VR=LO, VM=1 Position Reference Indicator
pub const PositionReferenceIndicator: u32 = 0x00201040;

/// (0020,1041) VR=DS, VM=1 Slice Location
pub const SliceLocation: u32 = 0x00201041;

/// (0020,1070) VR=IS, VM=1-n Other Study Numbers (retired)
pub const OtherStudyNumbers: u32 = 0x00201070;

/// (0020,1200) VR=IS, VM=1 Number of Patient Related Studies
pub const NumberOfPatientRelatedStudies: u32 = 0x00201200;

/// (0020,1202) VR=IS, VM=1 Number of Patient Related Series
pub const NumberOfPatientRelatedSeries: u32 = 0x00201202;

/// (0020,1204) VR=IS, VM=1 Number of Patient Related Instances
pub const NumberOfPatientRelatedInstances: u32 = 0x00201204;

/// (0020,1206) VR=IS, VM=1 Number of Study Related Series
pub const NumberOfStudyRelatedSeries: u32 = 0x00201206;

/// (0020,1208) VR=IS, VM=1 Number of Study Related Instances
pub const NumberOfStudyRelatedInstances: u32 = 0x00201208;

/// (0020,1209) VR=IS, VM=1 Number of Series Related Instances
pub const NumberOfSeriesRelatedInstances: u32 = 0x00201209;

/// (0020,31xx) VR=CS, VM=1-n Source Image IDs (retired)
pub const SourceImageIDs: u32 = 0x00203100;

/// (0020,3401) VR=CS, VM=1 Modifying Device ID (retired)
pub const ModifyingDeviceID: u32 = 0x00203401;

/// (0020,3402) VR=CS, VM=1 Modified Image ID (retired)
pub const ModifiedImageID: u32 = 0x00203402;

/// (0020,3403) VR=DA, VM=1 Modified Image Date (retired)
pub const ModifiedImageDate: u32 = 0x00203403;

/// (0020,3404) VR=LO, VM=1 Modifying Device Manufacturer (retired)
pub const ModifyingDeviceManufacturer: u32 = 0x00203404;

/// (0020,3405) VR=TM, VM=1 Modified Image Time (retired)
pub const ModifiedImageTime: u32 = 0x00203405;

/// (0020,3406) VR=LO, VM=1 Modified Image Description (retired)
pub const ModifiedImageDescription: u32 = 0x00203406;

/// (0020,4000) VR=LT, VM=1 Image Comments
pub const ImageComments: u32 = 0x00204000;

/// (0020,5000) VR=AT, VM=1-n Original Image Identification (retired)
pub const OriginalImageIdentification: u32 = 0x00205000;

/// (0020,5002) VR=LO, VM=1-n Original Image Identification Nomenclature (retired)
pub const OriginalImageIdentificationNomenclature: u32 = 0x00205002;

/// (0020,9056) VR=SH, VM=1 Stack ID
pub const StackID: u32 = 0x00209056;

/// (0020,9057) VR=UL, VM=1 In-Stack Position Number
pub const InStackPositionNumber: u32 = 0x00209057;

/// (0020,9071) VR=SQ, VM=1 Frame Anatomy Sequence
pub const FrameAnatomySequence: u32 = 0x00209071;

/// (0020,9072) VR=CS, VM=1 Frame Laterality
pub const FrameLaterality: u32 = 0x00209072;

/// (0020,9111) VR=SQ, VM=1 Frame Content Sequence
pub const FrameContentSequence: u32 = 0x00209111;

/// (0020,9113) VR=SQ, VM=1 Plane Position Sequence
pub const PlanePositionSequence: u32 = 0x00209113;

/// (0020,9116) VR=SQ, VM=1 Plane Orientation Sequence
pub const PlaneOrientationSequence: u32 = 0x00209116;

/// (0020,9128) VR=UL, VM=1 Temporal Position Index
pub const TemporalPositionIndex: u32 = 0x00209128;

/// (0020,9153) VR=FD, VM=1 Nominal Cardiac Trigger Delay Time
pub const NominalCardiacTriggerDelayTime: u32 = 0x00209153;

/// (0020,9154) VR=FL, VM=1 Nominal Cardiac Trigger Time Prior To R-Peak
pub const NominalCardiacTriggerTimePriorToRPeak: u32 = 0x00209154;

/// (0020,9155) VR=FL, VM=1 Actual Cardiac Trigger Time Prior To R-Peak
pub const ActualCardiacTriggerTimePriorToRPeak: u32 = 0x00209155;

/// (0020,9156) VR=US, VM=1 Frame Acquisition Number
pub const FrameAcquisitionNumber: u32 = 0x00209156;

/// (0020,9157) VR=UL, VM=1-n Dimension Index Values
pub const DimensionIndexValues: u32 = 0x00209157;

/// (0020,9158) VR=LT, VM=1 Frame Comments
pub const FrameComments: u32 = 0x00209158;

/// (0020,9161) VR=UI, VM=1 Concatenation UID
pub const ConcatenationUID: u32 = 0x00209161;

/// (0020,9162) VR=US, VM=1 In-concatenation Number
pub const InConcatenationNumber: u32 = 0x00209162;

/// (0020,9163) VR=US, VM=1 In-concatenation Total Number
pub const InConcatenationTotalNumber: u32 = 0x00209163;

/// (0020,9164) VR=UI, VM=1 Dimension Organization UID
pub const DimensionOrganizationUID: u32 = 0x00209164;

/// (0020,9165) VR=AT, VM=1 Dimension Index Pointer
pub const DimensionIndexPointer: u32 = 0x00209165;

/// (0020,9167) VR=AT, VM=1 Functional Group Pointer
pub const FunctionalGroupPointer: u32 = 0x00209167;

/// (0020,9213) VR=LO, VM=1 Dimension Index Private Creator
pub const DimensionIndexPrivateCreator: u32 = 0x00209213;

/// (0020,9221) VR=SQ, VM=1 Dimension Organization Sequence
pub const DimensionOrganizationSequence: u32 = 0x00209221;

/// (0020,9222) VR=SQ, VM=1 Dimension Index Sequence
pub const DimensionIndexSequence: u32 = 0x00209222;

/// (0020,9228) VR=UL, VM=1 Concatenation Frame Offset Number
pub const ConcatenationFrameOffsetNumber: u32 = 0x00209228;

/// (0020,9238) VR=LO, VM=1 Functional Group Private Creator
pub const FunctionalGroupPrivateCreator: u32 = 0x00209238;

/// (0020,9241) VR=FL, VM=1 Nominal Percentage of Cardiac Phase
pub const NominalPercentageOfCardiacPhase: u32 = 0x00209241;

/// (0020,9245) VR=FL, VM=1 Nominal Percentage of Respiratory Phase
pub const NominalPercentageOfRespiratoryPhase: u32 = 0x00209245;

/// (0020,9246) VR=FL, VM=1 Starting Respiratory Amplitude
pub const StartingRespiratoryAmplitude: u32 = 0x00209246;

/// (0020,9247) VR=CS, VM=1 Starting Respiratory Phase
pub const StartingRespiratoryPhase: u32 = 0x00209247;

/// (0020,9248) VR=FL, VM=1 Ending Respiratory Amplitude
pub const EndingRespiratoryAmplitude: u32 = 0x00209248;

/// (0020,9249) VR=CS, VM=1 Ending Respiratory Phase
pub const EndingRespiratoryPhase: u32 = 0x00209249;

/// (0020,9250) VR=CS, VM=1 Respiratory Trigger Type
pub const RespiratoryTriggerType: u32 = 0x00209250;

/// (0020,9251) VR=FD, VM=1 R-R Interval Time Nominal
pub const RRIntervalTimeNominal: u32 = 0x00209251;

/// (0020,9252) VR=FD, VM=1 Actual Cardiac Trigger Delay Time
pub const ActualCardiacTriggerDelayTime: u32 = 0x00209252;

/// (0020,9253) VR=SQ, VM=1 Respiratory Synchronization Sequence
pub const RespiratorySynchronizationSequence: u32 = 0x00209253;

/// (0020,9254) VR=FD, VM=1 Respiratory Interval Time
pub const RespiratoryIntervalTime: u32 = 0x00209254;

/// (0020,9255) VR=FD, VM=1 Nominal Respiratory Trigger Delay Time
pub const NominalRespiratoryTriggerDelayTime: u32 = 0x00209255;

/// (0020,9256) VR=FD, VM=1 Respiratory Trigger Delay Threshold
pub const RespiratoryTriggerDelayThreshold: u32 = 0x00209256;

/// (0020,9257) VR=FD, VM=1 Actual Respiratory Trigger Delay Time
pub const ActualRespiratoryTriggerDelayTime: u32 = 0x00209257;

/// (0020,9301) VR=FD, VM=3 Image Position (Volume)
pub const ImagePositionVolume: u32 = 0x00209301;

/// (0020,9302) VR=FD, VM=6 Image Orientation (Volume)
pub const ImageOrientationVolume: u32 = 0x00209302;

/// (0020,9307) VR=CS, VM=1 Ultrasound Acquisition Geometry
pub const UltrasoundAcquisitionGeometry: u32 = 0x00209307;

/// (0020,9308) VR=FD, VM=3 Apex Position
pub const ApexPosition: u32 = 0x00209308;

/// (0020,9309) VR=FD, VM=16 Volume to Transducer Mapping Matrix
pub const VolumeToTransducerMappingMatrix: u32 = 0x00209309;

/// (0020,930A) VR=FD, VM=16 Volume to Table Mapping Matrix
pub const VolumeToTableMappingMatrix: u32 = 0x0020930A;

/// (0020,930C) VR=CS, VM=1 Patient Frame of Reference Source
pub const PatientFrameOfReferenceSource: u32 = 0x0020930C;

/// (0020,930D) VR=FD, VM=1 Temporal Position Time Offset
pub const TemporalPositionTimeOffset: u32 = 0x0020930D;

/// (0020,930E) VR=SQ, VM=1 Plane Position (Volume) Sequence
pub const PlanePositionVolumeSequence: u32 = 0x0020930E;

/// (0020,930F) VR=SQ, VM=1 Plane Orientation (Volume) Sequence
pub const PlaneOrientationVolumeSequence: u32 = 0x0020930F;

/// (0020,9310) VR=SQ, VM=1 Temporal Position Sequence
pub const TemporalPositionSequence: u32 = 0x00209310;

/// (0020,9311) VR=CS, VM=1 Dimension Organization Type
pub const DimensionOrganizationType: u32 = 0x00209311;

/// (0020,9312) VR=UI, VM=1 Volume Frame of Reference UID
pub const VolumeFrameOfReferenceUID: u32 = 0x00209312;

/// (0020,9313) VR=UI, VM=1 Table Frame of Reference UID
pub const TableFrameOfReferenceUID: u32 = 0x00209313;

/// (0020,9421) VR=LO, VM=1 Dimension Description Label
pub const DimensionDescriptionLabel: u32 = 0x00209421;

/// (0020,9450) VR=SQ, VM=1 Patient Orientation in Frame Sequence
pub const PatientOrientationInFrameSequence: u32 = 0x00209450;

/// (0020,9453) VR=LO, VM=1 Frame Label
pub const FrameLabel: u32 = 0x00209453;

/// (0020,9518) VR=US, VM=1-n Acquisition Index
pub const AcquisitionIndex: u32 = 0x00209518;

/// (0020,9529) VR=SQ, VM=1 Contributing SOP Instances Reference Sequence
pub const ContributingSOPInstancesReferenceSequence: u32 = 0x00209529;

/// (0020,9536) VR=US, VM=1 Reconstruction Index
pub const ReconstructionIndex: u32 = 0x00209536;

/// (0022,0001) VR=US, VM=1 Light Path Filter Pass-Through Wavelength
pub const LightPathFilterPassThroughWavelength: u32 = 0x00220001;

/// (0022,0002) VR=US, VM=2 Light Path Filter Pass Band
pub const LightPathFilterPassBand: u32 = 0x00220002;

/// (0022,0003) VR=US, VM=1 Image Path Filter Pass-Through Wavelength
pub const ImagePathFilterPassThroughWavelength: u32 = 0x00220003;

/// (0022,0004) VR=US, VM=2 Image Path Filter Pass Band
pub const ImagePathFilterPassBand: u32 = 0x00220004;

/// (0022,0005) VR=CS, VM=1 Patient Eye Movement Commanded
pub const PatientEyeMovementCommanded: u32 = 0x00220005;

/// (0022,0006) VR=SQ, VM=1 Patient Eye Movement Command Code Sequence
pub const PatientEyeMovementCommandCodeSequence: u32 = 0x00220006;

/// (0022,0007) VR=FL, VM=1 Spherical Lens Power
pub const SphericalLensPower: u32 = 0x00220007;

/// (0022,0008) VR=FL, VM=1 Cylinder Lens Power
pub const CylinderLensPower: u32 = 0x00220008;

/// (0022,0009) VR=FL, VM=1 Cylinder Axis
pub const CylinderAxis: u32 = 0x00220009;

/// (0022,000A) VR=FL, VM=1 Emmetropic Magnification
pub const EmmetropicMagnification: u32 = 0x0022000A;

/// (0022,000B) VR=FL, VM=1 Intra Ocular Pressure
pub const IntraOcularPressure: u32 = 0x0022000B;

/// (0022,000C) VR=FL, VM=1 Horizontal Field of View
pub const HorizontalFieldOfView: u32 = 0x0022000C;

/// (0022,000D) VR=CS, VM=1 Pupil Dilated
pub const PupilDilated: u32 = 0x0022000D;

/// (0022,000E) VR=FL, VM=1 Degree of Dilation
pub const DegreeOfDilation: u32 = 0x0022000E;

/// (0022,0010) VR=FL, VM=1 Stereo Baseline Angle
pub const StereoBaselineAngle: u32 = 0x00220010;

/// (0022,0011) VR=FL, VM=1 Stereo Baseline Displacement
pub const StereoBaselineDisplacement: u32 = 0x00220011;

/// (0022,0012) VR=FL, VM=1 Stereo Horizontal Pixel Offset
pub const StereoHorizontalPixelOffset: u32 = 0x00220012;

/// (0022,0013) VR=FL, VM=1 Stereo Vertical Pixel Offset
pub const StereoVerticalPixelOffset: u32 = 0x00220013;

/// (0022,0014) VR=FL, VM=1 Stereo Rotation
pub const StereoRotation: u32 = 0x00220014;

/// (0022,0015) VR=SQ, VM=1 Acquisition Device Type Code Sequence
pub const AcquisitionDeviceTypeCodeSequence: u32 = 0x00220015;

/// (0022,0016) VR=SQ, VM=1 Illumination Type Code Sequence
pub const IlluminationTypeCodeSequence: u32 = 0x00220016;

/// (0022,0017) VR=SQ, VM=1 Light Path Filter Type Stack Code Sequence
pub const LightPathFilterTypeStackCodeSequence: u32 = 0x00220017;

/// (0022,0018) VR=SQ, VM=1 Image Path Filter Type Stack Code Sequence
pub const ImagePathFilterTypeStackCodeSequence: u32 = 0x00220018;

/// (0022,0019) VR=SQ, VM=1 Lenses Code Sequence
pub const LensesCodeSequence: u32 = 0x00220019;

/// (0022,001A) VR=SQ, VM=1 Channel Description Code Sequence
pub const ChannelDescriptionCodeSequence: u32 = 0x0022001A;

/// (0022,001B) VR=SQ, VM=1 Refractive State Sequence
pub const RefractiveStateSequence: u32 = 0x0022001B;

/// (0022,001C) VR=SQ, VM=1 Mydriatic Agent Code Sequence
pub const MydriaticAgentCodeSequence: u32 = 0x0022001C;

/// (0022,001D) VR=SQ, VM=1 Relative Image Position Code Sequence
pub const RelativeImagePositionCodeSequence: u32 = 0x0022001D;

/// (0022,001E) VR=FL, VM=1 Camera Angle of View
pub const CameraAngleOfView: u32 = 0x0022001E;

/// (0022,0020) VR=SQ, VM=1 Stereo Pairs Sequence
pub const StereoPairsSequence: u32 = 0x00220020;

/// (0022,0021) VR=SQ, VM=1 Left Image Sequence
pub const LeftImageSequence: u32 = 0x00220021;

/// (0022,0022) VR=SQ, VM=1 Right Image Sequence
pub const RightImageSequence: u32 = 0x00220022;

/// (0022,0030) VR=FL, VM=1 Axial Length of the Eye
pub const AxialLengthOfTheEye: u32 = 0x00220030;

/// (0022,0031) VR=SQ, VM=1 Ophthalmic Frame Location Sequence
pub const OphthalmicFrameLocationSequence: u32 = 0x00220031;

/// (0022,0032) VR=FL, VM=2-2n Reference Coordinates
pub const ReferenceCoordinates: u32 = 0x00220032;

/// (0022,0035) VR=FL, VM=1 Depth Spatial Resolution
pub const DepthSpatialResolution: u32 = 0x00220035;

/// (0022,0036) VR=FL, VM=1 Maximum Depth Distortion
pub const MaximumDepthDistortion: u32 = 0x00220036;

/// (0022,0037) VR=FL, VM=1 Along-scan Spatial Resolution
pub const AlongScanSpatialResolution: u32 = 0x00220037;

/// (0022,0038) VR=FL, VM=1 Maximum Along-scan Distortion
pub const MaximumAlongScanDistortion: u32 = 0x00220038;

/// (0022,0039) VR=CS, VM=1 Ophthalmic Image Orientation
pub const OphthalmicImageOrientation: u32 = 0x00220039;

/// (0022,0041) VR=FL, VM=1 Depth of Transverse Image
pub const DepthOfTransverseImage: u32 = 0x00220041;

/// (0022,0042) VR=SQ, VM=1 Mydriatic Agent Concentration Units Sequence
pub const MydriaticAgentConcentrationUnitsSequence: u32 = 0x00220042;

/// (0022,0048) VR=FL, VM=1 Across-scan Spatial Resolution
pub const AcrossScanSpatialResolution: u32 = 0x00220048;

/// (0022,0049) VR=FL, VM=1 Maximum Across-scan Distortion
pub const MaximumAcrossScanDistortion: u32 = 0x00220049;

/// (0022,004E) VR=DS, VM=1 Mydriatic Agent Concentration
pub const MydriaticAgentConcentration: u32 = 0x0022004E;

/// (0022,0055) VR=FL, VM=1 Illumination Wave Length
pub const IlluminationWaveLength: u32 = 0x00220055;

/// (0022,0056) VR=FL, VM=1 Illumination Power
pub const IlluminationPower: u32 = 0x00220056;

/// (0022,0057) VR=FL, VM=1 Illumination Bandwidth
pub const IlluminationBandwidth: u32 = 0x00220057;

/// (0022,0058) VR=SQ, VM=1 Mydriatic Agent Sequence
pub const MydriaticAgentSequence: u32 = 0x00220058;

/// (0022,1007) VR=SQ, VM=1 Ophthalmic Axial Measurements Right Eye Sequence
pub const OphthalmicAxialMeasurementsRightEyeSequence: u32 = 0x00221007;

/// (0022,1008) VR=SQ, VM=1 Ophthalmic Axial Measurements Left Eye Sequence
pub const OphthalmicAxialMeasurementsLeftEyeSequence: u32 = 0x00221008;

/// (0022,1010) VR=CS, VM=1 Ophthalmic Axial Length Measurements Type
pub const OphthalmicAxialLengthMeasurementsType: u32 = 0x00221010;

/// (0022,1019) VR=FL, VM=1 Ophthalmic Axial Length
pub const OphthalmicAxialLength: u32 = 0x00221019;

/// (0022,1024) VR=SQ, VM=1 Lens Status Code Sequence
pub const LensStatusCodeSequence: u32 = 0x00221024;

/// (0022,1025) VR=SQ, VM=1 Vitreous Status Code Sequence
pub const VitreousStatusCodeSequence: u32 = 0x00221025;

/// (0022,1028) VR=SQ, VM=1 IOL Formula Code Sequence
pub const IOLFormulaCodeSequence: u32 = 0x00221028;

/// (0022,1029) VR=LO, VM=1 IOL Formula Detail
pub const IOLFormulaDetail: u32 = 0x00221029;

/// (0022,1033) VR=FL, VM=1 Keratometer Index
pub const KeratometerIndex: u32 = 0x00221033;

/// (0022,1035) VR=SQ, VM=1 Source of Ophthalmic Axial Length Code Sequence
pub const SourceOfOphthalmicAxialLengthCodeSequence: u32 = 0x00221035;

/// (0022,1037) VR=FL, VM=1 Target Refraction
pub const TargetRefraction: u32 = 0x00221037;

/// (0022,1039) VR=CS, VM=1 Refractive Procedure Occurred
pub const RefractiveProcedureOccurred: u32 = 0x00221039;

/// (0022,1040) VR=SQ, VM=1 Refractive Surgery Type Code Sequence
pub const RefractiveSurgeryTypeCodeSequence: u32 = 0x00221040;

/// (0022,1044) VR=SQ, VM=1 Ophthalmic Ultrasound Axial Measurements Type Code Sequence
pub const OphthalmicUltrasoundAxialMeasurementsTypeCodeSequence: u32 = 0x00221044;

/// (0022,1050) VR=SQ, VM=1 Ophthalmic Axial Length Measurements Sequence
pub const OphthalmicAxialLengthMeasurementsSequence: u32 = 0x00221050;

/// (0022,1053) VR=FL, VM=1 IOL Power
pub const IOLPower: u32 = 0x00221053;

/// (0022,1054) VR=FL, VM=1 Predicted Refractive Error
pub const PredictedRefractiveError: u32 = 0x00221054;

/// (0022,1059) VR=FL, VM=1 Ophthalmic Axial Length Velocity
pub const OphthalmicAxialLengthVelocity: u32 = 0x00221059;

/// (0022,1065) VR=LO, VM=1 Lens Status Description
pub const LensStatusDescription: u32 = 0x00221065;

/// (0022,1066) VR=LO, VM=1 Vitreous Status Description
pub const VitreousStatusDescription: u32 = 0x00221066;

/// (0022,1090) VR=SQ, VM=1 IOL Power Sequence
pub const IOLPowerSequence: u32 = 0x00221090;

/// (0022,1092) VR=SQ, VM=1 Lens Constant Sequence
pub const LensConstantSequence: u32 = 0x00221092;

/// (0022,1093) VR=LO, VM=1 IOL Manufacturer
pub const IOLManufacturer: u32 = 0x00221093;

/// (0022,1094) VR=LO, VM=1 Lens Constant Description
pub const LensConstantDescription: u32 = 0x00221094;

/// (0022,1096) VR=SQ, VM=1 Keratometry Measurement Type Code Sequence
pub const KeratometryMeasurementTypeCodeSequence: u32 = 0x00221096;

/// (0022,1100) VR=SQ, VM=1 Referenced Ophthalmic Axial Measurements Sequence
pub const ReferencedOphthalmicAxialMeasurementsSequence: u32 = 0x00221100;

/// (0022,1101) VR=SQ, VM=1 Ophthalmic Axial Length Measurements Segment Name Code Sequence
pub const OphthalmicAxialLengthMeasurementsSegmentNameCodeSequence: u32 = 0x00221101;

/// (0022,1103) VR=SQ, VM=1 Refractive Error Before Refractive Surgery Code Sequence
pub const RefractiveErrorBeforeRefractiveSurgeryCodeSequence: u32 = 0x00221103;

/// (0022,1121) VR=FL, VM=1 IOL Power For Exact Emmetropia
pub const IOLPowerForExactEmmetropia: u32 = 0x00221121;

/// (0022,1122) VR=FL, VM=1 IOL Power For Exact Target Refraction
pub const IOLPowerForExactTargetRefraction: u32 = 0x00221122;

/// (0022,1125) VR=SQ, VM=1 Anterior Chamber Depth Definition Code Sequence
pub const AnteriorChamberDepthDefinitionCodeSequence: u32 = 0x00221125;

/// (0022,1130) VR=FL, VM=1 Lens Thickness
pub const LensThickness: u32 = 0x00221130;

/// (0022,1131) VR=FL, VM=1 Anterior Chamber Depth
pub const AnteriorChamberDepth: u32 = 0x00221131;

/// (0022,1132) VR=SQ, VM=1 Source of Lens Thickness Data Code Sequence
pub const SourceOfLensThicknessDataCodeSequence: u32 = 0x00221132;

/// (0022,1133) VR=SQ, VM=1 Source of Anterior Chamber Depth Data Code Sequence
pub const SourceOfAnteriorChamberDepthDataCodeSequence: u32 = 0x00221133;

/// (0022,1135) VR=SQ, VM=1 Source of Refractive Error Data Code Sequence
pub const SourceOfRefractiveErrorDataCodeSequence: u32 = 0x00221135;

/// (0022,1140) VR=CS, VM=1 Ophthalmic Axial Length Measurement Modified
pub const OphthalmicAxialLengthMeasurementModified: u32 = 0x00221140;

/// (0022,1150) VR=SQ, VM=1 Ophthalmic Axial Length Data Source Code Sequence
pub const OphthalmicAxialLengthDataSourceCodeSequence: u32 = 0x00221150;

/// (0022,1153) VR=SQ, VM=1 Ophthalmic Axial Length Acquisition Method Code Sequence
pub const OphthalmicAxialLengthAcquisitionMethodCodeSequence: u32 = 0x00221153;

/// (0022,1155) VR=FL, VM=1 Signal to Noise Ratio
pub const SignalToNoiseRatio: u32 = 0x00221155;

/// (0022,1159) VR=LO, VM=1 Ophthalmic Axial Length Data Source Description
pub const OphthalmicAxialLengthDataSourceDescription: u32 = 0x00221159;

/// (0022,1210) VR=SQ, VM=1 Ophthalmic Axial Length Measurements Total Length Sequence
pub const OphthalmicAxialLengthMeasurementsTotalLengthSequence: u32 = 0x00221210;

/// (0022,1211) VR=SQ, VM=1 Ophthalmic Axial Length Measurements Segmental Length Sequence
pub const OphthalmicAxialLengthMeasurementsSegmentalLengthSequence: u32 = 0x00221211;

/// (0022,1212) VR=SQ, VM=1 Ophthalmic Axial Length Measurements Length Summation Sequence
pub const OphthalmicAxialLengthMeasurementsLengthSummationSequence: u32 = 0x00221212;

/// (0022,1220) VR=SQ, VM=1 Ultrasound Ophthalmic Axial Length Measurements Sequence
pub const UltrasoundOphthalmicAxialLengthMeasurementsSequence: u32 = 0x00221220;

/// (0022,1225) VR=SQ, VM=1 Optical Ophthalmic Axial Length Measurements Sequence
pub const OpticalOphthalmicAxialLengthMeasurementsSequence: u32 = 0x00221225;

/// (0022,1230) VR=SQ, VM=1 Ultrasound Selected Ophthalmic Axial Length Sequence
pub const UltrasoundSelectedOphthalmicAxialLengthSequence: u32 = 0x00221230;

/// (0022,1250) VR=SQ, VM=1 Ophthalmic Axial Length Selection Method Code Sequence
pub const OphthalmicAxialLengthSelectionMethodCodeSequence: u32 = 0x00221250;

/// (0022,1255) VR=SQ, VM=1 Optical Selected Ophthalmic Axial Length Sequence
pub const OpticalSelectedOphthalmicAxialLengthSequence: u32 = 0x00221255;

/// (0022,1257) VR=SQ, VM=1 Selected Segmental Ophthalmic Axial Length Sequence
pub const SelectedSegmentalOphthalmicAxialLengthSequence: u32 = 0x00221257;

/// (0022,1260) VR=SQ, VM=1 Selected Total Ophthalmic Axial Length Sequence
pub const SelectedTotalOphthalmicAxialLengthSequence: u32 = 0x00221260;

/// (0022,1262) VR=SQ, VM=1 Ophthalmic Axial Length Quality Metric Sequence
pub const OphthalmicAxialLengthQualityMetricSequence: u32 = 0x00221262;

/// (0022,1273) VR=LO, VM=1 Ophthalmic Axial  Length Quality Metric Type Description
pub const OphthalmicAxialLengthQualityMetricTypeDescription: u32 = 0x00221273;

/// (0022,1300) VR=SQ, VM=1 Intraocular Lens Calculations Right Eye Sequence
pub const IntraocularLensCalculationsRightEyeSequence: u32 = 0x00221300;

/// (0022,1310) VR=SQ, VM=1 Intraocular Lens Calculations Left Eye Sequence
pub const IntraocularLensCalculationsLeftEyeSequence: u32 = 0x00221310;

/// (0022,1330) VR=SQ, VM=1 Referenced Ophthalmic Axial Length Measurement QC Image Sequence
pub const ReferencedOphthalmicAxialLengthMeasurementQCImageSequence: u32 = 0x00221330;

/// (0024,0010) VR=FL, VM=1 Visual Field Horizontal Extent
pub const VisualFieldHorizontalExtent: u32 = 0x00240010;

/// (0024,0011) VR=FL, VM=1 Visual Field Vertical Extent
pub const VisualFieldVerticalExtent: u32 = 0x00240011;

/// (0024,0012) VR=CS, VM=1 Visual Field Shape
pub const VisualFieldShape: u32 = 0x00240012;

/// (0024,0016) VR=SQ, VM=1 Screening Test Mode Code Sequence
pub const ScreeningTestModeCodeSequence: u32 = 0x00240016;

/// (0024,0018) VR=FL, VM=1 Maximum Stimulus Luminance
pub const MaximumStimulusLuminance: u32 = 0x00240018;

/// (0024,0020) VR=FL, VM=1 Background Luminance
pub const BackgroundLuminance: u32 = 0x00240020;

/// (0024,0021) VR=SQ, VM=1 Stimulus Color Code Sequence
pub const StimulusColorCodeSequence: u32 = 0x00240021;

/// (0024,0024) VR=SQ, VM=1 Background Illumination Color Code Sequence
pub const BackgroundIlluminationColorCodeSequence: u32 = 0x00240024;

/// (0024,0025) VR=FL, VM=1 Stimulus Area
pub const StimulusArea: u32 = 0x00240025;

/// (0024,0028) VR=FL, VM=1 Stimulus Presentation Time
pub const StimulusPresentationTime: u32 = 0x00240028;

/// (0024,0032) VR=SQ, VM=1 Fixation Sequence
pub const FixationSequence: u32 = 0x00240032;

/// (0024,0033) VR=SQ, VM=1 Fixation Monitoring Code Sequence
pub const FixationMonitoringCodeSequence: u32 = 0x00240033;

/// (0024,0034) VR=SQ, VM=1 Visual Field Catch Trial Sequence
pub const VisualFieldCatchTrialSequence: u32 = 0x00240034;

/// (0024,0035) VR=US, VM=1 Fixation Checked Quantity
pub const FixationCheckedQuantity: u32 = 0x00240035;

/// (0024,0036) VR=US, VM=1 Patient Not Properly Fixated Quantity
pub const PatientNotProperlyFixatedQuantity: u32 = 0x00240036;

/// (0024,0037) VR=CS, VM=1 Presented Visual Stimuli Data Flag
pub const PresentedVisualStimuliDataFlag: u32 = 0x00240037;

/// (0024,0038) VR=US, VM=1 Number of Visual Stimuli
pub const NumberOfVisualStimuli: u32 = 0x00240038;

/// (0024,0039) VR=CS, VM=1 Excessive Fixation Losses Data Flag
pub const ExcessiveFixationLossesDataFlag: u32 = 0x00240039;

/// (0024,0040) VR=CS, VM=1 Excessive Fixation Losses
pub const ExcessiveFixationLosses: u32 = 0x00240040;

/// (0024,0042) VR=US, VM=1 Stimuli Retesting Quantity
pub const StimuliRetestingQuantity: u32 = 0x00240042;

/// (0024,0044) VR=LT, VM=1 Comments on Patient’s Performance of Visual Field
pub const CommentsOnPatientPerformanceOfVisualField: u32 = 0x00240044;

/// (0024,0045) VR=CS, VM=1 False Negatives Estimate Flag
pub const FalseNegativesEstimateFlag: u32 = 0x00240045;

/// (0024,0046) VR=FL, VM=1 False Negatives Estimate
pub const FalseNegativesEstimate: u32 = 0x00240046;

/// (0024,0048) VR=US, VM=1 Negative Catch Trials Quantity
pub const NegativeCatchTrialsQuantity: u32 = 0x00240048;

/// (0024,0050) VR=US, VM=1 False Negatives Quantity
pub const FalseNegativesQuantity: u32 = 0x00240050;

/// (0024,0051) VR=CS, VM=1 Excessive False Negatives Data Flag
pub const ExcessiveFalseNegativesDataFlag: u32 = 0x00240051;

/// (0024,0052) VR=CS, VM=1 Excessive False Negatives
pub const ExcessiveFalseNegatives: u32 = 0x00240052;

/// (0024,0053) VR=CS, VM=1 False Positives Estimate Flag
pub const FalsePositivesEstimateFlag: u32 = 0x00240053;

/// (0024,0054) VR=FL, VM=1 False Positives Estimate
pub const FalsePositivesEstimate: u32 = 0x00240054;

/// (0024,0055) VR=CS, VM=1 Catch Trials Data Flag
pub const CatchTrialsDataFlag: u32 = 0x00240055;

/// (0024,0056) VR=US, VM=1 Positive Catch Trials Quantity
pub const PositiveCatchTrialsQuantity: u32 = 0x00240056;

/// (0024,0057) VR=CS, VM=1 Test Point Normals Data Flag
pub const TestPointNormalsDataFlag: u32 = 0x00240057;

/// (0024,0058) VR=SQ, VM=1 Test Point Normals Sequence
pub const TestPointNormalsSequence: u32 = 0x00240058;

/// (0024,0059) VR=CS, VM=1 Global Deviation Probability Normals Flag
pub const GlobalDeviationProbabilityNormalsFlag: u32 = 0x00240059;

/// (0024,0060) VR=US, VM=1 False Positives Quantity
pub const FalsePositivesQuantity: u32 = 0x00240060;

/// (0024,0061) VR=CS, VM=1 Excessive False Positives Data Flag
pub const ExcessiveFalsePositivesDataFlag: u32 = 0x00240061;

/// (0024,0062) VR=CS, VM=1 Excessive False Positives
pub const ExcessiveFalsePositives: u32 = 0x00240062;

/// (0024,0063) VR=CS, VM=1 Visual Field Test Normals Flag
pub const VisualFieldTestNormalsFlag: u32 = 0x00240063;

/// (0024,0064) VR=SQ, VM=1 Results Normals Sequence
pub const ResultsNormalsSequence: u32 = 0x00240064;

/// (0024,0065) VR=SQ, VM=1 Age Corrected Sensitivity Deviation Algorithm Sequence
pub const AgeCorrectedSensitivityDeviationAlgorithmSequence: u32 = 0x00240065;

/// (0024,0066) VR=FL, VM=1 Global Deviation From Normal
pub const GlobalDeviationFromNormal: u32 = 0x00240066;

/// (0024,0067) VR=SQ, VM=1 Generalized Defect Sensitivity Deviation Algorithm Sequence
pub const GeneralizedDefectSensitivityDeviationAlgorithmSequence: u32 = 0x00240067;

/// (0024,0068) VR=FL, VM=1 Localized Deviation from Normal
pub const LocalizedDeviationfromNormal: u32 = 0x00240068;

/// (0024,0069) VR=LO, VM=1 Patient Reliability Indicator
pub const PatientReliabilityIndicator: u32 = 0x00240069;

/// (0024,0070) VR=FL, VM=1 Visual Field Mean Sensitivity
pub const VisualFieldMeanSensitivity: u32 = 0x00240070;

/// (0024,0071) VR=FL, VM=1 Global Deviation Probability
pub const GlobalDeviationProbability: u32 = 0x00240071;

/// (0024,0072) VR=CS, VM=1 Local Deviation Probability Normals Flag
pub const LocalDeviationProbabilityNormalsFlag: u32 = 0x00240072;

/// (0024,0073) VR=FL, VM=1 Localized Deviation Probability
pub const LocalizedDeviationProbability: u32 = 0x00240073;

/// (0024,0074) VR=CS, VM=1 Short Term Fluctuation Calculated
pub const ShortTermFluctuationCalculated: u32 = 0x00240074;

/// (0024,0075) VR=FL, VM=1 Short Term Fluctuation
pub const ShortTermFluctuation: u32 = 0x00240075;

/// (0024,0076) VR=CS, VM=1 Short Term Fluctuation Probability Calculated
pub const ShortTermFluctuationProbabilityCalculated: u32 = 0x00240076;

/// (0024,0077) VR=FL, VM=1 Short Term Fluctuation Probability
pub const ShortTermFluctuationProbability: u32 = 0x00240077;

/// (0024,0078) VR=CS, VM=1 Corrected Localized Deviation From Normal Calculated
pub const CorrectedLocalizedDeviationFromNormalCalculated: u32 = 0x00240078;

/// (0024,0079) VR=FL, VM=1 Corrected Localized Deviation From Normal
pub const CorrectedLocalizedDeviationFromNormal: u32 = 0x00240079;

/// (0024,0080) VR=CS, VM=1 Corrected Localized Deviation From Normal Probability Calculated
pub const CorrectedLocalizedDeviationFromNormalProbabilityCalculated: u32 = 0x00240080;

/// (0024,0081) VR=FL, VM=1 Corrected Localized Deviation From Normal Probability
pub const CorrectedLocalizedDeviationFromNormalProbability: u32 = 0x00240081;

/// (0024,0083) VR=SQ, VM=1 Global Deviation Probability Sequence
pub const GlobalDeviationProbabilitySequence: u32 = 0x00240083;

/// (0024,0085) VR=SQ, VM=1 Localized Deviation Probability Sequence
pub const LocalizedDeviationProbabilitySequence: u32 = 0x00240085;

/// (0024,0086) VR=CS, VM=1 Foveal Sensitivity Measured
pub const FovealSensitivityMeasured: u32 = 0x00240086;

/// (0024,0087) VR=FL, VM=1 Foveal Sensitivity
pub const FovealSensitivity: u32 = 0x00240087;

/// (0024,0088) VR=FL, VM=1 Visual Field Test Duration
pub const VisualFieldTestDuration: u32 = 0x00240088;

/// (0024,0089) VR=SQ, VM=1 Visual Field Test Point Sequence
pub const VisualFieldTestPointSequence: u32 = 0x00240089;

/// (0024,0090) VR=FL, VM=1 Visual Field Test Point X-Coordinate
pub const VisualFieldTestPointXCoordinate: u32 = 0x00240090;

/// (0024,0091) VR=FL, VM=1 Visual Field Test Point Y-Coordinate
pub const VisualFieldTestPointYCoordinate: u32 = 0x00240091;

/// (0024,0092) VR=FL, VM=1 Age Corrected Sensitivity Deviation Value
pub const AgeCorrectedSensitivityDeviationValue: u32 = 0x00240092;

/// (0024,0093) VR=CS, VM=1 Stimulus Results
pub const StimulusResults: u32 = 0x00240093;

/// (0024,0094) VR=FL, VM=1 Sensitivity Value
pub const SensitivityValue: u32 = 0x00240094;

/// (0024,0095) VR=CS, VM=1 Retest Stimulus Seen
pub const RetestStimulusSeen: u32 = 0x00240095;

/// (0024,0096) VR=FL, VM=1 Retest Sensitivity Value
pub const RetestSensitivityValue: u32 = 0x00240096;

/// (0024,0097) VR=SQ, VM=1 Visual Field Test Point Normals Sequence
pub const VisualFieldTestPointNormalsSequence: u32 = 0x00240097;

/// (0024,0098) VR=FL, VM=1 Quantified Defect
pub const QuantifiedDefect: u32 = 0x00240098;

/// (0024,0100) VR=FL, VM=1 Age Corrected Sensitivity Deviation Probability Value
pub const AgeCorrectedSensitivityDeviationProbabilityValue: u32 = 0x00240100;

/// (0024,0102) VR=CS, VM=1 Generalized Defect Corrected Sensitivity Deviation Flag
pub const GeneralizedDefectCorrectedSensitivityDeviationFlag: u32  = 0x00240102;

/// (0024,0103) VR=FL, VM=1 Generalized Defect Corrected Sensitivity Deviation Value
pub const GeneralizedDefectCorrectedSensitivityDeviationValue: u32  = 0x00240103;

/// (0024,0104) VR=FL, VM=1 Generalized Defect Corrected Sensitivity Deviation Probability Value
pub const GeneralizedDefectCorrectedSensitivityDeviationProbabilityValue: u32 = 0x00240104;

/// (0024,0105) VR=FL, VM=1 Minimum Sensitivity Value
pub const MinimumSensitivityValue: u32 = 0x00240105;

/// (0024,0106) VR=CS, VM=1 Blind Spot Localized
pub const BlindSpotLocalized: u32 = 0x00240106;

/// (0024,0107) VR=FL, VM=1 Blind Spot X-Coordinate
pub const BlindSpotXCoordinate: u32 = 0x00240107;

/// (0024,0108) VR=FL, VM=1 Blind Spot Y-Coordinate
pub const BlindSpotYCoordinate: u32  = 0x00240108;

/// (0024,0110) VR=SQ, VM=1 Visual Acuity Measurement Sequence
pub const VisualAcuityMeasurementSequence: u32 = 0x00240110;

/// (0024,0112) VR=SQ, VM=1 Refractive Parameters Used on Patient Sequence
pub const RefractiveParametersUsedOnPatientSequence: u32 = 0x00240112;

/// (0024,0113) VR=CS, VM=1 Measurement Laterality
pub const MeasurementLaterality: u32 = 0x00240113;

/// (0024,0114) VR=SQ, VM=1 Ophthalmic Patient Clinical Information Left Eye Sequence
pub const OphthalmicPatientClinicalInformationLeftEyeSequence: u32 = 0x00240114;

/// (0024,0115) VR=SQ, VM=1 Ophthalmic Patient Clinical Information Right Eye Sequence
pub const OphthalmicPatientClinicalInformationRightEyeSequence: u32 = 0x00240115;

/// (0024,0117) VR=CS, VM=1 Foveal Point Normative Data Flag
pub const FovealPointNormativeDataFlag: u32 = 0x00240117;

/// (0024,0118) VR=FL, VM=1 Foveal Point Probability Value
pub const FovealPointProbabilityValue: u32 = 0x00240118;

/// (0024,0120) VR=CS, VM=1 Screening Baseline Measured
pub const ScreeningBaselineMeasured: u32 = 0x00240120;

/// (0024,0122) VR=SQ, VM=1 Screening Baseline Measured Sequence
pub const ScreeningBaselineMeasuredSequence: u32 = 0x00240122;

/// (0024,0124) VR=CS, VM=1 Screening Baseline Type
pub const ScreeningBaselineType: u32 = 0x00240124;

/// (0024,0126) VR=FL, VM=1 Screening Baseline Value
pub const ScreeningBaselineValue: u32 = 0x00240126;

/// (0024,0202) VR=LO, VM=1 Algorithm Source
pub const AlgorithmSource: u32 = 0x00240202;

/// (0024,0306) VR=LO, VM=1 Data Set Name
pub const DataSetName: u32 = 0x00240306;

/// (0024,0307) VR=LO, VM=1 Data Set Version
pub const DataSetVersion: u32 = 0x00240307;

/// (0024,0308) VR=LO, VM=1 Data Set Source
pub const DataSetSource: u32 = 0x00240308;

/// (0024,0309) VR=LO, VM=1 Data Set Description
pub const DataSetDescription: u32 = 0x00240309;

/// (0024,0317) VR=SQ, VM=1 Visual Field Test Reliability Global Index Sequence
pub const VisualFieldTestReliabilityGlobalIndexSequence: u32 = 0x00240317;

/// (0024,0320) VR=SQ, VM=1 Visual Field Global Results Index Sequence
pub const VisualFieldGlobalResultsIndexSequence: u32 = 0x00240320;

/// (0024,0325) VR=SQ, VM=1 Data Observation Sequence
pub const DataObservationSequence: u32 = 0x00240325;

/// (0024,0338) VR=CS, VM=1 Index Normals Flag
pub const IndexNormalsFlag: u32 = 0x00240338;

/// (0024,0341) VR=FL, VM=1 Index Probability
pub const IndexProbability: u32 = 0x00240341;

/// (0024,0344) VR=SQ, VM=1 Index Probability Sequence
pub const IndexProbabilitySequence: u32 = 0x00240344;

/// (0028,0002) VR=US, VM=1 Samples per Pixel
pub const SamplesPerPixel: u32 = 0x00280002;

/// (0028,0003) VR=US, VM=1 Samples per Pixel Used
pub const SamplesPerPixelUsed: u32 = 0x00280003;

/// (0028,0004) VR=CS, VM=1 Photometric Interpretation
pub const PhotometricInterpretation: u32 = 0x00280004;

/// (0028,0005) VR=US, VM=1 Image Dimensions (retired)
pub const ImageDimensions: u32 = 0x00280005;

/// (0028,0006) VR=US, VM=1 Planar Configuration
pub const PlanarConfiguration: u32 = 0x00280006;

/// (0028,0008) VR=IS, VM=1 Number of Frames
pub const NumberOfFrames: u32 = 0x00280008;

/// (0028,0009) VR=AT, VM=1-n Frame Increment Pointer
pub const FrameIncrementPointer: u32 = 0x00280009;

/// (0028,000A) VR=AT, VM=1-n Frame Dimension Pointer
pub const FrameDimensionPointer: u32 = 0x0028000A;

/// (0028,0010) VR=US, VM=1 Rows
pub const Rows: u32 = 0x00280010;

/// (0028,0011) VR=US, VM=1 Columns
pub const Columns: u32 = 0x00280011;

/// (0028,0012) VR=US, VM=1 Planes (retired)
pub const Planes: u32 = 0x00280012;

/// (0028,0014) VR=US, VM=1 Ultrasound Color Data Present
pub const UltrasoundColorDataPresent: u32 = 0x00280014;

/// (0028,0030) VR=DS, VM=2 Pixel Spacing
pub const PixelSpacing: u32 = 0x00280030;

/// (0028,0031) VR=DS, VM=2 Zoom Factor
pub const ZoomFactor: u32 = 0x00280031;

/// (0028,0032) VR=DS, VM=2 Zoom Center
pub const ZoomCenter: u32 = 0x00280032;

/// (0028,0034) VR=IS, VM=2 Pixel Aspect Ratio
pub const PixelAspectRatio: u32 = 0x00280034;

/// (0028,0040) VR=CS, VM=1 Image Format (retired)
pub const ImageFormat: u32 = 0x00280040;

/// (0028,0050) VR=LO, VM=1-n Manipulated Image (retired)
pub const ManipulatedImage: u32 = 0x00280050;

/// (0028,0051) VR=CS, VM=1-n Corrected Image
pub const CorrectedImage: u32 = 0x00280051;

/// (0028,005F) VR=LO, VM=1 Compression Recognition Code (retired)
pub const CompressionRecognitionCode: u32 = 0x0028005F;

/// (0028,0060) VR=CS, VM=1 Compression Code (retired)
pub const CompressionCode: u32 = 0x00280060;

/// (0028,0061) VR=SH, VM=1 Compression Originator (retired)
pub const CompressionOriginator: u32 = 0x00280061;

/// (0028,0062) VR=LO, VM=1 Compression Label (retired)
pub const CompressionLabel: u32 = 0x00280062;

/// (0028,0063) VR=SH, VM=1 Compression Description (retired)
pub const CompressionDescription: u32 = 0x00280063;

/// (0028,0065) VR=CS, VM=1-n Compression Sequence (retired)
pub const CompressionSequence: u32 = 0x00280065;

/// (0028,0066) VR=AT, VM=1-n Compression Step Pointers (retired)
pub const CompressionStepPointers: u32 = 0x00280066;

/// (0028,0068) VR=US, VM=1 Repeat Interval (retired)
pub const RepeatInterval: u32 = 0x00280068;

/// (0028,0069) VR=US, VM=1 Bits Grouped (retired)
pub const BitsGrouped: u32 = 0x00280069;

/// (0028,0070) VR=US, VM=1-n Perimeter Table (retired)
pub const PerimeterTable: u32 = 0x00280070;

/// (0028,0071) VR=US|SS, VM=1 Perimeter Value (retired)
pub const PerimeterValue: u32 = 0x00280071;

/// (0028,0080) VR=US, VM=1 Predictor Rows (retired)
pub const PredictorRows: u32 = 0x00280080;

/// (0028,0081) VR=US, VM=1 Predictor Columns (retired)
pub const PredictorColumns: u32 = 0x00280081;

/// (0028,0082) VR=US, VM=1-n Predictor Constants (retired)
pub const PredictorConstants: u32 = 0x00280082;

/// (0028,0090) VR=CS, VM=1 Blocked Pixels (retired)
pub const BlockedPixels: u32 = 0x00280090;

/// (0028,0091) VR=US, VM=1 Block Rows (retired)
pub const BlockRows: u32 = 0x00280091;

/// (0028,0092) VR=US, VM=1 Block Columns (retired)
pub const BlockColumns: u32 = 0x00280092;

/// (0028,0093) VR=US, VM=1 Row Overlap (retired)
pub const RowOverlap: u32 = 0x00280093;

/// (0028,0094) VR=US, VM=1 Column Overlap (retired)
pub const ColumnOverlap: u32 = 0x00280094;

/// (0028,0100) VR=US, VM=1 Bits Allocated
pub const BitsAllocated: u32 = 0x00280100;

/// (0028,0101) VR=US, VM=1 Bits Stored
pub const BitsStored: u32 = 0x00280101;

/// (0028,0102) VR=US, VM=1 High Bit
pub const HighBit: u32 = 0x00280102;

/// (0028,0103) VR=US, VM=1 Pixel Representation
pub const PixelRepresentation: u32 = 0x00280103;

/// (0028,0104) VR=US|SS, VM=1 Smallest Valid Pixel Value (retired)
pub const SmallestValidPixelValue: u32 = 0x00280104;

/// (0028,0105) VR=US|SS, VM=1 Largest Valid Pixel Value (retired)
pub const LargestValidPixelValue: u32 = 0x00280105;

/// (0028,0106) VR=US|SS, VM=1 Smallest Image Pixel Value
pub const SmallestImagePixelValue: u32 = 0x00280106;

/// (0028,0107) VR=US|SS, VM=1 Largest Image Pixel Value
pub const LargestImagePixelValue: u32 = 0x00280107;

/// (0028,0108) VR=US|SS, VM=1 Smallest Pixel Value in Series
pub const SmallestPixelValueInSeries: u32 = 0x00280108;

/// (0028,0109) VR=US|SS, VM=1 Largest Pixel Value in Series
pub const LargestPixelValueInSeries: u32 = 0x00280109;

/// (0028,0110) VR=US|SS, VM=1 Smallest Image Pixel Value in Plane (retired)
pub const SmallestImagePixelValueInPlane: u32 = 0x00280110;

/// (0028,0111) VR=US|SS, VM=1 Largest Image Pixel Value in Plane (retired)
pub const LargestImagePixelValueInPlane: u32 = 0x00280111;

/// (0028,0120) VR=US|SS, VM=1 Pixel Padding Value
pub const PixelPaddingValue: u32 = 0x00280120;

/// (0028,0121) VR=US|SS, VM=1 Pixel Padding Range Limit
pub const PixelPaddingRangeLimit: u32 = 0x00280121;

/// (0028,0200) VR=US, VM=1 Image Location (retired)
pub const ImageLocation: u32 = 0x00280200;

/// (0028,0300) VR=CS, VM=1 Quality Control Image
pub const QualityControlImage: u32 = 0x00280300;

/// (0028,0301) VR=CS, VM=1 Burned In Annotation
pub const BurnedInAnnotation: u32 = 0x00280301;

/// (0028,0302) VR=CS, VM=1 Recognizable Visual Features
pub const RecognizableVisualFeatures: u32 = 0x00280302;

/// (0028,0303) VR=CS, VM=1 Longitudinal Temporal Information Modified
pub const LongitudinalTemporalInformationModified: u32 = 0x00280303;

/// (0028,0400) VR=LO, VM=1 Transform Label (retired)
pub const TransformLabel: u32 = 0x00280400;

/// (0028,0401) VR=LO, VM=1 Transform Version Number (retired)
pub const TransformVersionNumber: u32 = 0x00280401;

/// (0028,0402) VR=US, VM=1 Number of Transform Steps (retired)
pub const NumberOfTransformSteps: u32 = 0x00280402;

/// (0028,0403) VR=LO, VM=1-n Sequence of Compressed Data (retired)
pub const SequenceOfCompressedData: u32 = 0x00280403;

/// (0028,0404) VR=AT, VM=1-n Details of Coefficients (retired)
pub const DetailsOfCoefficients: u32 = 0x00280404;

/// (0028,04x0) VR=US, VM=1 Rows For Nth Order Coefficients (retired)
pub const RowsForNthOrderCoefficients: u32 = 0x00280400;

/// (0028,04x1) VR=US, VM=1 Columns For Nth Order Coefficients (retired)
pub const ColumnsForNthOrderCoefficients: u32 = 0x00280401;

/// (0028,04x2) VR=LO, VM=1-n Coefficient Coding (retired)
pub const CoefficientCoding: u32 = 0x00280402;

/// (0028,04x3) VR=AT, VM=1-n Coefficient Coding Pointers (retired)
pub const CoefficientCodingPointers: u32 = 0x00280403;

/// (0028,0700) VR=LO, VM=1 DCT Label (retired)
pub const DCTLabel: u32 = 0x00280700;

/// (0028,0701) VR=CS, VM=1-n Data Block Description (retired)
pub const DataBlockDescription: u32 = 0x00280701;

/// (0028,0702) VR=AT, VM=1-n Data Block (retired)
pub const DataBlock: u32 = 0x00280702;

/// (0028,0710) VR=US, VM=1 Normalization Factor Format (retired)
pub const NormalizationFactorFormat: u32 = 0x00280710;

/// (0028,0720) VR=US, VM=1 Zonal Map Number Format (retired)
pub const ZonalMapNumberFormat: u32 = 0x00280720;

/// (0028,0721) VR=AT, VM=1-n Zonal Map Location (retired)
pub const ZonalMapLocation: u32 = 0x00280721;

/// (0028,0722) VR=US, VM=1 Zonal Map Format (retired)
pub const ZonalMapFormat: u32 = 0x00280722;

/// (0028,0730) VR=US, VM=1 Adaptive Map Format (retired)
pub const AdaptiveMapFormat: u32 = 0x00280730;

/// (0028,0740) VR=US, VM=1 Code Number Format (retired)
pub const CodeNumberFormat: u32 = 0x00280740;

/// (0028,08x0) VR=CS, VM=1-n Code Label (retired)
pub const CodeLabel: u32 = 0x00280800;

/// (0028,08x2) VR=US, VM=1 Number of Tables (retired)
pub const NumberOfTables: u32 = 0x00280802;

/// (0028,08x3) VR=AT, VM=1-n Code Table Location (retired)
pub const CodeTableLocation: u32 = 0x00280803;

/// (0028,08x4) VR=US, VM=1 Bits For Code Word (retired)
pub const BitsForCodeWord: u32 = 0x00280804;

/// (0028,08x8) VR=AT, VM=1-n Image Data Location (retired)
pub const ImageDataLocation: u32 = 0x00280808;

/// (0028,0A02) VR=CS, VM=1 Pixel Spacing Calibration Type
pub const PixelSpacingCalibrationType: u32 = 0x00280A02;

/// (0028,0A04) VR=LO, VM=1 Pixel Spacing Calibration Description
pub const PixelSpacingCalibrationDescription: u32 = 0x00280A04;

/// (0028,1040) VR=CS, VM=1 Pixel Intensity Relationship
pub const PixelIntensityRelationship: u32 = 0x00281040;

/// (0028,1041) VR=SS, VM=1 Pixel Intensity Relationship Sign
pub const PixelIntensityRelationshipSign: u32 = 0x00281041;

/// (0028,1050) VR=DS, VM=1-n Window Center
pub const WindowCenter: u32 = 0x00281050;

/// (0028,1051) VR=DS, VM=1-n Window Width
pub const WindowWidth: u32 = 0x00281051;

/// (0028,1052) VR=DS, VM=1 Rescale Intercept
pub const RescaleIntercept: u32 = 0x00281052;

/// (0028,1053) VR=DS, VM=1 Rescale Slope
pub const RescaleSlope: u32 = 0x00281053;

/// (0028,1054) VR=LO, VM=1 Rescale Type
pub const RescaleType: u32 = 0x00281054;

/// (0028,1055) VR=LO, VM=1-n Window Center & Width Explanation
pub const WindowCenterWidthExplanation: u32 = 0x00281055;

/// (0028,1056) VR=CS, VM=1 VOI LUT Function
pub const VOILUTFunction: u32 = 0x00281056;

/// (0028,1080) VR=CS, VM=1 Gray Scale (retired)
pub const GrayScale: u32 = 0x00281080;

/// (0028,1090) VR=CS, VM=1 Recommended Viewing Mode
pub const RecommendedViewingMode: u32 = 0x00281090;

/// (0028,1100) VR=US|SS, VM=3 Gray Lookup Table Descriptor (retired)
pub const GrayLookupTableDescriptor: u32 = 0x00281100;

/// (0028,1101) VR=US|SS, VM=3 Red Palette Color Lookup Table Descriptor
pub const RedPaletteColorLookupTableDescriptor: u32 = 0x00281101;

/// (0028,1102) VR=US|SS, VM=3 Green Palette Color Lookup Table Descriptor
pub const GreenPaletteColorLookupTableDescriptor: u32 = 0x00281102;

/// (0028,1103) VR=US|SS, VM=3 Blue Palette Color Lookup Table Descriptor
pub const BluePaletteColorLookupTableDescriptor: u32 = 0x00281103;

/// (0028,1104) VR=US, VM=3 Alpha Palette Color Lookup Table Descriptor
pub const AlphaPaletteColorLookupTableDescriptor: u32 = 0x00281104;

/// (0028,1111) VR=US|SS, VM=4 Large Red Palette Color Lookup Table Descriptor (retired)
pub const LargeRedPaletteColorLookupTableDescriptor: u32 = 0x00281111;

/// (0028,1112) VR=US|SS, VM=4 Large Green Palette Color Lookup Table Descriptor (retired)
pub const LargeGreenPaletteColorLookupTableDescriptor: u32 = 0x00281112;

/// (0028,1113) VR=US|SS, VM=4 Large Blue Palette Color Lookup Table Descriptor (retired)
pub const LargeBluePaletteColorLookupTableDescriptor: u32 = 0x00281113;

/// (0028,1199) VR=UI, VM=1 Palette Color Lookup Table UID
pub const PaletteColorLookupTableUID: u32 = 0x00281199;

/// (0028,1200) VR=US|SS|OW, VM=1-n1 Gray Lookup Table Data (retired)
pub const GrayLookupTableData: u32 = 0x00281200;

/// (0028,1201) VR=OW, VM=1 Red Palette Color Lookup Table Data
pub const RedPaletteColorLookupTableData: u32 = 0x00281201;

/// (0028,1202) VR=OW, VM=1 Green Palette Color Lookup Table Data
pub const GreenPaletteColorLookupTableData: u32 = 0x00281202;

/// (0028,1203) VR=OW, VM=1 Blue Palette Color Lookup Table Data
pub const BluePaletteColorLookupTableData: u32 = 0x00281203;

/// (0028,1204) VR=OW, VM=1 Alpha Palette Color Lookup Table Data
pub const AlphaPaletteColorLookupTableData: u32 = 0x00281204;

/// (0028,1211) VR=OW, VM=1 Large Red Palette Color Lookup Table Data (retired)
pub const LargeRedPaletteColorLookupTableData: u32 = 0x00281211;

/// (0028,1212) VR=OW, VM=1 Large Green Palette Color Lookup Table Data (retired)
pub const LargeGreenPaletteColorLookupTableData: u32 = 0x00281212;

/// (0028,1213) VR=OW, VM=1 Large Blue Palette Color Lookup Table Data (retired)
pub const LargeBluePaletteColorLookupTableData: u32 = 0x00281213;

/// (0028,1214) VR=UI, VM=1 Large Palette Color Lookup Table UID (retired)
pub const LargePaletteColorLookupTableUID: u32 = 0x00281214;

/// (0028,1221) VR=OW, VM=1 Segmented Red Palette Color Lookup Table Data
pub const SegmentedRedPaletteColorLookupTableData: u32 = 0x00281221;

/// (0028,1222) VR=OW, VM=1 Segmented Green Palette Color Lookup Table Data
pub const SegmentedGreenPaletteColorLookupTableData: u32 = 0x00281222;

/// (0028,1223) VR=OW, VM=1 Segmented Blue Palette Color Lookup Table Data
pub const SegmentedBluePaletteColorLookupTableData: u32 = 0x00281223;

/// (0028,1300) VR=CS, VM=1 Breast Implant Present
pub const BreastImplantPresent: u32 = 0x00281300;

/// (0028,1350) VR=CS, VM=1 Partial View
pub const PartialView: u32 = 0x00281350;

/// (0028,1351) VR=ST, VM=1 Partial View Description
pub const PartialViewDescription: u32 = 0x00281351;

/// (0028,1352) VR=SQ, VM=1 Partial View Code Sequence
pub const PartialViewCodeSequence: u32 = 0x00281352;

/// (0028,135A) VR=CS, VM=1 Spatial Locations Preserved
pub const SpatialLocationsPreserved: u32 = 0x0028135A;

/// (0028,1401) VR=SQ, VM=1 Data Frame Assignment Sequence
pub const DataFrameAssignmentSequence: u32 = 0x00281401;

/// (0028,1402) VR=CS, VM=1 Data Path Assignment
pub const DataPathAssignment: u32 = 0x00281402;

/// (0028,1403) VR=US, VM=1 Bits Mapped to Color Lookup Table
pub const BitsMappedToColorLookupTable: u32 = 0x00281403;

/// (0028,1404) VR=SQ, VM=1 Blending LUT 1 Sequence
pub const BlendingLUT1Sequence: u32 = 0x00281404;

/// (0028,1405) VR=CS, VM=1 Blending LUT 1 Transfer Function
pub const BlendingLUT1TransferFunction: u32 = 0x00281405;

/// (0028,1406) VR=FD, VM=1 Blending Weight Constant
pub const BlendingWeightConstant: u32 = 0x00281406;

/// (0028,1407) VR=US, VM=3 Blending Lookup Table Descriptor
pub const BlendingLookupTableDescriptor: u32 = 0x00281407;

/// (0028,1408) VR=OW, VM=1 Blending Lookup Table Data
pub const BlendingLookupTableData: u32 = 0x00281408;

/// (0028,140B) VR=SQ, VM=1 Enhanced Palette Color Lookup Table Sequence
pub const EnhancedPaletteColorLookupTableSequence: u32 = 0x0028140B;

/// (0028,140C) VR=SQ, VM=1 Blending LUT 2 Sequence
pub const BlendingLUT2Sequence: u32 = 0x0028140C;

/// (0028,140D) VR=CS, VM=1 Blending LUT 2 Transfer Function
pub const BlendingLUT2TransferFunction: u32 = 0x0028140D;

/// (0028,140E) VR=CS, VM=1 Data Path ID
pub const DataPathID: u32 = 0x0028140E;

/// (0028,140F) VR=CS, VM=1 RGB LUT Transfer Function
pub const RGBLUTTransferFunction: u32 = 0x0028140F;

/// (0028,1410) VR=CS, VM=1 Alpha LUT Transfer Function
pub const AlphaLUTTransferFunction: u32 = 0x00281410;

/// (0028,2000) VR=OB, VM=1 ICC Profile
pub const ICCProfile: u32 = 0x00282000;

/// (0028,2110) VR=CS, VM=1 Lossy Image Compression
pub const LossyImageCompression: u32 = 0x00282110;

/// (0028,2112) VR=DS, VM=1-n Lossy Image Compression Ratio
pub const LossyImageCompressionRatio: u32 = 0x00282112;

/// (0028,2114) VR=CS, VM=1-n Lossy Image Compression Method
pub const LossyImageCompressionMethod: u32 = 0x00282114;

/// (0028,3000) VR=SQ, VM=1 Modality LUT Sequence
pub const ModalityLUTSequence: u32 = 0x00283000;

/// (0028,3002) VR=US|SS, VM=3 LUT Descriptor
pub const LUTDescriptor: u32 = 0x00283002;

/// (0028,3003) VR=LO, VM=1 LUT Explanation
pub const LUTExplanation: u32 = 0x00283003;

/// (0028,3004) VR=LO, VM=1 Modality LUT Type
pub const ModalityLUTType: u32 = 0x00283004;

/// (0028,3006) VR=US|OW, VM=1-n1 LUT Data
pub const LUTData: u32 = 0x00283006;

/// (0028,3010) VR=SQ, VM=1 VOI LUT Sequence
pub const VOILUTSequence: u32 = 0x00283010;

/// (0028,3110) VR=SQ, VM=1 Softcopy VOI LUT Sequence
pub const SoftcopyVOILUTSequence: u32 = 0x00283110;

/// (0028,4000) VR=LT, VM=1 Image Presentation Comments (retired)
pub const ImagePresentationComments: u32 = 0x00284000;

/// (0028,5000) VR=SQ, VM=1 Bi-Plane Acquisition Sequence (retired)
pub const BiPlaneAcquisitionSequence: u32 = 0x00285000;

/// (0028,6010) VR=US, VM=1 Representative Frame Number
pub const RepresentativeFrameNumber: u32 = 0x00286010;

/// (0028,6020) VR=US, VM=1-n Frame Numbers of Interest (FOI)
pub const FrameNumbersOfInterest: u32 = 0x00286020;

/// (0028,6022) VR=LO, VM=1-n Frame of Interest Description
pub const FrameOfInterestDescription: u32 = 0x00286022;

/// (0028,6023) VR=CS, VM=1-n Frame of Interest Type
pub const FrameOfInterestType: u32 = 0x00286023;

/// (0028,6030) VR=US, VM=1-n Mask Pointer(s) (retired)
pub const MaskPointers: u32 = 0x00286030;

/// (0028,6040) VR=US, VM=1-n R Wave Pointer
pub const RWavePointer: u32 = 0x00286040;

/// (0028,6100) VR=SQ, VM=1 Mask Subtraction Sequence
pub const MaskSubtractionSequence: u32 = 0x00286100;

/// (0028,6101) VR=CS, VM=1 Mask Operation
pub const MaskOperation: u32 = 0x00286101;

/// (0028,6102) VR=US, VM=2-2n Applicable Frame Range
pub const ApplicableFrameRange: u32 = 0x00286102;

/// (0028,6110) VR=US, VM=1-n Mask Frame Numbers
pub const MaskFrameNumbers: u32 = 0x00286110;

/// (0028,6112) VR=US, VM=1 Contrast Frame Averaging
pub const ContrastFrameAveraging: u32 = 0x00286112;

/// (0028,6114) VR=FL, VM=2 Mask Sub-pixel Shift
pub const MaskSubPixelShift: u32 = 0x00286114;

/// (0028,6120) VR=SS, VM=1 TID Offset
pub const TIDOffset: u32 = 0x00286120;

/// (0028,6190) VR=ST, VM=1 Mask Operation Explanation
pub const MaskOperationExplanation: u32 = 0x00286190;

/// (0028,7FE0) VR=UT, VM=1 Pixel Data Provider URL
pub const PixelDataProviderURL: u32 = 0x00287FE0;

/// (0028,9001) VR=UL, VM=1 Data Point Rows
pub const DataPointRows: u32 = 0x00289001;

/// (0028,9002) VR=UL, VM=1 Data Point Columns
pub const DataPointColumns: u32 = 0x00289002;

/// (0028,9003) VR=CS, VM=1 Signal Domain Columns
pub const SignalDomainColumns: u32 = 0x00289003;

/// (0028,9099) VR=US, VM=1 Largest Monochrome Pixel Value (retired)
pub const LargestMonochromePixelValue: u32 = 0x00289099;

/// (0028,9108) VR=CS, VM=1 Data Representation
pub const DataRepresentation: u32 = 0x00289108;

/// (0028,9110) VR=SQ, VM=1 Pixel Measures Sequence
pub const PixelMeasuresSequence: u32 = 0x00289110;

/// (0028,9132) VR=SQ, VM=1 Frame VOI LUT Sequence
pub const FrameVOILUTSequence: u32 = 0x00289132;

/// (0028,9145) VR=SQ, VM=1 Pixel Value Transformation Sequence
pub const PixelValueTransformationSequence: u32 = 0x00289145;

/// (0028,9235) VR=CS, VM=1 Signal Domain Rows
pub const SignalDomainRows: u32 = 0x00289235;

/// (0028,9411) VR=FL, VM=1 Display Filter Percentage
pub const DisplayFilterPercentage: u32 = 0x00289411;

/// (0028,9415) VR=SQ, VM=1 Frame Pixel Shift Sequence
pub const FramePixelShiftSequence: u32 = 0x00289415;

/// (0028,9416) VR=US, VM=1 Subtraction Item ID
pub const SubtractionItemID: u32 = 0x00289416;

/// (0028,9422) VR=SQ, VM=1 Pixel Intensity Relationship LUT Sequence
pub const PixelIntensityRelationshipLUTSequence: u32 = 0x00289422;

/// (0028,9443) VR=SQ, VM=1 Frame Pixel Data Properties Sequence
pub const FramePixelDataPropertiesSequence: u32 = 0x00289443;

/// (0028,9444) VR=CS, VM=1 Geometrical Properties
pub const GeometricalProperties: u32 = 0x00289444;

/// (0028,9445) VR=FL, VM=1 Geometric Maximum Distortion
pub const GeometricMaximumDistortion: u32 = 0x00289445;

/// (0028,9446) VR=CS, VM=1-n Image Processing Applied
pub const ImageProcessingApplied: u32 = 0x00289446;

/// (0028,9454) VR=CS, VM=1 Mask Selection Mode
pub const MaskSelectionMode: u32 = 0x00289454;

/// (0028,9474) VR=CS, VM=1 LUT Function
pub const LUTFunction: u32 = 0x00289474;

/// (0028,9478) VR=FL, VM=1 Mask Visibility Percentage
pub const MaskVisibilityPercentage: u32 = 0x00289478;

/// (0028,9501) VR=SQ, VM=1 Pixel Shift Sequence
pub const PixelShiftSequence: u32 = 0x00289501;

/// (0028,9502) VR=SQ, VM=1 Region Pixel Shift Sequence
pub const RegionPixelShiftSequence: u32 = 0x00289502;

/// (0028,9503) VR=SS, VM=2-2n Vertices of the Region
pub const VerticesOfTheRegion: u32 = 0x00289503;

/// (0028,9505) VR=SQ, VM=1 Multi-frame Presentation Sequence
pub const MultiFramePresentationSequence: u32 = 0x00289505;

/// (0028,9506) VR=US, VM=2-2n Pixel Shift Frame Range
pub const PixelShiftFrameRange: u32 = 0x00289506;

/// (0028,9507) VR=US, VM=2-2n LUT Frame Range
pub const LUTFrameRange: u32 = 0x00289507;

/// (0028,9520) VR=DS, VM=16 Image to Equipment Mapping Matrix
pub const ImageToEquipmentMappingMatrix: u32 = 0x00289520;

/// (0028,9537) VR=CS, VM=1 Equipment Coordinate System Identification
pub const EquipmentCoordinateSystemIdentification: u32 = 0x00289537;

/// (0032,000A) VR=CS, VM=1 Study Status ID (retired)
pub const StudyStatusID: u32 = 0x0032000A;

/// (0032,000C) VR=CS, VM=1 Study Priority ID (retired)
pub const StudyPriorityID: u32 = 0x0032000C;

/// (0032,0012) VR=LO, VM=1 Study ID Issuer (retired)
pub const StudyIDIssuer: u32 = 0x00320012;

/// (0032,0032) VR=DA, VM=1 Study Verified Date (retired)
pub const StudyVerifiedDate: u32 = 0x00320032;

/// (0032,0033) VR=TM, VM=1 Study Verified Time (retired)
pub const StudyVerifiedTime: u32 = 0x00320033;

/// (0032,0034) VR=DA, VM=1 Study Read Date (retired)
pub const StudyReadDate: u32 = 0x00320034;

/// (0032,0035) VR=TM, VM=1 Study Read Time (retired)
pub const StudyReadTime: u32 = 0x00320035;

/// (0032,1000) VR=DA, VM=1 Scheduled Study Start Date (retired)
pub const ScheduledStudyStartDate: u32 = 0x00321000;

/// (0032,1001) VR=TM, VM=1 Scheduled Study Start Time (retired)
pub const ScheduledStudyStartTime: u32 = 0x00321001;

/// (0032,1010) VR=DA, VM=1 Scheduled Study Stop Date (retired)
pub const ScheduledStudyStopDate: u32 = 0x00321010;

/// (0032,1011) VR=TM, VM=1 Scheduled Study Stop Time (retired)
pub const ScheduledStudyStopTime: u32 = 0x00321011;

/// (0032,1020) VR=LO, VM=1 Scheduled Study Location (retired)
pub const ScheduledStudyLocation: u32 = 0x00321020;

/// (0032,1021) VR=AE, VM=1-n Scheduled Study Location AE Title (retired)
pub const ScheduledStudyLocationAETitle: u32 = 0x00321021;

/// (0032,1030) VR=LO, VM=1 Reason for Study (retired)
pub const ReasonForStudy: u32 = 0x00321030;

/// (0032,1031) VR=SQ, VM=1 Requesting Physician Identification Sequence
pub const RequestingPhysicianIdentificationSequence: u32 = 0x00321031;

/// (0032,1032) VR=PN, VM=1 Requesting Physician
pub const RequestingPhysician: u32 = 0x00321032;

/// (0032,1033) VR=LO, VM=1 Requesting Service
pub const RequestingService: u32 = 0x00321033;

/// (0032,1034) VR=SQ, VM=1 Requesting Service Code Sequence
pub const RequestingServiceCodeSequence: u32 = 0x00321034;

/// (0032,1040) VR=DA, VM=1 Study Arrival Date (retired)
pub const StudyArrivalDate: u32 = 0x00321040;

/// (0032,1041) VR=TM, VM=1 Study Arrival Time (retired)
pub const StudyArrivalTime: u32 = 0x00321041;

/// (0032,1050) VR=DA, VM=1 Study Completion Date (retired)
pub const StudyCompletionDate: u32 = 0x00321050;

/// (0032,1051) VR=TM, VM=1 Study Completion Time (retired)
pub const StudyCompletionTime: u32 = 0x00321051;

/// (0032,1055) VR=CS, VM=1 Study Component Status ID (retired)
pub const StudyComponentStatusID: u32 = 0x00321055;

/// (0032,1060) VR=LO, VM=1 Requested Procedure Description
pub const RequestedProcedureDescription: u32 = 0x00321060;

/// (0032,1064) VR=SQ, VM=1 Requested Procedure Code Sequence
pub const RequestedProcedureCodeSequence: u32 = 0x00321064;

/// (0032,1070) VR=LO, VM=1 Requested Contrast Agent
pub const RequestedContrastAgent: u32 = 0x00321070;

/// (0032,4000) VR=LT, VM=1 Study Comments (retired)
pub const StudyComments: u32 = 0x00324000;

/// (0038,0004) VR=SQ, VM=1 Referenced Patient Alias Sequence
pub const ReferencedPatientAliasSequence: u32 = 0x00380004;

/// (0038,0008) VR=CS, VM=1 Visit Status ID
pub const VisitStatusID: u32 = 0x00380008;

/// (0038,0010) VR=LO, VM=1 Admission ID
pub const AdmissionID: u32 = 0x00380010;

/// (0038,0011) VR=LO, VM=1 Issuer of Admission ID (retired)
pub const IssuerOfAdmissionID: u32 = 0x00380011;

/// (0038,0014) VR=SQ, VM=1 Issuer of Admission ID Sequence
pub const IssuerOfAdmissionIDSequence: u32 = 0x00380014;

/// (0038,0016) VR=LO, VM=1 Route of Admissions
pub const RouteOfAdmissions: u32 = 0x00380016;

/// (0038,001A) VR=DA, VM=1 Scheduled Admission Date (retired)
pub const ScheduledAdmissionDate: u32 = 0x0038001A;

/// (0038,001B) VR=TM, VM=1 Scheduled Admission Time (retired)
pub const ScheduledAdmissionTime: u32 = 0x0038001B;

/// (0038,001C) VR=DA, VM=1 Scheduled Discharge Date (retired)
pub const ScheduledDischargeDate: u32 = 0x0038001C;

/// (0038,001D) VR=TM, VM=1 Scheduled Discharge Time (retired)
pub const ScheduledDischargeTime: u32 = 0x0038001D;

/// (0038,001E) VR=LO, VM=1 Scheduled Patient Institution Residence (retired)
pub const ScheduledPatientInstitutionResidence: u32 = 0x0038001E;

/// (0038,0020) VR=DA, VM=1 Admitting Date
pub const AdmittingDate: u32 = 0x00380020;

/// (0038,0021) VR=TM, VM=1 Admitting Time
pub const AdmittingTime: u32 = 0x00380021;

/// (0038,0030) VR=DA, VM=1 Discharge Date (retired)
pub const DischargeDate: u32 = 0x00380030;

/// (0038,0032) VR=TM, VM=1 Discharge Time (retired)
pub const DischargeTime: u32 = 0x00380032;

/// (0038,0040) VR=LO, VM=1 Discharge Diagnosis Description (retired)
pub const DischargeDiagnosisDescription: u32 = 0x00380040;

/// (0038,0044) VR=SQ, VM=1 Discharge Diagnosis Code Sequence (retired)
pub const DischargeDiagnosisCodeSequence: u32 = 0x00380044;

/// (0038,0050) VR=LO, VM=1 Special Needs
pub const SpecialNeeds: u32 = 0x00380050;

/// (0038,0060) VR=LO, VM=1 Service Episode ID
pub const ServiceEpisodeID: u32 = 0x00380060;

/// (0038,0061) VR=LO, VM=1 Issuer of Service Episode ID (retired)
pub const IssuerOfServiceEpisodeID: u32 = 0x00380061;

/// (0038,0062) VR=LO, VM=1 Service Episode Description
pub const ServiceEpisodeDescription: u32 = 0x00380062;

/// (0038,0064) VR=SQ, VM=1 Issuer of Service Episode ID Sequence
pub const IssuerOfServiceEpisodeIDSequence: u32 = 0x00380064;

/// (0038,0100) VR=SQ, VM=1 Pertinent Documents Sequence
pub const PertinentDocumentsSequence: u32 = 0x00380100;

/// (0038,0300) VR=LO, VM=1 Current Patient Location
pub const CurrentPatientLocation: u32 = 0x00380300;

/// (0038,0400) VR=LO, VM=1 Patient’s Institution Residence
pub const PatientInstitutionResidence: u32 = 0x00380400;

/// (0038,0500) VR=LO, VM=1 Patient State
pub const PatientState: u32 = 0x00380500;

/// (0038,0502) VR=SQ, VM=1 Patient Clinical Trial Participation Sequence
pub const PatientClinicalTrialParticipationSequence: u32 = 0x00380502;

/// (0038,4000) VR=LT, VM=1 Visit Comments
pub const VisitComments: u32 = 0x00384000;

/// (003A,0004) VR=CS, VM=1 Waveform Originality
pub const WaveformOriginality: u32 = 0x003A0004;

/// (003A,0005) VR=US, VM=1 Number of Waveform Channels
pub const NumberOfWaveformChannels: u32 = 0x003A0005;

/// (003A,0010) VR=UL, VM=1 Number of Waveform Samples
pub const NumberOfWaveformSamples: u32 = 0x003A0010;

/// (003A,001A) VR=DS, VM=1 Sampling Frequency
pub const SamplingFrequency: u32 = 0x003A001A;

/// (003A,0020) VR=SH, VM=1 Multiplex Group Label
pub const MultiplexGroupLabel: u32 = 0x003A0020;

/// (003A,0200) VR=SQ, VM=1 Channel Definition Sequence
pub const ChannelDefinitionSequence: u32 = 0x003A0200;

/// (003A,0202) VR=IS, VM=1 Waveform Channel Number
pub const WaveformChannelNumber: u32 = 0x003A0202;

/// (003A,0203) VR=SH, VM=1 Channel Label
pub const ChannelLabel: u32 = 0x003A0203;

/// (003A,0205) VR=CS, VM=1-n Channel Status
pub const ChannelStatus: u32 = 0x003A0205;

/// (003A,0208) VR=SQ, VM=1 Channel Source Sequence
pub const ChannelSourceSequence: u32 = 0x003A0208;

/// (003A,0209) VR=SQ, VM=1 Channel Source Modifiers Sequence
pub const ChannelSourceModifiersSequence: u32 = 0x003A0209;

/// (003A,020A) VR=SQ, VM=1 Source Waveform Sequence
pub const SourceWaveformSequence: u32 = 0x003A020A;

/// (003A,020C) VR=LO, VM=1 Channel Derivation Description
pub const ChannelDerivationDescription: u32 = 0x003A020C;

/// (003A,0210) VR=DS, VM=1 Channel Sensitivity
pub const ChannelSensitivity: u32 = 0x003A0210;

/// (003A,0211) VR=SQ, VM=1 Channel Sensitivity Units Sequence
pub const ChannelSensitivityUnitsSequence: u32 = 0x003A0211;

/// (003A,0212) VR=DS, VM=1 Channel Sensitivity Correction Factor
pub const ChannelSensitivityCorrectionFactor: u32 = 0x003A0212;

/// (003A,0213) VR=DS, VM=1 Channel Baseline
pub const ChannelBaseline: u32 = 0x003A0213;

/// (003A,0214) VR=DS, VM=1 Channel Time Skew
pub const ChannelTimeSkew: u32 = 0x003A0214;

/// (003A,0215) VR=DS, VM=1 Channel Sample Skew
pub const ChannelSampleSkew: u32 = 0x003A0215;

/// (003A,0218) VR=DS, VM=1 Channel Offset
pub const ChannelOffset: u32 = 0x003A0218;

/// (003A,021A) VR=US, VM=1 Waveform Bits Stored
pub const WaveformBitsStored: u32 = 0x003A021A;

/// (003A,0220) VR=DS, VM=1 Filter Low Frequency
pub const FilterLowFrequency: u32 = 0x003A0220;

/// (003A,0221) VR=DS, VM=1 Filter High Frequency
pub const FilterHighFrequency: u32 = 0x003A0221;

/// (003A,0222) VR=DS, VM=1 Notch Filter Frequency
pub const NotchFilterFrequency: u32 = 0x003A0222;

/// (003A,0223) VR=DS, VM=1 Notch Filter Bandwidth
pub const NotchFilterBandwidth: u32 = 0x003A0223;

/// (003A,0230) VR=FL, VM=1 Waveform Data Display Scale
pub const WaveformDataDisplayScale: u32 = 0x003A0230;

/// (003A,0231) VR=US, VM=3 Waveform Display Background CIELab Value
pub const WaveformDisplayBackgroundCIELabValue: u32 = 0x003A0231;

/// (003A,0240) VR=SQ, VM=1 Waveform Presentation Group Sequence
pub const WaveformPresentationGroupSequence: u32 = 0x003A0240;

/// (003A,0241) VR=US, VM=1 Presentation Group Number
pub const PresentationGroupNumber: u32 = 0x003A0241;

/// (003A,0242) VR=SQ, VM=1 Channel Display Sequence
pub const ChannelDisplaySequence: u32 = 0x003A0242;

/// (003A,0244) VR=US, VM=3 Channel Recommended Display CIELab Value
pub const ChannelRecommendedDisplayCIELabValue: u32 = 0x003A0244;

/// (003A,0245) VR=FL, VM=1 Channel Position
pub const ChannelPosition: u32 = 0x003A0245;

/// (003A,0246) VR=CS, VM=1 Display Shading Flag
pub const DisplayShadingFlag: u32 = 0x003A0246;

/// (003A,0247) VR=FL, VM=1 Fractional Channel Display Scale
pub const FractionalChannelDisplayScale: u32 = 0x003A0247;

/// (003A,0248) VR=FL, VM=1 Absolute Channel Display Scale
pub const AbsoluteChannelDisplayScale: u32 = 0x003A0248;

/// (003A,0300) VR=SQ, VM=1 Multiplexed Audio Channels Description Code Sequence
pub const MultiplexedAudioChannelsDescriptionCodeSequence: u32 = 0x003A0300;

/// (003A,0301) VR=IS, VM=1 Channel Identification Code
pub const ChannelIdentificationCode: u32 = 0x003A0301;

/// (003A,0302) VR=CS, VM=1 Channel Mode
pub const ChannelMode: u32 = 0x003A0302;

/// (0040,0001) VR=AE, VM=1-n Scheduled Station AE Title
pub const ScheduledStationAETitle: u32 = 0x00400001;

/// (0040,0002) VR=DA, VM=1 Scheduled Procedure Step Start Date
pub const ScheduledProcedureStepStartDate: u32 = 0x00400002;

/// (0040,0003) VR=TM, VM=1 Scheduled Procedure Step Start Time
pub const ScheduledProcedureStepStartTime: u32 = 0x00400003;

/// (0040,0004) VR=DA, VM=1 Scheduled Procedure Step End Date
pub const ScheduledProcedureStepEndDate: u32 = 0x00400004;

/// (0040,0005) VR=TM, VM=1 Scheduled Procedure Step End Time
pub const ScheduledProcedureStepEndTime: u32 = 0x00400005;

/// (0040,0006) VR=PN, VM=1 Scheduled Performing Physician’s Name
pub const ScheduledPerformingPhysicianName: u32 = 0x00400006;

/// (0040,0007) VR=LO, VM=1 Scheduled Procedure Step Description
pub const ScheduledProcedureStepDescription: u32 = 0x00400007;

/// (0040,0008) VR=SQ, VM=1 Scheduled Protocol Code Sequence
pub const ScheduledProtocolCodeSequence: u32 = 0x00400008;

/// (0040,0009) VR=SH, VM=1 Scheduled Procedure Step ID
pub const ScheduledProcedureStepID: u32 = 0x00400009;

/// (0040,000A) VR=SQ, VM=1 Stage Code Sequence
pub const StageCodeSequence: u32 = 0x0040000A;

/// (0040,000B) VR=SQ, VM=1 Scheduled Performing Physician Identification Sequence
pub const ScheduledPerformingPhysicianIdentificationSequence: u32 = 0x0040000B;

/// (0040,0010) VR=SH, VM=1-n Scheduled Station Name
pub const ScheduledStationName: u32 = 0x00400010;

/// (0040,0011) VR=SH, VM=1 Scheduled Procedure Step Location
pub const ScheduledProcedureStepLocation: u32 = 0x00400011;

/// (0040,0012) VR=LO, VM=1 Pre-Medication
pub const PreMedication: u32 = 0x00400012;

/// (0040,0020) VR=CS, VM=1 Scheduled Procedure Step Status
pub const ScheduledProcedureStepStatus: u32 = 0x00400020;

/// (0040,0026) VR=SQ, VM=1 Order Placer Identifier Sequence
pub const OrderPlacerIdentifierSequence: u32 = 0x00400026;

/// (0040,0027) VR=SQ, VM=1 Order Filler Identifier Sequence
pub const OrderFillerIdentifierSequence: u32 = 0x00400027;

/// (0040,0031) VR=UT, VM=1 Local Namespace Entity ID
pub const LocalNamespaceEntityID: u32 = 0x00400031;

/// (0040,0032) VR=UT, VM=1 Universal Entity ID
pub const UniversalEntityID: u32 = 0x00400032;

/// (0040,0033) VR=CS, VM=1 Universal Entity ID Type
pub const UniversalEntityIDType: u32 = 0x00400033;

/// (0040,0035) VR=CS, VM=1 Identifier Type Code
pub const IdentifierTypeCode: u32 = 0x00400035;

/// (0040,0036) VR=SQ, VM=1 Assigning Facility Sequence
pub const AssigningFacilitySequence: u32 = 0x00400036;

/// (0040,0039) VR=SQ, VM=1 Assigning Jurisdiction Code Sequence
pub const AssigningJurisdictionCodeSequence: u32 = 0x00400039;

/// (0040,003A) VR=SQ, VM=1 Assigning Agency or Department Code Sequence
pub const AssigningAgencyOrDepartmentCodeSequence: u32 = 0x0040003A;

/// (0040,0100) VR=SQ, VM=1 Scheduled Procedure Step Sequence
pub const ScheduledProcedureStepSequence: u32 = 0x00400100;

/// (0040,0220) VR=SQ, VM=1 Referenced Non-Image Composite SOP Instance Sequence
pub const ReferencedNonImageCompositeSOPInstanceSequence: u32 = 0x00400220;

/// (0040,0241) VR=AE, VM=1 Performed Station AE Title
pub const PerformedStationAETitle: u32 = 0x00400241;

/// (0040,0242) VR=SH, VM=1 Performed Station Name
pub const PerformedStationName: u32 = 0x00400242;

/// (0040,0243) VR=SH, VM=1 Performed Location
pub const PerformedLocation: u32 = 0x00400243;

/// (0040,0244) VR=DA, VM=1 Performed Procedure Step Start Date
pub const PerformedProcedureStepStartDate: u32 = 0x00400244;

/// (0040,0245) VR=TM, VM=1 Performed Procedure Step Start Time
pub const PerformedProcedureStepStartTime: u32 = 0x00400245;

/// (0040,0250) VR=DA, VM=1 Performed Procedure Step End Date
pub const PerformedProcedureStepEndDate: u32 = 0x00400250;

/// (0040,0251) VR=TM, VM=1 Performed Procedure Step End Time
pub const PerformedProcedureStepEndTime: u32 = 0x00400251;

/// (0040,0252) VR=CS, VM=1 Performed Procedure Step Status
pub const PerformedProcedureStepStatus: u32 = 0x00400252;

/// (0040,0253) VR=SH, VM=1 Performed Procedure Step ID
pub const PerformedProcedureStepID: u32 = 0x00400253;

/// (0040,0254) VR=LO, VM=1 Performed Procedure Step Description
pub const PerformedProcedureStepDescription: u32 = 0x00400254;

/// (0040,0255) VR=LO, VM=1 Performed Procedure Type Description
pub const PerformedProcedureTypeDescription: u32 = 0x00400255;

/// (0040,0260) VR=SQ, VM=1 Performed Protocol Code Sequence
pub const PerformedProtocolCodeSequence: u32 = 0x00400260;

/// (0040,0261) VR=CS, VM=1 Performed Protocol Type
pub const PerformedProtocolType: u32 = 0x00400261;

/// (0040,0270) VR=SQ, VM=1 Scheduled Step Attributes Sequence
pub const ScheduledStepAttributesSequence: u32 = 0x00400270;

/// (0040,0275) VR=SQ, VM=1 Request Attributes Sequence
pub const RequestAttributesSequence: u32 = 0x00400275;

/// (0040,0280) VR=ST, VM=1 Comments on the Performed Procedure Step
pub const CommentsOnThePerformedProcedureStep: u32 = 0x00400280;

/// (0040,0281) VR=SQ, VM=1 Performed Procedure Step Discontinuation Reason Code Sequence
pub const PerformedProcedureStepDiscontinuationReasonCodeSequence: u32 = 0x00400281;

/// (0040,0293) VR=SQ, VM=1 Quantity Sequence
pub const QuantitySequence: u32 = 0x00400293;

/// (0040,0294) VR=DS, VM=1 Quantity
pub const Quantity: u32 = 0x00400294;

/// (0040,0295) VR=SQ, VM=1 Measuring Units Sequence
pub const MeasuringUnitsSequence: u32 = 0x00400295;

/// (0040,0296) VR=SQ, VM=1 Billing Item Sequence
pub const BillingItemSequence: u32 = 0x00400296;

/// (0040,0300) VR=US, VM=1 Total Time of Fluoroscopy
pub const TotalTimeOfFluoroscopy: u32 = 0x00400300;

/// (0040,0301) VR=US, VM=1 Total Number of Exposures
pub const TotalNumberOfExposures: u32 = 0x00400301;

/// (0040,0302) VR=US, VM=1 Entrance Dose
pub const EntranceDose: u32 = 0x00400302;

/// (0040,0303) VR=US, VM=1-2 Exposed Area
pub const ExposedArea: u32 = 0x00400303;

/// (0040,0306) VR=DS, VM=1 Distance Source to Entrance
pub const DistanceSourceToEntrance: u32 = 0x00400306;

/// (0040,0307) VR=DS, VM=1 Distance Source to Support (retired)
pub const DistanceSourceToSupport: u32 = 0x00400307;

/// (0040,030E) VR=SQ, VM=1 Exposure Dose Sequence
pub const ExposureDoseSequence: u32 = 0x0040030E;

/// (0040,0310) VR=ST, VM=1 Comments on Radiation Dose
pub const CommentsOnRadiationDose: u32 = 0x00400310;

/// (0040,0312) VR=DS, VM=1 X-Ray Output
pub const XRayOutput: u32 = 0x00400312;

/// (0040,0314) VR=DS, VM=1 Half Value Layer
pub const HalfValueLayer: u32 = 0x00400314;

/// (0040,0316) VR=DS, VM=1 Organ Dose
pub const OrganDose: u32 = 0x00400316;

/// (0040,0318) VR=CS, VM=1 Organ Exposed
pub const OrganExposed: u32 = 0x00400318;

/// (0040,0320) VR=SQ, VM=1 Billing Procedure Step Sequence
pub const BillingProcedureStepSequence: u32 = 0x00400320;

/// (0040,0321) VR=SQ, VM=1 Film Consumption Sequence
pub const FilmConsumptionSequence: u32 = 0x00400321;

/// (0040,0324) VR=SQ, VM=1 Billing Supplies and Devices Sequence
pub const BillingSuppliesAndDevicesSequence: u32 = 0x00400324;

/// (0040,0330) VR=SQ, VM=1 Referenced Procedure Step Sequence (retired)
pub const ReferencedProcedureStepSequence: u32 = 0x00400330;

/// (0040,0340) VR=SQ, VM=1 Performed Series Sequence
pub const PerformedSeriesSequence: u32 = 0x00400340;

/// (0040,0400) VR=LT, VM=1 Comments on the Scheduled Procedure Step
pub const CommentsOnTheScheduledProcedureStep: u32 = 0x00400400;

/// (0040,0440) VR=SQ, VM=1 Protocol Context Sequence
pub const ProtocolContextSequence: u32 = 0x00400440;

/// (0040,0441) VR=SQ, VM=1 Content Item Modifier Sequence
pub const ContentItemModifierSequence: u32 = 0x00400441;

/// (0040,0500) VR=SQ, VM=1 Scheduled Specimen Sequence
pub const ScheduledSpecimenSequence: u32 = 0x00400500;

/// (0040,050A) VR=LO, VM=1 Specimen Accession Number (retired)
pub const SpecimenAccessionNumber: u32 = 0x0040050A;

/// (0040,0512) VR=LO, VM=1 Container Identifier
pub const ContainerIdentifier: u32 = 0x00400512;

/// (0040,0513) VR=SQ, VM=1 Issuer of the Container Identifier Sequence
pub const IssuerOfTheContainerIdentifierSequence: u32 = 0x00400513;

/// (0040,0515) VR=SQ, VM=1 Alternate Container Identifier Sequence
pub const AlternateContainerIdentifierSequence: u32 = 0x00400515;

/// (0040,0518) VR=SQ, VM=1 Container Type Code Sequence
pub const ContainerTypeCodeSequence: u32 = 0x00400518;

/// (0040,051A) VR=LO, VM=1 Container Description
pub const ContainerDescription: u32 = 0x0040051A;

/// (0040,0520) VR=SQ, VM=1 Container Component Sequence
pub const ContainerComponentSequence: u32 = 0x00400520;

/// (0040,0550) VR=SQ, VM=1 Specimen Sequence (retired)
pub const SpecimenSequence: u32 = 0x00400550;

/// (0040,0551) VR=LO, VM=1 Specimen Identifier
pub const SpecimenIdentifier: u32 = 0x00400551;

/// (0040,0552) VR=SQ, VM=1 Specimen Description Sequence (Trial) (retired)
pub const SpecimenDescriptionSequenceTrial: u32 = 0x00400552;

/// (0040,0553) VR=ST, VM=1 Specimen Description (Trial) (retired)
pub const SpecimenDescriptionTrial: u32 = 0x00400553;

/// (0040,0554) VR=UI, VM=1 Specimen UID
pub const SpecimenUID: u32 = 0x00400554;

/// (0040,0555) VR=SQ, VM=1 Acquisition Context Sequence
pub const AcquisitionContextSequence: u32 = 0x00400555;

/// (0040,0556) VR=ST, VM=1 Acquisition Context Description
pub const AcquisitionContextDescription: u32 = 0x00400556;

/// (0040,0560) VR=SQ, VM=1 Specimen Description Sequence
pub const SpecimenDescriptionSequence: u32 = 0x00400560;

/// (0040,0562) VR=SQ, VM=1 Issuer of the Specimen Identifier Sequence
pub const IssuerOfTheSpecimenIdentifierSequence: u32 = 0x00400562;

/// (0040,059A) VR=SQ, VM=1 Specimen Type Code Sequence
pub const SpecimenTypeCodeSequence: u32 = 0x0040059A;

/// (0040,0600) VR=LO, VM=1 Specimen Short Description
pub const SpecimenShortDescription: u32 = 0x00400600;

/// (0040,0602) VR=UT, VM=1 Specimen Detailed Description
pub const SpecimenDetailedDescription: u32 = 0x00400602;

/// (0040,0610) VR=SQ, VM=1 Specimen Preparation Sequence
pub const SpecimenPreparationSequence: u32 = 0x00400610;

/// (0040,0612) VR=SQ, VM=1 Specimen Preparation Step Content Item Sequence
pub const SpecimenPreparationStepContentItemSequence: u32 = 0x00400612;

/// (0040,0620) VR=SQ, VM=1 Specimen Localization Content Item Sequence
pub const SpecimenLocalizationContentItemSequence: u32 = 0x00400620;

/// (0040,06FA) VR=LO, VM=1 Slide Identifier (retired)
pub const SlideIdentifier: u32 = 0x004006FA;

/// (0040,071A) VR=SQ, VM=1 Image Center Point Coordinates Sequence
pub const ImageCenterPointCoordinatesSequence: u32 = 0x0040071A;

/// (0040,072A) VR=DS, VM=1 X Offset in Slide Coordinate System
pub const XOffsetInSlideCoordinateSystem: u32 = 0x0040072A;

/// (0040,073A) VR=DS, VM=1 Y Offset in Slide Coordinate System
pub const YOffsetInSlideCoordinateSystem: u32 = 0x0040073A;

/// (0040,074A) VR=DS, VM=1 Z Offset in Slide Coordinate System
pub const ZOffsetInSlideCoordinateSystem: u32 = 0x0040074A;

/// (0040,08D8) VR=SQ, VM=1 Pixel Spacing Sequence (retired)
pub const PixelSpacingSequence: u32 = 0x004008D8;

/// (0040,08DA) VR=SQ, VM=1 Coordinate System Axis Code Sequence (retired)
pub const CoordinateSystemAxisCodeSequence: u32 = 0x004008DA;

/// (0040,08EA) VR=SQ, VM=1 Measurement Units Code Sequence
pub const MeasurementUnitsCodeSequence: u32 = 0x004008EA;

/// (0040,09F8) VR=SQ, VM=1 Vital Stain Code Sequence (Trial) (retired)
pub const VitalStainCodeSequenceTrial: u32 = 0x004009F8;

/// (0040,1001) VR=SH, VM=1 Requested Procedure ID
pub const RequestedProcedureID: u32 = 0x00401001;

/// (0040,1002) VR=LO, VM=1 Reason for the Requested Procedure
pub const ReasonForTheRequestedProcedure: u32 = 0x00401002;

/// (0040,1003) VR=SH, VM=1 Requested Procedure Priority
pub const RequestedProcedurePriority: u32 = 0x00401003;

/// (0040,1004) VR=LO, VM=1 Patient Transport Arrangements
pub const PatientTransportArrangements: u32 = 0x00401004;

/// (0040,1005) VR=LO, VM=1 Requested Procedure Location
pub const RequestedProcedureLocation: u32 = 0x00401005;

/// (0040,1006) VR=SH, VM=1 Placer Order Number / Procedure (retired)
pub const PlacerOrderNumberProcedure: u32 = 0x00401006;

/// (0040,1007) VR=SH, VM=1 Filler Order Number / Procedure (retired)
pub const FillerOrderNumberProcedure: u32 = 0x00401007;

/// (0040,1008) VR=LO, VM=1 Confidentiality Code
pub const ConfidentialityCode: u32 = 0x00401008;

/// (0040,1009) VR=SH, VM=1 Reporting Priority
pub const ReportingPriority: u32 = 0x00401009;

/// (0040,100A) VR=SQ, VM=1 Reason for Requested Procedure Code Sequence
pub const ReasonForRequestedProcedureCodeSequence: u32 = 0x0040100A;

/// (0040,1010) VR=PN, VM=1-n Names of Intended Recipients of Results
pub const NamesOfIntendedRecipientsOfResults: u32 = 0x00401010;

/// (0040,1011) VR=SQ, VM=1 Intended Recipients of Results Identification Sequence
pub const IntendedRecipientsOfResultsIdentificationSequence: u32 = 0x00401011;

/// (0040,1012) VR=SQ, VM=1 Reason For Performed Procedure Code Sequence
pub const ReasonForPerformedProcedureCodeSequence: u32 = 0x00401012;

/// (0040,1060) VR=LO, VM=1 Requested Procedure Description (Trial) (retired)
pub const RequestedProcedureDescriptionTrial: u32 = 0x00401060;

/// (0040,1101) VR=SQ, VM=1 Person Identification Code Sequence
pub const PersonIdentificationCodeSequence: u32 = 0x00401101;

/// (0040,1102) VR=ST, VM=1 Person’s Address
pub const PersonAddress: u32 = 0x00401102;

/// (0040,1103) VR=LO, VM=1-n Person’s Telephone Numbers
pub const PersonTelephoneNumbers: u32 = 0x00401103;

/// (0040,1400) VR=LT, VM=1 Requested Procedure Comments
pub const RequestedProcedureComments: u32 = 0x00401400;

/// (0040,2001) VR=LO, VM=1 Reason for the Imaging Service Request (retired)
pub const ReasonForTheImagingServiceRequest: u32 = 0x00402001;

/// (0040,2004) VR=DA, VM=1 Issue Date of Imaging Service Request
pub const IssueDateOfImagingServiceRequest: u32 = 0x00402004;

/// (0040,2005) VR=TM, VM=1 Issue Time of Imaging Service Request
pub const IssueTimeOfImagingServiceRequest: u32 = 0x00402005;

/// (0040,2006) VR=SH, VM=1 Placer Order Number / Imaging Service Request (Retired) (retired)
pub const PlacerOrderNumberImagingServiceRequestRetired: u32 = 0x00402006;

/// (0040,2007) VR=SH, VM=1 Filler Order Number / Imaging Service Request (Retired) (retired)
pub const FillerOrderNumberImagingServiceRequestRetired: u32 = 0x00402007;

/// (0040,2008) VR=PN, VM=1 Order Entered By
pub const OrderEnteredBy: u32 = 0x00402008;

/// (0040,2009) VR=SH, VM=1 Order Enterer’s Location
pub const OrderEntererLocation: u32 = 0x00402009;

/// (0040,2010) VR=SH, VM=1 Order Callback Phone Number
pub const OrderCallbackPhoneNumber: u32 = 0x00402010;

/// (0040,2016) VR=LO, VM=1 Placer Order Number / Imaging Service Request
pub const PlacerOrderNumberImagingServiceRequest: u32 = 0x00402016;

/// (0040,2017) VR=LO, VM=1 Filler Order Number / Imaging Service Request
pub const FillerOrderNumberImagingServiceRequest: u32 = 0x00402017;

/// (0040,2400) VR=LT, VM=1 Imaging Service Request Comments
pub const ImagingServiceRequestComments: u32 = 0x00402400;

/// (0040,3001) VR=LO, VM=1 Confidentiality Constraint on Patient Data Description
pub const ConfidentialityConstraintOnPatientDataDescription: u32 = 0x00403001;

/// (0040,4001) VR=CS, VM=1 General Purpose Scheduled Procedure Step Status
pub const GeneralPurposeScheduledProcedureStepStatus: u32 = 0x00404001;

/// (0040,4002) VR=CS, VM=1 General Purpose Performed Procedure Step Status
pub const GeneralPurposePerformedProcedureStepStatus: u32 = 0x00404002;

/// (0040,4003) VR=CS, VM=1 General Purpose Scheduled Procedure Step Priority
pub const GeneralPurposeScheduledProcedureStepPriority: u32 = 0x00404003;

/// (0040,4004) VR=SQ, VM=1 Scheduled Processing Applications Code Sequence
pub const ScheduledProcessingApplicationsCodeSequence: u32 = 0x00404004;

/// (0040,4005) VR=DT, VM=1 Scheduled Procedure Step Start DateTime
pub const ScheduledProcedureStepStartDateTime: u32 = 0x00404005;

/// (0040,4006) VR=CS, VM=1 Multiple Copies Flag
pub const MultipleCopiesFlag: u32 = 0x00404006;

/// (0040,4007) VR=SQ, VM=1 Performed Processing Applications Code Sequence
pub const PerformedProcessingApplicationsCodeSequence: u32 = 0x00404007;

/// (0040,4009) VR=SQ, VM=1 Human Performer Code Sequence
pub const HumanPerformerCodeSequence: u32 = 0x00404009;

/// (0040,4010) VR=DT, VM=1 Scheduled Procedure Step Modification Date Time
pub const ScheduledProcedureStepModificationDateTime: u32 = 0x00404010;

/// (0040,4011) VR=DT, VM=1 Expected Completion Date Time
pub const ExpectedCompletionDateTime: u32 = 0x00404011;

/// (0040,4015) VR=SQ, VM=1 Resulting General Purpose Performed Procedure Steps Sequence
pub const ResultingGeneralPurposePerformedProcedureStepsSequence: u32 = 0x00404015;

/// (0040,4016) VR=SQ, VM=1 Referenced General Purpose Scheduled Procedure Step Sequence
pub const ReferencedGeneralPurposeScheduledProcedureStepSequence: u32 = 0x00404016;

/// (0040,4018) VR=SQ, VM=1 Scheduled Workitem Code Sequence
pub const ScheduledWorkitemCodeSequence: u32 = 0x00404018;

/// (0040,4019) VR=SQ, VM=1 Performed Workitem Code Sequence
pub const PerformedWorkitemCodeSequence: u32 = 0x00404019;

/// (0040,4020) VR=CS, VM=1 Input Availability Flag
pub const InputAvailabilityFlag: u32 = 0x00404020;

/// (0040,4021) VR=SQ, VM=1 Input Information Sequence
pub const InputInformationSequence: u32 = 0x00404021;

/// (0040,4022) VR=SQ, VM=1 Relevant Information Sequence
pub const RelevantInformationSequence: u32 = 0x00404022;

/// (0040,4023) VR=UI, VM=1 Referenced General Purpose Scheduled Procedure Step Transaction UID
pub const ReferencedGeneralPurposeScheduledProcedureStepTransactionUID: u32 = 0x00404023;

/// (0040,4025) VR=SQ, VM=1 Scheduled Station Name Code Sequence
pub const ScheduledStationNameCodeSequence: u32 = 0x00404025;

/// (0040,4026) VR=SQ, VM=1 Scheduled Station Class Code Sequence
pub const ScheduledStationClassCodeSequence: u32 = 0x00404026;

/// (0040,4027) VR=SQ, VM=1 Scheduled Station Geographic Location Code Sequence
pub const ScheduledStationGeographicLocationCodeSequence: u32 = 0x00404027;

/// (0040,4028) VR=SQ, VM=1 Performed Station Name Code Sequence
pub const PerformedStationNameCodeSequence: u32 = 0x00404028;

/// (0040,4029) VR=SQ, VM=1 Performed Station Class Code Sequence
pub const PerformedStationClassCodeSequence: u32 = 0x00404029;

/// (0040,4030) VR=SQ, VM=1 Performed Station Geographic Location Code Sequence
pub const PerformedStationGeographicLocationCodeSequence: u32 = 0x00404030;

/// (0040,4031) VR=SQ, VM=1 Requested Subsequent Workitem Code Sequence
pub const RequestedSubsequentWorkitemCodeSequence: u32 = 0x00404031;

/// (0040,4032) VR=SQ, VM=1 Non-DICOM Output Code Sequence
pub const NonDICOMOutputCodeSequence: u32 = 0x00404032;

/// (0040,4033) VR=SQ, VM=1 Output Information Sequence
pub const OutputInformationSequence: u32 = 0x00404033;

/// (0040,4034) VR=SQ, VM=1 Scheduled Human Performers Sequence
pub const ScheduledHumanPerformersSequence: u32 = 0x00404034;

/// (0040,4035) VR=SQ, VM=1 Actual Human Performers Sequence
pub const ActualHumanPerformersSequence: u32 = 0x00404035;

/// (0040,4036) VR=LO, VM=1 Human Performer’s Organization
pub const HumanPerformerOrganization: u32 = 0x00404036;

/// (0040,4037) VR=PN, VM=1 Human Performer’s Name
pub const HumanPerformerName: u32 = 0x00404037;

/// (0040,4040) VR=CS, VM=1 Raw Data Handling
pub const RawDataHandling: u32 = 0x00404040;

/// (0040,4041) VR=CS, VM=1 Input Readiness State
pub const InputReadinessState: u32 = 0x00404041;

/// (0040,4050) VR=DT, VM=1 Performed Procedure Step Start DateTime
pub const PerformedProcedureStepStartDateTime: u32 = 0x00404050;

/// (0040,4051) VR=DT, VM=1 Performed Procedure Step End DateTime
pub const PerformedProcedureStepEndDateTime: u32 = 0x00404051;

/// (0040,4052) VR=DT, VM=1 Procedure Step Cancellation DateTime
pub const ProcedureStepCancellationDateTime: u32 = 0x00404052;

/// (0040,8302) VR=DS, VM=1 Entrance Dose in mGy
pub const EntranceDoseInmGy: u32 = 0x00408302;

/// (0040,9094) VR=SQ, VM=1 Referenced Image Real World Value Mapping Sequence
pub const ReferencedImageRealWorldValueMappingSequence: u32 = 0x00409094;

/// (0040,9096) VR=SQ, VM=1 Real World Value Mapping Sequence
pub const RealWorldValueMappingSequence: u32 = 0x00409096;

/// (0040,9098) VR=SQ, VM=1 Pixel Value Mapping Code Sequence
pub const PixelValueMappingCodeSequence: u32 = 0x00409098;

/// (0040,9210) VR=SH, VM=1 LUT Label
pub const LUTLabel: u32 = 0x00409210;

/// (0040,9211) VR=US|SS, VM=1 Real World Value Last Value Mapped
pub const RealWorldValueLastValueMapped: u32 = 0x00409211;

/// (0040,9212) VR=FD, VM=1-n Real World Value LUT Data
pub const RealWorldValueLUTData: u32 = 0x00409212;

/// (0040,9216) VR=US|SS, VM=1 Real World Value First Value Mapped
pub const RealWorldValueFirstValueMapped: u32 = 0x00409216;

/// (0040,9224) VR=FD, VM=1 Real World Value Intercept
pub const RealWorldValueIntercept: u32 = 0x00409224;

/// (0040,9225) VR=FD, VM=1 Real World Value Slope
pub const RealWorldValueSlope: u32 = 0x00409225;

/// (0040,A007) VR=CS, VM=1 Findings Flag (Trial) (retired)
pub const FindingsFlagTrial: u32 = 0x0040A007;

/// (0040,A010) VR=CS, VM=1 Relationship Type
pub const RelationshipType: u32 = 0x0040A010;

/// (0040,A020) VR=SQ, VM=1 Findings Sequence (Trial) (retired)
pub const FindingsSequenceTrial: u32 = 0x0040A020;

/// (0040,A021) VR=UI, VM=1 Findings Group UID (Trial) (retired)
pub const FindingsGroupUIDTrial: u32 = 0x0040A021;

/// (0040,A022) VR=UI, VM=1 Referenced Findings Group UID (Trial) (retired)
pub const ReferencedFindingsGroupUIDTrial: u32 = 0x0040A022;

/// (0040,A023) VR=DA, VM=1 Findings Group Recording Date (Trial) (retired)
pub const FindingsGroupRecordingDateTrial: u32 = 0x0040A023;

/// (0040,A024) VR=TM, VM=1 Findings Group Recording Time (Trial) (retired)
pub const FindingsGroupRecordingTimeTrial: u32 = 0x0040A024;

/// (0040,A026) VR=SQ, VM=1 Findings Source Category Code Sequence (Trial) (retired)
pub const FindingsSourceCategoryCodeSequenceTrial: u32 = 0x0040A026;

/// (0040,A027) VR=LO, VM=1 Verifying Organization
pub const VerifyingOrganization: u32 = 0x0040A027;

/// (0040,A028) VR=SQ, VM=1 Documenting Organization Identifier Code Sequence (Trial) (retired)
pub const DocumentingOrganizationIdentifierCodeSequenceTrial: u32 = 0x0040A028;

/// (0040,A030) VR=DT, VM=1 Verification Date Time
pub const VerificationDateTime: u32 = 0x0040A030;

/// (0040,A032) VR=DT, VM=1 Observation Date Time
pub const ObservationDateTime: u32 = 0x0040A032;

/// (0040,A040) VR=CS, VM=1 Value Type
pub const ValueType: u32 = 0x0040A040;

/// (0040,A043) VR=SQ, VM=1 Concept Name Code Sequence
pub const ConceptNameCodeSequence: u32 = 0x0040A043;

/// (0040,A047) VR=LO, VM=1 Measurement Precision Description (Trial) (retired)
pub const MeasurementPrecisionDescriptionTrial: u32 = 0x0040A047;

/// (0040,A050) VR=CS, VM=1 Continuity Of Content
pub const ContinuityOfContent: u32 = 0x0040A050;

/// (0040,A057) VR=CS, VM=1-n Urgency or Priority Alerts (Trial) (retired)
pub const UrgencyOrPriorityAlertsTrial: u32 = 0x0040A057;

/// (0040,A060) VR=LO, VM=1 Sequencing Indicator (Trial) (retired)
pub const SequencingIndicatorTrial: u32 = 0x0040A060;

/// (0040,A066) VR=SQ, VM=1 Document Identifier Code Sequence (Trial) (retired)
pub const DocumentIdentifierCodeSequenceTrial: u32 = 0x0040A066;

/// (0040,A067) VR=PN, VM=1 Document Author (Trial) (retired)
pub const DocumentAuthorTrial: u32 = 0x0040A067;

/// (0040,A068) VR=SQ, VM=1 Document Author Identifier Code Sequence (Trial) (retired)
pub const DocumentAuthorIdentifierCodeSequenceTrial: u32 = 0x0040A068;

/// (0040,A070) VR=SQ, VM=1 Identifier Code Sequence (Trial) (retired)
pub const IdentifierCodeSequenceTrial: u32 = 0x0040A070;

/// (0040,A073) VR=SQ, VM=1 Verifying Observer Sequence
pub const VerifyingObserverSequence: u32 = 0x0040A073;

/// (0040,A074) VR=OB, VM=1 Object Binary Identifier (Trial) (retired)
pub const ObjectBinaryIdentifierTrial: u32 = 0x0040A074;

/// (0040,A075) VR=PN, VM=1 Verifying Observer Name
pub const VerifyingObserverName: u32 = 0x0040A075;

/// (0040,A076) VR=SQ, VM=1 Documenting Observer Identifier Code Sequence (Trial) (retired)
pub const DocumentingObserverIdentifierCodeSequenceTrial: u32 = 0x0040A076;

/// (0040,A078) VR=SQ, VM=1 Author Observer Sequence
pub const AuthorObserverSequence: u32 = 0x0040A078;

/// (0040,A07A) VR=SQ, VM=1 Participant Sequence
pub const ParticipantSequence: u32 = 0x0040A07A;

/// (0040,A07C) VR=SQ, VM=1 Custodial Organization Sequence
pub const CustodialOrganizationSequence: u32 = 0x0040A07C;

/// (0040,A080) VR=CS, VM=1 Participation Type
pub const ParticipationType: u32 = 0x0040A080;

/// (0040,A082) VR=DT, VM=1 Participation DateTime
pub const ParticipationDateTime: u32 = 0x0040A082;

/// (0040,A084) VR=CS, VM=1 Observer Type
pub const ObserverType: u32 = 0x0040A084;

/// (0040,A085) VR=SQ, VM=1 Procedure Identifier Code Sequence (Trial) (retired)
pub const ProcedureIdentifierCodeSequenceTrial: u32 = 0x0040A085;

/// (0040,A088) VR=SQ, VM=1 Verifying Observer Identification Code Sequence
pub const VerifyingObserverIdentificationCodeSequence: u32 = 0x0040A088;

/// (0040,A089) VR=OB, VM=1 Object Directory Binary Identifier (Trial) (retired)
pub const ObjectDirectoryBinaryIdentifierTrial: u32 = 0x0040A089;

/// (0040,A090) VR=SQ, VM=1 Equivalent CDA Document Sequence (retired)
pub const EquivalentCDADocumentSequence: u32 = 0x0040A090;

/// (0040,A0B0) VR=US, VM=2-2n Referenced Waveform Channels
pub const ReferencedWaveformChannels: u32 = 0x0040A0B0;

/// (0040,A110) VR=DA, VM=1 Date of Document or Verbal Transaction (Trial) (retired)
pub const DateOfDocumentOrVerbalTransactionTrial: u32 = 0x0040A110;

/// (0040,A112) VR=TM, VM=1 Time of Document Creation or Verbal Transaction (Trial) (retired)
pub const TimeOfDocumentCreationOrVerbalTransactionTrial: u32 = 0x0040A112;

/// (0040,A120) VR=DT, VM=1 DateTime
pub const DateTime: u32 = 0x0040A120;

/// (0040,A121) VR=DA, VM=1 Date
pub const Date: u32 = 0x0040A121;

/// (0040,A122) VR=TM, VM=1 Time
pub const Time: u32 = 0x0040A122;

/// (0040,A123) VR=PN, VM=1 Person Name
pub const PersonName: u32 = 0x0040A123;

/// (0040,A124) VR=UI, VM=1 UID
pub const UID: u32 = 0x0040A124;

/// (0040,A125) VR=CS, VM=2 Report Status ID (Trial) (retired)
pub const ReportStatusIDTrial: u32 = 0x0040A125;

/// (0040,A130) VR=CS, VM=1 Temporal Range Type
pub const TemporalRangeType: u32 = 0x0040A130;

/// (0040,A132) VR=UL, VM=1-n Referenced Sample Positions
pub const ReferencedSamplePositions: u32 = 0x0040A132;

/// (0040,A136) VR=US, VM=1-n Referenced Frame Numbers
pub const ReferencedFrameNumbers: u32 = 0x0040A136;

/// (0040,A138) VR=DS, VM=1-n Referenced Time Offsets
pub const ReferencedTimeOffsets: u32 = 0x0040A138;

/// (0040,A13A) VR=DT, VM=1-n Referenced DateTime
pub const ReferencedDateTime: u32 = 0x0040A13A;

/// (0040,A160) VR=UT, VM=1 Text Value
pub const TextValue: u32 = 0x0040A160;

/// (0040,A167) VR=SQ, VM=1 Observation Category Code Sequence (Trial) (retired)
pub const ObservationCategoryCodeSequenceTrial: u32 = 0x0040A167;

/// (0040,A168) VR=SQ, VM=1 Concept Code Sequence
pub const ConceptCodeSequence: u32 = 0x0040A168;

/// (0040,A16A) VR=ST, VM=1 Bibliographic Citation (Trial) (retired)
pub const BibliographicCitationTrial: u32 = 0x0040A16A;

/// (0040,A170) VR=SQ, VM=1 Purpose of Reference Code Sequence
pub const PurposeOfReferenceCodeSequence: u32 = 0x0040A170;

/// (0040,A171) VR=UI, VM=1 Observation UID (Trial) (retired)
pub const ObservationUIDTrial: u32 = 0x0040A171;

/// (0040,A172) VR=UI, VM=1 Referenced Observation UID (Trial) (retired)
pub const ReferencedObservationUIDTrial: u32 = 0x0040A172;

/// (0040,A173) VR=CS, VM=1 Referenced Observation Class (Trial) (retired)
pub const ReferencedObservationClassTrial: u32 = 0x0040A173;

/// (0040,A174) VR=CS, VM=1 Referenced Object Observation Class (Trial) (retired)
pub const ReferencedObjectObservationClassTrial: u32 = 0x0040A174;

/// (0040,A180) VR=US, VM=1 Annotation Group Number
pub const AnnotationGroupNumber: u32 = 0x0040A180;

/// (0040,A192) VR=DA, VM=1 Observation Date (Trial) (retired)
pub const ObservationDateTrial: u32 = 0x0040A192;

/// (0040,A193) VR=TM, VM=1 Observation Time (Trial) (retired)
pub const ObservationTimeTrial: u32 = 0x0040A193;

/// (0040,A194) VR=CS, VM=1 Measurement Automation (Trial) (retired)
pub const MeasurementAutomationTrial: u32 = 0x0040A194;

/// (0040,A195) VR=SQ, VM=1 Modifier Code Sequence
pub const ModifierCodeSequence: u32 = 0x0040A195;

/// (0040,A224) VR=ST, VM=1 Identification Description (Trial) (retired)
pub const IdentificationDescriptionTrial: u32 = 0x0040A224;

/// (0040,A290) VR=CS, VM=1 Coordinates Set Geometric Type (Trial) (retired)
pub const CoordinatesSetGeometricTypeTrial: u32 = 0x0040A290;

/// (0040,A296) VR=SQ, VM=1 Algorithm Code Sequence (Trial) (retired)
pub const AlgorithmCodeSequenceTrial: u32 = 0x0040A296;

/// (0040,A297) VR=ST, VM=1 Algorithm Description (Trial) (retired)
pub const AlgorithmDescriptionTrial: u32 = 0x0040A297;

/// (0040,A29A) VR=SL, VM=2-2n Pixel Coordinates Set (Trial) (retired)
pub const PixelCoordinatesSetTrial: u32 = 0x0040A29A;

/// (0040,A300) VR=SQ, VM=1 Measured Value Sequence
pub const MeasuredValueSequence: u32 = 0x0040A300;

/// (0040,A301) VR=SQ, VM=1 Numeric Value Qualifier Code Sequence
pub const NumericValueQualifierCodeSequence: u32 = 0x0040A301;

/// (0040,A307) VR=PN, VM=1 Current Observer (Trial) (retired)
pub const CurrentObserverTrial: u32 = 0x0040A307;

/// (0040,A30A) VR=DS, VM=1-n Numeric Value
pub const NumericValue: u32 = 0x0040A30A;

/// (0040,A313) VR=SQ, VM=1 Referenced Accession Sequence (Trial) (retired)
pub const ReferencedAccessionSequenceTrial: u32 = 0x0040A313;

/// (0040,A33A) VR=ST, VM=1 Report Status Comment (Trial) (retired)
pub const ReportStatusCommentTrial: u32 = 0x0040A33A;

/// (0040,A340) VR=SQ, VM=1 Procedure Context Sequence (Trial) (retired)
pub const ProcedureContextSequenceTrial: u32 = 0x0040A340;

/// (0040,A352) VR=PN, VM=1 Verbal Source (Trial) (retired)
pub const VerbalSourceTrial: u32 = 0x0040A352;

/// (0040,A353) VR=ST, VM=1 Address (Trial) (retired)
pub const AddressTrial: u32 = 0x0040A353;

/// (0040,A354) VR=LO, VM=1 Telephone Number (Trial) (retired)
pub const TelephoneNumberTrial: u32 = 0x0040A354;

/// (0040,A358) VR=SQ, VM=1 Verbal Source Identifier Code Sequence (Trial) (retired)
pub const VerbalSourceIdentifierCodeSequenceTrial: u32 = 0x0040A358;

/// (0040,A360) VR=SQ, VM=1 Predecessor Documents Sequence
pub const PredecessorDocumentsSequence: u32 = 0x0040A360;

/// (0040,A370) VR=SQ, VM=1 Referenced Request Sequence
pub const ReferencedRequestSequence: u32 = 0x0040A370;

/// (0040,A372) VR=SQ, VM=1 Performed Procedure Code Sequence
pub const PerformedProcedureCodeSequence: u32 = 0x0040A372;

/// (0040,A375) VR=SQ, VM=1 Current Requested Procedure Evidence Sequence
pub const CurrentRequestedProcedureEvidenceSequence: u32 = 0x0040A375;

/// (0040,A380) VR=SQ, VM=1 Report Detail Sequence (Trial) (retired)
pub const ReportDetailSequenceTrial: u32 = 0x0040A380;

/// (0040,A385) VR=SQ, VM=1 Pertinent Other Evidence Sequence
pub const PertinentOtherEvidenceSequence: u32 = 0x0040A385;

/// (0040,A390) VR=SQ, VM=1 HL7 Structured Document Reference Sequence
pub const HL7StructuredDocumentReferenceSequence: u32 = 0x0040A390;

/// (0040,A402) VR=UI, VM=1 Observation Subject UID (Trial) (retired)
pub const ObservationSubjectUIDTrial: u32 = 0x0040A402;

/// (0040,A403) VR=CS, VM=1 Observation Subject Class (Trial) (retired)
pub const ObservationSubjectClassTrial: u32 = 0x0040A403;

/// (0040,A404) VR=SQ, VM=1 Observation Subject Type Code Sequence (Trial) (retired)
pub const ObservationSubjectTypeCodeSequenceTrial: u32 = 0x0040A404;

/// (0040,A491) VR=CS, VM=1 Completion Flag
pub const CompletionFlag: u32 = 0x0040A491;

/// (0040,A492) VR=LO, VM=1 Completion Flag Description
pub const CompletionFlagDescription: u32 = 0x0040A492;

/// (0040,A493) VR=CS, VM=1 Verification Flag
pub const VerificationFlag: u32 = 0x0040A493;

/// (0040,A494) VR=CS, VM=1 Archive Requested
pub const ArchiveRequested: u32 = 0x0040A494;

/// (0040,A496) VR=CS, VM=1 Preliminary Flag
pub const PreliminaryFlag: u32 = 0x0040A496;

/// (0040,A504) VR=SQ, VM=1 Content Template Sequence
pub const ContentTemplateSequence: u32 = 0x0040A504;

/// (0040,A525) VR=SQ, VM=1 Identical Documents Sequence
pub const IdenticalDocumentsSequence: u32 = 0x0040A525;

/// (0040,A600) VR=CS, VM=1 Observation Subject Context Flag (Trial) (retired)
pub const ObservationSubjectContextFlagTrial: u32 = 0x0040A600;

/// (0040,A601) VR=CS, VM=1 Observer Context Flag (Trial) (retired)
pub const ObserverContextFlagTrial: u32 = 0x0040A601;

/// (0040,A603) VR=CS, VM=1 Procedure Context Flag (Trial) (retired)
pub const ProcedureContextFlagTrial: u32 = 0x0040A603;

/// (0040,A730) VR=SQ, VM=1 Content Sequence
pub const ContentSequence: u32 = 0x0040A730;

/// (0040,A731) VR=SQ, VM=1 Relationship Sequence (Trial) (retired)
pub const RelationshipSequenceTrial: u32 = 0x0040A731;

/// (0040,A732) VR=SQ, VM=1 Relationship Type Code Sequence (Trial) (retired)
pub const RelationshipTypeCodeSequenceTrial: u32 = 0x0040A732;

/// (0040,A744) VR=SQ, VM=1 Language Code Sequence (Trial) (retired)
pub const LanguageCodeSequenceTrial: u32 = 0x0040A744;

/// (0040,A992) VR=ST, VM=1 Uniform Resource Locator (Trial) (retired)
pub const UniformResourceLocatorTrial: u32 = 0x0040A992;

/// (0040,B020) VR=SQ, VM=1 Waveform Annotation Sequence
pub const WaveformAnnotationSequence: u32 = 0x0040B020;

/// (0040,DB00) VR=CS, VM=1 Template Identifier
pub const TemplateIdentifier: u32 = 0x0040DB00;

/// (0040,DB06) VR=DT, VM=1 Template Version (retired)
pub const TemplateVersion: u32 = 0x0040DB06;

/// (0040,DB07) VR=DT, VM=1 Template Local Version (retired)
pub const TemplateLocalVersion: u32 = 0x0040DB07;

/// (0040,DB0B) VR=CS, VM=1 Template Extension Flag (retired)
pub const TemplateExtensionFlag: u32 = 0x0040DB0B;

/// (0040,DB0C) VR=UI, VM=1 Template Extension Organization UID (retired)
pub const TemplateExtensionOrganizationUID: u32 = 0x0040DB0C;

/// (0040,DB0D) VR=UI, VM=1 Template Extension Creator UID (retired)
pub const TemplateExtensionCreatorUID: u32 = 0x0040DB0D;

/// (0040,DB73) VR=UL, VM=1-n Referenced Content Item Identifier
pub const ReferencedContentItemIdentifier: u32 = 0x0040DB73;

/// (0040,E001) VR=ST, VM=1 HL7 Instance Identifier
pub const HL7InstanceIdentifier: u32 = 0x0040E001;

/// (0040,E004) VR=DT, VM=1 HL7 Document Effective Time
pub const HL7DocumentEffectiveTime: u32 = 0x0040E004;

/// (0040,E006) VR=SQ, VM=1 HL7 Document Type Code Sequence
pub const HL7DocumentTypeCodeSequence: u32 = 0x0040E006;

/// (0040,E008) VR=SQ, VM=1 Document Class Code Sequence
pub const DocumentClassCodeSequence: u32 = 0x0040E008;

/// (0040,E010) VR=UT, VM=1 Retrieve URI
pub const RetrieveURI: u32 = 0x0040E010;

/// (0040,E011) VR=UI, VM=1 Retrieve Location UID
pub const RetrieveLocationUID: u32 = 0x0040E011;

/// (0040,E020) VR=CS, VM=1 Type of Instances
pub const TypeOfInstances: u32 = 0x0040E020;

/// (0040,E021) VR=SQ, VM=1 DICOM Retrieval Sequence
pub const DICOMRetrievalSequence: u32 = 0x0040E021;

/// (0040,E022) VR=SQ, VM=1 DICOM Media Retrieval Sequence
pub const DICOMMediaRetrievalSequence: u32 = 0x0040E022;

/// (0040,E023) VR=SQ, VM=1 WADO Retrieval Sequence
pub const WADORetrievalSequence: u32 = 0x0040E023;

/// (0040,E024) VR=SQ, VM=1 XDS Retrieval Sequence
pub const XDSRetrievalSequence: u32 = 0x0040E024;

/// (0040,E030) VR=UI, VM=1 Repository Unique ID
pub const RepositoryUniqueID: u32 = 0x0040E030;

/// (0040,E031) VR=UI, VM=1 Home Community ID
pub const HomeCommunityID: u32 = 0x0040E031;

/// (0042,0010) VR=ST, VM=1 Document Title
pub const DocumentTitle: u32 = 0x00420010;

/// (0042,0011) VR=OB, VM=1 Encapsulated Document
pub const EncapsulatedDocument: u32 = 0x00420011;

/// (0042,0012) VR=LO, VM=1 MIME Type of Encapsulated Document
pub const MIMETypeOfEncapsulatedDocument: u32 = 0x00420012;

/// (0042,0013) VR=SQ, VM=1 Source Instance Sequence
pub const SourceInstanceSequence: u32 = 0x00420013;

/// (0042,0014) VR=LO, VM=1-n List of MIME Types
pub const ListOfMIMETypes: u32 = 0x00420014;

/// (0044,0001) VR=ST, VM=1 Product Package Identifier
pub const ProductPackageIdentifier: u32 = 0x00440001;

/// (0044,0002) VR=CS, VM=1 Substance Administration Approval
pub const SubstanceAdministrationApproval: u32 = 0x00440002;

/// (0044,0003) VR=LT, VM=1 Approval Status Further Description
pub const ApprovalStatusFurtherDescription: u32 = 0x00440003;

/// (0044,0004) VR=DT, VM=1 Approval Status DateTime
pub const ApprovalStatusDateTime: u32 = 0x00440004;

/// (0044,0007) VR=SQ, VM=1 Product Type Code Sequence
pub const ProductTypeCodeSequence: u32 = 0x00440007;

/// (0044,0008) VR=LO, VM=1-n Product Name
pub const ProductName: u32 = 0x00440008;

/// (0044,0009) VR=LT, VM=1 Product Description
pub const ProductDescription: u32 = 0x00440009;

/// (0044,000A) VR=LO, VM=1 Product Lot Identifier
pub const ProductLotIdentifier: u32 = 0x0044000A;

/// (0044,000B) VR=DT, VM=1 Product Expiration DateTime
pub const ProductExpirationDateTime: u32 = 0x0044000B;

/// (0044,0010) VR=DT, VM=1 Substance Administration DateTime
pub const SubstanceAdministrationDateTime: u32 = 0x00440010;

/// (0044,0011) VR=LO, VM=1 Substance Administration Notes
pub const SubstanceAdministrationNotes: u32 = 0x00440011;

/// (0044,0012) VR=LO, VM=1 Substance Administration Device ID
pub const SubstanceAdministrationDeviceID: u32 = 0x00440012;

/// (0044,0013) VR=SQ, VM=1 Product Parameter Sequence
pub const ProductParameterSequence: u32 = 0x00440013;

/// (0044,0019) VR=SQ, VM=1 Substance Administration Parameter Sequence
pub const SubstanceAdministrationParameterSequence: u32 = 0x00440019;

/// (0046,0012) VR=LO, VM=1 Lens Description
pub const LensDescription: u32 = 0x00460012;

/// (0046,0014) VR=SQ, VM=1 Right Lens Sequence
pub const RightLensSequence: u32 = 0x00460014;

/// (0046,0015) VR=SQ, VM=1 Left Lens Sequence
pub const LeftLensSequence: u32 = 0x00460015;

/// (0046,0016) VR=SQ, VM=1 Unspecified Laterality Lens Sequence
pub const UnspecifiedLateralityLensSequence: u32 = 0x00460016;

/// (0046,0018) VR=SQ, VM=1 Cylinder Sequence
pub const CylinderSequence: u32 = 0x00460018;

/// (0046,0028) VR=SQ, VM=1 Prism Sequence
pub const PrismSequence: u32 = 0x00460028;

/// (0046,0030) VR=FD, VM=1 Horizontal Prism Power
pub const HorizontalPrismPower: u32 = 0x00460030;

/// (0046,0032) VR=CS, VM=1 Horizontal Prism Base
pub const HorizontalPrismBase: u32 = 0x00460032;

/// (0046,0034) VR=FD, VM=1 Vertical Prism Power
pub const VerticalPrismPower: u32 = 0x00460034;

/// (0046,0036) VR=CS, VM=1 Vertical Prism Base
pub const VerticalPrismBase: u32 = 0x00460036;

/// (0046,0038) VR=CS, VM=1 Lens Segment Type
pub const LensSegmentType: u32 = 0x00460038;

/// (0046,0040) VR=FD, VM=1 Optical Transmittance
pub const OpticalTransmittance: u32 = 0x00460040;

/// (0046,0042) VR=FD, VM=1 Channel Width
pub const ChannelWidth: u32 = 0x00460042;

/// (0046,0044) VR=FD, VM=1 Pupil Size
pub const PupilSize: u32 = 0x00460044;

/// (0046,0046) VR=FD, VM=1 Corneal Size
pub const CornealSize: u32 = 0x00460046;

/// (0046,0050) VR=SQ, VM=1 Autorefraction Right Eye Sequence
pub const AutorefractionRightEyeSequence: u32 = 0x00460050;

/// (0046,0052) VR=SQ, VM=1 Autorefraction Left Eye Sequence
pub const AutorefractionLeftEyeSequence: u32 = 0x00460052;

/// (0046,0060) VR=FD, VM=1 Distance Pupillary Distance
pub const DistancePupillaryDistance: u32 = 0x00460060;

/// (0046,0062) VR=FD, VM=1 Near Pupillary Distance
pub const NearPupillaryDistance: u32 = 0x00460062;

/// (0046,0063) VR=FD, VM=1 Intermediate Pupillary Distance
pub const IntermediatePupillaryDistance: u32 = 0x00460063;

/// (0046,0064) VR=FD, VM=1 Other Pupillary Distance
pub const OtherPupillaryDistance: u32 = 0x00460064;

/// (0046,0070) VR=SQ, VM=1 Keratometry Right Eye Sequence
pub const KeratometryRightEyeSequence: u32 = 0x00460070;

/// (0046,0071) VR=SQ, VM=1 Keratometry Left Eye Sequence
pub const KeratometryLeftEyeSequence: u32 = 0x00460071;

/// (0046,0074) VR=SQ, VM=1 Steep Keratometric Axis Sequence
pub const SteepKeratometricAxisSequence: u32 = 0x00460074;

/// (0046,0075) VR=FD, VM=1 Radius of Curvature
pub const RadiusOfCurvature: u32 = 0x00460075;

/// (0046,0076) VR=FD, VM=1 Keratometric Power
pub const KeratometricPower: u32 = 0x00460076;

/// (0046,0077) VR=FD, VM=1 Keratometric Axis
pub const KeratometricAxis: u32 = 0x00460077;

/// (0046,0080) VR=SQ, VM=1 Flat Keratometric Axis Sequence
pub const FlatKeratometricAxisSequence: u32 = 0x00460080;

/// (0046,0092) VR=CS, VM=1 Background Color
pub const BackgroundColor: u32 = 0x00460092;

/// (0046,0094) VR=CS, VM=1 Optotype
pub const Optotype: u32 = 0x00460094;

/// (0046,0095) VR=CS, VM=1 Optotype Presentation
pub const OptotypePresentation: u32 = 0x00460095;

/// (0046,0097) VR=SQ, VM=1 Subjective Refraction Right Eye Sequence
pub const SubjectiveRefractionRightEyeSequence: u32 = 0x00460097;

/// (0046,0098) VR=SQ, VM=1 Subjective Refraction Left Eye Sequence
pub const SubjectiveRefractionLeftEyeSequence: u32 = 0x00460098;

/// (0046,0100) VR=SQ, VM=1 Add Near Sequence
pub const AddNearSequence: u32 = 0x00460100;

/// (0046,0101) VR=SQ, VM=1 Add Intermediate Sequence
pub const AddIntermediateSequence: u32 = 0x00460101;

/// (0046,0102) VR=SQ, VM=1 Add Other Sequence
pub const AddOtherSequence: u32 = 0x00460102;

/// (0046,0104) VR=FD, VM=1 Add Power
pub const AddPower: u32 = 0x00460104;

/// (0046,0106) VR=FD, VM=1 Viewing Distance
pub const ViewingDistance: u32 = 0x00460106;

/// (0046,0121) VR=SQ, VM=1 Visual Acuity Type Code Sequence
pub const VisualAcuityTypeCodeSequence: u32 = 0x00460121;

/// (0046,0122) VR=SQ, VM=1 Visual Acuity Right Eye Sequence
pub const VisualAcuityRightEyeSequence: u32 = 0x00460122;

/// (0046,0123) VR=SQ, VM=1 Visual Acuity Left Eye Sequence
pub const VisualAcuityLeftEyeSequence: u32 = 0x00460123;

/// (0046,0124) VR=SQ, VM=1 Visual Acuity Both Eyes Open Sequence
pub const VisualAcuityBothEyesOpenSequence: u32 = 0x00460124;

/// (0046,0125) VR=CS, VM=1 Viewing Distance Type
pub const ViewingDistanceType: u32 = 0x00460125;

/// (0046,0135) VR=SS, VM=2 Visual Acuity Modifiers
pub const VisualAcuityModifiers: u32 = 0x00460135;

/// (0046,0137) VR=FD, VM=1 Decimal Visual Acuity
pub const DecimalVisualAcuity: u32 = 0x00460137;

/// (0046,0139) VR=LO, VM=1 Optotype Detailed Definition
pub const OptotypeDetailedDefinition: u32 = 0x00460139;

/// (0046,0145) VR=SQ, VM=1 Referenced Refractive Measurements Sequence
pub const ReferencedRefractiveMeasurementsSequence: u32 = 0x00460145;

/// (0046,0146) VR=FD, VM=1 Sphere Power
pub const SpherePower: u32 = 0x00460146;

/// (0046,0147) VR=FD, VM=1 Cylinder Power
pub const CylinderPower: u32 = 0x00460147;

/// (0048,0001) VR=FL, VM=1 Imaged Volume Width
pub const ImagedVolumeWidth: u32 = 0x00480001;

/// (0048,0002) VR=FL, VM=1 Imaged Volume Height
pub const ImagedVolumeHeight: u32 = 0x00480002;

/// (0048,0003) VR=FL, VM=1 Imaged Volume Depth
pub const ImagedVolumeDepth: u32 = 0x00480003;

/// (0048,0006) VR=UL, VM=1 Total Pixel Matrix Columns
pub const TotalPixelMatrixColumns: u32 = 0x00480006;

/// (0048,0007) VR=UL, VM=1 Total Pixel Matrix Rows
pub const TotalPixelMatrixRows: u32 = 0x00480007;

/// (0048,0008) VR=SQ, VM=1 Total Pixel Matrix Origin Sequence
pub const TotalPixelMatrixOriginSequence: u32 = 0x00480008;

/// (0048,0010) VR=CS, VM=1 Specimen Label in Image
pub const SpecimenLabelInImage: u32 = 0x00480010;

/// (0048,0011) VR=CS, VM=1 Focus Method
pub const FocusMethod: u32 = 0x00480011;

/// (0048,0012) VR=CS, VM=1 Extended Depth of Field
pub const ExtendedDepthOfField: u32 = 0x00480012;

/// (0048,0013) VR=US, VM=1 Number of Focal Planes
pub const NumberOfFocalPlanes: u32 = 0x00480013;

/// (0048,0014) VR=FL, VM=1 Distance Between Focal Planes
pub const DistanceBetweenFocalPlanes: u32 = 0x00480014;

/// (0048,0015) VR=US, VM=3 Recommended Absent Pixel CIELab Value
pub const RecommendedAbsentPixelCIELabValue: u32 = 0x00480015;

/// (0048,0100) VR=SQ, VM=1 Illuminator Type Code Sequence
pub const IlluminatorTypeCodeSequence: u32 = 0x00480100;

/// (0048,0102) VR=DS, VM=6 Image Orientation (Slide)
pub const ImageOrientationSlide: u32 = 0x00480102;

/// (0048,0105) VR=SQ, VM=1 Optical Path Sequence
pub const OpticalPathSequence: u32 = 0x00480105;

/// (0048,0106) VR=SH, VM=1 Optical Path Identifier
pub const OpticalPathIdentifier: u32 = 0x00480106;

/// (0048,0107) VR=ST, VM=1 Optical Path Description
pub const OpticalPathDescription: u32 = 0x00480107;

/// (0048,0108) VR=SQ, VM=1 Illumination Color Code Sequence
pub const IlluminationColorCodeSequence: u32 = 0x00480108;

/// (0048,0110) VR=SQ, VM=1 Specimen Reference Sequence
pub const SpecimenReferenceSequence: u32 = 0x00480110;

/// (0048,0111) VR=DS, VM=1 Condenser Lens Power
pub const CondenserLensPower: u32 = 0x00480111;

/// (0048,0112) VR=DS, VM=1 Objective Lens Power
pub const ObjectiveLensPower: u32 = 0x00480112;

/// (0048,0113) VR=DS, VM=1 Objective Lens Numerical Aperture
pub const ObjectiveLensNumericalAperture: u32 = 0x00480113;

/// (0048,0120) VR=SQ, VM=1 Palette Color Lookup Table Sequence
pub const PaletteColorLookupTableSequence: u32 = 0x00480120;

/// (0048,0200) VR=SQ, VM=1 Referenced Image Navigation Sequence
pub const ReferencedImageNavigationSequence: u32 = 0x00480200;

/// (0048,0201) VR=US, VM=2 Top Left Hand Corner of Localizer Area
pub const TopLeftHandCornerOfLocalizerArea: u32 = 0x00480201;

/// (0048,0202) VR=US, VM=2 Bottom Right Hand Corner of Localizer Area
pub const BottomRightHandCornerOfLocalizerArea: u32 = 0x00480202;

/// (0048,0207) VR=SQ, VM=1 Optical Path Identification Sequence
pub const OpticalPathIdentificationSequence: u32 = 0x00480207;

/// (0048,021A) VR=SQ, VM=1 Plane Position (Slide) Sequence
pub const PlanePositionSlideSequence: u32 = 0x0048021A;

/// (0048,021E) VR=SL, VM=1 Row Position In Total Image Pixel Matrix
pub const RowPositionInTotalImagePixelMatrix: u32 = 0x0048021E;

/// (0048,021F) VR=SL, VM=1 Column Position In Total Image Pixel Matrix
pub const ColumnPositionInTotalImagePixelMatrix: u32 = 0x0048021F;

/// (0048,0301) VR=CS, VM=1 Pixel Origin Interpretation
pub const PixelOriginInterpretation: u32 = 0x00480301;

/// (0050,0004) VR=CS, VM=1 Calibration Image
pub const CalibrationImage: u32 = 0x00500004;

/// (0050,0010) VR=SQ, VM=1 Device Sequence
pub const DeviceSequence: u32 = 0x00500010;

/// (0050,0012) VR=SQ, VM=1 Container Component Type Code Sequence
pub const ContainerComponentTypeCodeSequence: u32 = 0x00500012;

/// (0050,0013) VR=FD, VM=1 Container Component Thickness
pub const ContainerComponentThickness: u32 = 0x00500013;

/// (0050,0014) VR=DS, VM=1 Device Length
pub const DeviceLength: u32 = 0x00500014;

/// (0050,0015) VR=FD, VM=1 Container Component Width
pub const ContainerComponentWidth: u32 = 0x00500015;

/// (0050,0016) VR=DS, VM=1 Device Diameter
pub const DeviceDiameter: u32 = 0x00500016;

/// (0050,0017) VR=CS, VM=1 Device Diameter Units
pub const DeviceDiameterUnits: u32 = 0x00500017;

/// (0050,0018) VR=DS, VM=1 Device Volume
pub const DeviceVolume: u32 = 0x00500018;

/// (0050,0019) VR=DS, VM=1 Inter-Marker Distance
pub const InterMarkerDistance: u32 = 0x00500019;

/// (0050,001A) VR=CS, VM=1 Container Component Material
pub const ContainerComponentMaterial: u32 = 0x0050001A;

/// (0050,001B) VR=LO, VM=1 Container Component ID
pub const ContainerComponentID: u32 = 0x0050001B;

/// (0050,001C) VR=FD, VM=1 Container Component Length
pub const ContainerComponentLength: u32 = 0x0050001C;

/// (0050,001D) VR=FD, VM=1 Container Component Diameter
pub const ContainerComponentDiameter: u32 = 0x0050001D;

/// (0050,001E) VR=LO, VM=1 Container Component Description
pub const ContainerComponentDescription: u32 = 0x0050001E;

/// (0050,0020) VR=LO, VM=1 Device Description
pub const DeviceDescription: u32 = 0x00500020;

/// (0052,0001) VR=FL, VM=1 Contrast/Bolus Ingredient Percent by Volume
pub const ContrastBolusIngredientPercentByVolume: u32 = 0x00520001;

/// (0052,0002) VR=FD, VM=1 OCT Focal Distance
pub const OCTFocalDistance: u32 = 0x00520002;

/// (0052,0003) VR=FD, VM=1 Beam Spot Size
pub const BeamSpotSize: u32 = 0x00520003;

/// (0052,0004) VR=FD, VM=1 Effective Refractive Index
pub const EffectiveRefractiveIndex: u32 = 0x00520004;

/// (0052,0006) VR=CS, VM=1 OCT Acquisition Domain
pub const OCTAcquisitionDomain: u32 = 0x00520006;

/// (0052,0007) VR=FD, VM=1 OCT Optical Center Wavelength
pub const OCTOpticalCenterWavelength: u32 = 0x00520007;

/// (0052,0008) VR=FD, VM=1 Axial Resolution
pub const AxialResolution: u32 = 0x00520008;

/// (0052,0009) VR=FD, VM=1 Ranging Depth
pub const RangingDepth: u32 = 0x00520009;

/// (0052,0011) VR=FD, VM=1 A‑line Rate
pub const ALineRate: u32 = 0x00520011;

/// (0052,0012) VR=US, VM=1 A‑lines Per Frame
pub const ALinesPerFrame: u32 = 0x00520012;

/// (0052,0013) VR=FD, VM=1 Catheter Rotational Rate
pub const CatheterRotationalRate: u32 = 0x00520013;

/// (0052,0014) VR=FD, VM=1 A‑line Pixel Spacing
pub const ALinePixelSpacing: u32 = 0x00520014;

/// (0052,0016) VR=SQ, VM=1 Mode of Percutaneous Access Sequence
pub const ModeOfPercutaneousAccessSequence: u32 = 0x00520016;

/// (0052,0025) VR=SQ, VM=1 Intravascular OCT Frame Type Sequence
pub const IntravascularOCTFrameTypeSequence: u32 = 0x00520025;

/// (0052,0026) VR=CS, VM=1 OCT Z Offset Applied
pub const OCTZOffsetApplied: u32 = 0x00520026;

/// (0052,0027) VR=SQ, VM=1 Intravascular Frame Content Sequence
pub const IntravascularFrameContentSequence: u32 = 0x00520027;

/// (0052,0028) VR=FD, VM=1 Intravascular Longitudinal Distance
pub const IntravascularLongitudinalDistance: u32 = 0x00520028;

/// (0052,0029) VR=SQ, VM=1 Intravascular OCT Frame Content Sequence
pub const IntravascularOCTFrameContentSequence: u32 = 0x00520029;

/// (0052,0030) VR=SS, VM=1 OCT Z Offset Correction
pub const OCTZOffsetCorrection: u32 = 0x00520030;

/// (0052,0031) VR=CS, VM=1 Catheter Direction of Rotation
pub const CatheterDirectionOfRotation: u32 = 0x00520031;

/// (0052,0033) VR=FD, VM=1 Seam Line Location
pub const SeamLineLocation: u32 = 0x00520033;

/// (0052,0034) VR=FD, VM=1 First A‑line Location
pub const FirstALineLocation: u32 = 0x00520034;

/// (0052,0036) VR=US, VM=1 Seam Line Index
pub const SeamLineIndex: u32 = 0x00520036;

/// (0052,0038) VR=US, VM=1 Number of Padded A‑lines
pub const NumberOfPaddedAlines: u32 = 0x00520038;

/// (0052,0039) VR=CS, VM=1 Interpolation Type
pub const InterpolationType: u32 = 0x00520039;

/// (0052,003A) VR=CS, VM=1 Refractive Index Applied
pub const RefractiveIndexApplied: u32 = 0x0052003A;

/// (0054,0011) VR=US, VM=1 Number of Energy Windows
pub const NumberOfEnergyWindows: u32 = 0x00540011;

/// (0054,0012) VR=SQ, VM=1 Energy Window Information Sequence
pub const EnergyWindowInformationSequence: u32 = 0x00540012;

/// (0054,0013) VR=SQ, VM=1 Energy Window Range Sequence
pub const EnergyWindowRangeSequence: u32 = 0x00540013;

/// (0054,0014) VR=DS, VM=1 Energy Window Lower Limit
pub const EnergyWindowLowerLimit: u32 = 0x00540014;

/// (0054,0015) VR=DS, VM=1 Energy Window Upper Limit
pub const EnergyWindowUpperLimit: u32 = 0x00540015;

/// (0054,0016) VR=SQ, VM=1 Radiopharmaceutical Information Sequence
pub const RadiopharmaceuticalInformationSequence: u32 = 0x00540016;

/// (0054,0017) VR=IS, VM=1 Residual Syringe Counts
pub const ResidualSyringeCounts: u32 = 0x00540017;

/// (0054,0018) VR=SH, VM=1 Energy Window Name
pub const EnergyWindowName: u32 = 0x00540018;

/// (0054,0020) VR=US, VM=1-n Detector Vector
pub const DetectorVector: u32 = 0x00540020;

/// (0054,0021) VR=US, VM=1 Number of Detectors
pub const NumberOfDetectors: u32 = 0x00540021;

/// (0054,0022) VR=SQ, VM=1 Detector Information Sequence
pub const DetectorInformationSequence: u32 = 0x00540022;

/// (0054,0030) VR=US, VM=1-n Phase Vector
pub const PhaseVector: u32 = 0x00540030;

/// (0054,0031) VR=US, VM=1 Number of Phases
pub const NumberOfPhases: u32 = 0x00540031;

/// (0054,0032) VR=SQ, VM=1 Phase Information Sequence
pub const PhaseInformationSequence: u32 = 0x00540032;

/// (0054,0033) VR=US, VM=1 Number of Frames in Phase
pub const NumberOfFramesInPhase: u32 = 0x00540033;

/// (0054,0036) VR=IS, VM=1 Phase Delay
pub const PhaseDelay: u32 = 0x00540036;

/// (0054,0038) VR=IS, VM=1 Pause Between Frames
pub const PauseBetweenFrames: u32 = 0x00540038;

/// (0054,0039) VR=CS, VM=1 Phase Description
pub const PhaseDescription: u32 = 0x00540039;

/// (0054,0050) VR=US, VM=1-n Rotation Vector
pub const RotationVector: u32 = 0x00540050;

/// (0054,0051) VR=US, VM=1 Number of Rotations
pub const NumberOfRotations: u32 = 0x00540051;

/// (0054,0052) VR=SQ, VM=1 Rotation Information Sequence
pub const RotationInformationSequence: u32 = 0x00540052;

/// (0054,0053) VR=US, VM=1 Number of Frames in Rotation
pub const NumberOfFramesInRotation: u32 = 0x00540053;

/// (0054,0060) VR=US, VM=1-n R-R Interval Vector
pub const RRIntervalVector: u32 = 0x00540060;

/// (0054,0061) VR=US, VM=1 Number of R-R Intervals
pub const NumberOfRRIntervals: u32 = 0x00540061;

/// (0054,0062) VR=SQ, VM=1 Gated Information Sequence
pub const GatedInformationSequence: u32 = 0x00540062;

/// (0054,0063) VR=SQ, VM=1 Data Information Sequence
pub const DataInformationSequence: u32 = 0x00540063;

/// (0054,0070) VR=US, VM=1-n Time Slot Vector
pub const TimeSlotVector: u32 = 0x00540070;

/// (0054,0071) VR=US, VM=1 Number of Time Slots
pub const NumberOfTimeSlots: u32 = 0x00540071;

/// (0054,0072) VR=SQ, VM=1 Time Slot Information Sequence
pub const TimeSlotInformationSequence: u32 = 0x00540072;

/// (0054,0073) VR=DS, VM=1 Time Slot Time
pub const TimeSlotTime: u32 = 0x00540073;

/// (0054,0080) VR=US, VM=1-n Slice Vector
pub const SliceVector: u32 = 0x00540080;

/// (0054,0081) VR=US, VM=1 Number of Slices
pub const NumberOfSlices: u32 = 0x00540081;

/// (0054,0090) VR=US, VM=1-n Angular View Vector
pub const AngularViewVector: u32 = 0x00540090;

/// (0054,0100) VR=US, VM=1-n Time Slice Vector
pub const TimeSliceVector: u32 = 0x00540100;

/// (0054,0101) VR=US, VM=1 Number of Time Slices
pub const NumberOfTimeSlices: u32 = 0x00540101;

/// (0054,0200) VR=DS, VM=1 Start Angle
pub const StartAngle: u32 = 0x00540200;

/// (0054,0202) VR=CS, VM=1 Type of Detector Motion
pub const TypeOfDetectorMotion: u32 = 0x00540202;

/// (0054,0210) VR=IS, VM=1-n Trigger Vector
pub const TriggerVector: u32 = 0x00540210;

/// (0054,0211) VR=US, VM=1 Number of Triggers in Phase
pub const NumberOfTriggersInPhase: u32 = 0x00540211;

/// (0054,0220) VR=SQ, VM=1 View Code Sequence
pub const ViewCodeSequence: u32 = 0x00540220;

/// (0054,0222) VR=SQ, VM=1 View Modifier Code Sequence
pub const ViewModifierCodeSequence: u32 = 0x00540222;

/// (0054,0300) VR=SQ, VM=1 Radionuclide Code Sequence
pub const RadionuclideCodeSequence: u32 = 0x00540300;

/// (0054,0302) VR=SQ, VM=1 Administration Route Code Sequence
pub const AdministrationRouteCodeSequence: u32 = 0x00540302;

/// (0054,0304) VR=SQ, VM=1 Radiopharmaceutical Code Sequence
pub const RadiopharmaceuticalCodeSequence: u32 = 0x00540304;

/// (0054,0306) VR=SQ, VM=1 Calibration Data Sequence
pub const CalibrationDataSequence: u32 = 0x00540306;

/// (0054,0308) VR=US, VM=1 Energy Window Number
pub const EnergyWindowNumber: u32 = 0x00540308;

/// (0054,0400) VR=SH, VM=1 Image ID
pub const ImageID: u32 = 0x00540400;

/// (0054,0410) VR=SQ, VM=1 Patient Orientation Code Sequence
pub const PatientOrientationCodeSequence: u32 = 0x00540410;

/// (0054,0412) VR=SQ, VM=1 Patient Orientation Modifier Code Sequence
pub const PatientOrientationModifierCodeSequence: u32 = 0x00540412;

/// (0054,0414) VR=SQ, VM=1 Patient Gantry Relationship Code Sequence
pub const PatientGantryRelationshipCodeSequence: u32 = 0x00540414;

/// (0054,0500) VR=CS, VM=1 Slice Progression Direction
pub const SliceProgressionDirection: u32 = 0x00540500;

/// (0054,1000) VR=CS, VM=2 Series Type
pub const SeriesType: u32 = 0x00541000;

/// (0054,1001) VR=CS, VM=1 Units
pub const Units: u32 = 0x00541001;

/// (0054,1002) VR=CS, VM=1 Counts Source
pub const CountsSource: u32 = 0x00541002;

/// (0054,1004) VR=CS, VM=1 Reprojection Method
pub const ReprojectionMethod: u32 = 0x00541004;

/// (0054,1006) VR=CS, VM=1 SUV Type
pub const SUVType: u32 = 0x00541006;

/// (0054,1100) VR=CS, VM=1 Randoms Correction Method
pub const RandomsCorrectionMethod: u32 = 0x00541100;

/// (0054,1101) VR=LO, VM=1 Attenuation Correction Method
pub const AttenuationCorrectionMethod: u32 = 0x00541101;

/// (0054,1102) VR=CS, VM=1 Decay Correction
pub const DecayCorrection: u32 = 0x00541102;

/// (0054,1103) VR=LO, VM=1 Reconstruction Method
pub const ReconstructionMethod: u32 = 0x00541103;

/// (0054,1104) VR=LO, VM=1 Detector Lines of Response Used
pub const DetectorLinesOfResponseUsed: u32 = 0x00541104;

/// (0054,1105) VR=LO, VM=1 Scatter Correction Method
pub const ScatterCorrectionMethod: u32 = 0x00541105;

/// (0054,1200) VR=DS, VM=1 Axial Acceptance
pub const AxialAcceptance: u32 = 0x00541200;

/// (0054,1201) VR=IS, VM=2 Axial Mash
pub const AxialMash: u32 = 0x00541201;

/// (0054,1202) VR=IS, VM=1 Transverse Mash
pub const TransverseMash: u32 = 0x00541202;

/// (0054,1203) VR=DS, VM=2 Detector Element Size
pub const DetectorElementSize: u32 = 0x00541203;

/// (0054,1210) VR=DS, VM=1 Coincidence Window Width
pub const CoincidenceWindowWidth: u32 = 0x00541210;

/// (0054,1220) VR=CS, VM=1-n Secondary Counts Type
pub const SecondaryCountsType: u32 = 0x00541220;

/// (0054,1300) VR=DS, VM=1 Frame Reference Time
pub const FrameReferenceTime: u32 = 0x00541300;

/// (0054,1310) VR=IS, VM=1 Primary (Prompts) Counts Accumulated
pub const PrimaryPromptsCountsAccumulated: u32 = 0x00541310;

/// (0054,1311) VR=IS, VM=1-n Secondary Counts Accumulated
pub const SecondaryCountsAccumulated: u32 = 0x00541311;

/// (0054,1320) VR=DS, VM=1 Slice Sensitivity Factor
pub const SliceSensitivityFactor: u32 = 0x00541320;

/// (0054,1321) VR=DS, VM=1 Decay Factor
pub const DecayFactor: u32 = 0x00541321;

/// (0054,1322) VR=DS, VM=1 Dose Calibration Factor
pub const DoseCalibrationFactor: u32 = 0x00541322;

/// (0054,1323) VR=DS, VM=1 Scatter Fraction Factor
pub const ScatterFractionFactor: u32 = 0x00541323;

/// (0054,1324) VR=DS, VM=1 Dead Time Factor
pub const DeadTimeFactor: u32 = 0x00541324;

/// (0054,1330) VR=US, VM=1 Image Index
pub const ImageIndex: u32 = 0x00541330;

/// (0054,1400) VR=CS, VM=1-n Counts Included (retired)
pub const CountsIncluded: u32 = 0x00541400;

/// (0054,1401) VR=CS, VM=1 Dead Time Correction Flag (retired)
pub const DeadTimeCorrectionFlag: u32 = 0x00541401;

/// (0060,3000) VR=SQ, VM=1 Histogram Sequence
pub const HistogramSequence: u32 = 0x00603000;

/// (0060,3002) VR=US, VM=1 Histogram Number of Bins
pub const HistogramNumberOfBins: u32 = 0x00603002;

/// (0060,3004) VR=US|SS, VM=1 Histogram First Bin Value
pub const HistogramFirstBinValue: u32 = 0x00603004;

/// (0060,3006) VR=US|SS, VM=1 Histogram Last Bin Value
pub const HistogramLastBinValue: u32 = 0x00603006;

/// (0060,3008) VR=US, VM=1 Histogram Bin Width
pub const HistogramBinWidth: u32 = 0x00603008;

/// (0060,3010) VR=LO, VM=1 Histogram Explanation
pub const HistogramExplanation: u32 = 0x00603010;

/// (0060,3020) VR=UL, VM=1-n Histogram Data
pub const HistogramData: u32 = 0x00603020;

/// (0062,0001) VR=CS, VM=1 Segmentation Type
pub const SegmentationType: u32 = 0x00620001;

/// (0062,0002) VR=SQ, VM=1 Segment Sequence
pub const SegmentSequence: u32 = 0x00620002;

/// (0062,0003) VR=SQ, VM=1 Segmented Property Category Code Sequence
pub const SegmentedPropertyCategoryCodeSequence: u32 = 0x00620003;

/// (0062,0004) VR=US, VM=1 Segment Number
pub const SegmentNumber: u32 = 0x00620004;

/// (0062,0005) VR=LO, VM=1 Segment Label
pub const SegmentLabel: u32 = 0x00620005;

/// (0062,0006) VR=ST, VM=1 Segment Description
pub const SegmentDescription: u32 = 0x00620006;

/// (0062,0008) VR=CS, VM=1 Segment Algorithm Type
pub const SegmentAlgorithmType: u32 = 0x00620008;

/// (0062,0009) VR=LO, VM=1 Segment Algorithm Name
pub const SegmentAlgorithmName: u32 = 0x00620009;

/// (0062,000A) VR=SQ, VM=1 Segment Identification Sequence
pub const SegmentIdentificationSequence: u32 = 0x0062000A;

/// (0062,000B) VR=US, VM=1-n Referenced Segment Number
pub const ReferencedSegmentNumber: u32 = 0x0062000B;

/// (0062,000C) VR=US, VM=1 Recommended Display Grayscale Value
pub const RecommendedDisplayGrayscaleValue: u32 = 0x0062000C;

/// (0062,000D) VR=US, VM=3 Recommended Display CIELab Value
pub const RecommendedDisplayCIELabValue: u32 = 0x0062000D;

/// (0062,000E) VR=US, VM=1 Maximum Fractional Value
pub const MaximumFractionalValue: u32 = 0x0062000E;

/// (0062,000F) VR=SQ, VM=1 Segmented Property Type Code Sequence
pub const SegmentedPropertyTypeCodeSequence: u32 = 0x0062000F;

/// (0062,0010) VR=CS, VM=1 Segmentation Fractional Type
pub const SegmentationFractionalType: u32 = 0x00620010;

/// (0064,0002) VR=SQ, VM=1 Deformable Registration Sequence
pub const DeformableRegistrationSequence: u32 = 0x00640002;

/// (0064,0003) VR=UI, VM=1 Source Frame of Reference UID
pub const SourceFrameOfReferenceUID: u32 = 0x00640003;

/// (0064,0005) VR=SQ, VM=1 Deformable Registration Grid Sequence
pub const DeformableRegistrationGridSequence: u32 = 0x00640005;

/// (0064,0007) VR=UL, VM=3 Grid Dimensions
pub const GridDimensions: u32 = 0x00640007;

/// (0064,0008) VR=FD, VM=3 Grid Resolution
pub const GridResolution: u32 = 0x00640008;

/// (0064,0009) VR=OF, VM=1 Vector Grid Data
pub const VectorGridData: u32 = 0x00640009;

/// (0064,000F) VR=SQ, VM=1 Pre Deformation Matrix Registration Sequence
pub const PreDeformationMatrixRegistrationSequence: u32 = 0x0064000F;

/// (0064,0010) VR=SQ, VM=1 Post Deformation Matrix Registration Sequence
pub const PostDeformationMatrixRegistrationSequence: u32 = 0x00640010;

/// (0066,0001) VR=UL, VM=1 Number of Surfaces
pub const NumberOfSurfaces: u32 = 0x00660001;

/// (0066,0002) VR=SQ, VM=1 Surface Sequence
pub const SurfaceSequence: u32 = 0x00660002;

/// (0066,0003) VR=UL, VM=1 Surface Number
pub const SurfaceNumber: u32 = 0x00660003;

/// (0066,0004) VR=LT, VM=1 Surface Comments
pub const SurfaceComments: u32 = 0x00660004;

/// (0066,0009) VR=CS, VM=1 Surface Processing
pub const SurfaceProcessing: u32 = 0x00660009;

/// (0066,000A) VR=FL, VM=1 Surface Processing Ratio
pub const SurfaceProcessingRatio: u32 = 0x0066000A;

/// (0066,000B) VR=LO, VM=1 Surface Processing Description
pub const SurfaceProcessingDescription: u32 = 0x0066000B;

/// (0066,000C) VR=FL, VM=1 Recommended Presentation Opacity
pub const RecommendedPresentationOpacity: u32 = 0x0066000C;

/// (0066,000D) VR=CS, VM=1 Recommended Presentation Type
pub const RecommendedPresentationType: u32 = 0x0066000D;

/// (0066,000E) VR=CS, VM=1 Finite Volume
pub const FiniteVolume: u32 = 0x0066000E;

/// (0066,0010) VR=CS, VM=1 Manifold
pub const Manifold: u32 = 0x00660010;

/// (0066,0011) VR=SQ, VM=1 Surface Points Sequence
pub const SurfacePointsSequence: u32 = 0x00660011;

/// (0066,0012) VR=SQ, VM=1 Surface Points Normals Sequence
pub const SurfacePointsNormalsSequence: u32 = 0x00660012;

/// (0066,0013) VR=SQ, VM=1 Surface Mesh Primitives Sequence
pub const SurfaceMeshPrimitivesSequence: u32 = 0x00660013;

/// (0066,0015) VR=UL, VM=1 Number of Surface Points
pub const NumberOfSurfacePoints: u32 = 0x00660015;

/// (0066,0016) VR=OF, VM=1 Point Coordinates Data
pub const PointCoordinatesData: u32 = 0x00660016;

/// (0066,0017) VR=FL, VM=3 Point Position Accuracy
pub const PointPositionAccuracy: u32 = 0x00660017;

/// (0066,0018) VR=FL, VM=1 Mean Point Distance
pub const MeanPointDistance: u32 = 0x00660018;

/// (0066,0019) VR=FL, VM=1 Maximum Point Distance
pub const MaximumPointDistance: u32 = 0x00660019;

/// (0066,001A) VR=FL, VM=6 Points Bounding Box Coordinates
pub const PointsBoundingBoxCoordinates: u32 = 0x0066001A;

/// (0066,001B) VR=FL, VM=3 Axis of Rotation
pub const AxisOfRotation: u32 = 0x0066001B;

/// (0066,001C) VR=FL, VM=3 Center of Rotation
pub const CenterOfRotation: u32 = 0x0066001C;

/// (0066,001E) VR=UL, VM=1 Number of Vectors
pub const NumberOfVectors: u32 = 0x0066001E;

/// (0066,001F) VR=US, VM=1 Vector Dimensionality
pub const VectorDimensionality: u32 = 0x0066001F;

/// (0066,0020) VR=FL, VM=1-n Vector Accuracy
pub const VectorAccuracy: u32 = 0x00660020;

/// (0066,0021) VR=OF, VM=1 Vector Coordinate Data
pub const VectorCoordinateData: u32 = 0x00660021;

/// (0066,0023) VR=OW, VM=1 Triangle Point Index List
pub const TrianglePointIndexList: u32 = 0x00660023;

/// (0066,0024) VR=OW, VM=1 Edge Point Index List
pub const EdgePointIndexList: u32 = 0x00660024;

/// (0066,0025) VR=OW, VM=1 Vertex Point Index List
pub const VertexPointIndexList: u32 = 0x00660025;

/// (0066,0026) VR=SQ, VM=1 Triangle Strip Sequence
pub const TriangleStripSequence: u32 = 0x00660026;

/// (0066,0027) VR=SQ, VM=1 Triangle Fan Sequence
pub const TriangleFanSequence: u32 = 0x00660027;

/// (0066,0028) VR=SQ, VM=1 Line Sequence
pub const LineSequence: u32 = 0x00660028;

/// (0066,0029) VR=OW, VM=1 Primitive Point Index List
pub const PrimitivePointIndexList: u32 = 0x00660029;

/// (0066,002A) VR=UL, VM=1 Surface Count
pub const SurfaceCount: u32 = 0x0066002A;

/// (0066,002B) VR=SQ, VM=1 Referenced Surface Sequence
pub const ReferencedSurfaceSequence: u32 = 0x0066002B;

/// (0066,002C) VR=UL, VM=1 Referenced Surface Number
pub const ReferencedSurfaceNumber: u32 = 0x0066002C;

/// (0066,002D) VR=SQ, VM=1 Segment Surface Generation Algorithm Identification Sequence
pub const SegmentSurfaceGenerationAlgorithmIdentificationSequence: u32 = 0x0066002D;

/// (0066,002E) VR=SQ, VM=1 Segment Surface Source Instance Sequence
pub const SegmentSurfaceSourceInstanceSequence: u32 = 0x0066002E;

/// (0066,002F) VR=SQ, VM=1 Algorithm Family Code Sequence
pub const AlgorithmFamilyCodeSequence: u32 = 0x0066002F;

/// (0066,0030) VR=SQ, VM=1 Algorithm Name Code Sequence
pub const AlgorithmNameCodeSequence: u32 = 0x00660030;

/// (0066,0031) VR=LO, VM=1 Algorithm Version
pub const AlgorithmVersion: u32 = 0x00660031;

/// (0066,0032) VR=LT, VM=1 Algorithm Parameters
pub const AlgorithmParameters: u32 = 0x00660032;

/// (0066,0034) VR=SQ, VM=1 Facet Sequence
pub const FacetSequence: u32 = 0x00660034;

/// (0066,0035) VR=SQ, VM=1 Surface Processing Algorithm Identification Sequence
pub const SurfaceProcessingAlgorithmIdentificationSequence: u32 = 0x00660035;

/// (0066,0036) VR=LO, VM=1 Algorithm Name
pub const AlgorithmName: u32 = 0x00660036;

/// (0068,6210) VR=LO, VM=1 Implant Size
pub const ImplantSize: u32 = 0x00686210;

/// (0068,6221) VR=LO, VM=1 Implant Template Version
pub const ImplantTemplateVersion: u32 = 0x00686221;

/// (0068,6222) VR=SQ, VM=1 Replaced Implant Template Sequence
pub const ReplacedImplantTemplateSequence: u32 = 0x00686222;

/// (0068,6223) VR=CS, VM=1 Implant Type
pub const ImplantType: u32 = 0x00686223;

/// (0068,6224) VR=SQ, VM=1 Derivation Implant Template Sequence
pub const DerivationImplantTemplateSequence: u32 = 0x00686224;

/// (0068,6225) VR=SQ, VM=1 Original Implant Template Sequence
pub const OriginalImplantTemplateSequence: u32 = 0x00686225;

/// (0068,6226) VR=DT, VM=1 Effective DateTime
pub const EffectiveDateTime: u32 = 0x00686226;

/// (0068,6230) VR=SQ, VM=1 Implant Target Anatomy Sequence
pub const ImplantTargetAnatomySequence: u32 = 0x00686230;

/// (0068,6260) VR=SQ, VM=1 Information From Manufacturer Sequence
pub const InformationFromManufacturerSequence: u32 = 0x00686260;

/// (0068,6265) VR=SQ, VM=1 Notification From Manufacturer Sequence
pub const NotificationFromManufacturerSequence: u32 = 0x00686265;

/// (0068,6270) VR=DT, VM=1 Information Issue DateTime
pub const InformationIssueDateTime: u32 = 0x00686270;

/// (0068,6280) VR=ST, VM=1 Information Summary
pub const InformationSummary: u32 = 0x00686280;

/// (0068,62A0) VR=SQ, VM=1 Implant Regulatory Disapproval Code Sequence
pub const ImplantRegulatoryDisapprovalCodeSequence: u32 = 0x006862A0;

/// (0068,62A5) VR=FD, VM=1 Overall Template Spatial Tolerance
pub const OverallTemplateSpatialTolerance: u32 = 0x006862A5;

/// (0068,62C0) VR=SQ, VM=1 HPGL Document Sequence
pub const HPGLDocumentSequence: u32 = 0x006862C0;

/// (0068,62D0) VR=US, VM=1 HPGL Document ID
pub const HPGLDocumentID: u32 = 0x006862D0;

/// (0068,62D5) VR=LO, VM=1 HPGL Document Label
pub const HPGLDocumentLabel: u32 = 0x006862D5;

/// (0068,62E0) VR=SQ, VM=1 View Orientation Code Sequence
pub const ViewOrientationCodeSequence: u32 = 0x006862E0;

/// (0068,62F0) VR=FD, VM=9 View Orientation Modifier
pub const ViewOrientationModifier: u32 = 0x006862F0;

/// (0068,62F2) VR=FD, VM=1 HPGL Document Scaling
pub const HPGLDocumentScaling: u32 = 0x006862F2;

/// (0068,6300) VR=OB, VM=1 HPGL Document
pub const HPGLDocument: u32 = 0x00686300;

/// (0068,6310) VR=US, VM=1 HPGL Contour Pen Number
pub const HPGLContourPenNumber: u32 = 0x00686310;

/// (0068,6320) VR=SQ, VM=1 HPGL Pen Sequence
pub const HPGLPenSequence: u32 = 0x00686320;

/// (0068,6330) VR=US, VM=1 HPGL Pen Number
pub const HPGLPenNumber: u32 = 0x00686330;

/// (0068,6340) VR=LO, VM=1 HPGL Pen Label
pub const HPGLPenLabel: u32 = 0x00686340;

/// (0068,6345) VR=ST, VM=1 HPGL Pen Description
pub const HPGLPenDescription: u32 = 0x00686345;

/// (0068,6346) VR=FD, VM=2 Recommended Rotation Point
pub const RecommendedRotationPoint: u32 = 0x00686346;

/// (0068,6347) VR=FD, VM=4 Bounding Rectangle
pub const BoundingRectangle: u32 = 0x00686347;

/// (0068,6350) VR=US, VM=1-n Implant Template 3D Model Surface Number
pub const ImplantTemplate3DModelSurfaceNumber: u32 = 0x00686350;

/// (0068,6360) VR=SQ, VM=1 Surface Model Description Sequence
pub const SurfaceModelDescriptionSequence: u32 = 0x00686360;

/// (0068,6380) VR=LO, VM=1 Surface Model Label
pub const SurfaceModelLabel: u32 = 0x00686380;

/// (0068,6390) VR=FD, VM=1 Surface Model Scaling Factor
pub const SurfaceModelScalingFactor: u32 = 0x00686390;

/// (0068,63A0) VR=SQ, VM=1 Materials Code Sequence
pub const MaterialsCodeSequence: u32 = 0x006863A0;

/// (0068,63A4) VR=SQ, VM=1 Coating Materials Code Sequence
pub const CoatingMaterialsCodeSequence: u32 = 0x006863A4;

/// (0068,63A8) VR=SQ, VM=1 Implant Type Code Sequence
pub const ImplantTypeCodeSequence: u32 = 0x006863A8;

/// (0068,63AC) VR=SQ, VM=1 Fixation Method Code Sequence
pub const FixationMethodCodeSequence: u32 = 0x006863AC;

/// (0068,63B0) VR=SQ, VM=1 Mating Feature Sets Sequence
pub const MatingFeatureSetsSequence: u32 = 0x006863B0;

/// (0068,63C0) VR=US, VM=1 Mating Feature Set ID
pub const MatingFeatureSetID: u32 = 0x006863C0;

/// (0068,63D0) VR=LO, VM=1 Mating Feature Set Label
pub const MatingFeatureSetLabel: u32 = 0x006863D0;

/// (0068,63E0) VR=SQ, VM=1 Mating Feature Sequence
pub const MatingFeatureSequence: u32 = 0x006863E0;

/// (0068,63F0) VR=US, VM=1 Mating Feature ID
pub const MatingFeatureID: u32 = 0x006863F0;

/// (0068,6400) VR=SQ, VM=1 Mating Feature Degree of Freedom Sequence
pub const MatingFeatureDegreeOfFreedomSequence: u32 = 0x00686400;

/// (0068,6410) VR=US, VM=1 Degree of Freedom ID
pub const DegreeOfFreedomID: u32 = 0x00686410;

/// (0068,6420) VR=CS, VM=1 Degree of Freedom Type
pub const DegreeOfFreedomType: u32 = 0x00686420;

/// (0068,6430) VR=SQ, VM=1 2D Mating Feature Coordinates Sequence
pub const TwoDMatingFeatureCoordinatesSequence: u32 = 0x00686430;

/// (0068,6440) VR=US, VM=1 Referenced HPGL Document ID
pub const ReferencedHPGLDocumentID: u32 = 0x00686440;

/// (0068,6450) VR=FD, VM=2 2D Mating Point
pub const TwoDMatingPoint: u32 = 0x00686450;

/// (0068,6460) VR=FD, VM=4 2D Mating Axes
pub const TwoDMatingAxes: u32 = 0x00686460;

/// (0068,6470) VR=SQ, VM=1 2D Degree of Freedom Sequence
pub const TwoDDegreeOfFreedomSequence: u32 = 0x00686470;

/// (0068,6490) VR=FD, VM=3 3D Degree of Freedom Axis
pub const ThreeDDegreeOfFreedomAxis: u32 = 0x00686490;

/// (0068,64A0) VR=FD, VM=2 Range of Freedom
pub const RangeOfFreedom: u32 = 0x006864A0;

/// (0068,64C0) VR=FD, VM=3 3D Mating Point
pub const ThreeDMatingPoint: u32 = 0x006864C0;

/// (0068,64D0) VR=FD, VM=9 3D Mating Axes
pub const ThreeDMatingAxes: u32 = 0x006864D0;

/// (0068,64F0) VR=FD, VM=3 2D Degree of Freedom Axis
pub const TwoDDegreeOfFreedomAxis: u32 = 0x006864F0;

/// (0068,6500) VR=SQ, VM=1 Planning Landmark Point Sequence
pub const PlanningLandmarkPointSequence: u32 = 0x00686500;

/// (0068,6510) VR=SQ, VM=1 Planning Landmark Line Sequence
pub const PlanningLandmarkLineSequence: u32 = 0x00686510;

/// (0068,6520) VR=SQ, VM=1 Planning Landmark Plane Sequence
pub const PlanningLandmarkPlaneSequence: u32 = 0x00686520;

/// (0068,6530) VR=US, VM=1 Planning Landmark ID
pub const PlanningLandmarkID: u32 = 0x00686530;

/// (0068,6540) VR=LO, VM=1 Planning Landmark Description
pub const PlanningLandmarkDescription: u32 = 0x00686540;

/// (0068,6545) VR=SQ, VM=1 Planning Landmark Identification Code Sequence
pub const PlanningLandmarkIdentificationCodeSequence: u32 = 0x00686545;

/// (0068,6550) VR=SQ, VM=1 2D Point Coordinates Sequence
pub const TwoDPointCoordinatesSequence: u32 = 0x00686550;

/// (0068,6560) VR=FD, VM=2 2D Point Coordinates
pub const TwoDPointCoordinates: u32 = 0x00686560;

/// (0068,6590) VR=FD, VM=3 3D Point Coordinates
pub const ThreeDPointCoordinates: u32 = 0x00686590;

/// (0068,65A0) VR=SQ, VM=1 2D Line Coordinates Sequence
pub const TwoDLineCoordinatesSequence: u32 = 0x006865A0;

/// (0068,65B0) VR=FD, VM=4 2D Line Coordinates
pub const TwoDLineCoordinates: u32 = 0x006865B0;

/// (0068,65D0) VR=FD, VM=6 3D Line Coordinates
pub const ThreeDLineCoordinates: u32 = 0x006865D0;

/// (0068,65E0) VR=SQ, VM=1 2D Plane Coordinates Sequence
pub const TwoDPlaneCoordinatesSequence: u32 = 0x006865E0;

/// (0068,65F0) VR=FD, VM=4 2D Plane Intersection
pub const TwoDPlaneIntersection: u32 = 0x006865F0;

/// (0068,6610) VR=FD, VM=3 3D Plane Origin
pub const ThreeDPlaneOrigin: u32 = 0x00686610;

/// (0068,6620) VR=FD, VM=3 3D Plane Normal
pub const ThreeDPlaneNormal: u32 = 0x00686620;

/// (0070,0001) VR=SQ, VM=1 Graphic Annotation Sequence
pub const GraphicAnnotationSequence: u32 = 0x00700001;

/// (0070,0002) VR=CS, VM=1 Graphic Layer
pub const GraphicLayer: u32 = 0x00700002;

/// (0070,0003) VR=CS, VM=1 Bounding Box Annotation Units
pub const BoundingBoxAnnotationUnits: u32 = 0x00700003;

/// (0070,0004) VR=CS, VM=1 Anchor Point Annotation Units
pub const AnchorPointAnnotationUnits: u32 = 0x00700004;

/// (0070,0005) VR=CS, VM=1 Graphic Annotation Units
pub const GraphicAnnotationUnits: u32 = 0x00700005;

/// (0070,0006) VR=ST, VM=1 Unformatted Text Value
pub const UnformattedTextValue: u32 = 0x00700006;

/// (0070,0008) VR=SQ, VM=1 Text Object Sequence
pub const TextObjectSequence: u32 = 0x00700008;

/// (0070,0009) VR=SQ, VM=1 Graphic Object Sequence
pub const GraphicObjectSequence: u32 = 0x00700009;

/// (0070,0010) VR=FL, VM=2 Bounding Box Top Left Hand Corner
pub const BoundingBoxTopLeftHandCorner: u32 = 0x00700010;

/// (0070,0011) VR=FL, VM=2 Bounding Box Bottom Right Hand Corner
pub const BoundingBoxBottomRightHandCorner: u32 = 0x00700011;

/// (0070,0012) VR=CS, VM=1 Bounding Box Text Horizontal Justification
pub const BoundingBoxTextHorizontalJustification: u32 = 0x00700012;

/// (0070,0014) VR=FL, VM=2 Anchor Point
pub const AnchorPoint: u32 = 0x00700014;

/// (0070,0015) VR=CS, VM=1 Anchor Point Visibility
pub const AnchorPointVisibility: u32 = 0x00700015;

/// (0070,0020) VR=US, VM=1 Graphic Dimensions
pub const GraphicDimensions: u32 = 0x00700020;

/// (0070,0021) VR=US, VM=1 Number of Graphic Points
pub const NumberOfGraphicPoints: u32 = 0x00700021;

/// (0070,0022) VR=FL, VM=2-n Graphic Data
pub const GraphicData: u32 = 0x00700022;

/// (0070,0023) VR=CS, VM=1 Graphic Type
pub const GraphicType: u32 = 0x00700023;

/// (0070,0024) VR=CS, VM=1 Graphic Filled
pub const GraphicFilled: u32 = 0x00700024;

/// (0070,0040) VR=IS, VM=1 Image Rotation (Retired) (retired)
pub const ImageRotationRetired: u32 = 0x00700040;

/// (0070,0041) VR=CS, VM=1 Image Horizontal Flip
pub const ImageHorizontalFlip: u32 = 0x00700041;

/// (0070,0042) VR=US, VM=1 Image Rotation
pub const ImageRotation: u32 = 0x00700042;

/// (0070,0050) VR=US, VM=2 Displayed Area Top Left Hand Corner (Trial) (retired)
pub const DisplayedAreaTopLeftHandCornerTrial: u32 = 0x00700050;

/// (0070,0051) VR=US, VM=2 Displayed Area Bottom Right Hand Corner (Trial) (retired)
pub const DisplayedAreaBottomRightHandCornerTrial: u32 = 0x00700051;

/// (0070,0052) VR=SL, VM=2 Displayed Area Top Left Hand Corner
pub const DisplayedAreaTopLeftHandCorner: u32 = 0x00700052;

/// (0070,0053) VR=SL, VM=2 Displayed Area Bottom Right Hand Corner
pub const DisplayedAreaBottomRightHandCorner: u32 = 0x00700053;

/// (0070,005A) VR=SQ, VM=1 Displayed Area Selection Sequence
pub const DisplayedAreaSelectionSequence: u32 = 0x0070005A;

/// (0070,0060) VR=SQ, VM=1 Graphic Layer Sequence
pub const GraphicLayerSequence: u32 = 0x00700060;

/// (0070,0062) VR=IS, VM=1 Graphic Layer Order
pub const GraphicLayerOrder: u32 = 0x00700062;

/// (0070,0066) VR=US, VM=1 Graphic Layer Recommended Display Grayscale Value
pub const GraphicLayerRecommendedDisplayGrayscaleValue: u32 = 0x00700066;

/// (0070,0067) VR=US, VM=3 Graphic Layer Recommended Display RGB Value (retired)
pub const GraphicLayerRecommendedDisplayRGBValue: u32 = 0x00700067;

/// (0070,0068) VR=LO, VM=1 Graphic Layer Description
pub const GraphicLayerDescription: u32 = 0x00700068;

/// (0070,0080) VR=CS, VM=1 Content Label
pub const ContentLabel: u32 = 0x00700080;

/// (0070,0081) VR=LO, VM=1 Content Description
pub const ContentDescription: u32 = 0x00700081;

/// (0070,0082) VR=DA, VM=1 Presentation Creation Date
pub const PresentationCreationDate: u32 = 0x00700082;

/// (0070,0083) VR=TM, VM=1 Presentation Creation Time
pub const PresentationCreationTime: u32 = 0x00700083;

/// (0070,0084) VR=PN, VM=1 Content Creator’s Name
pub const ContentCreatorName: u32 = 0x00700084;

/// (0070,0086) VR=SQ, VM=1 Content Creator’s Identification Code Sequence
pub const ContentCreatorIdentificationCodeSequence: u32 = 0x00700086;

/// (0070,0087) VR=SQ, VM=1 Alternate Content Description Sequence
pub const AlternateContentDescriptionSequence: u32 = 0x00700087;

/// (0070,0100) VR=CS, VM=1 Presentation Size Mode
pub const PresentationSizeMode: u32 = 0x00700100;

/// (0070,0101) VR=DS, VM=2 Presentation Pixel Spacing
pub const PresentationPixelSpacing: u32 = 0x00700101;

/// (0070,0102) VR=IS, VM=2 Presentation Pixel Aspect Ratio
pub const PresentationPixelAspectRatio: u32 = 0x00700102;

/// (0070,0103) VR=FL, VM=1 Presentation Pixel Magnification Ratio
pub const PresentationPixelMagnificationRatio: u32 = 0x00700103;

/// (0070,0207) VR=LO, VM=1 Graphic Group Label
pub const GraphicGroupLabel: u32 = 0x00700207;

/// (0070,0208) VR=ST, VM=1 Graphic Group Description
pub const GraphicGroupDescription: u32 = 0x00700208;

/// (0070,0209) VR=SQ, VM=1 Compound Graphic Sequence
pub const CompoundGraphicSequence: u32 = 0x00700209;

/// (0070,0226) VR=UL, VM=1 Compound Graphic Instance ID
pub const CompoundGraphicInstanceID: u32 = 0x00700226;

/// (0070,0227) VR=LO, VM=1 Font Name
pub const FontName: u32 = 0x00700227;

/// (0070,0228) VR=CS, VM=1 Font Name Type
pub const FontNameType: u32 = 0x00700228;

/// (0070,0229) VR=LO, VM=1 CSS Font Name
pub const CSSFontName: u32 = 0x00700229;

/// (0070,0230) VR=FD, VM=1 Rotation Angle
pub const RotationAngle: u32 = 0x00700230;

/// (0070,0231) VR=SQ, VM=1 Text Style Sequence
pub const TextStyleSequence: u32 = 0x00700231;

/// (0070,0232) VR=SQ, VM=1 Line Style Sequence
pub const LineStyleSequence: u32 = 0x00700232;

/// (0070,0233) VR=SQ, VM=1 Fill Style Sequence
pub const FillStyleSequence: u32 = 0x00700233;

/// (0070,0234) VR=SQ, VM=1 Graphic Group Sequence
pub const GraphicGroupSequence: u32 = 0x00700234;

/// (0070,0241) VR=US, VM=3 Text Color CIELab Value
pub const TextColorCIELabValue: u32 = 0x00700241;

/// (0070,0242) VR=CS, VM=1 Horizontal Alignment
pub const HorizontalAlignment: u32 = 0x00700242;

/// (0070,0243) VR=CS, VM=1 Vertical Alignment
pub const VerticalAlignment: u32 = 0x00700243;

/// (0070,0244) VR=CS, VM=1 Shadow Style
pub const ShadowStyle: u32 = 0x00700244;

/// (0070,0245) VR=FL, VM=1 Shadow Offset X
pub const ShadowOffsetX: u32 = 0x00700245;

/// (0070,0246) VR=FL, VM=1 Shadow Offset Y
pub const ShadowOffsetY: u32 = 0x00700246;

/// (0070,0247) VR=US, VM=3 Shadow Color CIELab Value
pub const ShadowColorCIELabValue: u32 = 0x00700247;

/// (0070,0248) VR=CS, VM=1 Underlined
pub const Underlined: u32 = 0x00700248;

/// (0070,0249) VR=CS, VM=1 Bold
pub const Bold: u32 = 0x00700249;

/// (0070,0250) VR=CS, VM=1 Italic
pub const Italic: u32 = 0x00700250;

/// (0070,0251) VR=US, VM=3 Pattern On Color CIELab Value
pub const PatternOnColorCIELabValue: u32 = 0x00700251;

/// (0070,0252) VR=US, VM=3 Pattern Off Color CIELab Value
pub const PatternOffColorCIELabValue: u32 = 0x00700252;

/// (0070,0253) VR=FL, VM=1 Line Thickness
pub const LineThickness: u32 = 0x00700253;

/// (0070,0254) VR=CS, VM=1 Line Dashing Style
pub const LineDashingStyle: u32 = 0x00700254;

/// (0070,0255) VR=UL, VM=1 Line Pattern
pub const LinePattern: u32 = 0x00700255;

/// (0070,0256) VR=OB, VM=1 Fill Pattern
pub const FillPattern: u32 = 0x00700256;

/// (0070,0257) VR=CS, VM=1 Fill Mode
pub const FillMode: u32 = 0x00700257;

/// (0070,0258) VR=FL, VM=1 Shadow Opacity
pub const ShadowOpacity: u32 = 0x00700258;

/// (0070,0261) VR=FL, VM=1 Gap Length
pub const GapLength: u32 = 0x00700261;

/// (0070,0262) VR=FL, VM=1 Diameter of Visibility
pub const DiameterOfVisibility: u32 = 0x00700262;

/// (0070,0273) VR=FL, VM=2 Rotation Point
pub const RotationPoint: u32 = 0x00700273;

/// (0070,0274) VR=CS, VM=1 Tick Alignment
pub const TickAlignment: u32 = 0x00700274;

/// (0070,0278) VR=CS, VM=1 Show Tick Label
pub const ShowTickLabel: u32 = 0x00700278;

/// (0070,0279) VR=CS, VM=1 Tick Label Alignment
pub const TickLabelAlignment: u32 = 0x00700279;

/// (0070,0282) VR=CS, VM=1 Compound Graphic Units
pub const CompoundGraphicUnits: u32 = 0x00700282;

/// (0070,0284) VR=FL, VM=1 Pattern On Opacity
pub const PatternOnOpacity: u32 = 0x00700284;

/// (0070,0285) VR=FL, VM=1 Pattern Off Opacity
pub const PatternOffOpacity: u32 = 0x00700285;

/// (0070,0287) VR=SQ, VM=1 Major Ticks Sequence
pub const MajorTicksSequence: u32 = 0x00700287;

/// (0070,0288) VR=FL, VM=1 Tick Position
pub const TickPosition: u32 = 0x00700288;

/// (0070,0289) VR=SH, VM=1 Tick Label
pub const TickLabel: u32 = 0x00700289;

/// (0070,0294) VR=CS, VM=1 Compound Graphic Type
pub const CompoundGraphicType: u32 = 0x00700294;

/// (0070,0295) VR=UL, VM=1 Graphic Group ID
pub const GraphicGroupID: u32 = 0x00700295;

/// (0070,0306) VR=CS, VM=1 Shape Type
pub const ShapeType: u32 = 0x00700306;

/// (0070,0308) VR=SQ, VM=1 Registration Sequence
pub const RegistrationSequence: u32 = 0x00700308;

/// (0070,0309) VR=SQ, VM=1 Matrix Registration Sequence
pub const MatrixRegistrationSequence: u32 = 0x00700309;

/// (0070,030A) VR=SQ, VM=1 Matrix Sequence
pub const MatrixSequence: u32 = 0x0070030A;

/// (0070,030C) VR=CS, VM=1 Frame of Reference Transformation Matrix Type
pub const FrameOfReferenceTransformationMatrixType: u32 = 0x0070030C;

/// (0070,030D) VR=SQ, VM=1 Registration Type Code Sequence
pub const RegistrationTypeCodeSequence: u32 = 0x0070030D;

/// (0070,030F) VR=ST, VM=1 Fiducial Description
pub const FiducialDescription: u32 = 0x0070030F;

/// (0070,0310) VR=SH, VM=1 Fiducial Identifier
pub const FiducialIdentifier: u32 = 0x00700310;

/// (0070,0311) VR=SQ, VM=1 Fiducial Identifier Code Sequence
pub const FiducialIdentifierCodeSequence: u32 = 0x00700311;

/// (0070,0312) VR=FD, VM=1 Contour Uncertainty Radius
pub const ContourUncertaintyRadius: u32 = 0x00700312;

/// (0070,0314) VR=SQ, VM=1 Used Fiducials Sequence
pub const UsedFiducialsSequence: u32 = 0x00700314;

/// (0070,0318) VR=SQ, VM=1 Graphic Coordinates Data Sequence
pub const GraphicCoordinatesDataSequence: u32 = 0x00700318;

/// (0070,031A) VR=UI, VM=1 Fiducial UID
pub const FiducialUID: u32 = 0x0070031A;

/// (0070,031C) VR=SQ, VM=1 Fiducial Set Sequence
pub const FiducialSetSequence: u32 = 0x0070031C;

/// (0070,031E) VR=SQ, VM=1 Fiducial Sequence
pub const FiducialSequence: u32 = 0x0070031E;

/// (0070,0401) VR=US, VM=3 Graphic Layer Recommended Display CIELab Value
pub const GraphicLayerRecommendedDisplayCIELabValue: u32 = 0x00700401;

/// (0070,0402) VR=SQ, VM=1 Blending Sequence
pub const BlendingSequence: u32 = 0x00700402;

/// (0070,0403) VR=FL, VM=1 Relative Opacity
pub const RelativeOpacity: u32 = 0x00700403;

/// (0070,0404) VR=SQ, VM=1 Referenced Spatial Registration Sequence
pub const ReferencedSpatialRegistrationSequence: u32 = 0x00700404;

/// (0070,0405) VR=CS, VM=1 Blending Position
pub const BlendingPosition: u32 = 0x00700405;

/// (0072,0002) VR=SH, VM=1 Hanging Protocol Name
pub const HangingProtocolName: u32 = 0x00720002;

/// (0072,0004) VR=LO, VM=1 Hanging Protocol Description
pub const HangingProtocolDescription: u32 = 0x00720004;

/// (0072,0006) VR=CS, VM=1 Hanging Protocol Level
pub const HangingProtocolLevel: u32 = 0x00720006;

/// (0072,0008) VR=LO, VM=1 Hanging Protocol Creator
pub const HangingProtocolCreator: u32 = 0x00720008;

/// (0072,000A) VR=DT, VM=1 Hanging Protocol Creation DateTime
pub const HangingProtocolCreationDateTime: u32 = 0x0072000A;

/// (0072,000C) VR=SQ, VM=1 Hanging Protocol Definition Sequence
pub const HangingProtocolDefinitionSequence: u32 = 0x0072000C;

/// (0072,000E) VR=SQ, VM=1 Hanging Protocol User Identification Code Sequence
pub const HangingProtocolUserIdentificationCodeSequence: u32 = 0x0072000E;

/// (0072,0010) VR=LO, VM=1 Hanging Protocol User Group Name
pub const HangingProtocolUserGroupName: u32 = 0x00720010;

/// (0072,0012) VR=SQ, VM=1 Source Hanging Protocol Sequence
pub const SourceHangingProtocolSequence: u32 = 0x00720012;

/// (0072,0014) VR=US, VM=1 Number of Priors Referenced
pub const NumberOfPriorsReferenced: u32 = 0x00720014;

/// (0072,0020) VR=SQ, VM=1 Image Sets Sequence
pub const ImageSetsSequence: u32 = 0x00720020;

/// (0072,0022) VR=SQ, VM=1 Image Set Selector Sequence
pub const ImageSetSelectorSequence: u32 = 0x00720022;

/// (0072,0024) VR=CS, VM=1 Image Set Selector Usage Flag
pub const ImageSetSelectorUsageFlag: u32 = 0x00720024;

/// (0072,0026) VR=AT, VM=1 Selector Attribute
pub const SelectorAttribute: u32 = 0x00720026;

/// (0072,0028) VR=US, VM=1 Selector Value Number
pub const SelectorValueNumber: u32 = 0x00720028;

/// (0072,0030) VR=SQ, VM=1 Time Based Image Sets Sequence
pub const TimeBasedImageSetsSequence: u32 = 0x00720030;

/// (0072,0032) VR=US, VM=1 Image Set Number
pub const ImageSetNumber: u32 = 0x00720032;

/// (0072,0034) VR=CS, VM=1 Image Set Selector Category
pub const ImageSetSelectorCategory: u32 = 0x00720034;

/// (0072,0038) VR=US, VM=2 Relative Time
pub const RelativeTime: u32 = 0x00720038;

/// (0072,003A) VR=CS, VM=1 Relative Time Units
pub const RelativeTimeUnits: u32 = 0x0072003A;

/// (0072,003C) VR=SS, VM=2 Abstract Prior Value
pub const AbstractPriorValue: u32 = 0x0072003C;

/// (0072,003E) VR=SQ, VM=1 Abstract Prior Code Sequence
pub const AbstractPriorCodeSequence: u32 = 0x0072003E;

/// (0072,0040) VR=LO, VM=1 Image Set Label
pub const ImageSetLabel: u32 = 0x00720040;

/// (0072,0050) VR=CS, VM=1 Selector Attribute VR
pub const SelectorAttributeVR: u32 = 0x00720050;

/// (0072,0052) VR=AT, VM=1-n Selector Sequence Pointer
pub const SelectorSequencePointer: u32 = 0x00720052;

/// (0072,0054) VR=LO, VM=1-n Selector Sequence Pointer Private Creator
pub const SelectorSequencePointerPrivateCreator: u32 = 0x00720054;

/// (0072,0056) VR=LO, VM=1 Selector Attribute Private Creator
pub const SelectorAttributePrivateCreator: u32 = 0x00720056;

/// (0072,0060) VR=AT, VM=1-n Selector AT Value
pub const SelectorATValue: u32 = 0x00720060;

/// (0072,0062) VR=CS, VM=1-n Selector CS Value
pub const SelectorCSValue: u32 = 0x00720062;

/// (0072,0064) VR=IS, VM=1-n Selector IS Value
pub const SelectorISValue: u32 = 0x00720064;

/// (0072,0066) VR=LO, VM=1-n Selector LO Value
pub const SelectorLOValue: u32 = 0x00720066;

/// (0072,0068) VR=LT, VM=1 Selector LT Value
pub const SelectorLTValue: u32 = 0x00720068;

/// (0072,006A) VR=PN, VM=1-n Selector PN Value
pub const SelectorPNValue: u32 = 0x0072006A;

/// (0072,006C) VR=SH, VM=1-n Selector SH Value
pub const SelectorSHValue: u32 = 0x0072006C;

/// (0072,006E) VR=ST, VM=1 Selector ST Value
pub const SelectorSTValue: u32 = 0x0072006E;

/// (0072,0070) VR=UT, VM=1 Selector UT Value
pub const SelectorUTValue: u32 = 0x00720070;

/// (0072,0072) VR=DS, VM=1-n Selector DS Value
pub const SelectorDSValue: u32 = 0x00720072;

/// (0072,0074) VR=FD, VM=1-n Selector FD Value
pub const SelectorFDValue: u32 = 0x00720074;

/// (0072,0076) VR=FL, VM=1-n Selector FL Value
pub const SelectorFLValue: u32 = 0x00720076;

/// (0072,0078) VR=UL, VM=1-n Selector UL Value
pub const SelectorULValue: u32 = 0x00720078;

/// (0072,007A) VR=US, VM=1-n Selector US Value
pub const SelectorUSValue: u32 = 0x0072007A;

/// (0072,007C) VR=SL, VM=1-n Selector SL Value
pub const SelectorSLValue: u32 = 0x0072007C;

/// (0072,007E) VR=SS, VM=1-n Selector SS Value
pub const SelectorSSValue: u32 = 0x0072007E;

/// (0072,0080) VR=SQ, VM=1 Selector Code Sequence Value
pub const SelectorCodeSequenceValue: u32 = 0x00720080;

/// (0072,0100) VR=US, VM=1 Number of Screens
pub const NumberOfScreens: u32 = 0x00720100;

/// (0072,0102) VR=SQ, VM=1 Nominal Screen Definition Sequence
pub const NominalScreenDefinitionSequence: u32 = 0x00720102;

/// (0072,0104) VR=US, VM=1 Number of Vertical Pixels
pub const NumberOfVerticalPixels: u32 = 0x00720104;

/// (0072,0106) VR=US, VM=1 Number of Horizontal Pixels
pub const NumberOfHorizontalPixels: u32 = 0x00720106;

/// (0072,0108) VR=FD, VM=4 Display Environment Spatial Position
pub const DisplayEnvironmentSpatialPosition: u32 = 0x00720108;

/// (0072,010A) VR=US, VM=1 Screen Minimum Grayscale Bit Depth
pub const ScreenMinimumGrayscaleBitDepth: u32 = 0x0072010A;

/// (0072,010C) VR=US, VM=1 Screen Minimum Color Bit Depth
pub const ScreenMinimumColorBitDepth: u32 = 0x0072010C;

/// (0072,010E) VR=US, VM=1 Application Maximum Repaint Time
pub const ApplicationMaximumRepaintTime: u32 = 0x0072010E;

/// (0072,0200) VR=SQ, VM=1 Display Sets Sequence
pub const DisplaySetsSequence: u32 = 0x00720200;

/// (0072,0202) VR=US, VM=1 Display Set Number
pub const DisplaySetNumber: u32 = 0x00720202;

/// (0072,0203) VR=LO, VM=1 Display Set Label
pub const DisplaySetLabel: u32 = 0x00720203;

/// (0072,0204) VR=US, VM=1 Display Set Presentation Group
pub const DisplaySetPresentationGroup: u32 = 0x00720204;

/// (0072,0206) VR=LO, VM=1 Display Set Presentation Group Description
pub const DisplaySetPresentationGroupDescription: u32 = 0x00720206;

/// (0072,0208) VR=CS, VM=1 Partial Data Display Handling
pub const PartialDataDisplayHandling: u32 = 0x00720208;

/// (0072,0210) VR=SQ, VM=1 Synchronized Scrolling Sequence
pub const SynchronizedScrollingSequence: u32 = 0x00720210;

/// (0072,0212) VR=US, VM=2-n Display Set Scrolling Group
pub const DisplaySetScrollingGroup: u32 = 0x00720212;

/// (0072,0214) VR=SQ, VM=1 Navigation Indicator Sequence
pub const NavigationIndicatorSequence: u32 = 0x00720214;

/// (0072,0216) VR=US, VM=1 Navigation Display Set
pub const NavigationDisplaySet: u32 = 0x00720216;

/// (0072,0218) VR=US, VM=1-n Reference Display Sets
pub const ReferenceDisplaySets: u32 = 0x00720218;

/// (0072,0300) VR=SQ, VM=1 Image Boxes Sequence
pub const ImageBoxesSequence: u32 = 0x00720300;

/// (0072,0302) VR=US, VM=1 Image Box Number
pub const ImageBoxNumber: u32 = 0x00720302;

/// (0072,0304) VR=CS, VM=1 Image Box Layout Type
pub const ImageBoxLayoutType: u32 = 0x00720304;

/// (0072,0306) VR=US, VM=1 Image Box Tile Horizontal Dimension
pub const ImageBoxTileHorizontalDimension: u32 = 0x00720306;

/// (0072,0308) VR=US, VM=1 Image Box Tile Vertical Dimension
pub const ImageBoxTileVerticalDimension: u32 = 0x00720308;

/// (0072,0310) VR=CS, VM=1 Image Box Scroll Direction
pub const ImageBoxScrollDirection: u32 = 0x00720310;

/// (0072,0312) VR=CS, VM=1 Image Box Small Scroll Type
pub const ImageBoxSmallScrollType: u32 = 0x00720312;

/// (0072,0314) VR=US, VM=1 Image Box Small Scroll Amount
pub const ImageBoxSmallScrollAmount: u32 = 0x00720314;

/// (0072,0316) VR=CS, VM=1 Image Box Large Scroll Type
pub const ImageBoxLargeScrollType: u32 = 0x00720316;

/// (0072,0318) VR=US, VM=1 Image Box Large Scroll Amount
pub const ImageBoxLargeScrollAmount: u32 = 0x00720318;

/// (0072,0320) VR=US, VM=1 Image Box Overlap Priority
pub const ImageBoxOverlapPriority: u32 = 0x00720320;

/// (0072,0330) VR=FD, VM=1 Cine Relative to Real-Time
pub const CineRelativeToRealTime: u32 = 0x00720330;

/// (0072,0400) VR=SQ, VM=1 Filter Operations Sequence
pub const FilterOperationsSequence: u32 = 0x00720400;

/// (0072,0402) VR=CS, VM=1 Filter-by Category
pub const FilterByCategory: u32 = 0x00720402;

/// (0072,0404) VR=CS, VM=1 Filter-by Attribute Presence
pub const FilterByAttributePresence: u32 = 0x00720404;

/// (0072,0406) VR=CS, VM=1 Filter-by Operator
pub const FilterByOperator: u32 = 0x00720406;

/// (0072,0420) VR=US, VM=3 Structured Display Background CIELab Value
pub const StructuredDisplayBackgroundCIELabValue: u32 = 0x00720420;

/// (0072,0421) VR=US, VM=3 Empty Image Box CIELab Value
pub const EmptyImageBoxCIELabValue: u32 = 0x00720421;

/// (0072,0422) VR=SQ, VM=1 Structured Display Image Box Sequence
pub const StructuredDisplayImageBoxSequence: u32 = 0x00720422;

/// (0072,0424) VR=SQ, VM=1 Structured Display Text Box Sequence
pub const StructuredDisplayTextBoxSequence: u32 = 0x00720424;

/// (0072,0427) VR=SQ, VM=1 Referenced First Frame Sequence
pub const ReferencedFirstFrameSequence: u32 = 0x00720427;

/// (0072,0430) VR=SQ, VM=1 Image Box Synchronization Sequence
pub const ImageBoxSynchronizationSequence: u32 = 0x00720430;

/// (0072,0432) VR=US, VM=2-n Synchronized Image Box List
pub const SynchronizedImageBoxList: u32 = 0x00720432;

/// (0072,0434) VR=CS, VM=1 Type of Synchronization
pub const TypeOfSynchronization: u32 = 0x00720434;

/// (0072,0500) VR=CS, VM=1 Blending Operation Type
pub const BlendingOperationType: u32 = 0x00720500;

/// (0072,0510) VR=CS, VM=1 Reformatting Operation Type
pub const ReformattingOperationType: u32 = 0x00720510;

/// (0072,0512) VR=FD, VM=1 Reformatting Thickness
pub const ReformattingThickness: u32 = 0x00720512;

/// (0072,0514) VR=FD, VM=1 Reformatting Interval
pub const ReformattingInterval: u32 = 0x00720514;

/// (0072,0516) VR=CS, VM=1 Reformatting Operation Initial View Direction
pub const ReformattingOperationInitialViewDirection: u32 = 0x00720516;

/// (0072,0520) VR=CS, VM=1-n 3D Rendering Type
pub const ThreeDRenderingType: u32 = 0x00720520;

/// (0072,0600) VR=SQ, VM=1 Sorting Operations Sequence
pub const SortingOperationsSequence: u32 = 0x00720600;

/// (0072,0602) VR=CS, VM=1 Sort-by Category
pub const SortByCategory: u32 = 0x00720602;

/// (0072,0604) VR=CS, VM=1 Sorting Direction
pub const SortingDirection: u32 = 0x00720604;

/// (0072,0700) VR=CS, VM=2 Display Set Patient Orientation
pub const DisplaySetPatientOrientation: u32 = 0x00720700;

/// (0072,0702) VR=CS, VM=1 VOI Type
pub const VOIType: u32 = 0x00720702;

/// (0072,0704) VR=CS, VM=1 Pseudo-Color Type
pub const PseudoColorType: u32 = 0x00720704;

/// (0072,0705) VR=SQ, VM=1 Pseudo-Color Palette Instance Reference Sequence
pub const PseudoColorPaletteInstanceReferenceSequence: u32 = 0x00720705;

/// (0072,0706) VR=CS, VM=1 Show Grayscale Inverted
pub const ShowGrayscaleInverted: u32 = 0x00720706;

/// (0072,0710) VR=CS, VM=1 Show Image True Size Flag
pub const ShowImageTrueSizeFlag: u32 = 0x00720710;

/// (0072,0712) VR=CS, VM=1 Show Graphic Annotation Flag
pub const ShowGraphicAnnotationFlag: u32 = 0x00720712;

/// (0072,0714) VR=CS, VM=1 Show Patient Demographics Flag
pub const ShowPatientDemographicsFlag: u32 = 0x00720714;

/// (0072,0716) VR=CS, VM=1 Show Acquisition Techniques Flag
pub const ShowAcquisitionTechniquesFlag: u32 = 0x00720716;

/// (0072,0717) VR=CS, VM=1 Display Set Horizontal Justification
pub const DisplaySetHorizontalJustification: u32 = 0x00720717;

/// (0072,0718) VR=CS, VM=1 Display Set Vertical Justification
pub const DisplaySetVerticalJustification: u32 = 0x00720718;

/// (0074,0120) VR=FD, VM=1 Continuation Start Meterset
pub const ContinuationStartMeterset: u32 = 0x00740120;

/// (0074,0121) VR=FD, VM=1 Continuation End Meterset
pub const ContinuationEndMeterset: u32 = 0x00740121;

/// (0074,1000) VR=CS, VM=1 Procedure Step State
pub const ProcedureStepState: u32 = 0x00741000;

/// (0074,1002) VR=SQ, VM=1 Procedure Step Progress Information Sequence
pub const ProcedureStepProgressInformationSequence: u32 = 0x00741002;

/// (0074,1004) VR=DS, VM=1 Procedure Step Progress
pub const ProcedureStepProgress: u32 = 0x00741004;

/// (0074,1006) VR=ST, VM=1 Procedure Step Progress Description
pub const ProcedureStepProgressDescription: u32 = 0x00741006;

/// (0074,1008) VR=SQ, VM=1 Procedure Step Communications URI Sequence
pub const ProcedureStepCommunicationsURISequence: u32 = 0x00741008;

/// (0074,100a) VR=ST, VM=1 Contact URI
pub const ContactURI: u32 = 0x0074100a;

/// (0074,100c) VR=LO, VM=1 Contact Display Name
pub const ContactDisplayName: u32 = 0x0074100c;

/// (0074,100e) VR=SQ, VM=1 Procedure Step Discontinuation Reason Code Sequence
pub const ProcedureStepDiscontinuationReasonCodeSequence: u32 = 0x0074100e;

/// (0074,1020) VR=SQ, VM=1 Beam Task Sequence
pub const BeamTaskSequence: u32 = 0x00741020;

/// (0074,1022) VR=CS, VM=1 Beam Task Type
pub const BeamTaskType: u32 = 0x00741022;

/// (0074,1024) VR=IS, VM=1 Beam Order Index (Trial) (retired)
pub const BeamOrderIndexTrial: u32 = 0x00741024;

/// (0074,1026) VR=FD, VM=1 Table Top Vertical Adjusted Position
pub const TableTopVerticalAdjustedPosition: u32 = 0x00741026;

/// (0074,1027) VR=FD, VM=1 Table Top Longitudinal Adjusted Position
pub const TableTopLongitudinalAdjustedPosition: u32 = 0x00741027;

/// (0074,1028) VR=FD, VM=1 Table Top Lateral Adjusted Position
pub const TableTopLateralAdjustedPosition: u32 = 0x00741028;

/// (0074,102A) VR=FD, VM=1 Patient Support Adjusted Angle
pub const PatientSupportAdjustedAngle: u32 = 0x0074102A;

/// (0074,102B) VR=FD, VM=1 Table Top Eccentric Adjusted Angle
pub const TableTopEccentricAdjustedAngle: u32 = 0x0074102B;

/// (0074,102C) VR=FD, VM=1 Table Top Pitch Adjusted Angle
pub const TableTopPitchAdjustedAngle: u32 = 0x0074102C;

/// (0074,102D) VR=FD, VM=1 Table Top Roll Adjusted Angle
pub const TableTopRollAdjustedAngle: u32 = 0x0074102D;

/// (0074,1030) VR=SQ, VM=1 Delivery Verification Image Sequence
pub const DeliveryVerificationImageSequence: u32 = 0x00741030;

/// (0074,1032) VR=CS, VM=1 Verification Image Timing
pub const VerificationImageTiming: u32 = 0x00741032;

/// (0074,1034) VR=CS, VM=1 Double Exposure Flag
pub const DoubleExposureFlag: u32 = 0x00741034;

/// (0074,1036) VR=CS, VM=1 Double Exposure Ordering
pub const DoubleExposureOrdering: u32 = 0x00741036;

/// (0074,1038) VR=DS, VM=1 Double Exposure Meterset (Trial) (retired)
pub const DoubleExposureMetersetTrial: u32 = 0x00741038;

/// (0074,103A) VR=DS, VM=4 Double Exposure Field Delta (Trial) (retired)
pub const DoubleExposureFieldDeltaTrial: u32 = 0x0074103A;

/// (0074,1040) VR=SQ, VM=1 Related Reference RT Image Sequence
pub const RelatedReferenceRTImageSequence: u32 = 0x00741040;

/// (0074,1042) VR=SQ, VM=1 General Machine Verification Sequence
pub const GeneralMachineVerificationSequence: u32 = 0x00741042;

/// (0074,1044) VR=SQ, VM=1 Conventional Machine Verification Sequence
pub const ConventionalMachineVerificationSequence: u32 = 0x00741044;

/// (0074,1046) VR=SQ, VM=1 Ion Machine Verification Sequence
pub const IonMachineVerificationSequence: u32 = 0x00741046;

/// (0074,1048) VR=SQ, VM=1 Failed Attributes Sequence
pub const FailedAttributesSequence: u32 = 0x00741048;

/// (0074,104A) VR=SQ, VM=1 Overridden Attributes Sequence
pub const OverriddenAttributesSequence: u32 = 0x0074104A;

/// (0074,104C) VR=SQ, VM=1 Conventional Control Point Verification Sequence
pub const ConventionalControlPointVerificationSequence: u32 = 0x0074104C;

/// (0074,104E) VR=SQ, VM=1 Ion Control Point Verification Sequence
pub const IonControlPointVerificationSequence: u32 = 0x0074104E;

/// (0074,1050) VR=SQ, VM=1 Attribute Occurrence Sequence
pub const AttributeOccurrenceSequence: u32 = 0x00741050;

/// (0074,1052) VR=AT, VM=1 Attribute Occurrence Pointer
pub const AttributeOccurrencePointer: u32 = 0x00741052;

/// (0074,1054) VR=UL, VM=1 Attribute Item Selector
pub const AttributeItemSelector: u32 = 0x00741054;

/// (0074,1056) VR=LO, VM=1 Attribute Occurrence Private Creator
pub const AttributeOccurrencePrivateCreator: u32 = 0x00741056;

/// (0074,1057) VR=IS, VM=1-n Selector Sequence Pointer Items
pub const SelectorSequencePointerItems: u32 = 0x00741057;

/// (0074,1200) VR=CS, VM=1 Scheduled Procedure Step Priority
pub const ScheduledProcedureStepPriority: u32 = 0x00741200;

/// (0074,1202) VR=LO, VM=1 Worklist Label
pub const WorklistLabel: u32 = 0x00741202;

/// (0074,1204) VR=LO, VM=1 Procedure Step Label
pub const ProcedureStepLabel: u32 = 0x00741204;

/// (0074,1210) VR=SQ, VM=1 Scheduled Processing Parameters Sequence
pub const ScheduledProcessingParametersSequence: u32 = 0x00741210;

/// (0074,1212) VR=SQ, VM=1 Performed Processing Parameters Sequence
pub const PerformedProcessingParametersSequence: u32 = 0x00741212;

/// (0074,1216) VR=SQ, VM=1 Unified Procedure Step Performed Procedure Sequence
pub const UnifiedProcedureStepPerformedProcedureSequence: u32 = 0x00741216;

/// (0074,1220) VR=SQ, VM=1 Related Procedure Step Sequence (retired)
pub const RelatedProcedureStepSequence: u32 = 0x00741220;

/// (0074,1222) VR=LO, VM=1 Procedure Step Relationship Type (retired)
pub const ProcedureStepRelationshipType: u32 = 0x00741222;

/// (0074,1224) VR=SQ, VM=1 Replaced Procedure Step Sequence
pub const ReplacedProcedureStepSequence: u32 = 0x00741224;

/// (0074,1230) VR=LO, VM=1 Deletion Lock
pub const DeletionLock: u32 = 0x00741230;

/// (0074,1234) VR=AE, VM=1 Receiving AE
pub const ReceivingAE: u32 = 0x00741234;

/// (0074,1236) VR=AE, VM=1 Requesting AE
pub const RequestingAE: u32 = 0x00741236;

/// (0074,1238) VR=LT, VM=1 Reason for Cancellation
pub const ReasonForCancellation: u32 = 0x00741238;

/// (0074,1242) VR=CS, VM=1 SCP Status
pub const SCPStatus: u32 = 0x00741242;

/// (0074,1244) VR=CS, VM=1 Subscription List Status
pub const SubscriptionListStatus: u32 = 0x00741244;

/// (0074,1246) VR=CS, VM=1 Unified Procedure Step List Status
pub const UnifiedProcedureStepListStatus: u32 = 0x00741246;

/// (0074,1324) VR=UL, VM=1 Beam Order Index
pub const BeamOrderIndex: u32 = 0x00741324;

/// (0074,1338) VR=FD, VM=1 Double Exposure Meterset
pub const DoubleExposureMeterset: u32 = 0x00741338;

/// (0074,133A) VR=FD, VM=4 Double Exposure Field Delta
pub const DoubleExposureFieldDelta: u32 = 0x0074133A;

/// (0076,0001) VR=LO, VM=1 Implant Assembly Template Name
pub const ImplantAssemblyTemplateName: u32 = 0x00760001;

/// (0076,0003) VR=LO, VM=1 Implant Assembly Template Issuer
pub const ImplantAssemblyTemplateIssuer: u32 = 0x00760003;

/// (0076,0006) VR=LO, VM=1 Implant Assembly Template Version
pub const ImplantAssemblyTemplateVersion: u32 = 0x00760006;

/// (0076,0008) VR=SQ, VM=1 Replaced Implant Assembly Template Sequence
pub const ReplacedImplantAssemblyTemplateSequence: u32 = 0x00760008;

/// (0076,000A) VR=CS, VM=1 Implant Assembly Template Type
pub const ImplantAssemblyTemplateType: u32 = 0x0076000A;

/// (0076,000C) VR=SQ, VM=1 Original Implant Assembly Template Sequence
pub const OriginalImplantAssemblyTemplateSequence: u32 = 0x0076000C;

/// (0076,000E) VR=SQ, VM=1 Derivation Implant Assembly Template Sequence
pub const DerivationImplantAssemblyTemplateSequence: u32 = 0x0076000E;

/// (0076,0010) VR=SQ, VM=1 Implant Assembly Template Target Anatomy Sequence
pub const ImplantAssemblyTemplateTargetAnatomySequence: u32 = 0x00760010;

/// (0076,0020) VR=SQ, VM=1 Procedure Type Code Sequence
pub const ProcedureTypeCodeSequence: u32 = 0x00760020;

/// (0076,0030) VR=LO, VM=1 Surgical Technique
pub const SurgicalTechnique: u32 = 0x00760030;

/// (0076,0032) VR=SQ, VM=1 Component Types Sequence
pub const ComponentTypesSequence: u32 = 0x00760032;

/// (0076,0034) VR=CS, VM=1 Component Type Code Sequence
pub const ComponentTypeCodeSequence: u32 = 0x00760034;

/// (0076,0036) VR=CS, VM=1 Exclusive Component Type
pub const ExclusiveComponentType: u32 = 0x00760036;

/// (0076,0038) VR=CS, VM=1 Mandatory Component Type
pub const MandatoryComponentType: u32 = 0x00760038;

/// (0076,0040) VR=SQ, VM=1 Component Sequence
pub const ComponentSequence: u32 = 0x00760040;

/// (0076,0055) VR=US, VM=1 Component ID
pub const ComponentID: u32 = 0x00760055;

/// (0076,0060) VR=SQ, VM=1 Component Assembly Sequence
pub const ComponentAssemblySequence: u32 = 0x00760060;

/// (0076,0070) VR=US, VM=1 Component 1 Referenced ID
pub const Component1ReferencedID: u32 = 0x00760070;

/// (0076,0080) VR=US, VM=1 Component 1 Referenced Mating Feature Set ID
pub const Component1ReferencedMatingFeatureSetID: u32 = 0x00760080;

/// (0076,0090) VR=US, VM=1 Component 1 Referenced Mating Feature ID
pub const Component1ReferencedMatingFeatureID: u32 = 0x00760090;

/// (0076,00A0) VR=US, VM=1 Component 2 Referenced ID
pub const Component2ReferencedID: u32 = 0x007600A0;

/// (0076,00B0) VR=US, VM=1 Component 2 Referenced Mating Feature Set ID
pub const Component2ReferencedMatingFeatureSetID: u32 = 0x007600B0;

/// (0076,00C0) VR=US, VM=1 Component 2 Referenced Mating Feature ID
pub const Component2ReferencedMatingFeatureID: u32 = 0x007600C0;

/// (0078,0001) VR=LO, VM=1 Implant Template Group Name
pub const ImplantTemplateGroupName: u32 = 0x00780001;

/// (0078,0010) VR=ST, VM=1 Implant Template Group Description
pub const ImplantTemplateGroupDescription: u32 = 0x00780010;

/// (0078,0020) VR=LO, VM=1 Implant Template Group Issuer
pub const ImplantTemplateGroupIssuer: u32 = 0x00780020;

/// (0078,0024) VR=LO, VM=1 Implant Template Group Version
pub const ImplantTemplateGroupVersion: u32 = 0x00780024;

/// (0078,0026) VR=SQ, VM=1 Replaced Implant Template Group Sequence
pub const ReplacedImplantTemplateGroupSequence: u32 = 0x00780026;

/// (0078,0028) VR=SQ, VM=1 Implant Template Group Target Anatomy Sequence
pub const ImplantTemplateGroupTargetAnatomySequence: u32 = 0x00780028;

/// (0078,002A) VR=SQ, VM=1 Implant Template Group Members Sequence
pub const ImplantTemplateGroupMembersSequence: u32 = 0x0078002A;

/// (0078,002E) VR=US, VM=1 Implant Template Group Member ID
pub const ImplantTemplateGroupMemberID: u32 = 0x0078002E;

/// (0078,0050) VR=FD, VM=3 3D Implant Template Group Member Matching Point
pub const ThreeDImplantTemplateGroupMemberMatchingPoint: u32 = 0x00780050;

/// (0078,0060) VR=FD, VM=9 3D Implant Template Group Member Matching Axes
pub const ThreeDImplantTemplateGroupMemberMatchingAxes: u32 = 0x00780060;

/// (0078,0070) VR=SQ, VM=1 Implant Template Group Member Matching 2D Coordinates Sequence
pub const ImplantTemplateGroupMemberMatching2DCoordinatesSequence: u32 = 0x00780070;

/// (0078,0090) VR=FD, VM=2 2D Implant Template Group Member Matching Point
pub const TwoDImplantTemplateGroupMemberMatchingPoint: u32 = 0x00780090;

/// (0078,00A0) VR=FD, VM=4 2D Implant Template Group Member Matching Axes
pub const TwoDImplantTemplateGroupMemberMatchingAxes: u32 = 0x007800A0;

/// (0078,00B0) VR=SQ, VM=1 Implant Template Group Variation Dimension Sequence
pub const ImplantTemplateGroupVariationDimensionSequence: u32 = 0x007800B0;

/// (0078,00B2) VR=LO, VM=1 Implant Template Group Variation Dimension Name
pub const ImplantTemplateGroupVariationDimensionName: u32 = 0x007800B2;

/// (0078,00B4) VR=SQ, VM=1 Implant Template Group Variation Dimension Rank Sequence
pub const ImplantTemplateGroupVariationDimensionRankSequence: u32 = 0x007800B4;

/// (0078,00B6) VR=US, VM=1 Referenced Implant Template Group Member ID
pub const ReferencedImplantTemplateGroupMemberID: u32 = 0x007800B6;

/// (0078,00B8) VR=US, VM=1 Implant Template Group Variation Dimension Rank
pub const ImplantTemplateGroupVariationDimensionRank: u32 = 0x007800B8;

/// (0088,0130) VR=SH, VM=1 Storage Media File-set ID
pub const StorageMediaFileSetID: u32 = 0x00880130;

/// (0088,0140) VR=UI, VM=1 Storage Media File-set UID
pub const StorageMediaFileSetUID: u32 = 0x00880140;

/// (0088,0200) VR=SQ, VM=1 Icon Image Sequence
pub const IconImageSequence: u32 = 0x00880200;

/// (0088,0904) VR=LO, VM=1 Topic Title (retired)
pub const TopicTitle: u32 = 0x00880904;

/// (0088,0906) VR=ST, VM=1 Topic Subject (retired)
pub const TopicSubject: u32 = 0x00880906;

/// (0088,0910) VR=LO, VM=1 Topic Author (retired)
pub const TopicAuthor: u32 = 0x00880910;

/// (0088,0912) VR=LO, VM=1-32 Topic Keywords (retired)
pub const TopicKeywords: u32 = 0x00880912;

/// (0100,0410) VR=CS, VM=1 SOP Instance Status
pub const SOPInstanceStatus: u32 = 0x01000410;

/// (0100,0420) VR=DT, VM=1 SOP Authorization DateTime
pub const SOPAuthorizationDateTime: u32 = 0x01000420;

/// (0100,0424) VR=LT, VM=1 SOP Authorization Comment
pub const SOPAuthorizationComment: u32 = 0x01000424;

/// (0100,0426) VR=LO, VM=1 Authorization Equipment Certification Number
pub const AuthorizationEquipmentCertificationNumber: u32 = 0x01000426;

/// (0400,0005) VR=US, VM=1 MAC ID Number
pub const MACIDNumber: u32 = 0x04000005;

/// (0400,0010) VR=UI, VM=1 MAC Calculation Transfer Syntax UID
pub const MACCalculationTransferSyntaxUID: u32 = 0x04000010;

/// (0400,0015) VR=CS, VM=1 MAC Algorithm
pub const MACAlgorithm: u32 = 0x04000015;

/// (0400,0020) VR=AT, VM=1-n Data Elements Signed
pub const DataElementsSigned: u32 = 0x04000020;

/// (0400,0100) VR=UI, VM=1 Digital Signature UID
pub const DigitalSignatureUID: u32 = 0x04000100;

/// (0400,0105) VR=DT, VM=1 Digital Signature DateTime
pub const DigitalSignatureDateTime: u32 = 0x04000105;

/// (0400,0110) VR=CS, VM=1 Certificate Type
pub const CertificateType: u32 = 0x04000110;

/// (0400,0115) VR=OB, VM=1 Certificate of Signer
pub const CertificateOfSigner: u32 = 0x04000115;

/// (0400,0120) VR=OB, VM=1 Signature
pub const Signature: u32 = 0x04000120;

/// (0400,0305) VR=CS, VM=1 Certified Timestamp Type
pub const CertifiedTimestampType: u32 = 0x04000305;

/// (0400,0310) VR=OB, VM=1 Certified Timestamp
pub const CertifiedTimestamp: u32 = 0x04000310;

/// (0400,0401) VR=SQ, VM=1 Digital Signature Purpose Code Sequence
pub const DigitalSignaturePurposeCodeSequence: u32 = 0x04000401;

/// (0400,0402) VR=SQ, VM=1 Referenced Digital Signature Sequence
pub const ReferencedDigitalSignatureSequence: u32 = 0x04000402;

/// (0400,0403) VR=SQ, VM=1 Referenced SOP Instance MAC Sequence
pub const ReferencedSOPInstanceMACSequence: u32 = 0x04000403;

/// (0400,0404) VR=OB, VM=1 MAC
pub const MAC: u32 = 0x04000404;

/// (0400,0500) VR=SQ, VM=1 Encrypted Attributes Sequence
pub const EncryptedAttributesSequence: u32 = 0x04000500;

/// (0400,0510) VR=UI, VM=1 Encrypted Content Transfer Syntax UID
pub const EncryptedContentTransferSyntaxUID: u32 = 0x04000510;

/// (0400,0520) VR=OB, VM=1 Encrypted Content
pub const EncryptedContent: u32 = 0x04000520;

/// (0400,0550) VR=SQ, VM=1 Modified Attributes Sequence
pub const ModifiedAttributesSequence: u32 = 0x04000550;

/// (0400,0561) VR=SQ, VM=1 Original Attributes Sequence
pub const OriginalAttributesSequence: u32 = 0x04000561;

/// (0400,0562) VR=DT, VM=1 Attribute Modification DateTime
pub const AttributeModificationDateTime: u32 = 0x04000562;

/// (0400,0563) VR=LO, VM=1 Modifying System
pub const ModifyingSystem: u32 = 0x04000563;

/// (0400,0564) VR=LO, VM=1 Source of Previous Values
pub const SourceOfPreviousValues: u32 = 0x04000564;

/// (0400,0565) VR=CS, VM=1 Reason for the Attribute Modification
pub const ReasonForTheAttributeModification: u32 = 0x04000565;

/// (1000,xxx0) VR=US, VM=3 Escape Triplet (retired)
pub const EscapeTriplet: u32 = 0x10000000;

/// (1000,xxx1) VR=US, VM=3 Run Length Triplet (retired)
pub const RunLengthTriplet: u32 = 0x10000001;

/// (1000,xxx2) VR=US, VM=1 Huffman Table Size (retired)
pub const HuffmanTableSize: u32 = 0x10000002;

/// (1000,xxx3) VR=US, VM=3 Huffman Table Triplet (retired)
pub const HuffmanTableTriplet: u32 = 0x10000003;

/// (1000,xxx4) VR=US, VM=1 Shift Table Size (retired)
pub const ShiftTableSize: u32 = 0x10000004;

/// (1000,xxx5) VR=US, VM=3 Shift Table Triplet (retired)
pub const ShiftTableTriplet: u32 = 0x10000005;

/// (1010,xxxx) VR=US, VM=1-n Zonal Map (retired)
pub const ZonalMap: u32 = 0x10100000;

/// (2000,0010) VR=IS, VM=1 Number of Copies
pub const NumberOfCopies: u32 = 0x20000010;

/// (2000,001E) VR=SQ, VM=1 Printer Configuration Sequence
pub const PrinterConfigurationSequence: u32 = 0x2000001E;

/// (2000,0020) VR=CS, VM=1 Print Priority
pub const PrintPriority: u32 = 0x20000020;

/// (2000,0030) VR=CS, VM=1 Medium Type
pub const MediumType: u32 = 0x20000030;

/// (2000,0040) VR=CS, VM=1 Film Destination
pub const FilmDestination: u32 = 0x20000040;

/// (2000,0050) VR=LO, VM=1 Film Session Label
pub const FilmSessionLabel: u32 = 0x20000050;

/// (2000,0060) VR=IS, VM=1 Memory Allocation
pub const MemoryAllocation: u32 = 0x20000060;

/// (2000,0061) VR=IS, VM=1 Maximum Memory Allocation
pub const MaximumMemoryAllocation: u32 = 0x20000061;

/// (2000,0062) VR=CS, VM=1 Color Image Printing Flag (retired)
pub const ColorImagePrintingFlag: u32 = 0x20000062;

/// (2000,0063) VR=CS, VM=1 Collation Flag (retired)
pub const CollationFlag: u32 = 0x20000063;

/// (2000,0065) VR=CS, VM=1 Annotation Flag (retired)
pub const AnnotationFlag: u32 = 0x20000065;

/// (2000,0067) VR=CS, VM=1 Image Overlay Flag (retired)
pub const ImageOverlayFlag: u32 = 0x20000067;

/// (2000,0069) VR=CS, VM=1 Presentation LUT Flag (retired)
pub const PresentationLUTFlag: u32 = 0x20000069;

/// (2000,006A) VR=CS, VM=1 Image Box Presentation LUT Flag (retired)
pub const ImageBoxPresentationLUTFlag: u32 = 0x2000006A;

/// (2000,00A0) VR=US, VM=1 Memory Bit Depth
pub const MemoryBitDepth: u32 = 0x200000A0;

/// (2000,00A1) VR=US, VM=1 Printing Bit Depth
pub const PrintingBitDepth: u32 = 0x200000A1;

/// (2000,00A2) VR=SQ, VM=1 Media Installed Sequence
pub const MediaInstalledSequence: u32 = 0x200000A2;

/// (2000,00A4) VR=SQ, VM=1 Other Media Available Sequence
pub const OtherMediaAvailableSequence: u32 = 0x200000A4;

/// (2000,00A8) VR=SQ, VM=1 Supported Image Display Formats Sequence
pub const SupportedImageDisplayFormatsSequence: u32 = 0x200000A8;

/// (2000,0500) VR=SQ, VM=1 Referenced Film Box Sequence
pub const ReferencedFilmBoxSequence: u32 = 0x20000500;

/// (2000,0510) VR=SQ, VM=1 Referenced Stored Print  Sequence (retired)
pub const ReferencedStoredPrintSequence: u32 = 0x20000510;

/// (2010,0010) VR=ST, VM=1 Image Display Format
pub const ImageDisplayFormat: u32 = 0x20100010;

/// (2010,0030) VR=CS, VM=1 Annotation Display Format ID
pub const AnnotationDisplayFormatID: u32 = 0x20100030;

/// (2010,0040) VR=CS, VM=1 Film Orientation
pub const FilmOrientation: u32 = 0x20100040;

/// (2010,0050) VR=CS, VM=1 Film Size ID
pub const FilmSizeID: u32 = 0x20100050;

/// (2010,0052) VR=CS, VM=1 Printer Resolution ID
pub const PrinterResolutionID: u32 = 0x20100052;

/// (2010,0054) VR=CS, VM=1 Default Printer Resolution ID
pub const DefaultPrinterResolutionID: u32 = 0x20100054;

/// (2010,0060) VR=CS, VM=1 Magnification Type
pub const MagnificationType: u32 = 0x20100060;

/// (2010,0080) VR=CS, VM=1 Smoothing Type
pub const SmoothingType: u32 = 0x20100080;

/// (2010,00A6) VR=CS, VM=1 Default Magnification Type
pub const DefaultMagnificationType: u32 = 0x201000A6;

/// (2010,00A7) VR=CS, VM=1-n Other Magnification Types Available
pub const OtherMagnificationTypesAvailable: u32 = 0x201000A7;

/// (2010,00A8) VR=CS, VM=1 Default Smoothing Type
pub const DefaultSmoothingType: u32 = 0x201000A8;

/// (2010,00A9) VR=CS, VM=1-n Other Smoothing Types Available
pub const OtherSmoothingTypesAvailable: u32 = 0x201000A9;

/// (2010,0100) VR=CS, VM=1 Border Density
pub const BorderDensity: u32 = 0x20100100;

/// (2010,0110) VR=CS, VM=1 Empty Image Density
pub const EmptyImageDensity: u32 = 0x20100110;

/// (2010,0120) VR=US, VM=1 Min Density
pub const MinDensity: u32 = 0x20100120;

/// (2010,0130) VR=US, VM=1 Max Density
pub const MaxDensity: u32 = 0x20100130;

/// (2010,0140) VR=CS, VM=1 Trim
pub const Trim: u32 = 0x20100140;

/// (2010,0150) VR=ST, VM=1 Configuration Information
pub const ConfigurationInformation: u32 = 0x20100150;

/// (2010,0152) VR=LT, VM=1 Configuration Information Description
pub const ConfigurationInformationDescription: u32 = 0x20100152;

/// (2010,0154) VR=IS, VM=1 Maximum Collated Films
pub const MaximumCollatedFilms: u32 = 0x20100154;

/// (2010,015E) VR=US, VM=1 Illumination
pub const Illumination: u32 = 0x2010015E;

/// (2010,0160) VR=US, VM=1 Reflected Ambient Light
pub const ReflectedAmbientLight: u32 = 0x20100160;

/// (2010,0376) VR=DS, VM=2 Printer Pixel Spacing
pub const PrinterPixelSpacing: u32 = 0x20100376;

/// (2010,0500) VR=SQ, VM=1 Referenced Film Session Sequence
pub const ReferencedFilmSessionSequence: u32 = 0x20100500;

/// (2010,0510) VR=SQ, VM=1 Referenced Image Box Sequence
pub const ReferencedImageBoxSequence: u32 = 0x20100510;

/// (2010,0520) VR=SQ, VM=1 Referenced Basic Annotation Box Sequence
pub const ReferencedBasicAnnotationBoxSequence: u32 = 0x20100520;

/// (2020,0010) VR=US, VM=1 Image Box Position
pub const ImageBoxPosition: u32 = 0x20200010;

/// (2020,0020) VR=CS, VM=1 Polarity
pub const Polarity: u32 = 0x20200020;

/// (2020,0030) VR=DS, VM=1 Requested Image Size
pub const RequestedImageSize: u32 = 0x20200030;

/// (2020,0040) VR=CS, VM=1 Requested Decimate/Crop Behavior
pub const RequestedDecimateCropBehavior: u32 = 0x20200040;

/// (2020,0050) VR=CS, VM=1 Requested Resolution ID
pub const RequestedResolutionID: u32 = 0x20200050;

/// (2020,00A0) VR=CS, VM=1 Requested Image Size Flag
pub const RequestedImageSizeFlag: u32 = 0x202000A0;

/// (2020,00A2) VR=CS, VM=1 Decimate/Crop Result
pub const DecimateCropResult: u32 = 0x202000A2;

/// (2020,0110) VR=SQ, VM=1 Basic Grayscale Image Sequence
pub const BasicGrayscaleImageSequence: u32 = 0x20200110;

/// (2020,0111) VR=SQ, VM=1 Basic Color Image Sequence
pub const BasicColorImageSequence: u32 = 0x20200111;

/// (2020,0130) VR=SQ, VM=1 Referenced Image Overlay Box Sequence (retired)
pub const ReferencedImageOverlayBoxSequence: u32 = 0x20200130;

/// (2020,0140) VR=SQ, VM=1 Referenced VOI LUT Box Sequence (retired)
pub const ReferencedVOILUTBoxSequence: u32 = 0x20200140;

/// (2030,0010) VR=US, VM=1 Annotation Position
pub const AnnotationPosition: u32 = 0x20300010;

/// (2030,0020) VR=LO, VM=1 Text String
pub const TextString: u32 = 0x20300020;

/// (2040,0010) VR=SQ, VM=1 Referenced Overlay Plane Sequence (retired)
pub const ReferencedOverlayPlaneSequence: u32 = 0x20400010;

/// (2040,0011) VR=US, VM=1-99 Referenced Overlay Plane Groups (retired)
pub const ReferencedOverlayPlaneGroups: u32 = 0x20400011;

/// (2040,0020) VR=SQ, VM=1 Overlay Pixel Data Sequence (retired)
pub const OverlayPixelDataSequence: u32 = 0x20400020;

/// (2040,0060) VR=CS, VM=1 Overlay Magnification Type (retired)
pub const OverlayMagnificationType: u32 = 0x20400060;

/// (2040,0070) VR=CS, VM=1 Overlay Smoothing Type (retired)
pub const OverlaySmoothingType: u32 = 0x20400070;

/// (2040,0072) VR=CS, VM=1 Overlay or Image Magnification (retired)
pub const OverlayOrImageMagnification: u32 = 0x20400072;

/// (2040,0074) VR=US, VM=1 Magnify to Number of Columns (retired)
pub const MagnifyToNumberOfColumns: u32 = 0x20400074;

/// (2040,0080) VR=CS, VM=1 Overlay Foreground Density (retired)
pub const OverlayForegroundDensity: u32 = 0x20400080;

/// (2040,0082) VR=CS, VM=1 Overlay Background Density (retired)
pub const OverlayBackgroundDensity: u32 = 0x20400082;

/// (2040,0090) VR=CS, VM=1 Overlay Mode (retired)
pub const OverlayMode: u32 = 0x20400090;

/// (2040,0100) VR=CS, VM=1 Threshold Density (retired)
pub const ThresholdDensity: u32 = 0x20400100;

/// (2040,0500) VR=SQ, VM=1 Referenced Image Box Sequence (Retired) (retired)
pub const ReferencedImageBoxSequenceRetired: u32 = 0x20400500;

/// (2050,0010) VR=SQ, VM=1 Presentation LUT Sequence
pub const PresentationLUTSequence: u32 = 0x20500010;

/// (2050,0020) VR=CS, VM=1 Presentation LUT Shape
pub const PresentationLUTShape: u32 = 0x20500020;

/// (2050,0500) VR=SQ, VM=1 Referenced Presentation  LUT Sequence
pub const ReferencedPresentationLUTSequence: u32 = 0x20500500;

/// (2100,0010) VR=SH, VM=1 Print Job ID (retired)
pub const PrintJobID: u32 = 0x21000010;

/// (2100,0020) VR=CS, VM=1 Execution Status
pub const ExecutionStatus: u32 = 0x21000020;

/// (2100,0030) VR=CS, VM=1 Execution Status Info
pub const ExecutionStatusInfo: u32 = 0x21000030;

/// (2100,0040) VR=DA, VM=1 Creation Date
pub const CreationDate: u32 = 0x21000040;

/// (2100,0050) VR=TM, VM=1 Creation Time
pub const CreationTime: u32 = 0x21000050;

/// (2100,0070) VR=AE, VM=1 Originator
pub const Originator: u32 = 0x21000070;

/// (2100,0140) VR=AE, VM=1 Destination AE (retired)
pub const DestinationAE: u32 = 0x21000140;

/// (2100,0160) VR=SH, VM=1 Owner ID
pub const OwnerID: u32 = 0x21000160;

/// (2100,0170) VR=IS, VM=1 Number of Films
pub const NumberOfFilms: u32 = 0x21000170;

/// (2100,0500) VR=SQ, VM=1 Referenced Print Job Sequence (Pull Stored Print) (retired)
pub const ReferencedPrintJobSequencePullStoredPrint: u32 = 0x21000500;

/// (2110,0010) VR=CS, VM=1 Printer Status
pub const PrinterStatus: u32 = 0x21100010;

/// (2110,0020) VR=CS, VM=1 Printer Status Info
pub const PrinterStatusInfo: u32 = 0x21100020;

/// (2110,0030) VR=LO, VM=1 Printer Name
pub const PrinterName: u32 = 0x21100030;

/// (2110,0099) VR=SH, VM=1 Print Queue ID (retired)
pub const PrintQueueID: u32 = 0x21100099;

/// (2120,0010) VR=CS, VM=1 Queue Status (retired)
pub const QueueStatus: u32 = 0x21200010;

/// (2120,0050) VR=SQ, VM=1 Print Job Description Sequence (retired)
pub const PrintJobDescriptionSequence: u32 = 0x21200050;

/// (2120,0070) VR=SQ, VM=1 Referenced Print Job Sequence (retired)
pub const ReferencedPrintJobSequence: u32 = 0x21200070;

/// (2130,0010) VR=SQ, VM=1 Print Management Capabilities Sequence (retired)
pub const PrintManagementCapabilitiesSequence: u32 = 0x21300010;

/// (2130,0015) VR=SQ, VM=1 Printer Characteristics Sequence (retired)
pub const PrinterCharacteristicsSequence: u32 = 0x21300015;

/// (2130,0030) VR=SQ, VM=1 Film Box Content Sequence (retired)
pub const FilmBoxContentSequence: u32 = 0x21300030;

/// (2130,0040) VR=SQ, VM=1 Image Box Content Sequence (retired)
pub const ImageBoxContentSequence: u32 = 0x21300040;

/// (2130,0050) VR=SQ, VM=1 Annotation Content Sequence (retired)
pub const AnnotationContentSequence: u32 = 0x21300050;

/// (2130,0060) VR=SQ, VM=1 Image Overlay Box Content Sequence (retired)
pub const ImageOverlayBoxContentSequence: u32 = 0x21300060;

/// (2130,0080) VR=SQ, VM=1 Presentation LUT Content Sequence (retired)
pub const PresentationLUTContentSequence: u32 = 0x21300080;

/// (2130,00A0) VR=SQ, VM=1 Proposed Study Sequence (retired)
pub const ProposedStudySequence: u32 = 0x213000A0;

/// (2130,00C0) VR=SQ, VM=1 Original Image Sequence (retired)
pub const OriginalImageSequence: u32 = 0x213000C0;

/// (2200,0001) VR=CS, VM=1 Label Using Information Extracted From Instances
pub const LabelUsingInformationExtractedFromInstances: u32 = 0x22000001;

/// (2200,0002) VR=UT, VM=1 Label Text
pub const LabelText: u32 = 0x22000002;

/// (2200,0003) VR=CS, VM=1 Label Style Selection
pub const LabelStyleSelection: u32 = 0x22000003;

/// (2200,0004) VR=LT, VM=1 Media Disposition
pub const MediaDisposition: u32 = 0x22000004;

/// (2200,0005) VR=LT, VM=1 Barcode Value
pub const BarcodeValue: u32 = 0x22000005;

/// (2200,0006) VR=CS, VM=1 Barcode Symbology
pub const BarcodeSymbology: u32 = 0x22000006;

/// (2200,0007) VR=CS, VM=1 Allow Media Splitting
pub const AllowMediaSplitting: u32 = 0x22000007;

/// (2200,0008) VR=CS, VM=1 Include Non-DICOM Objects
pub const IncludeNonDICOMObjects: u32 = 0x22000008;

/// (2200,0009) VR=CS, VM=1 Include Display Application
pub const IncludeDisplayApplication: u32 = 0x22000009;

/// (2200,000A) VR=CS, VM=1 Preserve Composite Instances After Media Creation
pub const PreserveCompositeInstancesAfterMediaCreation: u32 = 0x2200000A;

/// (2200,000B) VR=US, VM=1 Total Number of Pieces of Media Created
pub const TotalNumberOfPiecesOfMediaCreated: u32 = 0x2200000B;

/// (2200,000C) VR=LO, VM=1 Requested Media Application Profile
pub const RequestedMediaApplicationProfile: u32 = 0x2200000C;

/// (2200,000D) VR=SQ, VM=1 Referenced Storage Media Sequence
pub const ReferencedStorageMediaSequence: u32 = 0x2200000D;

/// (2200,000E) VR=AT, VM=1-n Failure Attributes
pub const FailureAttributes: u32 = 0x2200000E;

/// (2200,000F) VR=CS, VM=1 Allow Lossy Compression
pub const AllowLossyCompression: u32 = 0x2200000F;

/// (2200,0020) VR=CS, VM=1 Request Priority
pub const RequestPriority: u32 = 0x22000020;

/// (3002,0002) VR=SH, VM=1 RT Image Label
pub const RTImageLabel: u32 = 0x30020002;

/// (3002,0003) VR=LO, VM=1 RT Image Name
pub const RTImageName: u32 = 0x30020003;

/// (3002,0004) VR=ST, VM=1 RT Image Description
pub const RTImageDescription: u32 = 0x30020004;

/// (3002,000A) VR=CS, VM=1 Reported Values Origin
pub const ReportedValuesOrigin: u32 = 0x3002000A;

/// (3002,000C) VR=CS, VM=1 RT Image Plane
pub const RTImagePlane: u32 = 0x3002000C;

/// (3002,000D) VR=DS, VM=3 X-Ray Image Receptor Translation
pub const XRayImageReceptorTranslation: u32 = 0x3002000D;

/// (3002,000E) VR=DS, VM=1 X-Ray Image Receptor Angle
pub const XRayImageReceptorAngle: u32 = 0x3002000E;

/// (3002,0010) VR=DS, VM=6 RT Image Orientation
pub const RTImageOrientation: u32 = 0x30020010;

/// (3002,0011) VR=DS, VM=2 Image Plane Pixel Spacing
pub const ImagePlanePixelSpacing: u32 = 0x30020011;

/// (3002,0012) VR=DS, VM=2 RT Image Position
pub const RTImagePosition: u32 = 0x30020012;

/// (3002,0020) VR=SH, VM=1 Radiation Machine Name
pub const RadiationMachineName: u32 = 0x30020020;

/// (3002,0022) VR=DS, VM=1 Radiation Machine SAD
pub const RadiationMachineSAD: u32 = 0x30020022;

/// (3002,0024) VR=DS, VM=1 Radiation Machine SSD
pub const RadiationMachineSSD: u32 = 0x30020024;

/// (3002,0026) VR=DS, VM=1 RT Image SID
pub const RTImageSID: u32 = 0x30020026;

/// (3002,0028) VR=DS, VM=1 Source to Reference Object Distance
pub const SourceToReferenceObjectDistance: u32 = 0x30020028;

/// (3002,0029) VR=IS, VM=1 Fraction Number
pub const FractionNumber: u32 = 0x30020029;

/// (3002,0030) VR=SQ, VM=1 Exposure Sequence
pub const ExposureSequence: u32 = 0x30020030;

/// (3002,0032) VR=DS, VM=1 Meterset Exposure
pub const MetersetExposure: u32 = 0x30020032;

/// (3002,0034) VR=DS, VM=4 Diaphragm Position
pub const DiaphragmPosition: u32 = 0x30020034;

/// (3002,0040) VR=SQ, VM=1 Fluence Map Sequence
pub const FluenceMapSequence: u32 = 0x30020040;

/// (3002,0041) VR=CS, VM=1 Fluence Data Source
pub const FluenceDataSource: u32 = 0x30020041;

/// (3002,0042) VR=DS, VM=1 Fluence Data Scale
pub const FluenceDataScale: u32 = 0x30020042;

/// (3002,0050) VR=SQ, VM=1 Primary Fluence Mode Sequence
pub const PrimaryFluenceModeSequence: u32 = 0x30020050;

/// (3002,0051) VR=CS, VM=1 Fluence Mode
pub const FluenceMode: u32 = 0x30020051;

/// (3002,0052) VR=SH, VM=1 Fluence Mode ID
pub const FluenceModeID: u32 = 0x30020052;

/// (3004,0001) VR=CS, VM=1 DVH Type
pub const DVHType: u32 = 0x30040001;

/// (3004,0002) VR=CS, VM=1 Dose Units
pub const DoseUnits: u32 = 0x30040002;

/// (3004,0004) VR=CS, VM=1 Dose Type
pub const DoseType: u32 = 0x30040004;

/// (3004,0006) VR=LO, VM=1 Dose Comment
pub const DoseComment: u32 = 0x30040006;

/// (3004,0008) VR=DS, VM=3 Normalization Point
pub const NormalizationPoint: u32 = 0x30040008;

/// (3004,000A) VR=CS, VM=1 Dose Summation Type
pub const DoseSummationType: u32 = 0x3004000A;

/// (3004,000C) VR=DS, VM=2-n Grid Frame Offset Vector
pub const GridFrameOffsetVector: u32 = 0x3004000C;

/// (3004,000E) VR=DS, VM=1 Dose Grid Scaling
pub const DoseGridScaling: u32 = 0x3004000E;

/// (3004,0010) VR=SQ, VM=1 RT Dose ROI Sequence
pub const RTDoseROISequence: u32 = 0x30040010;

/// (3004,0012) VR=DS, VM=1 Dose Value
pub const DoseValue: u32 = 0x30040012;

/// (3004,0014) VR=CS, VM=1-3 Tissue Heterogeneity Correction
pub const TissueHeterogeneityCorrection: u32 = 0x30040014;

/// (3004,0040) VR=DS, VM=3 DVH Normalization Point
pub const DVHNormalizationPoint: u32 = 0x30040040;

/// (3004,0042) VR=DS, VM=1 DVH Normalization Dose Value
pub const DVHNormalizationDoseValue: u32 = 0x30040042;

/// (3004,0050) VR=SQ, VM=1 DVH Sequence
pub const DVHSequence: u32 = 0x30040050;

/// (3004,0052) VR=DS, VM=1 DVH Dose Scaling
pub const DVHDoseScaling: u32 = 0x30040052;

/// (3004,0054) VR=CS, VM=1 DVH Volume Units
pub const DVHVolumeUnits: u32 = 0x30040054;

/// (3004,0056) VR=IS, VM=1 DVH Number of Bins
pub const DVHNumberOfBins: u32 = 0x30040056;

/// (3004,0058) VR=DS, VM=2-2n DVH Data
pub const DVHData: u32 = 0x30040058;

/// (3004,0060) VR=SQ, VM=1 DVH Referenced ROI Sequence
pub const DVHReferencedROISequence: u32 = 0x30040060;

/// (3004,0062) VR=CS, VM=1 DVH ROI Contribution Type
pub const DVHROIContributionType: u32 = 0x30040062;

/// (3004,0070) VR=DS, VM=1 DVH Minimum Dose
pub const DVHMinimumDose: u32 = 0x30040070;

/// (3004,0072) VR=DS, VM=1 DVH Maximum Dose
pub const DVHMaximumDose: u32 = 0x30040072;

/// (3004,0074) VR=DS, VM=1 DVH Mean Dose
pub const DVHMeanDose: u32 = 0x30040074;

/// (3006,0002) VR=SH, VM=1 Structure Set Label
pub const StructureSetLabel: u32 = 0x30060002;

/// (3006,0004) VR=LO, VM=1 Structure Set Name
pub const StructureSetName: u32 = 0x30060004;

/// (3006,0006) VR=ST, VM=1 Structure Set Description
pub const StructureSetDescription: u32 = 0x30060006;

/// (3006,0008) VR=DA, VM=1 Structure Set Date
pub const StructureSetDate: u32 = 0x30060008;

/// (3006,0009) VR=TM, VM=1 Structure Set Time
pub const StructureSetTime: u32 = 0x30060009;

/// (3006,0010) VR=SQ, VM=1 Referenced Frame of Reference Sequence
pub const ReferencedFrameOfReferenceSequence: u32 = 0x30060010;

/// (3006,0012) VR=SQ, VM=1 RT Referenced Study Sequence
pub const RTReferencedStudySequence: u32 = 0x30060012;

/// (3006,0014) VR=SQ, VM=1 RT Referenced Series Sequence
pub const RTReferencedSeriesSequence: u32 = 0x30060014;

/// (3006,0016) VR=SQ, VM=1 Contour Image Sequence
pub const ContourImageSequence: u32 = 0x30060016;

/// (3006,0020) VR=SQ, VM=1 Structure Set ROI Sequence
pub const StructureSetROISequence: u32 = 0x30060020;

/// (3006,0022) VR=IS, VM=1 ROI Number
pub const ROINumber: u32 = 0x30060022;

/// (3006,0024) VR=UI, VM=1 Referenced Frame of Reference UID
pub const ReferencedFrameOfReferenceUID: u32 = 0x30060024;

/// (3006,0026) VR=LO, VM=1 ROI Name
pub const ROIName: u32 = 0x30060026;

/// (3006,0028) VR=ST, VM=1 ROI Description
pub const ROIDescription: u32 = 0x30060028;

/// (3006,002A) VR=IS, VM=3 ROI Display Color
pub const ROIDisplayColor: u32 = 0x3006002A;

/// (3006,002C) VR=DS, VM=1 ROI Volume
pub const ROIVolume: u32 = 0x3006002C;

/// (3006,0030) VR=SQ, VM=1 RT Related ROI Sequence
pub const RTRelatedROISequence: u32 = 0x30060030;

/// (3006,0033) VR=CS, VM=1 RT ROI Relationship
pub const RTROIRelationship: u32 = 0x30060033;

/// (3006,0036) VR=CS, VM=1 ROI Generation Algorithm
pub const ROIGenerationAlgorithm: u32 = 0x30060036;

/// (3006,0038) VR=LO, VM=1 ROI Generation Description
pub const ROIGenerationDescription: u32 = 0x30060038;

/// (3006,0039) VR=SQ, VM=1 ROI Contour Sequence
pub const ROIContourSequence: u32 = 0x30060039;

/// (3006,0040) VR=SQ, VM=1 Contour Sequence
pub const ContourSequence: u32 = 0x30060040;

/// (3006,0042) VR=CS, VM=1 Contour Geometric Type
pub const ContourGeometricType: u32 = 0x30060042;

/// (3006,0044) VR=DS, VM=1 Contour Slab Thickness
pub const ContourSlabThickness: u32 = 0x30060044;

/// (3006,0045) VR=DS, VM=3 Contour Offset Vector
pub const ContourOffsetVector: u32 = 0x30060045;

/// (3006,0046) VR=IS, VM=1 Number of Contour Points
pub const NumberOfContourPoints: u32 = 0x30060046;

/// (3006,0048) VR=IS, VM=1 Contour Number
pub const ContourNumber: u32 = 0x30060048;

/// (3006,0049) VR=IS, VM=1-n Attached Contours
pub const AttachedContours: u32 = 0x30060049;

/// (3006,0050) VR=DS, VM=3-3n Contour Data
pub const ContourData: u32 = 0x30060050;

/// (3006,0080) VR=SQ, VM=1 RT ROI Observations Sequence
pub const RTROIObservationsSequence: u32 = 0x30060080;

/// (3006,0082) VR=IS, VM=1 Observation Number
pub const ObservationNumber: u32 = 0x30060082;

/// (3006,0084) VR=IS, VM=1 Referenced ROI Number
pub const ReferencedROINumber: u32 = 0x30060084;

/// (3006,0085) VR=SH, VM=1 ROI Observation Label
pub const ROIObservationLabel: u32 = 0x30060085;

/// (3006,0086) VR=SQ, VM=1 RT ROI Identification Code Sequence
pub const RTROIIdentificationCodeSequence: u32 = 0x30060086;

/// (3006,0088) VR=ST, VM=1 ROI Observation Description
pub const ROIObservationDescription: u32 = 0x30060088;

/// (3006,00A0) VR=SQ, VM=1 Related RT ROI Observations Sequence
pub const RelatedRTROIObservationsSequence: u32 = 0x300600A0;

/// (3006,00A4) VR=CS, VM=1 RT ROI Interpreted Type
pub const RTROIInterpretedType: u32 = 0x300600A4;

/// (3006,00A6) VR=PN, VM=1 ROI Interpreter
pub const ROIInterpreter: u32 = 0x300600A6;

/// (3006,00B0) VR=SQ, VM=1 ROI Physical Properties Sequence
pub const ROIPhysicalPropertiesSequence: u32 = 0x300600B0;

/// (3006,00B2) VR=CS, VM=1 ROI Physical Property
pub const ROIPhysicalProperty: u32 = 0x300600B2;

/// (3006,00B4) VR=DS, VM=1 ROI Physical Property Value
pub const ROIPhysicalPropertyValue: u32 = 0x300600B4;

/// (3006,00B6) VR=SQ, VM=1 ROI Elemental Composition Sequence
pub const ROIElementalCompositionSequence: u32 = 0x300600B6;

/// (3006,00B7) VR=US, VM=1 ROI Elemental Composition Atomic Number
pub const ROIElementalCompositionAtomicNumber: u32 = 0x300600B7;

/// (3006,00B8) VR=FL, VM=1 ROI Elemental Composition Atomic Mass Fraction
pub const ROIElementalCompositionAtomicMassFraction: u32 = 0x300600B8;

/// (3006,00C0) VR=SQ, VM=1 Frame of Reference Relationship Sequence
pub const FrameOfReferenceRelationshipSequence: u32 = 0x300600C0;

/// (3006,00C2) VR=UI, VM=1 Related Frame of Reference UID
pub const RelatedFrameOfReferenceUID: u32 = 0x300600C2;

/// (3006,00C4) VR=CS, VM=1 Frame of Reference Transformation Type
pub const FrameOfReferenceTransformationType: u32 = 0x300600C4;

/// (3006,00C6) VR=DS, VM=16 Frame of Reference Transformation Matrix
pub const FrameOfReferenceTransformationMatrix: u32 = 0x300600C6;

/// (3006,00C8) VR=LO, VM=1 Frame of Reference Transformation Comment
pub const FrameOfReferenceTransformationComment: u32 = 0x300600C8;

/// (3008,0010) VR=SQ, VM=1 Measured Dose Reference Sequence
pub const MeasuredDoseReferenceSequence: u32 = 0x30080010;

/// (3008,0012) VR=ST, VM=1 Measured Dose Description
pub const MeasuredDoseDescription: u32 = 0x30080012;

/// (3008,0014) VR=CS, VM=1 Measured Dose Type
pub const MeasuredDoseType: u32 = 0x30080014;

/// (3008,0016) VR=DS, VM=1 Measured Dose Value
pub const MeasuredDoseValue: u32 = 0x30080016;

/// (3008,0020) VR=SQ, VM=1 Treatment Session Beam Sequence
pub const TreatmentSessionBeamSequence: u32 = 0x30080020;

/// (3008,0021) VR=SQ, VM=1 Treatment Session Ion Beam Sequence
pub const TreatmentSessionIonBeamSequence: u32 = 0x30080021;

/// (3008,0022) VR=IS, VM=1 Current Fraction Number
pub const CurrentFractionNumber: u32 = 0x30080022;

/// (3008,0024) VR=DA, VM=1 Treatment Control Point Date
pub const TreatmentControlPointDate: u32 = 0x30080024;

/// (3008,0025) VR=TM, VM=1 Treatment Control Point Time
pub const TreatmentControlPointTime: u32 = 0x30080025;

/// (3008,002A) VR=CS, VM=1 Treatment Termination Status
pub const TreatmentTerminationStatus: u32 = 0x3008002A;

/// (3008,002B) VR=SH, VM=1 Treatment Termination Code
pub const TreatmentTerminationCode: u32 = 0x3008002B;

/// (3008,002C) VR=CS, VM=1 Treatment Verification Status
pub const TreatmentVerificationStatus: u32 = 0x3008002C;

/// (3008,0030) VR=SQ, VM=1 Referenced Treatment Record Sequence
pub const ReferencedTreatmentRecordSequence: u32 = 0x30080030;

/// (3008,0032) VR=DS, VM=1 Specified Primary Meterset
pub const SpecifiedPrimaryMeterset: u32 = 0x30080032;

/// (3008,0033) VR=DS, VM=1 Specified Secondary Meterset
pub const SpecifiedSecondaryMeterset: u32 = 0x30080033;

/// (3008,0036) VR=DS, VM=1 Delivered Primary Meterset
pub const DeliveredPrimaryMeterset: u32 = 0x30080036;

/// (3008,0037) VR=DS, VM=1 Delivered Secondary Meterset
pub const DeliveredSecondaryMeterset: u32 = 0x30080037;

/// (3008,003A) VR=DS, VM=1 Specified Treatment Time
pub const SpecifiedTreatmentTime: u32 = 0x3008003A;

/// (3008,003B) VR=DS, VM=1 Delivered Treatment Time
pub const DeliveredTreatmentTime: u32 = 0x3008003B;

/// (3008,0040) VR=SQ, VM=1 Control Point Delivery Sequence
pub const ControlPointDeliverySequence: u32 = 0x30080040;

/// (3008,0041) VR=SQ, VM=1 Ion Control Point Delivery Sequence
pub const IonControlPointDeliverySequence: u32 = 0x30080041;

/// (3008,0042) VR=DS, VM=1 Specified Meterset
pub const SpecifiedMeterset: u32 = 0x30080042;

/// (3008,0044) VR=DS, VM=1 Delivered Meterset
pub const DeliveredMeterset: u32 = 0x30080044;

/// (3008,0045) VR=FL, VM=1 Meterset Rate Set
pub const MetersetRateSet: u32 = 0x30080045;

/// (3008,0046) VR=FL, VM=1 Meterset Rate Delivered
pub const MetersetRateDelivered: u32 = 0x30080046;

/// (3008,0047) VR=FL, VM=1-n Scan Spot Metersets Delivered
pub const ScanSpotMetersetsDelivered: u32 = 0x30080047;

/// (3008,0048) VR=DS, VM=1 Dose Rate Delivered
pub const DoseRateDelivered: u32 = 0x30080048;

/// (3008,0050) VR=SQ, VM=1 Treatment Summary Calculated Dose Reference Sequence
pub const TreatmentSummaryCalculatedDoseReferenceSequence: u32 = 0x30080050;

/// (3008,0052) VR=DS, VM=1 Cumulative Dose to Dose Reference
pub const CumulativeDoseToDoseReference: u32 = 0x30080052;

/// (3008,0054) VR=DA, VM=1 First Treatment Date
pub const FirstTreatmentDate: u32 = 0x30080054;

/// (3008,0056) VR=DA, VM=1 Most Recent Treatment Date
pub const MostRecentTreatmentDate: u32 = 0x30080056;

/// (3008,005A) VR=IS, VM=1 Number of Fractions Delivered
pub const NumberOfFractionsDelivered: u32 = 0x3008005A;

/// (3008,0060) VR=SQ, VM=1 Override Sequence
pub const OverrideSequence: u32 = 0x30080060;

/// (3008,0061) VR=AT, VM=1 Parameter Sequence Pointer
pub const ParameterSequencePointer: u32 = 0x30080061;

/// (3008,0062) VR=AT, VM=1 Override Parameter Pointer
pub const OverrideParameterPointer: u32 = 0x30080062;

/// (3008,0063) VR=IS, VM=1 Parameter Item Index
pub const ParameterItemIndex: u32 = 0x30080063;

/// (3008,0064) VR=IS, VM=1 Measured Dose Reference Number
pub const MeasuredDoseReferenceNumber: u32 = 0x30080064;

/// (3008,0065) VR=AT, VM=1 Parameter Pointer
pub const ParameterPointer: u32 = 0x30080065;

/// (3008,0066) VR=ST, VM=1 Override Reason
pub const OverrideReason: u32 = 0x30080066;

/// (3008,0068) VR=SQ, VM=1 Corrected Parameter Sequence
pub const CorrectedParameterSequence: u32 = 0x30080068;

/// (3008,006A) VR=FL, VM=1 Correction Value
pub const CorrectionValue: u32 = 0x3008006A;

/// (3008,0070) VR=SQ, VM=1 Calculated Dose Reference Sequence
pub const CalculatedDoseReferenceSequence: u32 = 0x30080070;

/// (3008,0072) VR=IS, VM=1 Calculated Dose Reference Number
pub const CalculatedDoseReferenceNumber: u32 = 0x30080072;

/// (3008,0074) VR=ST, VM=1 Calculated Dose Reference Description
pub const CalculatedDoseReferenceDescription: u32 = 0x30080074;

/// (3008,0076) VR=DS, VM=1 Calculated Dose Reference Dose Value
pub const CalculatedDoseReferenceDoseValue: u32 = 0x30080076;

/// (3008,0078) VR=DS, VM=1 Start Meterset
pub const StartMeterset: u32 = 0x30080078;

/// (3008,007A) VR=DS, VM=1 End Meterset
pub const EndMeterset: u32 = 0x3008007A;

/// (3008,0080) VR=SQ, VM=1 Referenced Measured Dose Reference Sequence
pub const ReferencedMeasuredDoseReferenceSequence: u32 = 0x30080080;

/// (3008,0082) VR=IS, VM=1 Referenced Measured Dose Reference Number
pub const ReferencedMeasuredDoseReferenceNumber: u32 = 0x30080082;

/// (3008,0090) VR=SQ, VM=1 Referenced Calculated Dose Reference Sequence
pub const ReferencedCalculatedDoseReferenceSequence: u32 = 0x30080090;

/// (3008,0092) VR=IS, VM=1 Referenced Calculated Dose Reference Number
pub const ReferencedCalculatedDoseReferenceNumber: u32 = 0x30080092;

/// (3008,00A0) VR=SQ, VM=1 Beam Limiting Device Leaf Pairs Sequence
pub const BeamLimitingDeviceLeafPairsSequence: u32 = 0x300800A0;

/// (3008,00B0) VR=SQ, VM=1 Recorded Wedge Sequence
pub const RecordedWedgeSequence: u32 = 0x300800B0;

/// (3008,00C0) VR=SQ, VM=1 Recorded Compensator Sequence
pub const RecordedCompensatorSequence: u32 = 0x300800C0;

/// (3008,00D0) VR=SQ, VM=1 Recorded Block Sequence
pub const RecordedBlockSequence: u32 = 0x300800D0;

/// (3008,00E0) VR=SQ, VM=1 Treatment Summary Measured Dose Reference Sequence
pub const TreatmentSummaryMeasuredDoseReferenceSequence: u32 = 0x300800E0;

/// (3008,00F0) VR=SQ, VM=1 Recorded Snout Sequence
pub const RecordedSnoutSequence: u32 = 0x300800F0;

/// (3008,00F2) VR=SQ, VM=1 Recorded Range Shifter Sequence
pub const RecordedRangeShifterSequence: u32 = 0x300800F2;

/// (3008,00F4) VR=SQ, VM=1 Recorded Lateral Spreading Device Sequence
pub const RecordedLateralSpreadingDeviceSequence: u32 = 0x300800F4;

/// (3008,00F6) VR=SQ, VM=1 Recorded Range Modulator Sequence
pub const RecordedRangeModulatorSequence: u32 = 0x300800F6;

/// (3008,0100) VR=SQ, VM=1 Recorded Source Sequence
pub const RecordedSourceSequence: u32 = 0x30080100;

/// (3008,0105) VR=LO, VM=1 Source Serial Number
pub const SourceSerialNumber: u32 = 0x30080105;

/// (3008,0110) VR=SQ, VM=1 Treatment Session Application Setup Sequence
pub const TreatmentSessionApplicationSetupSequence: u32 = 0x30080110;

/// (3008,0116) VR=CS, VM=1 Application Setup Check
pub const ApplicationSetupCheck: u32 = 0x30080116;

/// (3008,0120) VR=SQ, VM=1 Recorded Brachy Accessory Device Sequence
pub const RecordedBrachyAccessoryDeviceSequence: u32 = 0x30080120;

/// (3008,0122) VR=IS, VM=1 Referenced Brachy Accessory Device Number
pub const ReferencedBrachyAccessoryDeviceNumber: u32 = 0x30080122;

/// (3008,0130) VR=SQ, VM=1 Recorded Channel Sequence
pub const RecordedChannelSequence: u32 = 0x30080130;

/// (3008,0132) VR=DS, VM=1 Specified Channel Total Time
pub const SpecifiedChannelTotalTime: u32 = 0x30080132;

/// (3008,0134) VR=DS, VM=1 Delivered Channel Total Time
pub const DeliveredChannelTotalTime: u32 = 0x30080134;

/// (3008,0136) VR=IS, VM=1 Specified Number of Pulses
pub const SpecifiedNumberOfPulses: u32 = 0x30080136;

/// (3008,0138) VR=IS, VM=1 Delivered Number of Pulses
pub const DeliveredNumberOfPulses: u32 = 0x30080138;

/// (3008,013A) VR=DS, VM=1 Specified Pulse Repetition Interval
pub const SpecifiedPulseRepetitionInterval: u32 = 0x3008013A;

/// (3008,013C) VR=DS, VM=1 Delivered Pulse Repetition Interval
pub const DeliveredPulseRepetitionInterval: u32 = 0x3008013C;

/// (3008,0140) VR=SQ, VM=1 Recorded Source Applicator Sequence
pub const RecordedSourceApplicatorSequence: u32 = 0x30080140;

/// (3008,0142) VR=IS, VM=1 Referenced Source Applicator Number
pub const ReferencedSourceApplicatorNumber: u32 = 0x30080142;

/// (3008,0150) VR=SQ, VM=1 Recorded Channel Shield Sequence
pub const RecordedChannelShieldSequence: u32 = 0x30080150;

/// (3008,0152) VR=IS, VM=1 Referenced Channel Shield Number
pub const ReferencedChannelShieldNumber: u32 = 0x30080152;

/// (3008,0160) VR=SQ, VM=1 Brachy Control Point Delivered Sequence
pub const BrachyControlPointDeliveredSequence: u32 = 0x30080160;

/// (3008,0162) VR=DA, VM=1 Safe Position Exit Date
pub const SafePositionExitDate: u32 = 0x30080162;

/// (3008,0164) VR=TM, VM=1 Safe Position Exit Time
pub const SafePositionExitTime: u32 = 0x30080164;

/// (3008,0166) VR=DA, VM=1 Safe Position Return Date
pub const SafePositionReturnDate: u32 = 0x30080166;

/// (3008,0168) VR=TM, VM=1 Safe Position Return Time
pub const SafePositionReturnTime: u32 = 0x30080168;

/// (3008,0200) VR=CS, VM=1 Current Treatment Status
pub const CurrentTreatmentStatus: u32 = 0x30080200;

/// (3008,0202) VR=ST, VM=1 Treatment Status Comment
pub const TreatmentStatusComment: u32 = 0x30080202;

/// (3008,0220) VR=SQ, VM=1 Fraction Group Summary Sequence
pub const FractionGroupSummarySequence: u32 = 0x30080220;

/// (3008,0223) VR=IS, VM=1 Referenced Fraction Number
pub const ReferencedFractionNumber: u32 = 0x30080223;

/// (3008,0224) VR=CS, VM=1 Fraction Group Type
pub const FractionGroupType: u32 = 0x30080224;

/// (3008,0230) VR=CS, VM=1 Beam Stopper Position
pub const BeamStopperPosition: u32 = 0x30080230;

/// (3008,0240) VR=SQ, VM=1 Fraction Status Summary Sequence
pub const FractionStatusSummarySequence: u32 = 0x30080240;

/// (3008,0250) VR=DA, VM=1 Treatment Date
pub const TreatmentDate: u32 = 0x30080250;

/// (3008,0251) VR=TM, VM=1 Treatment Time
pub const TreatmentTime: u32 = 0x30080251;

/// (300A,0002) VR=SH, VM=1 RT Plan Label
pub const RTPlanLabel: u32 = 0x300A0002;

/// (300A,0003) VR=LO, VM=1 RT Plan Name
pub const RTPlanName: u32 = 0x300A0003;

/// (300A,0004) VR=ST, VM=1 RT Plan Description
pub const RTPlanDescription: u32 = 0x300A0004;

/// (300A,0006) VR=DA, VM=1 RT Plan Date
pub const RTPlanDate: u32 = 0x300A0006;

/// (300A,0007) VR=TM, VM=1 RT Plan Time
pub const RTPlanTime: u32 = 0x300A0007;

/// (300A,0009) VR=LO, VM=1-n Treatment Protocols
pub const TreatmentProtocols: u32 = 0x300A0009;

/// (300A,000A) VR=CS, VM=1 Plan Intent
pub const PlanIntent: u32 = 0x300A000A;

/// (300A,000B) VR=LO, VM=1-n Treatment Sites
pub const TreatmentSites: u32 = 0x300A000B;

/// (300A,000C) VR=CS, VM=1 RT Plan Geometry
pub const RTPlanGeometry: u32 = 0x300A000C;

/// (300A,000E) VR=ST, VM=1 Prescription Description
pub const PrescriptionDescription: u32 = 0x300A000E;

/// (300A,0010) VR=SQ, VM=1 Dose Reference Sequence
pub const DoseReferenceSequence: u32 = 0x300A0010;

/// (300A,0012) VR=IS, VM=1 Dose Reference Number
pub const DoseReferenceNumber: u32 = 0x300A0012;

/// (300A,0013) VR=UI, VM=1 Dose Reference UID
pub const DoseReferenceUID: u32 = 0x300A0013;

/// (300A,0014) VR=CS, VM=1 Dose Reference Structure Type
pub const DoseReferenceStructureType: u32 = 0x300A0014;

/// (300A,0015) VR=CS, VM=1 Nominal Beam Energy Unit
pub const NominalBeamEnergyUnit: u32 = 0x300A0015;

/// (300A,0016) VR=LO, VM=1 Dose Reference Description
pub const DoseReferenceDescription: u32 = 0x300A0016;

/// (300A,0018) VR=DS, VM=3 Dose Reference Point Coordinates
pub const DoseReferencePointCoordinates: u32 = 0x300A0018;

/// (300A,001A) VR=DS, VM=1 Nominal Prior Dose
pub const NominalPriorDose: u32 = 0x300A001A;

/// (300A,0020) VR=CS, VM=1 Dose Reference Type
pub const DoseReferenceType: u32 = 0x300A0020;

/// (300A,0021) VR=DS, VM=1 Constraint Weight
pub const ConstraintWeight: u32 = 0x300A0021;

/// (300A,0022) VR=DS, VM=1 Delivery Warning Dose
pub const DeliveryWarningDose: u32 = 0x300A0022;

/// (300A,0023) VR=DS, VM=1 Delivery Maximum Dose
pub const DeliveryMaximumDose: u32 = 0x300A0023;

/// (300A,0025) VR=DS, VM=1 Target Minimum Dose
pub const TargetMinimumDose: u32 = 0x300A0025;

/// (300A,0026) VR=DS, VM=1 Target Prescription Dose
pub const TargetPrescriptionDose: u32 = 0x300A0026;

/// (300A,0027) VR=DS, VM=1 Target Maximum Dose
pub const TargetMaximumDose: u32 = 0x300A0027;

/// (300A,0028) VR=DS, VM=1 Target Underdose Volume Fraction
pub const TargetUnderdoseVolumeFraction: u32 = 0x300A0028;

/// (300A,002A) VR=DS, VM=1 Organ at Risk Full-volume Dose
pub const OrganAtRiskFullVolumeDose: u32 = 0x300A002A;

/// (300A,002B) VR=DS, VM=1 Organ at Risk Limit Dose
pub const OrganAtRiskLimitDose: u32 = 0x300A002B;

/// (300A,002C) VR=DS, VM=1 Organ at Risk Maximum Dose
pub const OrganAtRiskMaximumDose: u32 = 0x300A002C;

/// (300A,002D) VR=DS, VM=1 Organ at Risk Overdose Volume Fraction
pub const OrganAtRiskOverdoseVolumeFraction: u32 = 0x300A002D;

/// (300A,0040) VR=SQ, VM=1 Tolerance Table Sequence
pub const ToleranceTableSequence: u32 = 0x300A0040;

/// (300A,0042) VR=IS, VM=1 Tolerance Table Number
pub const ToleranceTableNumber: u32 = 0x300A0042;

/// (300A,0043) VR=SH, VM=1 Tolerance Table Label
pub const ToleranceTableLabel: u32 = 0x300A0043;

/// (300A,0044) VR=DS, VM=1 Gantry Angle Tolerance
pub const GantryAngleTolerance: u32 = 0x300A0044;

/// (300A,0046) VR=DS, VM=1 Beam Limiting Device Angle Tolerance
pub const BeamLimitingDeviceAngleTolerance: u32 = 0x300A0046;

/// (300A,0048) VR=SQ, VM=1 Beam Limiting Device Tolerance Sequence
pub const BeamLimitingDeviceToleranceSequence: u32 = 0x300A0048;

/// (300A,004A) VR=DS, VM=1 Beam Limiting Device Position Tolerance
pub const BeamLimitingDevicePositionTolerance: u32 = 0x300A004A;

/// (300A,004B) VR=FL, VM=1 Snout Position Tolerance
pub const SnoutPositionTolerance: u32 = 0x300A004B;

/// (300A,004C) VR=DS, VM=1 Patient Support Angle Tolerance
pub const PatientSupportAngleTolerance: u32 = 0x300A004C;

/// (300A,004E) VR=DS, VM=1 Table Top Eccentric Angle Tolerance
pub const TableTopEccentricAngleTolerance: u32 = 0x300A004E;

/// (300A,004F) VR=FL, VM=1 Table Top Pitch Angle Tolerance
pub const TableTopPitchAngleTolerance: u32 = 0x300A004F;

/// (300A,0050) VR=FL, VM=1 Table Top Roll Angle Tolerance
pub const TableTopRollAngleTolerance: u32 = 0x300A0050;

/// (300A,0051) VR=DS, VM=1 Table Top Vertical Position Tolerance
pub const TableTopVerticalPositionTolerance: u32 = 0x300A0051;

/// (300A,0052) VR=DS, VM=1 Table Top Longitudinal Position Tolerance
pub const TableTopLongitudinalPositionTolerance: u32 = 0x300A0052;

/// (300A,0053) VR=DS, VM=1 Table Top Lateral Position Tolerance
pub const TableTopLateralPositionTolerance: u32 = 0x300A0053;

/// (300A,0055) VR=CS, VM=1 RT Plan Relationship
pub const RTPlanRelationship: u32 = 0x300A0055;

/// (300A,0070) VR=SQ, VM=1 Fraction Group Sequence
pub const FractionGroupSequence: u32 = 0x300A0070;

/// (300A,0071) VR=IS, VM=1 Fraction Group Number
pub const FractionGroupNumber: u32 = 0x300A0071;

/// (300A,0072) VR=LO, VM=1 Fraction Group Description
pub const FractionGroupDescription: u32 = 0x300A0072;

/// (300A,0078) VR=IS, VM=1 Number of Fractions Planned
pub const NumberOfFractionsPlanned: u32 = 0x300A0078;

/// (300A,0079) VR=IS, VM=1 Number of Fraction Pattern Digits Per Day
pub const NumberOfFractionPatternDigitsPerDay: u32 = 0x300A0079;

/// (300A,007A) VR=IS, VM=1 Repeat Fraction Cycle Length
pub const RepeatFractionCycleLength: u32 = 0x300A007A;

/// (300A,007B) VR=LT, VM=1 Fraction Pattern
pub const FractionPattern: u32 = 0x300A007B;

/// (300A,0080) VR=IS, VM=1 Number of Beams
pub const NumberOfBeams: u32 = 0x300A0080;

/// (300A,0082) VR=DS, VM=3 Beam Dose Specification Point
pub const BeamDoseSpecificationPoint: u32 = 0x300A0082;

/// (300A,0084) VR=DS, VM=1 Beam Dose
pub const BeamDose: u32 = 0x300A0084;

/// (300A,0086) VR=DS, VM=1 Beam Meterset
pub const BeamMeterset: u32 = 0x300A0086;

/// (300A,0088) VR=FL, VM=1 Beam Dose Point Depth
pub const BeamDosePointDepth: u32 = 0x300A0088;

/// (300A,0089) VR=FL, VM=1 Beam Dose Point Equivalent Depth
pub const BeamDosePointEquivalentDepth: u32 = 0x300A0089;

/// (300A,008A) VR=FL, VM=1 Beam Dose Point SSD
pub const BeamDosePointSSD: u32 = 0x300A008A;

/// (300A,00A0) VR=IS, VM=1 Number of Brachy Application Setups
pub const NumberOfBrachyApplicationSetups: u32 = 0x300A00A0;

/// (300A,00A2) VR=DS, VM=3 Brachy Application Setup Dose Specification Point
pub const BrachyApplicationSetupDoseSpecificationPoint: u32 = 0x300A00A2;

/// (300A,00A4) VR=DS, VM=1 Brachy Application Setup Dose
pub const BrachyApplicationSetupDose: u32 = 0x300A00A4;

/// (300A,00B0) VR=SQ, VM=1 Beam Sequence
pub const BeamSequence: u32 = 0x300A00B0;

/// (300A,00B2) VR=SH, VM=1 Treatment Machine Name
pub const TreatmentMachineName: u32 = 0x300A00B2;

/// (300A,00B3) VR=CS, VM=1 Primary Dosimeter Unit
pub const PrimaryDosimeterUnit: u32 = 0x300A00B3;

/// (300A,00B4) VR=DS, VM=1 Source-Axis Distance
pub const SourceAxisDistance: u32 = 0x300A00B4;

/// (300A,00B6) VR=SQ, VM=1 Beam Limiting Device Sequence
pub const BeamLimitingDeviceSequence: u32 = 0x300A00B6;

/// (300A,00B8) VR=CS, VM=1 RT Beam Limiting Device Type
pub const RTBeamLimitingDeviceType: u32 = 0x300A00B8;

/// (300A,00BA) VR=DS, VM=1 Source to Beam Limiting Device Distance
pub const SourceToBeamLimitingDeviceDistance: u32 = 0x300A00BA;

/// (300A,00BB) VR=FL, VM=1 Isocenter to Beam Limiting Device Distance
pub const IsocenterToBeamLimitingDeviceDistance: u32 = 0x300A00BB;

/// (300A,00BC) VR=IS, VM=1 Number of Leaf/Jaw Pairs
pub const NumberOfLeafJawPairs: u32 = 0x300A00BC;

/// (300A,00BE) VR=DS, VM=3-n Leaf Position Boundaries
pub const LeafPositionBoundaries: u32 = 0x300A00BE;

/// (300A,00C0) VR=IS, VM=1 Beam Number
pub const BeamNumber: u32 = 0x300A00C0;

/// (300A,00C2) VR=LO, VM=1 Beam Name
pub const BeamName: u32 = 0x300A00C2;

/// (300A,00C3) VR=ST, VM=1 Beam Description
pub const BeamDescription: u32 = 0x300A00C3;

/// (300A,00C4) VR=CS, VM=1 Beam Type
pub const BeamType: u32 = 0x300A00C4;

/// (300A,00C6) VR=CS, VM=1 Radiation Type
pub const RadiationType: u32 = 0x300A00C6;

/// (300A,00C7) VR=CS, VM=1 High-Dose Technique Type
pub const HighDoseTechniqueType: u32 = 0x300A00C7;

/// (300A,00C8) VR=IS, VM=1 Reference Image Number
pub const ReferenceImageNumber: u32 = 0x300A00C8;

/// (300A,00CA) VR=SQ, VM=1 Planned Verification Image Sequence
pub const PlannedVerificationImageSequence: u32 = 0x300A00CA;

/// (300A,00CC) VR=LO, VM=1-n Imaging Device-Specific Acquisition Parameters
pub const ImagingDeviceSpecificAcquisitionParameters: u32 = 0x300A00CC;

/// (300A,00CE) VR=CS, VM=1 Treatment Delivery Type
pub const TreatmentDeliveryType: u32 = 0x300A00CE;

/// (300A,00D0) VR=IS, VM=1 Number of Wedges
pub const NumberOfWedges: u32 = 0x300A00D0;

/// (300A,00D1) VR=SQ, VM=1 Wedge Sequence
pub const WedgeSequence: u32 = 0x300A00D1;

/// (300A,00D2) VR=IS, VM=1 Wedge Number
pub const WedgeNumber: u32 = 0x300A00D2;

/// (300A,00D3) VR=CS, VM=1 Wedge Type
pub const WedgeType: u32 = 0x300A00D3;

/// (300A,00D4) VR=SH, VM=1 Wedge ID
pub const WedgeID: u32 = 0x300A00D4;

/// (300A,00D5) VR=IS, VM=1 Wedge Angle
pub const WedgeAngle: u32 = 0x300A00D5;

/// (300A,00D6) VR=DS, VM=1 Wedge Factor
pub const WedgeFactor: u32 = 0x300A00D6;

/// (300A,00D7) VR=FL, VM=1 Total Wedge Tray Water-Equivalent Thickness
pub const TotalWedgeTrayWaterEquivalentThickness: u32 = 0x300A00D7;

/// (300A,00D8) VR=DS, VM=1 Wedge Orientation
pub const WedgeOrientation: u32 = 0x300A00D8;

/// (300A,00D9) VR=FL, VM=1 Isocenter to Wedge Tray Distance
pub const IsocenterToWedgeTrayDistance: u32 = 0x300A00D9;

/// (300A,00DA) VR=DS, VM=1 Source to Wedge Tray Distance
pub const SourceToWedgeTrayDistance: u32 = 0x300A00DA;

/// (300A,00DB) VR=FL, VM=1 Wedge Thin Edge Position
pub const WedgeThinEdgePosition: u32 = 0x300A00DB;

/// (300A,00DC) VR=SH, VM=1 Bolus ID
pub const BolusID: u32 = 0x300A00DC;

/// (300A,00DD) VR=ST, VM=1 Bolus Description
pub const BolusDescription: u32 = 0x300A00DD;

/// (300A,00E0) VR=IS, VM=1 Number of Compensators
pub const NumberOfCompensators: u32 = 0x300A00E0;

/// (300A,00E1) VR=SH, VM=1 Material ID
pub const MaterialID: u32 = 0x300A00E1;

/// (300A,00E2) VR=DS, VM=1 Total Compensator Tray Factor
pub const TotalCompensatorTrayFactor: u32 = 0x300A00E2;

/// (300A,00E3) VR=SQ, VM=1 Compensator Sequence
pub const CompensatorSequence: u32 = 0x300A00E3;

/// (300A,00E4) VR=IS, VM=1 Compensator Number
pub const CompensatorNumber: u32 = 0x300A00E4;

/// (300A,00E5) VR=SH, VM=1 Compensator ID
pub const CompensatorID: u32 = 0x300A00E5;

/// (300A,00E6) VR=DS, VM=1 Source to Compensator Tray Distance
pub const SourceToCompensatorTrayDistance: u32 = 0x300A00E6;

/// (300A,00E7) VR=IS, VM=1 Compensator Rows
pub const CompensatorRows: u32 = 0x300A00E7;

/// (300A,00E8) VR=IS, VM=1 Compensator Columns
pub const CompensatorColumns: u32 = 0x300A00E8;

/// (300A,00E9) VR=DS, VM=2 Compensator Pixel Spacing
pub const CompensatorPixelSpacing: u32 = 0x300A00E9;

/// (300A,00EA) VR=DS, VM=2 Compensator Position
pub const CompensatorPosition: u32 = 0x300A00EA;

/// (300A,00EB) VR=DS, VM=1-n Compensator Transmission Data
pub const CompensatorTransmissionData: u32 = 0x300A00EB;

/// (300A,00EC) VR=DS, VM=1-n Compensator Thickness Data
pub const CompensatorThicknessData: u32 = 0x300A00EC;

/// (300A,00ED) VR=IS, VM=1 Number of Boli
pub const NumberOfBoli: u32 = 0x300A00ED;

/// (300A,00EE) VR=CS, VM=1 Compensator Type
pub const CompensatorType: u32 = 0x300A00EE;

/// (300A,00F0) VR=IS, VM=1 Number of Blocks
pub const NumberOfBlocks: u32 = 0x300A00F0;

/// (300A,00F2) VR=DS, VM=1 Total Block Tray Factor
pub const TotalBlockTrayFactor: u32 = 0x300A00F2;

/// (300A,00F3) VR=FL, VM=1 Total Block Tray Water-Equivalent Thickness
pub const TotalBlockTrayWaterEquivalentThickness: u32 = 0x300A00F3;

/// (300A,00F4) VR=SQ, VM=1 Block Sequence
pub const BlockSequence: u32 = 0x300A00F4;

/// (300A,00F5) VR=SH, VM=1 Block Tray ID
pub const BlockTrayID: u32 = 0x300A00F5;

/// (300A,00F6) VR=DS, VM=1 Source to Block Tray Distance
pub const SourceToBlockTrayDistance: u32 = 0x300A00F6;

/// (300A,00F7) VR=FL, VM=1 Isocenter to Block Tray Distance
pub const IsocenterToBlockTrayDistance: u32 = 0x300A00F7;

/// (300A,00F8) VR=CS, VM=1 Block Type
pub const BlockType: u32 = 0x300A00F8;

/// (300A,00F9) VR=LO, VM=1 Accessory Code
pub const AccessoryCode: u32 = 0x300A00F9;

/// (300A,00FA) VR=CS, VM=1 Block Divergence
pub const BlockDivergence: u32 = 0x300A00FA;

/// (300A,00FB) VR=CS, VM=1 Block Mounting Position
pub const BlockMountingPosition: u32 = 0x300A00FB;

/// (300A,00FC) VR=IS, VM=1 Block Number
pub const BlockNumber: u32 = 0x300A00FC;

/// (300A,00FE) VR=LO, VM=1 Block Name
pub const BlockName: u32 = 0x300A00FE;

/// (300A,0100) VR=DS, VM=1 Block Thickness
pub const BlockThickness: u32 = 0x300A0100;

/// (300A,0102) VR=DS, VM=1 Block Transmission
pub const BlockTransmission: u32 = 0x300A0102;

/// (300A,0104) VR=IS, VM=1 Block Number of Points
pub const BlockNumberOfPoints: u32 = 0x300A0104;

/// (300A,0106) VR=DS, VM=2-2n Block Data
pub const BlockData: u32 = 0x300A0106;

/// (300A,0107) VR=SQ, VM=1 Applicator Sequence
pub const ApplicatorSequence: u32 = 0x300A0107;

/// (300A,0108) VR=SH, VM=1 Applicator ID
pub const ApplicatorID: u32 = 0x300A0108;

/// (300A,0109) VR=CS, VM=1 Applicator Type
pub const ApplicatorType: u32 = 0x300A0109;

/// (300A,010A) VR=LO, VM=1 Applicator Description
pub const ApplicatorDescription: u32 = 0x300A010A;

/// (300A,010C) VR=DS, VM=1 Cumulative Dose Reference Coefficient
pub const CumulativeDoseReferenceCoefficient: u32 = 0x300A010C;

/// (300A,010E) VR=DS, VM=1 Final Cumulative Meterset Weight
pub const FinalCumulativeMetersetWeight: u32 = 0x300A010E;

/// (300A,0110) VR=IS, VM=1 Number of Control Points
pub const NumberOfControlPoints: u32 = 0x300A0110;

/// (300A,0111) VR=SQ, VM=1 Control Point Sequence
pub const ControlPointSequence: u32 = 0x300A0111;

/// (300A,0112) VR=IS, VM=1 Control Point Index
pub const ControlPointIndex: u32 = 0x300A0112;

/// (300A,0114) VR=DS, VM=1 Nominal Beam Energy
pub const NominalBeamEnergy: u32 = 0x300A0114;

/// (300A,0115) VR=DS, VM=1 Dose Rate Set
pub const DoseRateSet: u32 = 0x300A0115;

/// (300A,0116) VR=SQ, VM=1 Wedge Position Sequence
pub const WedgePositionSequence: u32 = 0x300A0116;

/// (300A,0118) VR=CS, VM=1 Wedge Position
pub const WedgePosition: u32 = 0x300A0118;

/// (300A,011A) VR=SQ, VM=1 Beam Limiting Device Position Sequence
pub const BeamLimitingDevicePositionSequence: u32 = 0x300A011A;

/// (300A,011C) VR=DS, VM=2-2n Leaf/Jaw Positions
pub const LeafJawPositions: u32 = 0x300A011C;

/// (300A,011E) VR=DS, VM=1 Gantry Angle
pub const GantryAngle: u32 = 0x300A011E;

/// (300A,011F) VR=CS, VM=1 Gantry Rotation Direction
pub const GantryRotationDirection: u32 = 0x300A011F;

/// (300A,0120) VR=DS, VM=1 Beam Limiting Device Angle
pub const BeamLimitingDeviceAngle: u32 = 0x300A0120;

/// (300A,0121) VR=CS, VM=1 Beam Limiting Device Rotation Direction
pub const BeamLimitingDeviceRotationDirection: u32 = 0x300A0121;

/// (300A,0122) VR=DS, VM=1 Patient Support Angle
pub const PatientSupportAngle: u32 = 0x300A0122;

/// (300A,0123) VR=CS, VM=1 Patient Support Rotation Direction
pub const PatientSupportRotationDirection: u32 = 0x300A0123;

/// (300A,0124) VR=DS, VM=1 Table Top Eccentric Axis Distance
pub const TableTopEccentricAxisDistance: u32 = 0x300A0124;

/// (300A,0125) VR=DS, VM=1 Table Top Eccentric Angle
pub const TableTopEccentricAngle: u32 = 0x300A0125;

/// (300A,0126) VR=CS, VM=1 Table Top Eccentric Rotation Direction
pub const TableTopEccentricRotationDirection: u32 = 0x300A0126;

/// (300A,0128) VR=DS, VM=1 Table Top Vertical Position
pub const TableTopVerticalPosition: u32 = 0x300A0128;

/// (300A,0129) VR=DS, VM=1 Table Top Longitudinal Position
pub const TableTopLongitudinalPosition: u32 = 0x300A0129;

/// (300A,012A) VR=DS, VM=1 Table Top Lateral Position
pub const TableTopLateralPosition: u32 = 0x300A012A;

/// (300A,012C) VR=DS, VM=3 Isocenter Position
pub const IsocenterPosition: u32 = 0x300A012C;

/// (300A,012E) VR=DS, VM=3 Surface Entry Point
pub const SurfaceEntryPoint: u32 = 0x300A012E;

/// (300A,0130) VR=DS, VM=1 Source to Surface Distance
pub const SourceToSurfaceDistance: u32 = 0x300A0130;

/// (300A,0134) VR=DS, VM=1 Cumulative Meterset Weight
pub const CumulativeMetersetWeight: u32 = 0x300A0134;

/// (300A,0140) VR=FL, VM=1 Table Top Pitch Angle
pub const TableTopPitchAngle: u32 = 0x300A0140;

/// (300A,0142) VR=CS, VM=1 Table Top Pitch Rotation Direction
pub const TableTopPitchRotationDirection: u32 = 0x300A0142;

/// (300A,0144) VR=FL, VM=1 Table Top Roll Angle
pub const TableTopRollAngle: u32 = 0x300A0144;

/// (300A,0146) VR=CS, VM=1 Table Top Roll Rotation Direction
pub const TableTopRollRotationDirection: u32 = 0x300A0146;

/// (300A,0148) VR=FL, VM=1 Head Fixation Angle
pub const HeadFixationAngle: u32 = 0x300A0148;

/// (300A,014A) VR=FL, VM=1 Gantry Pitch Angle
pub const GantryPitchAngle: u32 = 0x300A014A;

/// (300A,014C) VR=CS, VM=1 Gantry Pitch Rotation Direction
pub const GantryPitchRotationDirection: u32 = 0x300A014C;

/// (300A,014E) VR=FL, VM=1 Gantry Pitch Angle Tolerance
pub const GantryPitchAngleTolerance: u32 = 0x300A014E;

/// (300A,0180) VR=SQ, VM=1 Patient Setup Sequence
pub const PatientSetupSequence: u32 = 0x300A0180;

/// (300A,0182) VR=IS, VM=1 Patient Setup Number
pub const PatientSetupNumber: u32 = 0x300A0182;

/// (300A,0183) VR=LO, VM=1 Patient Setup Label
pub const PatientSetupLabel: u32 = 0x300A0183;

/// (300A,0184) VR=LO, VM=1 Patient Additional Position
pub const PatientAdditionalPosition: u32 = 0x300A0184;

/// (300A,0190) VR=SQ, VM=1 Fixation Device Sequence
pub const FixationDeviceSequence: u32 = 0x300A0190;

/// (300A,0192) VR=CS, VM=1 Fixation Device Type
pub const FixationDeviceType: u32 = 0x300A0192;

/// (300A,0194) VR=SH, VM=1 Fixation Device Label
pub const FixationDeviceLabel: u32 = 0x300A0194;

/// (300A,0196) VR=ST, VM=1 Fixation Device Description
pub const FixationDeviceDescription: u32 = 0x300A0196;

/// (300A,0198) VR=SH, VM=1 Fixation Device Position
pub const FixationDevicePosition: u32 = 0x300A0198;

/// (300A,0199) VR=FL, VM=1 Fixation Device Pitch Angle
pub const FixationDevicePitchAngle: u32 = 0x300A0199;

/// (300A,019A) VR=FL, VM=1 Fixation Device Roll Angle
pub const FixationDeviceRollAngle: u32 = 0x300A019A;

/// (300A,01A0) VR=SQ, VM=1 Shielding Device Sequence
pub const ShieldingDeviceSequence: u32 = 0x300A01A0;

/// (300A,01A2) VR=CS, VM=1 Shielding Device Type
pub const ShieldingDeviceType: u32 = 0x300A01A2;

/// (300A,01A4) VR=SH, VM=1 Shielding Device Label
pub const ShieldingDeviceLabel: u32 = 0x300A01A4;

/// (300A,01A6) VR=ST, VM=1 Shielding Device Description
pub const ShieldingDeviceDescription: u32 = 0x300A01A6;

/// (300A,01A8) VR=SH, VM=1 Shielding Device Position
pub const ShieldingDevicePosition: u32 = 0x300A01A8;

/// (300A,01B0) VR=CS, VM=1 Setup Technique
pub const SetupTechnique: u32 = 0x300A01B0;

/// (300A,01B2) VR=ST, VM=1 Setup Technique Description
pub const SetupTechniqueDescription: u32 = 0x300A01B2;

/// (300A,01B4) VR=SQ, VM=1 Setup Device Sequence
pub const SetupDeviceSequence: u32 = 0x300A01B4;

/// (300A,01B6) VR=CS, VM=1 Setup Device Type
pub const SetupDeviceType: u32 = 0x300A01B6;

/// (300A,01B8) VR=SH, VM=1 Setup Device Label
pub const SetupDeviceLabel: u32 = 0x300A01B8;

/// (300A,01BA) VR=ST, VM=1 Setup Device Description
pub const SetupDeviceDescription: u32 = 0x300A01BA;

/// (300A,01BC) VR=DS, VM=1 Setup Device Parameter
pub const SetupDeviceParameter: u32 = 0x300A01BC;

/// (300A,01D0) VR=ST, VM=1 Setup Reference Description
pub const SetupReferenceDescription: u32 = 0x300A01D0;

/// (300A,01D2) VR=DS, VM=1 Table Top Vertical Setup Displacement
pub const TableTopVerticalSetupDisplacement: u32 = 0x300A01D2;

/// (300A,01D4) VR=DS, VM=1 Table Top Longitudinal Setup Displacement
pub const TableTopLongitudinalSetupDisplacement: u32 = 0x300A01D4;

/// (300A,01D6) VR=DS, VM=1 Table Top Lateral Setup Displacement
pub const TableTopLateralSetupDisplacement: u32 = 0x300A01D6;

/// (300A,0200) VR=CS, VM=1 Brachy Treatment Technique
pub const BrachyTreatmentTechnique: u32 = 0x300A0200;

/// (300A,0202) VR=CS, VM=1 Brachy Treatment Type
pub const BrachyTreatmentType: u32 = 0x300A0202;

/// (300A,0206) VR=SQ, VM=1 Treatment Machine Sequence
pub const TreatmentMachineSequence: u32 = 0x300A0206;

/// (300A,0210) VR=SQ, VM=1 Source Sequence
pub const SourceSequence: u32 = 0x300A0210;

/// (300A,0212) VR=IS, VM=1 Source Number
pub const SourceNumber: u32 = 0x300A0212;

/// (300A,0214) VR=CS, VM=1 Source Type
pub const SourceType: u32 = 0x300A0214;

/// (300A,0216) VR=LO, VM=1 Source Manufacturer
pub const SourceManufacturer: u32 = 0x300A0216;

/// (300A,0218) VR=DS, VM=1 Active Source Diameter
pub const ActiveSourceDiameter: u32 = 0x300A0218;

/// (300A,021A) VR=DS, VM=1 Active Source Length
pub const ActiveSourceLength: u32 = 0x300A021A;

/// (300A,0222) VR=DS, VM=1 Source Encapsulation Nominal Thickness
pub const SourceEncapsulationNominalThickness: u32 = 0x300A0222;

/// (300A,0224) VR=DS, VM=1 Source Encapsulation Nominal Transmission
pub const SourceEncapsulationNominalTransmission: u32 = 0x300A0224;

/// (300A,0226) VR=LO, VM=1 Source Isotope Name
pub const SourceIsotopeName: u32 = 0x300A0226;

/// (300A,0228) VR=DS, VM=1 Source Isotope Half Life
pub const SourceIsotopeHalfLife: u32 = 0x300A0228;

/// (300A,0229) VR=CS, VM=1 Source Strength Units
pub const SourceStrengthUnits: u32 = 0x300A0229;

/// (300A,022A) VR=DS, VM=1 Reference Air Kerma Rate
pub const ReferenceAirKermaRate: u32 = 0x300A022A;

/// (300A,022B) VR=DS, VM=1 Source Strength
pub const SourceStrength: u32 = 0x300A022B;

/// (300A,022C) VR=DA, VM=1 Source Strength Reference Date
pub const SourceStrengthReferenceDate: u32 = 0x300A022C;

/// (300A,022E) VR=TM, VM=1 Source Strength Reference Time
pub const SourceStrengthReferenceTime: u32 = 0x300A022E;

/// (300A,0230) VR=SQ, VM=1 Application Setup Sequence
pub const ApplicationSetupSequence: u32 = 0x300A0230;

/// (300A,0232) VR=CS, VM=1 Application Setup Type
pub const ApplicationSetupType: u32 = 0x300A0232;

/// (300A,0234) VR=IS, VM=1 Application Setup Number
pub const ApplicationSetupNumber: u32 = 0x300A0234;

/// (300A,0236) VR=LO, VM=1 Application Setup Name
pub const ApplicationSetupName: u32 = 0x300A0236;

/// (300A,0238) VR=LO, VM=1 Application Setup Manufacturer
pub const ApplicationSetupManufacturer: u32 = 0x300A0238;

/// (300A,0240) VR=IS, VM=1 Template Number
pub const TemplateNumber: u32 = 0x300A0240;

/// (300A,0242) VR=SH, VM=1 Template Type
pub const TemplateType: u32 = 0x300A0242;

/// (300A,0244) VR=LO, VM=1 Template Name
pub const TemplateName: u32 = 0x300A0244;

/// (300A,0250) VR=DS, VM=1 Total Reference Air Kerma
pub const TotalReferenceAirKerma: u32 = 0x300A0250;

/// (300A,0260) VR=SQ, VM=1 Brachy Accessory Device Sequence
pub const BrachyAccessoryDeviceSequence: u32 = 0x300A0260;

/// (300A,0262) VR=IS, VM=1 Brachy Accessory Device Number
pub const BrachyAccessoryDeviceNumber: u32 = 0x300A0262;

/// (300A,0263) VR=SH, VM=1 Brachy Accessory Device ID
pub const BrachyAccessoryDeviceID: u32 = 0x300A0263;

/// (300A,0264) VR=CS, VM=1 Brachy Accessory Device Type
pub const BrachyAccessoryDeviceType: u32 = 0x300A0264;

/// (300A,0266) VR=LO, VM=1 Brachy Accessory Device Name
pub const BrachyAccessoryDeviceName: u32 = 0x300A0266;

/// (300A,026A) VR=DS, VM=1 Brachy Accessory Device Nominal Thickness
pub const BrachyAccessoryDeviceNominalThickness: u32 = 0x300A026A;

/// (300A,026C) VR=DS, VM=1 Brachy Accessory Device Nominal Transmission
pub const BrachyAccessoryDeviceNominalTransmission: u32 = 0x300A026C;

/// (300A,0280) VR=SQ, VM=1 Channel Sequence
pub const ChannelSequence: u32 = 0x300A0280;

/// (300A,0282) VR=IS, VM=1 Channel Number
pub const ChannelNumber: u32 = 0x300A0282;

/// (300A,0284) VR=DS, VM=1 Channel Length
pub const ChannelLength: u32 = 0x300A0284;

/// (300A,0286) VR=DS, VM=1 Channel Total Time
pub const ChannelTotalTime: u32 = 0x300A0286;

/// (300A,0288) VR=CS, VM=1 Source Movement Type
pub const SourceMovementType: u32 = 0x300A0288;

/// (300A,028A) VR=IS, VM=1 Number of Pulses
pub const NumberOfPulses: u32 = 0x300A028A;

/// (300A,028C) VR=DS, VM=1 Pulse Repetition Interval
pub const PulseRepetitionInterval: u32 = 0x300A028C;

/// (300A,0290) VR=IS, VM=1 Source Applicator Number
pub const SourceApplicatorNumber: u32 = 0x300A0290;

/// (300A,0291) VR=SH, VM=1 Source Applicator ID
pub const SourceApplicatorID: u32 = 0x300A0291;

/// (300A,0292) VR=CS, VM=1 Source Applicator Type
pub const SourceApplicatorType: u32 = 0x300A0292;

/// (300A,0294) VR=LO, VM=1 Source Applicator Name
pub const SourceApplicatorName: u32 = 0x300A0294;

/// (300A,0296) VR=DS, VM=1 Source Applicator Length
pub const SourceApplicatorLength: u32 = 0x300A0296;

/// (300A,0298) VR=LO, VM=1 Source Applicator Manufacturer
pub const SourceApplicatorManufacturer: u32 = 0x300A0298;

/// (300A,029C) VR=DS, VM=1 Source Applicator Wall Nominal Thickness
pub const SourceApplicatorWallNominalThickness: u32 = 0x300A029C;

/// (300A,029E) VR=DS, VM=1 Source Applicator Wall Nominal Transmission
pub const SourceApplicatorWallNominalTransmission: u32 = 0x300A029E;

/// (300A,02A0) VR=DS, VM=1 Source Applicator Step Size
pub const SourceApplicatorStepSize: u32 = 0x300A02A0;

/// (300A,02A2) VR=IS, VM=1 Transfer Tube Number
pub const TransferTubeNumber: u32 = 0x300A02A2;

/// (300A,02A4) VR=DS, VM=1 Transfer Tube Length
pub const TransferTubeLength: u32 = 0x300A02A4;

/// (300A,02B0) VR=SQ, VM=1 Channel Shield Sequence
pub const ChannelShieldSequence: u32 = 0x300A02B0;

/// (300A,02B2) VR=IS, VM=1 Channel Shield Number
pub const ChannelShieldNumber: u32 = 0x300A02B2;

/// (300A,02B3) VR=SH, VM=1 Channel Shield ID
pub const ChannelShieldID: u32 = 0x300A02B3;

/// (300A,02B4) VR=LO, VM=1 Channel Shield Name
pub const ChannelShieldName: u32 = 0x300A02B4;

/// (300A,02B8) VR=DS, VM=1 Channel Shield Nominal Thickness
pub const ChannelShieldNominalThickness: u32 = 0x300A02B8;

/// (300A,02BA) VR=DS, VM=1 Channel Shield Nominal Transmission
pub const ChannelShieldNominalTransmission: u32 = 0x300A02BA;

/// (300A,02C8) VR=DS, VM=1 Final Cumulative Time Weight
pub const FinalCumulativeTimeWeight: u32 = 0x300A02C8;

/// (300A,02D0) VR=SQ, VM=1 Brachy Control Point Sequence
pub const BrachyControlPointSequence: u32 = 0x300A02D0;

/// (300A,02D2) VR=DS, VM=1 Control Point Relative Position
pub const ControlPointRelativePosition: u32 = 0x300A02D2;

/// (300A,02D4) VR=DS, VM=3 Control Point 3D Position
pub const ControlPoint3DPosition: u32 = 0x300A02D4;

/// (300A,02D6) VR=DS, VM=1 Cumulative Time Weight
pub const CumulativeTimeWeight: u32 = 0x300A02D6;

/// (300A,02E0) VR=CS, VM=1 Compensator Divergence
pub const CompensatorDivergence: u32 = 0x300A02E0;

/// (300A,02E1) VR=CS, VM=1 Compensator Mounting Position
pub const CompensatorMountingPosition: u32 = 0x300A02E1;

/// (300A,02E2) VR=DS, VM=1-n Source to Compensator Distance
pub const SourceToCompensatorDistance: u32 = 0x300A02E2;

/// (300A,02E3) VR=FL, VM=1 Total Compensator Tray Water-Equivalent Thickness
pub const TotalCompensatorTrayWaterEquivalentThickness: u32 = 0x300A02E3;

/// (300A,02E4) VR=FL, VM=1 Isocenter to Compensator Tray Distance
pub const IsocenterToCompensatorTrayDistance: u32 = 0x300A02E4;

/// (300A,02E5) VR=FL, VM=1 Compensator Column Offset
pub const CompensatorColumnOffset: u32 = 0x300A02E5;

/// (300A,02E6) VR=FL, VM=1-n Isocenter to Compensator Distances
pub const IsocenterToCompensatorDistances: u32 = 0x300A02E6;

/// (300A,02E7) VR=FL, VM=1 Compensator Relative Stopping Power Ratio
pub const CompensatorRelativeStoppingPowerRatio: u32 = 0x300A02E7;

/// (300A,02E8) VR=FL, VM=1 Compensator Milling Tool Diameter
pub const CompensatorMillingToolDiameter: u32 = 0x300A02E8;

/// (300A,02EA) VR=SQ, VM=1 Ion Range Compensator Sequence
pub const IonRangeCompensatorSequence: u32 = 0x300A02EA;

/// (300A,02EB) VR=LT, VM=1 Compensator Description
pub const CompensatorDescription: u32 = 0x300A02EB;

/// (300A,0302) VR=IS, VM=1 Radiation Mass Number
pub const RadiationMassNumber: u32 = 0x300A0302;

/// (300A,0304) VR=IS, VM=1 Radiation Atomic Number
pub const RadiationAtomicNumber: u32 = 0x300A0304;

/// (300A,0306) VR=SS, VM=1 Radiation Charge State
pub const RadiationChargeState: u32 = 0x300A0306;

/// (300A,0308) VR=CS, VM=1 Scan Mode
pub const ScanMode: u32 = 0x300A0308;

/// (300A,030A) VR=FL, VM=2 Virtual Source-Axis Distances
pub const VirtualSourceAxisDistances: u32 = 0x300A030A;

/// (300A,030C) VR=SQ, VM=1 Snout Sequence
pub const SnoutSequence: u32 = 0x300A030C;

/// (300A,030D) VR=FL, VM=1 Snout Position
pub const SnoutPosition: u32 = 0x300A030D;

/// (300A,030F) VR=SH, VM=1 Snout ID
pub const SnoutID: u32 = 0x300A030F;

/// (300A,0312) VR=IS, VM=1 Number of Range Shifters
pub const NumberOfRangeShifters: u32 = 0x300A0312;

/// (300A,0314) VR=SQ, VM=1 Range Shifter Sequence
pub const RangeShifterSequence: u32 = 0x300A0314;

/// (300A,0316) VR=IS, VM=1 Range Shifter Number
pub const RangeShifterNumber: u32 = 0x300A0316;

/// (300A,0318) VR=SH, VM=1 Range Shifter ID
pub const RangeShifterID: u32 = 0x300A0318;

/// (300A,0320) VR=CS, VM=1 Range Shifter Type
pub const RangeShifterType: u32 = 0x300A0320;

/// (300A,0322) VR=LO, VM=1 Range Shifter Description
pub const RangeShifterDescription: u32 = 0x300A0322;

/// (300A,0330) VR=IS, VM=1 Number of Lateral Spreading Devices
pub const NumberOfLateralSpreadingDevices: u32 = 0x300A0330;

/// (300A,0332) VR=SQ, VM=1 Lateral Spreading Device Sequence
pub const LateralSpreadingDeviceSequence: u32 = 0x300A0332;

/// (300A,0334) VR=IS, VM=1 Lateral Spreading Device Number
pub const LateralSpreadingDeviceNumber: u32 = 0x300A0334;

/// (300A,0336) VR=SH, VM=1 Lateral Spreading Device ID
pub const LateralSpreadingDeviceID: u32 = 0x300A0336;

/// (300A,0338) VR=CS, VM=1 Lateral Spreading Device Type
pub const LateralSpreadingDeviceType: u32 = 0x300A0338;

/// (300A,033A) VR=LO, VM=1 Lateral Spreading Device Description
pub const LateralSpreadingDeviceDescription: u32 = 0x300A033A;

/// (300A,033C) VR=FL, VM=1 Lateral Spreading Device Water Equivalent Thickness
pub const LateralSpreadingDeviceWaterEquivalentThickness: u32 = 0x300A033C;

/// (300A,0340) VR=IS, VM=1 Number of Range Modulators
pub const NumberOfRangeModulators: u32 = 0x300A0340;

/// (300A,0342) VR=SQ, VM=1 Range Modulator Sequence
pub const RangeModulatorSequence: u32 = 0x300A0342;

/// (300A,0344) VR=IS, VM=1 Range Modulator Number
pub const RangeModulatorNumber: u32 = 0x300A0344;

/// (300A,0346) VR=SH, VM=1 Range Modulator ID
pub const RangeModulatorID: u32 = 0x300A0346;

/// (300A,0348) VR=CS, VM=1 Range Modulator Type
pub const RangeModulatorType: u32 = 0x300A0348;

/// (300A,034A) VR=LO, VM=1 Range Modulator Description
pub const RangeModulatorDescription: u32 = 0x300A034A;

/// (300A,034C) VR=SH, VM=1 Beam Current Modulation ID
pub const BeamCurrentModulationID: u32 = 0x300A034C;

/// (300A,0350) VR=CS, VM=1 Patient Support Type
pub const PatientSupportType: u32 = 0x300A0350;

/// (300A,0352) VR=SH, VM=1 Patient Support ID
pub const PatientSupportID: u32 = 0x300A0352;

/// (300A,0354) VR=LO, VM=1 Patient Support Accessory Code
pub const PatientSupportAccessoryCode: u32 = 0x300A0354;

/// (300A,0356) VR=FL, VM=1 Fixation Light Azimuthal Angle
pub const FixationLightAzimuthalAngle: u32 = 0x300A0356;

/// (300A,0358) VR=FL, VM=1 Fixation Light Polar Angle
pub const FixationLightPolarAngle: u32 = 0x300A0358;

/// (300A,035A) VR=FL, VM=1 Meterset Rate
pub const MetersetRate: u32 = 0x300A035A;

/// (300A,0360) VR=SQ, VM=1 Range Shifter Settings Sequence
pub const RangeShifterSettingsSequence: u32 = 0x300A0360;

/// (300A,0362) VR=LO, VM=1 Range Shifter Setting
pub const RangeShifterSetting: u32 = 0x300A0362;

/// (300A,0364) VR=FL, VM=1 Isocenter to Range Shifter Distance
pub const IsocenterToRangeShifterDistance: u32 = 0x300A0364;

/// (300A,0366) VR=FL, VM=1 Range Shifter Water Equivalent Thickness
pub const RangeShifterWaterEquivalentThickness: u32 = 0x300A0366;

/// (300A,0370) VR=SQ, VM=1 Lateral Spreading Device Settings Sequence
pub const LateralSpreadingDeviceSettingsSequence: u32 = 0x300A0370;

/// (300A,0372) VR=LO, VM=1 Lateral Spreading Device Setting
pub const LateralSpreadingDeviceSetting: u32 = 0x300A0372;

/// (300A,0374) VR=FL, VM=1 Isocenter to Lateral Spreading Device Distance
pub const IsocenterToLateralSpreadingDeviceDistance: u32 = 0x300A0374;

/// (300A,0380) VR=SQ, VM=1 Range Modulator Settings Sequence
pub const RangeModulatorSettingsSequence: u32 = 0x300A0380;

/// (300A,0382) VR=FL, VM=1 Range Modulator Gating Start Value
pub const RangeModulatorGatingStartValue: u32 = 0x300A0382;

/// (300A,0384) VR=FL, VM=1 Range Modulator Gating Stop Value
pub const RangeModulatorGatingStopValue: u32 = 0x300A0384;

/// (300A,0386) VR=FL, VM=1 Range Modulator Gating Start Water Equivalent Thickness
pub const RangeModulatorGatingStartWaterEquivalentThickness: u32 = 0x300A0386;

/// (300A,0388) VR=FL, VM=1 Range Modulator Gating Stop Water Equivalent Thickness
pub const RangeModulatorGatingStopWaterEquivalentThickness: u32 = 0x300A0388;

/// (300A,038A) VR=FL, VM=1 Isocenter to Range Modulator Distance
pub const IsocenterToRangeModulatorDistance: u32 = 0x300A038A;

/// (300A,0390) VR=SH, VM=1 Scan Spot Tune ID
pub const ScanSpotTuneID: u32 = 0x300A0390;

/// (300A,0392) VR=IS, VM=1 Number of Scan Spot Positions
pub const NumberOfScanSpotPositions: u32 = 0x300A0392;

/// (300A,0394) VR=FL, VM=1-n Scan Spot Position Map
pub const ScanSpotPositionMap: u32 = 0x300A0394;

/// (300A,0396) VR=FL, VM=1-n Scan Spot Meterset Weights
pub const ScanSpotMetersetWeights: u32 = 0x300A0396;

/// (300A,0398) VR=FL, VM=2 Scanning Spot Size
pub const ScanningSpotSize: u32 = 0x300A0398;

/// (300A,039A) VR=IS, VM=1 Number of Paintings
pub const NumberOfPaintings: u32 = 0x300A039A;

/// (300A,03A0) VR=SQ, VM=1 Ion Tolerance Table Sequence
pub const IonToleranceTableSequence: u32 = 0x300A03A0;

/// (300A,03A2) VR=SQ, VM=1 Ion Beam Sequence
pub const IonBeamSequence: u32 = 0x300A03A2;

/// (300A,03A4) VR=SQ, VM=1 Ion Beam Limiting Device Sequence
pub const IonBeamLimitingDeviceSequence: u32 = 0x300A03A4;

/// (300A,03A6) VR=SQ, VM=1 Ion Block Sequence
pub const IonBlockSequence: u32 = 0x300A03A6;

/// (300A,03A8) VR=SQ, VM=1 Ion Control Point Sequence
pub const IonControlPointSequence: u32 = 0x300A03A8;

/// (300A,03AA) VR=SQ, VM=1 Ion Wedge Sequence
pub const IonWedgeSequence: u32 = 0x300A03AA;

/// (300A,03AC) VR=SQ, VM=1 Ion Wedge Position Sequence
pub const IonWedgePositionSequence: u32 = 0x300A03AC;

/// (300A,0401) VR=SQ, VM=1 Referenced Setup Image Sequence
pub const ReferencedSetupImageSequence: u32 = 0x300A0401;

/// (300A,0402) VR=ST, VM=1 Setup Image Comment
pub const SetupImageComment: u32 = 0x300A0402;

/// (300A,0410) VR=SQ, VM=1 Motion Synchronization Sequence
pub const MotionSynchronizationSequence: u32 = 0x300A0410;

/// (300A,0412) VR=FL, VM=3 Control Point Orientation
pub const ControlPointOrientation: u32 = 0x300A0412;

/// (300A,0420) VR=SQ, VM=1 General Accessory Sequence
pub const GeneralAccessorySequence: u32 = 0x300A0420;

/// (300A,0421) VR=SH, VM=1 General Accessory ID
pub const GeneralAccessoryID: u32 = 0x300A0421;

/// (300A,0422) VR=ST, VM=1 General Accessory Description
pub const GeneralAccessoryDescription: u32 = 0x300A0422;

/// (300A,0423) VR=CS, VM=1 General Accessory Type
pub const GeneralAccessoryType: u32 = 0x300A0423;

/// (300A,0424) VR=IS, VM=1 General Accessory Number
pub const GeneralAccessoryNumber: u32 = 0x300A0424;

/// (300A,0431) VR=SQ, VM=1 Applicator Geometry Sequence
pub const ApplicatorGeometrySequence: u32 = 0x300A0431;

/// (300A,0432) VR=CS, VM=1 Applicator Aperture Shape
pub const ApplicatorApertureShape: u32 = 0x300A0432;

/// (300A,0433) VR=FL, VM=1 Applicator Opening
pub const ApplicatorOpening: u32 = 0x300A0433;

/// (300A,0434) VR=FL, VM=1 Applicator Opening X
pub const ApplicatorOpeningX: u32 = 0x300A0434;

/// (300A,0435) VR=FL, VM=1 Applicator Opening Y
pub const ApplicatorOpeningY: u32 = 0x300A0435;

/// (300A,0436) VR=FL, VM=1 Source to Applicator Mounting Position Distance
pub const SourceToApplicatorMountingPositionDistance: u32 = 0x300A0436;

/// (300C,0002) VR=SQ, VM=1 Referenced RT Plan Sequence
pub const ReferencedRTPlanSequence: u32 = 0x300C0002;

/// (300C,0004) VR=SQ, VM=1 Referenced Beam Sequence
pub const ReferencedBeamSequence: u32 = 0x300C0004;

/// (300C,0006) VR=IS, VM=1 Referenced Beam Number
pub const ReferencedBeamNumber: u32 = 0x300C0006;

/// (300C,0007) VR=IS, VM=1 Referenced Reference Image Number
pub const ReferencedReferenceImageNumber: u32 = 0x300C0007;

/// (300C,0008) VR=DS, VM=1 Start Cumulative Meterset Weight
pub const StartCumulativeMetersetWeight: u32 = 0x300C0008;

/// (300C,0009) VR=DS, VM=1 End Cumulative Meterset Weight
pub const EndCumulativeMetersetWeight: u32 = 0x300C0009;

/// (300C,000A) VR=SQ, VM=1 Referenced Brachy Application Setup Sequence
pub const ReferencedBrachyApplicationSetupSequence: u32 = 0x300C000A;

/// (300C,000C) VR=IS, VM=1 Referenced Brachy Application Setup Number
pub const ReferencedBrachyApplicationSetupNumber: u32 = 0x300C000C;

/// (300C,000E) VR=IS, VM=1 Referenced Source Number
pub const ReferencedSourceNumber: u32 = 0x300C000E;

/// (300C,0020) VR=SQ, VM=1 Referenced Fraction Group Sequence
pub const ReferencedFractionGroupSequence: u32 = 0x300C0020;

/// (300C,0022) VR=IS, VM=1 Referenced Fraction Group Number
pub const ReferencedFractionGroupNumber: u32 = 0x300C0022;

/// (300C,0040) VR=SQ, VM=1 Referenced Verification Image Sequence
pub const ReferencedVerificationImageSequence: u32 = 0x300C0040;

/// (300C,0042) VR=SQ, VM=1 Referenced Reference Image Sequence
pub const ReferencedReferenceImageSequence: u32 = 0x300C0042;

/// (300C,0050) VR=SQ, VM=1 Referenced Dose Reference Sequence
pub const ReferencedDoseReferenceSequence: u32 = 0x300C0050;

/// (300C,0051) VR=IS, VM=1 Referenced Dose Reference Number
pub const ReferencedDoseReferenceNumber: u32 = 0x300C0051;

/// (300C,0055) VR=SQ, VM=1 Brachy Referenced Dose Reference Sequence
pub const BrachyReferencedDoseReferenceSequence: u32 = 0x300C0055;

/// (300C,0060) VR=SQ, VM=1 Referenced Structure Set Sequence
pub const ReferencedStructureSetSequence: u32 = 0x300C0060;

/// (300C,006A) VR=IS, VM=1 Referenced Patient Setup Number
pub const ReferencedPatientSetupNumber: u32 = 0x300C006A;

/// (300C,0080) VR=SQ, VM=1 Referenced Dose Sequence
pub const ReferencedDoseSequence: u32 = 0x300C0080;

/// (300C,00A0) VR=IS, VM=1 Referenced Tolerance Table Number
pub const ReferencedToleranceTableNumber: u32 = 0x300C00A0;

/// (300C,00B0) VR=SQ, VM=1 Referenced Bolus Sequence
pub const ReferencedBolusSequence: u32 = 0x300C00B0;

/// (300C,00C0) VR=IS, VM=1 Referenced Wedge Number
pub const ReferencedWedgeNumber: u32 = 0x300C00C0;

/// (300C,00D0) VR=IS, VM=1 Referenced Compensator Number
pub const ReferencedCompensatorNumber: u32 = 0x300C00D0;

/// (300C,00E0) VR=IS, VM=1 Referenced Block Number
pub const ReferencedBlockNumber: u32 = 0x300C00E0;

/// (300C,00F0) VR=IS, VM=1 Referenced Control Point Index
pub const ReferencedControlPointIndex: u32 = 0x300C00F0;

/// (300C,00F2) VR=SQ, VM=1 Referenced Control Point Sequence
pub const ReferencedControlPointSequence: u32 = 0x300C00F2;

/// (300C,00F4) VR=IS, VM=1 Referenced Start Control Point Index
pub const ReferencedStartControlPointIndex: u32 = 0x300C00F4;

/// (300C,00F6) VR=IS, VM=1 Referenced Stop Control Point Index
pub const ReferencedStopControlPointIndex: u32 = 0x300C00F6;

/// (300C,0100) VR=IS, VM=1 Referenced Range Shifter Number
pub const ReferencedRangeShifterNumber: u32 = 0x300C0100;

/// (300C,0102) VR=IS, VM=1 Referenced Lateral Spreading Device Number
pub const ReferencedLateralSpreadingDeviceNumber: u32 = 0x300C0102;

/// (300C,0104) VR=IS, VM=1 Referenced Range Modulator Number
pub const ReferencedRangeModulatorNumber: u32 = 0x300C0104;

/// (300E,0002) VR=CS, VM=1 Approval Status
pub const ApprovalStatus: u32 = 0x300E0002;

/// (300E,0004) VR=DA, VM=1 Review Date
pub const ReviewDate: u32 = 0x300E0004;

/// (300E,0005) VR=TM, VM=1 Review Time
pub const ReviewTime: u32 = 0x300E0005;

/// (300E,0008) VR=PN, VM=1 Reviewer Name
pub const ReviewerName: u32 = 0x300E0008;

/// (4000,0010) VR=LT, VM=1 Arbitrary (retired)
pub const Arbitrary: u32 = 0x40000010;

/// (4000,4000) VR=LT, VM=1 Text Comments (retired)
pub const TextComments: u32 = 0x40004000;

/// (4008,0040) VR=SH, VM=1 Results ID (retired)
pub const ResultsID: u32 = 0x40080040;

/// (4008,0042) VR=LO, VM=1 Results ID Issuer (retired)
pub const ResultsIDIssuer: u32 = 0x40080042;

/// (4008,0050) VR=SQ, VM=1 Referenced Interpretation Sequence (retired)
pub const ReferencedInterpretationSequence: u32 = 0x40080050;

/// (4008,00FF) VR=CS, VM=1 Report Production Status (Trial) (retired)
pub const ReportProductionStatusTrial: u32 = 0x400800FF;

/// (4008,0100) VR=DA, VM=1 Interpretation Recorded Date (retired)
pub const InterpretationRecordedDate: u32 = 0x40080100;

/// (4008,0101) VR=TM, VM=1 Interpretation Recorded Time (retired)
pub const InterpretationRecordedTime: u32 = 0x40080101;

/// (4008,0102) VR=PN, VM=1 Interpretation Recorder (retired)
pub const InterpretationRecorder: u32 = 0x40080102;

/// (4008,0103) VR=LO, VM=1 Reference to Recorded Sound (retired)
pub const ReferenceToRecordedSound: u32 = 0x40080103;

/// (4008,0108) VR=DA, VM=1 Interpretation Transcription Date (retired)
pub const InterpretationTranscriptionDate: u32 = 0x40080108;

/// (4008,0109) VR=TM, VM=1 Interpretation Transcription Time (retired)
pub const InterpretationTranscriptionTime: u32 = 0x40080109;

/// (4008,010A) VR=PN, VM=1 Interpretation Transcriber (retired)
pub const InterpretationTranscriber: u32 = 0x4008010A;

/// (4008,010B) VR=ST, VM=1 Interpretation Text (retired)
pub const InterpretationText: u32 = 0x4008010B;

/// (4008,010C) VR=PN, VM=1 Interpretation Author (retired)
pub const InterpretationAuthor: u32 = 0x4008010C;

/// (4008,0111) VR=SQ, VM=1 Interpretation Approver Sequence (retired)
pub const InterpretationApproverSequence: u32 = 0x40080111;

/// (4008,0112) VR=DA, VM=1 Interpretation Approval Date (retired)
pub const InterpretationApprovalDate: u32 = 0x40080112;

/// (4008,0113) VR=TM, VM=1 Interpretation Approval Time (retired)
pub const InterpretationApprovalTime: u32 = 0x40080113;

/// (4008,0114) VR=PN, VM=1 Physician Approving Interpretation (retired)
pub const PhysicianApprovingInterpretation: u32 = 0x40080114;

/// (4008,0115) VR=LT, VM=1 Interpretation Diagnosis Description (retired)
pub const InterpretationDiagnosisDescription: u32 = 0x40080115;

/// (4008,0117) VR=SQ, VM=1 Interpretation Diagnosis Code Sequence (retired)
pub const InterpretationDiagnosisCodeSequence: u32 = 0x40080117;

/// (4008,0118) VR=SQ, VM=1 Results Distribution List Sequence (retired)
pub const ResultsDistributionListSequence: u32 = 0x40080118;

/// (4008,0119) VR=PN, VM=1 Distribution Name (retired)
pub const DistributionName: u32 = 0x40080119;

/// (4008,011A) VR=LO, VM=1 Distribution Address (retired)
pub const DistributionAddress: u32 = 0x4008011A;

/// (4008,0200) VR=SH, VM=1 Interpretation ID (retired)
pub const InterpretationID: u32 = 0x40080200;

/// (4008,0202) VR=LO, VM=1 Interpretation ID Issuer (retired)
pub const InterpretationIDIssuer: u32 = 0x40080202;

/// (4008,0210) VR=CS, VM=1 Interpretation Type ID (retired)
pub const InterpretationTypeID: u32 = 0x40080210;

/// (4008,0212) VR=CS, VM=1 Interpretation Status ID (retired)
pub const InterpretationStatusID: u32 = 0x40080212;

/// (4008,0300) VR=ST, VM=1 Impressions (retired)
pub const Impressions: u32 = 0x40080300;

/// (4008,4000) VR=ST, VM=1  Results Comments (retired)
pub const ResultsComments: u32 = 0x40084000;

/// (4010,0001) VR=CS, VM=1 Low Energy Detectors
pub const LowEnergyDetectors: u32 = 0x40100001;

/// (4010,0002) VR=CS, VM=1 High Energy Detectors
pub const HighEnergyDetectors: u32 = 0x40100002;

/// (4010,0004) VR=SQ, VM=1 Detector Geometry Sequence
pub const DetectorGeometrySequence: u32 = 0x40100004;

/// (4010,1001) VR=SQ, VM=1 Threat ROI Voxel Sequence
pub const ThreatROIVoxelSequence: u32 = 0x40101001;

/// (4010,1004) VR=FL, VM=3 Threat ROI Base
pub const ThreatROIBase: u32 = 0x40101004;

/// (4010,1005) VR=FL, VM=3 Threat ROI Extents
pub const ThreatROIExtents: u32 = 0x40101005;

/// (4010,1006) VR=OB, VM=1 Threat ROI Bitmap
pub const ThreatROIBitmap: u32 = 0x40101006;

/// (4010,1007) VR=SH, VM=1 Route Segment ID
pub const RouteSegmentID: u32 = 0x40101007;

/// (4010,1008) VR=CS, VM=1 Gantry Type
pub const GantryType: u32 = 0x40101008;

/// (4010,1009) VR=CS, VM=1 OOI Owner Type
pub const OOIOwnerType: u32 = 0x40101009;

/// (4010,100A) VR=SQ, VM=1 Route Segment Sequence
pub const RouteSegmentSequence: u32 = 0x4010100A;

/// (4010,1010) VR=US, VM=1 Potential Threat Object ID
pub const PotentialThreatObjectID: u32 = 0x40101010;

/// (4010,1011) VR=SQ, VM=1 Threat Sequence
pub const ThreatSequence: u32 = 0x40101011;

/// (4010,1012) VR=CS, VM=1 Threat Category
pub const ThreatCategory: u32 = 0x40101012;

/// (4010,1013) VR=LT, VM=1 Threat Category Description
pub const ThreatCategoryDescription: u32 = 0x40101013;

/// (4010,1014) VR=CS, VM=1 ATD Ability Assessment
pub const ATDAbilityAssessment: u32 = 0x40101014;

/// (4010,1015) VR=CS, VM=1 ATD Assessment Flag
pub const ATDAssessmentFlag: u32 = 0x40101015;

/// (4010,1016) VR=FL, VM=1 ATD Assessment Probability
pub const ATDAssessmentProbability: u32 = 0x40101016;

/// (4010,1017) VR=FL, VM=1 Mass
pub const Mass: u32 = 0x40101017;

/// (4010,1018) VR=FL, VM=1 Density
pub const Density: u32 = 0x40101018;

/// (4010,1019) VR=FL, VM=1 Z Effective
pub const ZEffective: u32 = 0x40101019;

/// (4010,101A) VR=SH, VM=1 Boarding Pass ID
pub const BoardingPassID: u32 = 0x4010101A;

/// (4010,101B) VR=FL, VM=3 Center of Mass
pub const CenterOfMass: u32 = 0x4010101B;

/// (4010,101C) VR=FL, VM=3 Center of PTO
pub const CenterOfPTO: u32 = 0x4010101C;

/// (4010,101D) VR=FL, VM=6-n Bounding Polygon
pub const BoundingPolygon: u32 = 0x4010101D;

/// (4010,101E) VR=SH, VM=1 Route Segment Start Location ID
pub const RouteSegmentStartLocationID: u32 = 0x4010101E;

/// (4010,101F) VR=SH, VM=1 Route Segment End Location ID
pub const RouteSegmentEndLocationID: u32 = 0x4010101F;

/// (4010,1020) VR=CS, VM=1 Route Segment Location ID Type
pub const RouteSegmentLocationIDType: u32 = 0x40101020;

/// (4010,1021) VR=CS, VM=1-n Abort Reason
pub const AbortReason: u32 = 0x40101021;

/// (4010,1023) VR=FL, VM=1 Volume of PTO
pub const VolumeOfPTO: u32 = 0x40101023;

/// (4010,1024) VR=CS, VM=1 Abort Flag
pub const AbortFlag: u32 = 0x40101024;

/// (4010,1025) VR=DT, VM=1 Route Segment Start Time
pub const RouteSegmentStartTime: u32 = 0x40101025;

/// (4010,1026) VR=DT, VM=1 Route Segment End Time
pub const RouteSegmentEndTime: u32 = 0x40101026;

/// (4010,1027) VR=CS, VM=1 TDR Type
pub const TDRType: u32 = 0x40101027;

/// (4010,1028) VR=CS, VM=1 International Route Segment
pub const InternationalRouteSegment: u32 = 0x40101028;

/// (4010,1029) VR=LO, VM=1-n Threat Detection Algorithm and Version
pub const ThreatDetectionAlgorithmandVersion: u32 = 0x40101029;

/// (4010,102A) VR=SH, VM=1 Assigned Location
pub const AssignedLocation: u32 = 0x4010102A;

/// (4010,102B) VR=DT, VM=1 Alarm Decision Time
pub const AlarmDecisionTime: u32 = 0x4010102B;

/// (4010,1031) VR=CS, VM=1 Alarm Decision
pub const AlarmDecision: u32 = 0x40101031;

/// (4010,1033) VR=US, VM=1 Number of Total Objects
pub const NumberOfTotalObjects: u32 = 0x40101033;

/// (4010,1034) VR=US, VM=1 Number of Alarm Objects
pub const NumberOfAlarmObjects: u32 = 0x40101034;

/// (4010,1037) VR=SQ, VM=1 PTO Representation Sequence
pub const PTORepresentationSequence: u32 = 0x40101037;

/// (4010,1038) VR=SQ, VM=1 ATD Assessment Sequence
pub const ATDAssessmentSequence: u32 = 0x40101038;

/// (4010,1039) VR=CS, VM=1 TIP Type
pub const TIPType: u32 = 0x40101039;

/// (4010,103A) VR=CS, VM=1 DICOS Version
pub const DICOSVersion: u32 = 0x4010103A;

/// (4010,1041) VR=DT, VM=1 OOI Owner Creation Time
pub const OOIOwnerCreationTime: u32 = 0x40101041;

/// (4010,1042) VR=CS, VM=1 OOI Type
pub const OOIType: u32 = 0x40101042;

/// (4010,1043) VR=FL, VM=3 OOI Size
pub const OOISize: u32 = 0x40101043;

/// (4010,1044) VR=CS, VM=1 Acquisition Status
pub const AcquisitionStatus: u32 = 0x40101044;

/// (4010,1045) VR=SQ, VM=1 Basis Materials Code Sequence
pub const BasisMaterialsCodeSequence: u32 = 0x40101045;

/// (4010,1046) VR=CS, VM=1 Phantom Type
pub const PhantomType: u32 = 0x40101046;

/// (4010,1047) VR=SQ, VM=1 OOI Owner Sequence
pub const OOIOwnerSequence: u32 = 0x40101047;

/// (4010,1048) VR=CS, VM=1 Scan Type
pub const ScanType: u32 = 0x40101048;

/// (4010,1051) VR=LO, VM=1 Itinerary ID
pub const ItineraryID: u32 = 0x40101051;

/// (4010,1052) VR=SH, VM=1 Itinerary ID Type
pub const ItineraryIDType: u32 = 0x40101052;

/// (4010,1053) VR=LO, VM=1 Itinerary ID Assigning Authority
pub const ItineraryIDAssigningAuthority: u32 = 0x40101053;

/// (4010,1054) VR=SH, VM=1 Route ID
pub const RouteID: u32 = 0x40101054;

/// (4010,1055) VR=SH, VM=1 Route ID Assigning Authority
pub const RouteIDAssigningAuthority: u32 = 0x40101055;

/// (4010,1056) VR=CS, VM=1 Inbound  Arrival Type
pub const InboundArrivalType: u32 = 0x40101056;

/// (4010,1058) VR=SH, VM=1 Carrier ID
pub const CarrierID: u32 = 0x40101058;

/// (4010,1059) VR=CS, VM=1 Carrier ID Assigning Authority
pub const CarrierIDAssigningAuthority: u32 = 0x40101059;

/// (4010,1060) VR=FL, VM=3 Source Orientation
pub const SourceOrientation: u32 = 0x40101060;

/// (4010,1061) VR=FL, VM=3 Source Position
pub const SourcePosition: u32 = 0x40101061;

/// (4010,1062) VR=FL, VM=1 Belt Height
pub const BeltHeight: u32 = 0x40101062;

/// (4010,1064) VR=SQ, VM=1 Algorithm Routing Code Sequence
pub const AlgorithmRoutingCodeSequence: u32 = 0x40101064;

/// (4010,1067) VR=CS, VM=1 Transport Classification
pub const TransportClassification: u32 = 0x40101067;

/// (4010,1068) VR=LT, VM=1 OOI Type Descriptor
pub const OOITypeDescriptor: u32 = 0x40101068;

/// (4010,1069) VR=FL, VM=1 Total Processing Time
pub const TotalProcessingTime: u32 = 0x40101069;

/// (4010,106C) VR=OB, VM=1 Detector Calibration Data
pub const DetectorCalibrationData: u32 = 0x4010106C;

/// (4FFE,0001) VR=SQ, VM=1 MAC Parameters Sequence
pub const MACParametersSequence: u32 = 0x4FFE0001;

/// (50xx,0005) VR=US, VM=1 Curve Dimensions (retired)
pub const CurveDimensions: u32 = 0x50000005;

/// (50xx,0010) VR=US, VM=1 Number of Points (retired)
pub const NumberOfPoints: u32 = 0x50000010;

/// (50xx,0020) VR=CS, VM=1 Type of Data (retired)
pub const TypeOfData: u32 = 0x50000020;

/// (50xx,0022) VR=LO, VM=1 Curve Description (retired)
pub const CurveDescription: u32 = 0x50000022;

/// (50xx,0030) VR=SH, VM=1-n Axis Units (retired)
pub const AxisUnits: u32 = 0x50000030;

/// (50xx,0040) VR=SH, VM=1-n Axis Labels (retired)
pub const AxisLabels: u32 = 0x50000040;

/// (50xx,0103) VR=US, VM=1 Data Value Representation (retired)
pub const DataValueRepresentation: u32 = 0x50000103;

/// (50xx,0104) VR=US, VM=1-n Minimum Coordinate Value (retired)
pub const MinimumCoordinateValue: u32 = 0x50000104;

/// (50xx,0105) VR=US, VM=1-n Maximum Coordinate Value (retired)
pub const MaximumCoordinateValue: u32 = 0x50000105;

/// (50xx,0106) VR=SH, VM=1-n Curve Range (retired)
pub const CurveRange: u32 = 0x50000106;

/// (50xx,0110) VR=US, VM=1-n Curve Data Descriptor (retired)
pub const CurveDataDescriptor: u32 = 0x50000110;

/// (50xx,0112) VR=US, VM=1-n Coordinate Start Value (retired)
pub const CoordinateStartValue: u32 = 0x50000112;

/// (50xx,0114) VR=US, VM=1-n Coordinate Step Value (retired)
pub const CoordinateStepValue: u32 = 0x50000114;

/// (50xx,1001) VR=CS, VM=1 Curve Activation Layer (retired)
pub const CurveActivationLayer: u32 = 0x50001001;

/// (50xx,2000) VR=US, VM=1 Audio Type (retired)
pub const AudioType: u32 = 0x50002000;

/// (50xx,2002) VR=US, VM=1 Audio Sample Format (retired)
pub const AudioSampleFormat: u32 = 0x50002002;

/// (50xx,2004) VR=US, VM=1 Number of Channels (retired)
pub const NumberOfChannels: u32 = 0x50002004;

/// (50xx,2006) VR=UL, VM=1 Number of Samples (retired)
pub const NumberOfSamples: u32 = 0x50002006;

/// (50xx,2008) VR=UL, VM=1 Sample Rate (retired)
pub const SampleRate: u32 = 0x50002008;

/// (50xx,200A) VR=UL, VM=1 Total Time (retired)
pub const TotalTime: u32 = 0x5000200A;

/// (50xx,200C) VR=OW|OB, VM=1 Audio Sample Data (retired)
pub const AudioSampleData: u32 = 0x5000200C;

/// (50xx,200E) VR=LT, VM=1  Audio Comments (retired)
pub const AudioComments: u32 = 0x5000200E;

/// (50xx,2500) VR=LO, VM=1 Curve Label (retired)
pub const CurveLabel: u32 = 0x50002500;

/// (50xx,2600) VR=SQ, VM=1 Curve Referenced Overlay Sequence (retired)
pub const CurveReferencedOverlaySequence: u32 = 0x50002600;

/// (50xx,2610) VR=US, VM=1 Curve Referenced Overlay Group (retired)
pub const CurveReferencedOverlayGroup: u32 = 0x50002610;

/// (50xx,3000) VR=OW|OB, VM=1 Curve Data (retired)
pub const CurveData: u32 = 0x50003000;

/// (5200,9229) VR=SQ, VM=1 Shared Functional Groups Sequence
pub const SharedFunctionalGroupsSequence: u32 = 0x52009229;

/// (5200,9230) VR=SQ, VM=1 Per-frame Functional Groups Sequence
pub const PerFrameFunctionalGroupsSequence: u32 = 0x52009230;

/// (5400,0100) VR=SQ, VM=1 Waveform Sequence
pub const WaveformSequence: u32 = 0x54000100;

/// (5400,0110) VR=OB|OW, VM=1 Channel Minimum Value
pub const ChannelMinimumValue: u32 = 0x54000110;

/// (5400,0112) VR=OB|OW, VM=1 Channel Maximum Value
pub const ChannelMaximumValue: u32 = 0x54000112;

/// (5400,1004) VR=US, VM=1 Waveform Bits Allocated
pub const WaveformBitsAllocated: u32 = 0x54001004;

/// (5400,1006) VR=CS, VM=1 Waveform Sample Interpretation
pub const WaveformSampleInterpretation: u32 = 0x54001006;

/// (5400,100A) VR=OB|OW, VM=1 Waveform Padding Value
pub const WaveformPaddingValue: u32 = 0x5400100A;

/// (5400,1010) VR=OB|OW, VM=1 Waveform Data
pub const WaveformData: u32 = 0x54001010;

/// (5600,0010) VR=OF, VM=1 First Order Phase Correction Angle
pub const FirstOrderPhaseCorrectionAngle: u32 = 0x56000010;

/// (5600,0020) VR=OF, VM=1 Spectroscopy Data
pub const SpectroscopyData: u32 = 0x56000020;

/// (60xx,0010) VR=US, VM=1 Overlay Rows
pub const OverlayRows: u32 = 0x60000010;

/// (60xx,0011) VR=US, VM=1 Overlay Columns
pub const OverlayColumns: u32 = 0x60000011;

/// (60xx,0012) VR=US, VM=1 Overlay Planes (retired)
pub const OverlayPlanes: u32 = 0x60000012;

/// (60xx,0015) VR=IS, VM=1 Number of Frames in Overlay
pub const NumberOfFramesInOverlay: u32 = 0x60000015;

/// (60xx,0022) VR=LO, VM=1 Overlay Description
pub const OverlayDescription: u32 = 0x60000022;

/// (60xx,0040) VR=CS, VM=1 Overlay Type
pub const OverlayType: u32 = 0x60000040;

/// (60xx,0045) VR=LO, VM=1 Overlay Subtype
pub const OverlaySubtype: u32 = 0x60000045;

/// (60xx,0050) VR=SS, VM=2 Overlay Origin
pub const OverlayOrigin: u32 = 0x60000050;

/// (60xx,0051) VR=US, VM=1 Image Frame Origin
pub const ImageFrameOrigin: u32 = 0x60000051;

/// (60xx,0052) VR=US, VM=1 Overlay Plane Origin (retired)
pub const OverlayPlaneOrigin: u32 = 0x60000052;

/// (60xx,0060) VR=CS, VM=1 Overlay Compression Code (retired)
pub const OverlayCompressionCode: u32 = 0x60000060;

/// (60xx,0061) VR=SH, VM=1 Overlay Compression Originator (retired)
pub const OverlayCompressionOriginator: u32 = 0x60000061;

/// (60xx,0062) VR=SH, VM=1 Overlay Compression Label (retired)
pub const OverlayCompressionLabel: u32 = 0x60000062;

/// (60xx,0063) VR=CS, VM=1 Overlay Compression Description (retired)
pub const OverlayCompressionDescription: u32 = 0x60000063;

/// (60xx,0066) VR=AT, VM=1-n Overlay Compression Step Pointers (retired)
pub const OverlayCompressionStepPointers: u32 = 0x60000066;

/// (60xx,0068) VR=US, VM=1 Overlay Repeat Interval (retired)
pub const OverlayRepeatInterval: u32 = 0x60000068;

/// (60xx,0069) VR=US, VM=1 Overlay Bits Grouped (retired)
pub const OverlayBitsGrouped: u32 = 0x60000069;

/// (60xx,0100) VR=US, VM=1 Overlay Bits Allocated
pub const OverlayBitsAllocated: u32 = 0x60000100;

/// (60xx,0102) VR=US, VM=1 Overlay Bit Position
pub const OverlayBitPosition: u32 = 0x60000102;

/// (60xx,0110) VR=CS, VM=1 Overlay Format (retired)
pub const OverlayFormat: u32 = 0x60000110;

/// (60xx,0200) VR=US, VM=1 Overlay Location (retired)
pub const OverlayLocation: u32 = 0x60000200;

/// (60xx,0800) VR=CS, VM=1-n Overlay Code Label (retired)
pub const OverlayCodeLabel: u32 = 0x60000800;

/// (60xx,0802) VR=US, VM=1 Overlay Number of Tables (retired)
pub const OverlayNumberOfTables: u32 = 0x60000802;

/// (60xx,0803) VR=AT, VM=1-n Overlay Code Table Location (retired)
pub const OverlayCodeTableLocation: u32 = 0x60000803;

/// (60xx,0804) VR=US, VM=1 Overlay Bits For Code Word (retired)
pub const OverlayBitsForCodeWord: u32 = 0x60000804;

/// (60xx,1001) VR=CS, VM=1 Overlay Activation Layer
pub const OverlayActivationLayer: u32 = 0x60001001;

/// (60xx,1100) VR=US, VM=1 Overlay Descriptor - Gray (retired)
pub const OverlayDescriptorGray: u32 = 0x60001100;

/// (60xx,1101) VR=US, VM=1 Overlay Descriptor - Red (retired)
pub const OverlayDescriptorRed: u32 = 0x60001101;

/// (60xx,1102) VR=US, VM=1 Overlay Descriptor - Green (retired)
pub const OverlayDescriptorGreen: u32 = 0x60001102;

/// (60xx,1103) VR=US, VM=1 Overlay Descriptor - Blue (retired)
pub const OverlayDescriptorBlue: u32 = 0x60001103;

/// (60xx,1200) VR=US, VM=1-n Overlays - Gray (retired)
pub const OverlaysGray: u32 = 0x60001200;

/// (60xx,1201) VR=US, VM=1-n Overlays - Red (retired)
pub const OverlaysRed: u32 = 0x60001201;

/// (60xx,1202) VR=US, VM=1-n Overlays - Green (retired)
pub const OverlaysGreen: u32 = 0x60001202;

/// (60xx,1203) VR=US, VM=1-n Overlays - Blue (retired)
pub const OverlaysBlue: u32 = 0x60001203;

/// (60xx,1301) VR=IS, VM=1 ROI Area
pub const ROIArea: u32 = 0x60001301;

/// (60xx,1302) VR=DS, VM=1 ROI Mean
pub const ROIMean: u32 = 0x60001302;

/// (60xx,1303) VR=DS, VM=1 ROI Standard Deviation
pub const ROIStandardDeviation: u32 = 0x60001303;

/// (60xx,1500) VR=LO, VM=1 Overlay Label
pub const OverlayLabel: u32 = 0x60001500;

/// (60xx,3000) VR=OB|OW, VM=1 Overlay Data
pub const OverlayData: u32 = 0x60003000;

/// (60xx,4000) VR=LT, VM=1 Overlay Comments (retired)
pub const OverlayComments: u32 = 0x60004000;

/// (7FE0,0010) VR=OW|OB, VM=1 Pixel Data
pub const PixelData: u32 = 0x7FE00010;

/// (7FE0,0020) VR=OW, VM=1 Coefficients SDVN (retired)
pub const CoefficientsSDVN: u32 = 0x7FE00020;

/// (7FE0,0030) VR=OW, VM=1 Coefficients SDHN (retired)
pub const CoefficientsSDHN: u32 = 0x7FE00030;

/// (7FE0,0040) VR=OW, VM=1 Coefficients SDDN (retired)
pub const CoefficientsSDDN: u32 = 0x7FE00040;

/// (7Fxx,0010) VR=OW|OB, VM=1 Variable Pixel Data (retired)
pub const VariablePixelData: u32 = 0x7F000010;

/// (7Fxx,0011) VR=US, VM=1 Variable Next Data Group (retired)
pub const VariableNextDataGroup: u32 = 0x7F000011;

/// (7Fxx,0020) VR=OW, VM=1 Variable Coefficients SDVN (retired)
pub const VariableCoefficientsSDVN: u32 = 0x7F000020;

/// (7Fxx,0030) VR=OW, VM=1 Variable Coefficients SDHN (retired)
pub const VariableCoefficientsSDHN: u32 = 0x7F000030;

/// (7Fxx,0040) VR=OW, VM=1 Variable Coefficients SDDN (retired)
pub const VariableCoefficientsSDDN: u32 = 0x7F000040;

/// (FFFA,FFFA) VR=SQ, VM=1 Digital Signatures Sequence
pub const DigitalSignaturesSequence: u32 = 0xFFFAFFFA;

/// (FFFC,FFFC) VR=OB, VM=1 Data Set Trailing Padding
pub const DataSetTrailingPadding: u32 = 0xFFFCFFFC;

/// (FFFE,E000) VR=, VM=1 Item
pub const Item: u32 = 0xFFFEE000;

/// (FFFE,E00D) VR=, VM=1 Item Delimitation Item
pub const ItemDelimitationItem: u32 = 0xFFFEE00D;

/// (FFFE,E0DD) VR=, VM=1 Sequence Delimitation Item
pub const SequenceDelimitationItem: u32 = 0xFFFEE0DD;
