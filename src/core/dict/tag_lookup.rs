//! This is an auto-generated file. Do not make modifications here.

use core::dict::dicom_elements as tags;
use core::dict::dir_structure_elements as dse;
use core::dict::file_meta_elements as fme;
use core::tag::Tag;

use std::collections::hash_map::HashMap;

pub struct TagLookup {
	ident_to_elem: HashMap<&'static str, &'static Tag>,
	tag_to_elem: HashMap<u32, &'static Tag>,
}

impl TagLookup {
	pub fn by_ident(&self, ident: &str) -> Option<&'static Tag> {
		self.ident_to_elem.get(ident).map(|tag| *tag)
	}

	pub fn by_tag(&self, tag: u32) -> Option<&'static Tag> {
		self.tag_to_elem.get(&tag).map(|tag| *tag)
	}

	pub fn new() -> TagLookup {
		let mut ident_to_elem: HashMap<&'static str, &'static Tag> = HashMap::new();
		let mut tag_to_elem: HashMap<u32, &'static Tag> = HashMap::new();

		ident_to_elem.insert("LengthToEnd", &tags::LengthToEnd);
		tag_to_elem.insert(0x00080001, &tags::LengthToEnd);

		ident_to_elem.insert("SpecificCharacterSet", &tags::SpecificCharacterSet);
		tag_to_elem.insert(0x00080005, &tags::SpecificCharacterSet);

		ident_to_elem.insert("LanguageCodeSequence", &tags::LanguageCodeSequence);
		tag_to_elem.insert(0x00080006, &tags::LanguageCodeSequence);

		ident_to_elem.insert("ImageType", &tags::ImageType);
		tag_to_elem.insert(0x00080008, &tags::ImageType);

		ident_to_elem.insert("RecognitionCode", &tags::RecognitionCode);
		tag_to_elem.insert(0x00080010, &tags::RecognitionCode);

		ident_to_elem.insert("InstanceCreationDate", &tags::InstanceCreationDate);
		tag_to_elem.insert(0x00080012, &tags::InstanceCreationDate);

		ident_to_elem.insert("InstanceCreationTime", &tags::InstanceCreationTime);
		tag_to_elem.insert(0x00080013, &tags::InstanceCreationTime);

		ident_to_elem.insert("InstanceCreatorUID", &tags::InstanceCreatorUID);
		tag_to_elem.insert(0x00080014, &tags::InstanceCreatorUID);

		ident_to_elem.insert("InstanceCoercionDateTime", &tags::InstanceCoercionDateTime);
		tag_to_elem.insert(0x00080015, &tags::InstanceCoercionDateTime);

		ident_to_elem.insert("SOPClassUID", &tags::SOPClassUID);
		tag_to_elem.insert(0x00080016, &tags::SOPClassUID);

		ident_to_elem.insert("SOPInstanceUID", &tags::SOPInstanceUID);
		tag_to_elem.insert(0x00080018, &tags::SOPInstanceUID);

		ident_to_elem.insert("RelatedGeneralSOPClassUID", &tags::RelatedGeneralSOPClassUID);
		tag_to_elem.insert(0x0008001A, &tags::RelatedGeneralSOPClassUID);

		ident_to_elem.insert("OriginalSpecializedSOPClassUID", &tags::OriginalSpecializedSOPClassUID);
		tag_to_elem.insert(0x0008001B, &tags::OriginalSpecializedSOPClassUID);

		ident_to_elem.insert("StudyDate", &tags::StudyDate);
		tag_to_elem.insert(0x00080020, &tags::StudyDate);

		ident_to_elem.insert("SeriesDate", &tags::SeriesDate);
		tag_to_elem.insert(0x00080021, &tags::SeriesDate);

		ident_to_elem.insert("AcquisitionDate", &tags::AcquisitionDate);
		tag_to_elem.insert(0x00080022, &tags::AcquisitionDate);

		ident_to_elem.insert("ContentDate", &tags::ContentDate);
		tag_to_elem.insert(0x00080023, &tags::ContentDate);

		ident_to_elem.insert("OverlayDate", &tags::OverlayDate);
		tag_to_elem.insert(0x00080024, &tags::OverlayDate);

		ident_to_elem.insert("CurveDate", &tags::CurveDate);
		tag_to_elem.insert(0x00080025, &tags::CurveDate);

		ident_to_elem.insert("AcquisitionDateTime", &tags::AcquisitionDateTime);
		tag_to_elem.insert(0x0008002A, &tags::AcquisitionDateTime);

		ident_to_elem.insert("StudyTime", &tags::StudyTime);
		tag_to_elem.insert(0x00080030, &tags::StudyTime);

		ident_to_elem.insert("SeriesTime", &tags::SeriesTime);
		tag_to_elem.insert(0x00080031, &tags::SeriesTime);

		ident_to_elem.insert("AcquisitionTime", &tags::AcquisitionTime);
		tag_to_elem.insert(0x00080032, &tags::AcquisitionTime);

		ident_to_elem.insert("ContentTime", &tags::ContentTime);
		tag_to_elem.insert(0x00080033, &tags::ContentTime);

		ident_to_elem.insert("OverlayTime", &tags::OverlayTime);
		tag_to_elem.insert(0x00080034, &tags::OverlayTime);

		ident_to_elem.insert("CurveTime", &tags::CurveTime);
		tag_to_elem.insert(0x00080035, &tags::CurveTime);

		ident_to_elem.insert("DataSetType", &tags::DataSetType);
		tag_to_elem.insert(0x00080040, &tags::DataSetType);

		ident_to_elem.insert("DataSetSubtype", &tags::DataSetSubtype);
		tag_to_elem.insert(0x00080041, &tags::DataSetSubtype);

		ident_to_elem.insert("NuclearMedicineSeriesType", &tags::NuclearMedicineSeriesType);
		tag_to_elem.insert(0x00080042, &tags::NuclearMedicineSeriesType);

		ident_to_elem.insert("AccessionNumber", &tags::AccessionNumber);
		tag_to_elem.insert(0x00080050, &tags::AccessionNumber);

		ident_to_elem.insert("IssuerOfAccessionNumberSequence", &tags::IssuerOfAccessionNumberSequence);
		tag_to_elem.insert(0x00080051, &tags::IssuerOfAccessionNumberSequence);

		ident_to_elem.insert("QueryRetrieveLevel", &tags::QueryRetrieveLevel);
		tag_to_elem.insert(0x00080052, &tags::QueryRetrieveLevel);

		ident_to_elem.insert("QueryRetrieveView", &tags::QueryRetrieveView);
		tag_to_elem.insert(0x00080053, &tags::QueryRetrieveView);

		ident_to_elem.insert("RetrieveAETitle", &tags::RetrieveAETitle);
		tag_to_elem.insert(0x00080054, &tags::RetrieveAETitle);

		ident_to_elem.insert("StationAETitle", &tags::StationAETitle);
		tag_to_elem.insert(0x00080055, &tags::StationAETitle);

		ident_to_elem.insert("InstanceAvailability", &tags::InstanceAvailability);
		tag_to_elem.insert(0x00080056, &tags::InstanceAvailability);

		ident_to_elem.insert("FailedSOPInstanceUIDList", &tags::FailedSOPInstanceUIDList);
		tag_to_elem.insert(0x00080058, &tags::FailedSOPInstanceUIDList);

		ident_to_elem.insert("Modality", &tags::Modality);
		tag_to_elem.insert(0x00080060, &tags::Modality);

		ident_to_elem.insert("ModalitiesInStudy", &tags::ModalitiesInStudy);
		tag_to_elem.insert(0x00080061, &tags::ModalitiesInStudy);

		ident_to_elem.insert("SOPClassesInStudy", &tags::SOPClassesInStudy);
		tag_to_elem.insert(0x00080062, &tags::SOPClassesInStudy);

		ident_to_elem.insert("ConversionType", &tags::ConversionType);
		tag_to_elem.insert(0x00080064, &tags::ConversionType);

		ident_to_elem.insert("PresentationIntentType", &tags::PresentationIntentType);
		tag_to_elem.insert(0x00080068, &tags::PresentationIntentType);

		ident_to_elem.insert("Manufacturer", &tags::Manufacturer);
		tag_to_elem.insert(0x00080070, &tags::Manufacturer);

		ident_to_elem.insert("InstitutionName", &tags::InstitutionName);
		tag_to_elem.insert(0x00080080, &tags::InstitutionName);

		ident_to_elem.insert("InstitutionAddress", &tags::InstitutionAddress);
		tag_to_elem.insert(0x00080081, &tags::InstitutionAddress);

		ident_to_elem.insert("InstitutionCodeSequence", &tags::InstitutionCodeSequence);
		tag_to_elem.insert(0x00080082, &tags::InstitutionCodeSequence);

		ident_to_elem.insert("ReferringPhysicianName", &tags::ReferringPhysicianName);
		tag_to_elem.insert(0x00080090, &tags::ReferringPhysicianName);

		ident_to_elem.insert("ReferringPhysicianAddress", &tags::ReferringPhysicianAddress);
		tag_to_elem.insert(0x00080092, &tags::ReferringPhysicianAddress);

		ident_to_elem.insert("ReferringPhysicianTelephoneNumbers", &tags::ReferringPhysicianTelephoneNumbers);
		tag_to_elem.insert(0x00080094, &tags::ReferringPhysicianTelephoneNumbers);

		ident_to_elem.insert("ReferringPhysicianIdentificationSequence", &tags::ReferringPhysicianIdentificationSequence);
		tag_to_elem.insert(0x00080096, &tags::ReferringPhysicianIdentificationSequence);

		ident_to_elem.insert("ConsultingPhysicianName", &tags::ConsultingPhysicianName);
		tag_to_elem.insert(0x0008009C, &tags::ConsultingPhysicianName);

		ident_to_elem.insert("ConsultingPhysicianIdentificationSequence", &tags::ConsultingPhysicianIdentificationSequence);
		tag_to_elem.insert(0x0008009D, &tags::ConsultingPhysicianIdentificationSequence);

		ident_to_elem.insert("CodeValue", &tags::CodeValue);
		tag_to_elem.insert(0x00080100, &tags::CodeValue);

		ident_to_elem.insert("ExtendedCodeValue", &tags::ExtendedCodeValue);
		tag_to_elem.insert(0x00080101, &tags::ExtendedCodeValue);

		ident_to_elem.insert("CodingSchemeDesignator", &tags::CodingSchemeDesignator);
		tag_to_elem.insert(0x00080102, &tags::CodingSchemeDesignator);

		ident_to_elem.insert("CodingSchemeVersion", &tags::CodingSchemeVersion);
		tag_to_elem.insert(0x00080103, &tags::CodingSchemeVersion);

		ident_to_elem.insert("CodeMeaning", &tags::CodeMeaning);
		tag_to_elem.insert(0x00080104, &tags::CodeMeaning);

		ident_to_elem.insert("MappingResource", &tags::MappingResource);
		tag_to_elem.insert(0x00080105, &tags::MappingResource);

		ident_to_elem.insert("ContextGroupVersion", &tags::ContextGroupVersion);
		tag_to_elem.insert(0x00080106, &tags::ContextGroupVersion);

		ident_to_elem.insert("ContextGroupLocalVersion", &tags::ContextGroupLocalVersion);
		tag_to_elem.insert(0x00080107, &tags::ContextGroupLocalVersion);

		ident_to_elem.insert("ExtendedCodeMeaning", &tags::ExtendedCodeMeaning);
		tag_to_elem.insert(0x00080108, &tags::ExtendedCodeMeaning);

		ident_to_elem.insert("ContextGroupExtensionFlag", &tags::ContextGroupExtensionFlag);
		tag_to_elem.insert(0x0008010B, &tags::ContextGroupExtensionFlag);

		ident_to_elem.insert("CodingSchemeUID", &tags::CodingSchemeUID);
		tag_to_elem.insert(0x0008010C, &tags::CodingSchemeUID);

		ident_to_elem.insert("ContextGroupExtensionCreatorUID", &tags::ContextGroupExtensionCreatorUID);
		tag_to_elem.insert(0x0008010D, &tags::ContextGroupExtensionCreatorUID);

		ident_to_elem.insert("ContextIdentifier", &tags::ContextIdentifier);
		tag_to_elem.insert(0x0008010F, &tags::ContextIdentifier);

		ident_to_elem.insert("CodingSchemeIdentificationSequence", &tags::CodingSchemeIdentificationSequence);
		tag_to_elem.insert(0x00080110, &tags::CodingSchemeIdentificationSequence);

		ident_to_elem.insert("CodingSchemeRegistry", &tags::CodingSchemeRegistry);
		tag_to_elem.insert(0x00080112, &tags::CodingSchemeRegistry);

		ident_to_elem.insert("CodingSchemeExternalID", &tags::CodingSchemeExternalID);
		tag_to_elem.insert(0x00080114, &tags::CodingSchemeExternalID);

		ident_to_elem.insert("CodingSchemeName", &tags::CodingSchemeName);
		tag_to_elem.insert(0x00080115, &tags::CodingSchemeName);

		ident_to_elem.insert("CodingSchemeResponsibleOrganization", &tags::CodingSchemeResponsibleOrganization);
		tag_to_elem.insert(0x00080116, &tags::CodingSchemeResponsibleOrganization);

		ident_to_elem.insert("ContextUID", &tags::ContextUID);
		tag_to_elem.insert(0x00080117, &tags::ContextUID);

		ident_to_elem.insert("MappingResourceUID", &tags::MappingResourceUID);
		tag_to_elem.insert(0x00080118, &tags::MappingResourceUID);

		ident_to_elem.insert("LongCodeValue", &tags::LongCodeValue);
		tag_to_elem.insert(0x00080119, &tags::LongCodeValue);

		ident_to_elem.insert("URNCodeValue", &tags::URNCodeValue);
		tag_to_elem.insert(0x00080120, &tags::URNCodeValue);

		ident_to_elem.insert("EquivalentCodeSequence", &tags::EquivalentCodeSequence);
		tag_to_elem.insert(0x00080121, &tags::EquivalentCodeSequence);

		ident_to_elem.insert("MappingResourceName", &tags::MappingResourceName);
		tag_to_elem.insert(0x00080122, &tags::MappingResourceName);

		ident_to_elem.insert("ContextGroupIdentificationSequence", &tags::ContextGroupIdentificationSequence);
		tag_to_elem.insert(0x00080123, &tags::ContextGroupIdentificationSequence);

		ident_to_elem.insert("MappingResourceIdentificationSequence", &tags::MappingResourceIdentificationSequence);
		tag_to_elem.insert(0x00080124, &tags::MappingResourceIdentificationSequence);

		ident_to_elem.insert("TimezoneOffsetFromUTC", &tags::TimezoneOffsetFromUTC);
		tag_to_elem.insert(0x00080201, &tags::TimezoneOffsetFromUTC);

		ident_to_elem.insert("ResponsibleGroupCodeSequence", &tags::ResponsibleGroupCodeSequence);
		tag_to_elem.insert(0x00080220, &tags::ResponsibleGroupCodeSequence);

		ident_to_elem.insert("EquipmentModality", &tags::EquipmentModality);
		tag_to_elem.insert(0x00080221, &tags::EquipmentModality);

		ident_to_elem.insert("ManufacturerRelatedModelGroup", &tags::ManufacturerRelatedModelGroup);
		tag_to_elem.insert(0x00080222, &tags::ManufacturerRelatedModelGroup);

		ident_to_elem.insert("PrivateDataElementCharacteristicsSequence", &tags::PrivateDataElementCharacteristicsSequence);
		tag_to_elem.insert(0x00080300, &tags::PrivateDataElementCharacteristicsSequence);

		ident_to_elem.insert("PrivateGroupReference", &tags::PrivateGroupReference);
		tag_to_elem.insert(0x00080301, &tags::PrivateGroupReference);

		ident_to_elem.insert("PrivateCreatorReference", &tags::PrivateCreatorReference);
		tag_to_elem.insert(0x00080302, &tags::PrivateCreatorReference);

		ident_to_elem.insert("BlockIdentifyingInformationStatus", &tags::BlockIdentifyingInformationStatus);
		tag_to_elem.insert(0x00080303, &tags::BlockIdentifyingInformationStatus);

		ident_to_elem.insert("NonidentifyingPrivateElements", &tags::NonidentifyingPrivateElements);
		tag_to_elem.insert(0x00080304, &tags::NonidentifyingPrivateElements);

		ident_to_elem.insert("IdentifyingPrivateElements", &tags::IdentifyingPrivateElements);
		tag_to_elem.insert(0x00080306, &tags::IdentifyingPrivateElements);

		ident_to_elem.insert("DeidentificationActionSequence", &tags::DeidentificationActionSequence);
		tag_to_elem.insert(0x00080305, &tags::DeidentificationActionSequence);

		ident_to_elem.insert("DeidentificationAction", &tags::DeidentificationAction);
		tag_to_elem.insert(0x00080307, &tags::DeidentificationAction);

		ident_to_elem.insert("PrivateDataElement", &tags::PrivateDataElement);
		tag_to_elem.insert(0x00080308, &tags::PrivateDataElement);

		ident_to_elem.insert("PrivateDataElementValueMultiplicity", &tags::PrivateDataElementValueMultiplicity);
		tag_to_elem.insert(0x00080309, &tags::PrivateDataElementValueMultiplicity);

		ident_to_elem.insert("PrivateDataElementValueRepresentation", &tags::PrivateDataElementValueRepresentation);
		tag_to_elem.insert(0x0008030A, &tags::PrivateDataElementValueRepresentation);

		ident_to_elem.insert("PrivateDataElementNumberOfItems", &tags::PrivateDataElementNumberOfItems);
		tag_to_elem.insert(0x0008030B, &tags::PrivateDataElementNumberOfItems);

		ident_to_elem.insert("PrivateDataElementName", &tags::PrivateDataElementName);
		tag_to_elem.insert(0x0008030C, &tags::PrivateDataElementName);

		ident_to_elem.insert("PrivateDataElementKeyword", &tags::PrivateDataElementKeyword);
		tag_to_elem.insert(0x0008030D, &tags::PrivateDataElementKeyword);

		ident_to_elem.insert("PrivateDataElementDescription", &tags::PrivateDataElementDescription);
		tag_to_elem.insert(0x0008030E, &tags::PrivateDataElementDescription);

		ident_to_elem.insert("PrivateDataElementEncoding", &tags::PrivateDataElementEncoding);
		tag_to_elem.insert(0x0008030F, &tags::PrivateDataElementEncoding);

		ident_to_elem.insert("PrivateDataElementDefinitionSequence", &tags::PrivateDataElementDefinitionSequence);
		tag_to_elem.insert(0x00080310, &tags::PrivateDataElementDefinitionSequence);

		ident_to_elem.insert("NetworkID", &tags::NetworkID);
		tag_to_elem.insert(0x00081000, &tags::NetworkID);

		ident_to_elem.insert("StationName", &tags::StationName);
		tag_to_elem.insert(0x00081010, &tags::StationName);

		ident_to_elem.insert("StudyDescription", &tags::StudyDescription);
		tag_to_elem.insert(0x00081030, &tags::StudyDescription);

		ident_to_elem.insert("ProcedureCodeSequence", &tags::ProcedureCodeSequence);
		tag_to_elem.insert(0x00081032, &tags::ProcedureCodeSequence);

		ident_to_elem.insert("SeriesDescription", &tags::SeriesDescription);
		tag_to_elem.insert(0x0008103E, &tags::SeriesDescription);

		ident_to_elem.insert("SeriesDescriptionCodeSequence", &tags::SeriesDescriptionCodeSequence);
		tag_to_elem.insert(0x0008103F, &tags::SeriesDescriptionCodeSequence);

		ident_to_elem.insert("InstitutionalDepartmentName", &tags::InstitutionalDepartmentName);
		tag_to_elem.insert(0x00081040, &tags::InstitutionalDepartmentName);

		ident_to_elem.insert("PhysiciansOfRecord", &tags::PhysiciansOfRecord);
		tag_to_elem.insert(0x00081048, &tags::PhysiciansOfRecord);

		ident_to_elem.insert("PhysiciansOfRecordIdentificationSequence", &tags::PhysiciansOfRecordIdentificationSequence);
		tag_to_elem.insert(0x00081049, &tags::PhysiciansOfRecordIdentificationSequence);

		ident_to_elem.insert("PerformingPhysicianName", &tags::PerformingPhysicianName);
		tag_to_elem.insert(0x00081050, &tags::PerformingPhysicianName);

		ident_to_elem.insert("PerformingPhysicianIdentificationSequence", &tags::PerformingPhysicianIdentificationSequence);
		tag_to_elem.insert(0x00081052, &tags::PerformingPhysicianIdentificationSequence);

		ident_to_elem.insert("NameOfPhysiciansReadingStudy", &tags::NameOfPhysiciansReadingStudy);
		tag_to_elem.insert(0x00081060, &tags::NameOfPhysiciansReadingStudy);

		ident_to_elem.insert("PhysiciansReadingStudyIdentificationSequence", &tags::PhysiciansReadingStudyIdentificationSequence);
		tag_to_elem.insert(0x00081062, &tags::PhysiciansReadingStudyIdentificationSequence);

		ident_to_elem.insert("OperatorsName", &tags::OperatorsName);
		tag_to_elem.insert(0x00081070, &tags::OperatorsName);

		ident_to_elem.insert("OperatorIdentificationSequence", &tags::OperatorIdentificationSequence);
		tag_to_elem.insert(0x00081072, &tags::OperatorIdentificationSequence);

		ident_to_elem.insert("AdmittingDiagnosesDescription", &tags::AdmittingDiagnosesDescription);
		tag_to_elem.insert(0x00081080, &tags::AdmittingDiagnosesDescription);

		ident_to_elem.insert("AdmittingDiagnosesCodeSequence", &tags::AdmittingDiagnosesCodeSequence);
		tag_to_elem.insert(0x00081084, &tags::AdmittingDiagnosesCodeSequence);

		ident_to_elem.insert("ManufacturerModelName", &tags::ManufacturerModelName);
		tag_to_elem.insert(0x00081090, &tags::ManufacturerModelName);

		ident_to_elem.insert("ReferencedResultsSequence", &tags::ReferencedResultsSequence);
		tag_to_elem.insert(0x00081100, &tags::ReferencedResultsSequence);

		ident_to_elem.insert("ReferencedStudySequence", &tags::ReferencedStudySequence);
		tag_to_elem.insert(0x00081110, &tags::ReferencedStudySequence);

		ident_to_elem.insert("ReferencedPerformedProcedureStepSequence", &tags::ReferencedPerformedProcedureStepSequence);
		tag_to_elem.insert(0x00081111, &tags::ReferencedPerformedProcedureStepSequence);

		ident_to_elem.insert("ReferencedSeriesSequence", &tags::ReferencedSeriesSequence);
		tag_to_elem.insert(0x00081115, &tags::ReferencedSeriesSequence);

		ident_to_elem.insert("ReferencedPatientSequence", &tags::ReferencedPatientSequence);
		tag_to_elem.insert(0x00081120, &tags::ReferencedPatientSequence);

		ident_to_elem.insert("ReferencedVisitSequence", &tags::ReferencedVisitSequence);
		tag_to_elem.insert(0x00081125, &tags::ReferencedVisitSequence);

		ident_to_elem.insert("ReferencedOverlaySequence", &tags::ReferencedOverlaySequence);
		tag_to_elem.insert(0x00081130, &tags::ReferencedOverlaySequence);

		ident_to_elem.insert("ReferencedStereometricInstanceSequence", &tags::ReferencedStereometricInstanceSequence);
		tag_to_elem.insert(0x00081134, &tags::ReferencedStereometricInstanceSequence);

		ident_to_elem.insert("ReferencedWaveformSequence", &tags::ReferencedWaveformSequence);
		tag_to_elem.insert(0x0008113A, &tags::ReferencedWaveformSequence);

		ident_to_elem.insert("ReferencedImageSequence", &tags::ReferencedImageSequence);
		tag_to_elem.insert(0x00081140, &tags::ReferencedImageSequence);

		ident_to_elem.insert("ReferencedCurveSequence", &tags::ReferencedCurveSequence);
		tag_to_elem.insert(0x00081145, &tags::ReferencedCurveSequence);

		ident_to_elem.insert("ReferencedInstanceSequence", &tags::ReferencedInstanceSequence);
		tag_to_elem.insert(0x0008114A, &tags::ReferencedInstanceSequence);

		ident_to_elem.insert("ReferencedRealWorldValueMappingInstanceSequence", &tags::ReferencedRealWorldValueMappingInstanceSequence);
		tag_to_elem.insert(0x0008114B, &tags::ReferencedRealWorldValueMappingInstanceSequence);

		ident_to_elem.insert("ReferencedSOPClassUID", &tags::ReferencedSOPClassUID);
		tag_to_elem.insert(0x00081150, &tags::ReferencedSOPClassUID);

		ident_to_elem.insert("ReferencedSOPInstanceUID", &tags::ReferencedSOPInstanceUID);
		tag_to_elem.insert(0x00081155, &tags::ReferencedSOPInstanceUID);

		ident_to_elem.insert("SOPClassesSupported", &tags::SOPClassesSupported);
		tag_to_elem.insert(0x0008115A, &tags::SOPClassesSupported);

		ident_to_elem.insert("ReferencedFrameNumber", &tags::ReferencedFrameNumber);
		tag_to_elem.insert(0x00081160, &tags::ReferencedFrameNumber);

		ident_to_elem.insert("SimpleFrameList", &tags::SimpleFrameList);
		tag_to_elem.insert(0x00081161, &tags::SimpleFrameList);

		ident_to_elem.insert("CalculatedFrameList", &tags::CalculatedFrameList);
		tag_to_elem.insert(0x00081162, &tags::CalculatedFrameList);

		ident_to_elem.insert("TimeRange", &tags::TimeRange);
		tag_to_elem.insert(0x00081163, &tags::TimeRange);

		ident_to_elem.insert("FrameExtractionSequence", &tags::FrameExtractionSequence);
		tag_to_elem.insert(0x00081164, &tags::FrameExtractionSequence);

		ident_to_elem.insert("MultiFrameSourceSOPInstanceUID", &tags::MultiFrameSourceSOPInstanceUID);
		tag_to_elem.insert(0x00081167, &tags::MultiFrameSourceSOPInstanceUID);

		ident_to_elem.insert("RetrieveURL", &tags::RetrieveURL);
		tag_to_elem.insert(0x00081190, &tags::RetrieveURL);

		ident_to_elem.insert("TransactionUID", &tags::TransactionUID);
		tag_to_elem.insert(0x00081195, &tags::TransactionUID);

		ident_to_elem.insert("WarningReason", &tags::WarningReason);
		tag_to_elem.insert(0x00081196, &tags::WarningReason);

		ident_to_elem.insert("FailureReason", &tags::FailureReason);
		tag_to_elem.insert(0x00081197, &tags::FailureReason);

		ident_to_elem.insert("FailedSOPSequence", &tags::FailedSOPSequence);
		tag_to_elem.insert(0x00081198, &tags::FailedSOPSequence);

		ident_to_elem.insert("ReferencedSOPSequence", &tags::ReferencedSOPSequence);
		tag_to_elem.insert(0x00081199, &tags::ReferencedSOPSequence);

		ident_to_elem.insert("OtherFailuresSequence", &tags::OtherFailuresSequence);
		tag_to_elem.insert(0x0008119A, &tags::OtherFailuresSequence);

		ident_to_elem.insert("StudiesContainingOtherReferencedInstancesSequence", &tags::StudiesContainingOtherReferencedInstancesSequence);
		tag_to_elem.insert(0x00081200, &tags::StudiesContainingOtherReferencedInstancesSequence);

		ident_to_elem.insert("RelatedSeriesSequence", &tags::RelatedSeriesSequence);
		tag_to_elem.insert(0x00081250, &tags::RelatedSeriesSequence);

		ident_to_elem.insert("LossyImageCompressionRetired", &tags::LossyImageCompressionRetired);
		tag_to_elem.insert(0x00082110, &tags::LossyImageCompressionRetired);

		ident_to_elem.insert("DerivationDescription", &tags::DerivationDescription);
		tag_to_elem.insert(0x00082111, &tags::DerivationDescription);

		ident_to_elem.insert("SourceImageSequence", &tags::SourceImageSequence);
		tag_to_elem.insert(0x00082112, &tags::SourceImageSequence);

		ident_to_elem.insert("StageName", &tags::StageName);
		tag_to_elem.insert(0x00082120, &tags::StageName);

		ident_to_elem.insert("StageNumber", &tags::StageNumber);
		tag_to_elem.insert(0x00082122, &tags::StageNumber);

		ident_to_elem.insert("NumberOfStages", &tags::NumberOfStages);
		tag_to_elem.insert(0x00082124, &tags::NumberOfStages);

		ident_to_elem.insert("ViewName", &tags::ViewName);
		tag_to_elem.insert(0x00082127, &tags::ViewName);

		ident_to_elem.insert("ViewNumber", &tags::ViewNumber);
		tag_to_elem.insert(0x00082128, &tags::ViewNumber);

		ident_to_elem.insert("NumberOfEventTimers", &tags::NumberOfEventTimers);
		tag_to_elem.insert(0x00082129, &tags::NumberOfEventTimers);

		ident_to_elem.insert("NumberOfViewsInStage", &tags::NumberOfViewsInStage);
		tag_to_elem.insert(0x0008212A, &tags::NumberOfViewsInStage);

		ident_to_elem.insert("EventElapsedTimes", &tags::EventElapsedTimes);
		tag_to_elem.insert(0x00082130, &tags::EventElapsedTimes);

		ident_to_elem.insert("EventTimerNames", &tags::EventTimerNames);
		tag_to_elem.insert(0x00082132, &tags::EventTimerNames);

		ident_to_elem.insert("EventTimerSequence", &tags::EventTimerSequence);
		tag_to_elem.insert(0x00082133, &tags::EventTimerSequence);

		ident_to_elem.insert("EventTimeOffset", &tags::EventTimeOffset);
		tag_to_elem.insert(0x00082134, &tags::EventTimeOffset);

		ident_to_elem.insert("EventCodeSequence", &tags::EventCodeSequence);
		tag_to_elem.insert(0x00082135, &tags::EventCodeSequence);

		ident_to_elem.insert("StartTrim", &tags::StartTrim);
		tag_to_elem.insert(0x00082142, &tags::StartTrim);

		ident_to_elem.insert("StopTrim", &tags::StopTrim);
		tag_to_elem.insert(0x00082143, &tags::StopTrim);

		ident_to_elem.insert("RecommendedDisplayFrameRate", &tags::RecommendedDisplayFrameRate);
		tag_to_elem.insert(0x00082144, &tags::RecommendedDisplayFrameRate);

		ident_to_elem.insert("TransducerPosition", &tags::TransducerPosition);
		tag_to_elem.insert(0x00082200, &tags::TransducerPosition);

		ident_to_elem.insert("TransducerOrientation", &tags::TransducerOrientation);
		tag_to_elem.insert(0x00082204, &tags::TransducerOrientation);

		ident_to_elem.insert("AnatomicStructure", &tags::AnatomicStructure);
		tag_to_elem.insert(0x00082208, &tags::AnatomicStructure);

		ident_to_elem.insert("AnatomicRegionSequence", &tags::AnatomicRegionSequence);
		tag_to_elem.insert(0x00082218, &tags::AnatomicRegionSequence);

		ident_to_elem.insert("AnatomicRegionModifierSequence", &tags::AnatomicRegionModifierSequence);
		tag_to_elem.insert(0x00082220, &tags::AnatomicRegionModifierSequence);

		ident_to_elem.insert("PrimaryAnatomicStructureSequence", &tags::PrimaryAnatomicStructureSequence);
		tag_to_elem.insert(0x00082228, &tags::PrimaryAnatomicStructureSequence);

		ident_to_elem.insert("AnatomicStructureSpaceOrRegionSequence", &tags::AnatomicStructureSpaceOrRegionSequence);
		tag_to_elem.insert(0x00082229, &tags::AnatomicStructureSpaceOrRegionSequence);

		ident_to_elem.insert("PrimaryAnatomicStructureModifierSequence", &tags::PrimaryAnatomicStructureModifierSequence);
		tag_to_elem.insert(0x00082230, &tags::PrimaryAnatomicStructureModifierSequence);

		ident_to_elem.insert("TransducerPositionSequence", &tags::TransducerPositionSequence);
		tag_to_elem.insert(0x00082240, &tags::TransducerPositionSequence);

		ident_to_elem.insert("TransducerPositionModifierSequence", &tags::TransducerPositionModifierSequence);
		tag_to_elem.insert(0x00082242, &tags::TransducerPositionModifierSequence);

		ident_to_elem.insert("TransducerOrientationSequence", &tags::TransducerOrientationSequence);
		tag_to_elem.insert(0x00082244, &tags::TransducerOrientationSequence);

		ident_to_elem.insert("TransducerOrientationModifierSequence", &tags::TransducerOrientationModifierSequence);
		tag_to_elem.insert(0x00082246, &tags::TransducerOrientationModifierSequence);

		ident_to_elem.insert("AnatomicStructureSpaceOrRegionCodeSequenceTrial", &tags::AnatomicStructureSpaceOrRegionCodeSequenceTrial);
		tag_to_elem.insert(0x00082251, &tags::AnatomicStructureSpaceOrRegionCodeSequenceTrial);

		ident_to_elem.insert("AnatomicPortalOfEntranceCodeSequenceTrial", &tags::AnatomicPortalOfEntranceCodeSequenceTrial);
		tag_to_elem.insert(0x00082253, &tags::AnatomicPortalOfEntranceCodeSequenceTrial);

		ident_to_elem.insert("AnatomicApproachDirectionCodeSequenceTrial", &tags::AnatomicApproachDirectionCodeSequenceTrial);
		tag_to_elem.insert(0x00082255, &tags::AnatomicApproachDirectionCodeSequenceTrial);

		ident_to_elem.insert("AnatomicPerspectiveDescriptionTrial", &tags::AnatomicPerspectiveDescriptionTrial);
		tag_to_elem.insert(0x00082256, &tags::AnatomicPerspectiveDescriptionTrial);

		ident_to_elem.insert("AnatomicPerspectiveCodeSequenceTrial", &tags::AnatomicPerspectiveCodeSequenceTrial);
		tag_to_elem.insert(0x00082257, &tags::AnatomicPerspectiveCodeSequenceTrial);

		ident_to_elem.insert("AnatomicLocationOfExaminingInstrumentDescriptionTrial", &tags::AnatomicLocationOfExaminingInstrumentDescriptionTrial);
		tag_to_elem.insert(0x00082258, &tags::AnatomicLocationOfExaminingInstrumentDescriptionTrial);

		ident_to_elem.insert("AnatomicLocationOfExaminingInstrumentCodeSequenceTrial", &tags::AnatomicLocationOfExaminingInstrumentCodeSequenceTrial);
		tag_to_elem.insert(0x00082259, &tags::AnatomicLocationOfExaminingInstrumentCodeSequenceTrial);

		ident_to_elem.insert("AnatomicStructureSpaceOrRegionModifierCodeSequenceTrial", &tags::AnatomicStructureSpaceOrRegionModifierCodeSequenceTrial);
		tag_to_elem.insert(0x0008225A, &tags::AnatomicStructureSpaceOrRegionModifierCodeSequenceTrial);

		ident_to_elem.insert("OnAxisBackgroundAnatomicStructureCodeSequenceTrial", &tags::OnAxisBackgroundAnatomicStructureCodeSequenceTrial);
		tag_to_elem.insert(0x0008225C, &tags::OnAxisBackgroundAnatomicStructureCodeSequenceTrial);

		ident_to_elem.insert("AlternateRepresentationSequence", &tags::AlternateRepresentationSequence);
		tag_to_elem.insert(0x00083001, &tags::AlternateRepresentationSequence);

		ident_to_elem.insert("IrradiationEventUID", &tags::IrradiationEventUID);
		tag_to_elem.insert(0x00083010, &tags::IrradiationEventUID);

		ident_to_elem.insert("SourceIrradiationEventSequence", &tags::SourceIrradiationEventSequence);
		tag_to_elem.insert(0x00083011, &tags::SourceIrradiationEventSequence);

		ident_to_elem.insert("RadiopharmaceuticalAdministrationEventUID", &tags::RadiopharmaceuticalAdministrationEventUID);
		tag_to_elem.insert(0x00083012, &tags::RadiopharmaceuticalAdministrationEventUID);

		ident_to_elem.insert("IdentifyingComments", &tags::IdentifyingComments);
		tag_to_elem.insert(0x00084000, &tags::IdentifyingComments);

		ident_to_elem.insert("FrameType", &tags::FrameType);
		tag_to_elem.insert(0x00089007, &tags::FrameType);

		ident_to_elem.insert("ReferencedImageEvidenceSequence", &tags::ReferencedImageEvidenceSequence);
		tag_to_elem.insert(0x00089092, &tags::ReferencedImageEvidenceSequence);

		ident_to_elem.insert("ReferencedRawDataSequence", &tags::ReferencedRawDataSequence);
		tag_to_elem.insert(0x00089121, &tags::ReferencedRawDataSequence);

		ident_to_elem.insert("CreatorVersionUID", &tags::CreatorVersionUID);
		tag_to_elem.insert(0x00089123, &tags::CreatorVersionUID);

		ident_to_elem.insert("DerivationImageSequence", &tags::DerivationImageSequence);
		tag_to_elem.insert(0x00089124, &tags::DerivationImageSequence);

		ident_to_elem.insert("SourceImageEvidenceSequence", &tags::SourceImageEvidenceSequence);
		tag_to_elem.insert(0x00089154, &tags::SourceImageEvidenceSequence);

		ident_to_elem.insert("PixelPresentation", &tags::PixelPresentation);
		tag_to_elem.insert(0x00089205, &tags::PixelPresentation);

		ident_to_elem.insert("VolumetricProperties", &tags::VolumetricProperties);
		tag_to_elem.insert(0x00089206, &tags::VolumetricProperties);

		ident_to_elem.insert("VolumeBasedCalculationTechnique", &tags::VolumeBasedCalculationTechnique);
		tag_to_elem.insert(0x00089207, &tags::VolumeBasedCalculationTechnique);

		ident_to_elem.insert("ComplexImageComponent", &tags::ComplexImageComponent);
		tag_to_elem.insert(0x00089208, &tags::ComplexImageComponent);

		ident_to_elem.insert("AcquisitionContrast", &tags::AcquisitionContrast);
		tag_to_elem.insert(0x00089209, &tags::AcquisitionContrast);

		ident_to_elem.insert("DerivationCodeSequence", &tags::DerivationCodeSequence);
		tag_to_elem.insert(0x00089215, &tags::DerivationCodeSequence);

		ident_to_elem.insert("ReferencedPresentationStateSequence", &tags::ReferencedPresentationStateSequence);
		tag_to_elem.insert(0x00089237, &tags::ReferencedPresentationStateSequence);

		ident_to_elem.insert("ReferencedOtherPlaneSequence", &tags::ReferencedOtherPlaneSequence);
		tag_to_elem.insert(0x00089410, &tags::ReferencedOtherPlaneSequence);

		ident_to_elem.insert("FrameDisplaySequence", &tags::FrameDisplaySequence);
		tag_to_elem.insert(0x00089458, &tags::FrameDisplaySequence);

		ident_to_elem.insert("RecommendedDisplayFrameRateInFloat", &tags::RecommendedDisplayFrameRateInFloat);
		tag_to_elem.insert(0x00089459, &tags::RecommendedDisplayFrameRateInFloat);

		ident_to_elem.insert("SkipFrameRangeFlag", &tags::SkipFrameRangeFlag);
		tag_to_elem.insert(0x00089460, &tags::SkipFrameRangeFlag);

		ident_to_elem.insert("PatientName", &tags::PatientName);
		tag_to_elem.insert(0x00100010, &tags::PatientName);

		ident_to_elem.insert("PatientID", &tags::PatientID);
		tag_to_elem.insert(0x00100020, &tags::PatientID);

		ident_to_elem.insert("IssuerOfPatientID", &tags::IssuerOfPatientID);
		tag_to_elem.insert(0x00100021, &tags::IssuerOfPatientID);

		ident_to_elem.insert("TypeOfPatientID", &tags::TypeOfPatientID);
		tag_to_elem.insert(0x00100022, &tags::TypeOfPatientID);

		ident_to_elem.insert("IssuerOfPatientIDQualifiersSequence", &tags::IssuerOfPatientIDQualifiersSequence);
		tag_to_elem.insert(0x00100024, &tags::IssuerOfPatientIDQualifiersSequence);

		ident_to_elem.insert("SourcePatientGroupIdentificationSequence", &tags::SourcePatientGroupIdentificationSequence);
		tag_to_elem.insert(0x00100026, &tags::SourcePatientGroupIdentificationSequence);

		ident_to_elem.insert("GroupOfPatientsIdentificationSequence", &tags::GroupOfPatientsIdentificationSequence);
		tag_to_elem.insert(0x00100027, &tags::GroupOfPatientsIdentificationSequence);

		ident_to_elem.insert("SubjectRelativePositionInImage", &tags::SubjectRelativePositionInImage);
		tag_to_elem.insert(0x00100028, &tags::SubjectRelativePositionInImage);

		ident_to_elem.insert("PatientBirthDate", &tags::PatientBirthDate);
		tag_to_elem.insert(0x00100030, &tags::PatientBirthDate);

		ident_to_elem.insert("PatientBirthTime", &tags::PatientBirthTime);
		tag_to_elem.insert(0x00100032, &tags::PatientBirthTime);

		ident_to_elem.insert("PatientBirthDateInAlternativeCalendar", &tags::PatientBirthDateInAlternativeCalendar);
		tag_to_elem.insert(0x00100033, &tags::PatientBirthDateInAlternativeCalendar);

		ident_to_elem.insert("PatientDeathDateInAlternativeCalendar", &tags::PatientDeathDateInAlternativeCalendar);
		tag_to_elem.insert(0x00100034, &tags::PatientDeathDateInAlternativeCalendar);

		ident_to_elem.insert("PatientAlternativeCalendar", &tags::PatientAlternativeCalendar);
		tag_to_elem.insert(0x00100035, &tags::PatientAlternativeCalendar);

		ident_to_elem.insert("PatientSex", &tags::PatientSex);
		tag_to_elem.insert(0x00100040, &tags::PatientSex);

		ident_to_elem.insert("PatientInsurancePlanCodeSequence", &tags::PatientInsurancePlanCodeSequence);
		tag_to_elem.insert(0x00100050, &tags::PatientInsurancePlanCodeSequence);

		ident_to_elem.insert("PatientPrimaryLanguageCodeSequence", &tags::PatientPrimaryLanguageCodeSequence);
		tag_to_elem.insert(0x00100101, &tags::PatientPrimaryLanguageCodeSequence);

		ident_to_elem.insert("PatientPrimaryLanguageModifierCodeSequence", &tags::PatientPrimaryLanguageModifierCodeSequence);
		tag_to_elem.insert(0x00100102, &tags::PatientPrimaryLanguageModifierCodeSequence);

		ident_to_elem.insert("QualityControlSubject", &tags::QualityControlSubject);
		tag_to_elem.insert(0x00100200, &tags::QualityControlSubject);

		ident_to_elem.insert("QualityControlSubjectTypeCodeSequence", &tags::QualityControlSubjectTypeCodeSequence);
		tag_to_elem.insert(0x00100201, &tags::QualityControlSubjectTypeCodeSequence);

		ident_to_elem.insert("StrainDescription", &tags::StrainDescription);
		tag_to_elem.insert(0x00100212, &tags::StrainDescription);

		ident_to_elem.insert("StrainNomenclature", &tags::StrainNomenclature);
		tag_to_elem.insert(0x00100213, &tags::StrainNomenclature);

		ident_to_elem.insert("StrainStockNumber", &tags::StrainStockNumber);
		tag_to_elem.insert(0x00100214, &tags::StrainStockNumber);

		ident_to_elem.insert("StrainSourceRegistryCodeSequence", &tags::StrainSourceRegistryCodeSequence);
		tag_to_elem.insert(0x00100215, &tags::StrainSourceRegistryCodeSequence);

		ident_to_elem.insert("StrainStockSequence", &tags::StrainStockSequence);
		tag_to_elem.insert(0x00100216, &tags::StrainStockSequence);

		ident_to_elem.insert("StrainSource", &tags::StrainSource);
		tag_to_elem.insert(0x00100217, &tags::StrainSource);

		ident_to_elem.insert("StrainAdditionalInformation", &tags::StrainAdditionalInformation);
		tag_to_elem.insert(0x00100218, &tags::StrainAdditionalInformation);

		ident_to_elem.insert("StrainCodeSequence", &tags::StrainCodeSequence);
		tag_to_elem.insert(0x00100219, &tags::StrainCodeSequence);

		ident_to_elem.insert("OtherPatientIDs", &tags::OtherPatientIDs);
		tag_to_elem.insert(0x00101000, &tags::OtherPatientIDs);

		ident_to_elem.insert("OtherPatientNames", &tags::OtherPatientNames);
		tag_to_elem.insert(0x00101001, &tags::OtherPatientNames);

		ident_to_elem.insert("OtherPatientIDsSequence", &tags::OtherPatientIDsSequence);
		tag_to_elem.insert(0x00101002, &tags::OtherPatientIDsSequence);

		ident_to_elem.insert("PatientBirthName", &tags::PatientBirthName);
		tag_to_elem.insert(0x00101005, &tags::PatientBirthName);

		ident_to_elem.insert("PatientAge", &tags::PatientAge);
		tag_to_elem.insert(0x00101010, &tags::PatientAge);

		ident_to_elem.insert("PatientSize", &tags::PatientSize);
		tag_to_elem.insert(0x00101020, &tags::PatientSize);

		ident_to_elem.insert("PatientSizeCodeSequence", &tags::PatientSizeCodeSequence);
		tag_to_elem.insert(0x00101021, &tags::PatientSizeCodeSequence);

		ident_to_elem.insert("PatientBodyMassIndex", &tags::PatientBodyMassIndex);
		tag_to_elem.insert(0x00101022, &tags::PatientBodyMassIndex);

		ident_to_elem.insert("MeasuredAPDimension", &tags::MeasuredAPDimension);
		tag_to_elem.insert(0x00101023, &tags::MeasuredAPDimension);

		ident_to_elem.insert("MeasuredLateralDimension", &tags::MeasuredLateralDimension);
		tag_to_elem.insert(0x00101024, &tags::MeasuredLateralDimension);

		ident_to_elem.insert("PatientWeight", &tags::PatientWeight);
		tag_to_elem.insert(0x00101030, &tags::PatientWeight);

		ident_to_elem.insert("PatientAddress", &tags::PatientAddress);
		tag_to_elem.insert(0x00101040, &tags::PatientAddress);

		ident_to_elem.insert("InsurancePlanIdentification", &tags::InsurancePlanIdentification);
		tag_to_elem.insert(0x00101050, &tags::InsurancePlanIdentification);

		ident_to_elem.insert("PatientMotherBirthName", &tags::PatientMotherBirthName);
		tag_to_elem.insert(0x00101060, &tags::PatientMotherBirthName);

		ident_to_elem.insert("MilitaryRank", &tags::MilitaryRank);
		tag_to_elem.insert(0x00101080, &tags::MilitaryRank);

		ident_to_elem.insert("BranchOfService", &tags::BranchOfService);
		tag_to_elem.insert(0x00101081, &tags::BranchOfService);

		ident_to_elem.insert("MedicalRecordLocator", &tags::MedicalRecordLocator);
		tag_to_elem.insert(0x00101090, &tags::MedicalRecordLocator);

		ident_to_elem.insert("ReferencedPatientPhotoSequence", &tags::ReferencedPatientPhotoSequence);
		tag_to_elem.insert(0x00101100, &tags::ReferencedPatientPhotoSequence);

		ident_to_elem.insert("MedicalAlerts", &tags::MedicalAlerts);
		tag_to_elem.insert(0x00102000, &tags::MedicalAlerts);

		ident_to_elem.insert("Allergies", &tags::Allergies);
		tag_to_elem.insert(0x00102110, &tags::Allergies);

		ident_to_elem.insert("CountryOfResidence", &tags::CountryOfResidence);
		tag_to_elem.insert(0x00102150, &tags::CountryOfResidence);

		ident_to_elem.insert("RegionOfResidence", &tags::RegionOfResidence);
		tag_to_elem.insert(0x00102152, &tags::RegionOfResidence);

		ident_to_elem.insert("PatientTelephoneNumbers", &tags::PatientTelephoneNumbers);
		tag_to_elem.insert(0x00102154, &tags::PatientTelephoneNumbers);

		ident_to_elem.insert("PatientTelecomInformation", &tags::PatientTelecomInformation);
		tag_to_elem.insert(0x00102155, &tags::PatientTelecomInformation);

		ident_to_elem.insert("EthnicGroup", &tags::EthnicGroup);
		tag_to_elem.insert(0x00102160, &tags::EthnicGroup);

		ident_to_elem.insert("Occupation", &tags::Occupation);
		tag_to_elem.insert(0x00102180, &tags::Occupation);

		ident_to_elem.insert("SmokingStatus", &tags::SmokingStatus);
		tag_to_elem.insert(0x001021A0, &tags::SmokingStatus);

		ident_to_elem.insert("AdditionalPatientHistory", &tags::AdditionalPatientHistory);
		tag_to_elem.insert(0x001021B0, &tags::AdditionalPatientHistory);

		ident_to_elem.insert("PregnancyStatus", &tags::PregnancyStatus);
		tag_to_elem.insert(0x001021C0, &tags::PregnancyStatus);

		ident_to_elem.insert("LastMenstrualDate", &tags::LastMenstrualDate);
		tag_to_elem.insert(0x001021D0, &tags::LastMenstrualDate);

		ident_to_elem.insert("PatientReligiousPreference", &tags::PatientReligiousPreference);
		tag_to_elem.insert(0x001021F0, &tags::PatientReligiousPreference);

		ident_to_elem.insert("PatientSpeciesDescription", &tags::PatientSpeciesDescription);
		tag_to_elem.insert(0x00102201, &tags::PatientSpeciesDescription);

		ident_to_elem.insert("PatientSpeciesCodeSequence", &tags::PatientSpeciesCodeSequence);
		tag_to_elem.insert(0x00102202, &tags::PatientSpeciesCodeSequence);

		ident_to_elem.insert("PatientSexNeutered", &tags::PatientSexNeutered);
		tag_to_elem.insert(0x00102203, &tags::PatientSexNeutered);

		ident_to_elem.insert("AnatomicalOrientationType", &tags::AnatomicalOrientationType);
		tag_to_elem.insert(0x00102210, &tags::AnatomicalOrientationType);

		ident_to_elem.insert("PatientBreedDescription", &tags::PatientBreedDescription);
		tag_to_elem.insert(0x00102292, &tags::PatientBreedDescription);

		ident_to_elem.insert("PatientBreedCodeSequence", &tags::PatientBreedCodeSequence);
		tag_to_elem.insert(0x00102293, &tags::PatientBreedCodeSequence);

		ident_to_elem.insert("BreedRegistrationSequence", &tags::BreedRegistrationSequence);
		tag_to_elem.insert(0x00102294, &tags::BreedRegistrationSequence);

		ident_to_elem.insert("BreedRegistrationNumber", &tags::BreedRegistrationNumber);
		tag_to_elem.insert(0x00102295, &tags::BreedRegistrationNumber);

		ident_to_elem.insert("BreedRegistryCodeSequence", &tags::BreedRegistryCodeSequence);
		tag_to_elem.insert(0x00102296, &tags::BreedRegistryCodeSequence);

		ident_to_elem.insert("ResponsiblePerson", &tags::ResponsiblePerson);
		tag_to_elem.insert(0x00102297, &tags::ResponsiblePerson);

		ident_to_elem.insert("ResponsiblePersonRole", &tags::ResponsiblePersonRole);
		tag_to_elem.insert(0x00102298, &tags::ResponsiblePersonRole);

		ident_to_elem.insert("ResponsibleOrganization", &tags::ResponsibleOrganization);
		tag_to_elem.insert(0x00102299, &tags::ResponsibleOrganization);

		ident_to_elem.insert("PatientComments", &tags::PatientComments);
		tag_to_elem.insert(0x00104000, &tags::PatientComments);

		ident_to_elem.insert("ExaminedBodyThickness", &tags::ExaminedBodyThickness);
		tag_to_elem.insert(0x00109431, &tags::ExaminedBodyThickness);

		ident_to_elem.insert("ClinicalTrialSponsorName", &tags::ClinicalTrialSponsorName);
		tag_to_elem.insert(0x00120010, &tags::ClinicalTrialSponsorName);

		ident_to_elem.insert("ClinicalTrialProtocolID", &tags::ClinicalTrialProtocolID);
		tag_to_elem.insert(0x00120020, &tags::ClinicalTrialProtocolID);

		ident_to_elem.insert("ClinicalTrialProtocolName", &tags::ClinicalTrialProtocolName);
		tag_to_elem.insert(0x00120021, &tags::ClinicalTrialProtocolName);

		ident_to_elem.insert("ClinicalTrialSiteID", &tags::ClinicalTrialSiteID);
		tag_to_elem.insert(0x00120030, &tags::ClinicalTrialSiteID);

		ident_to_elem.insert("ClinicalTrialSiteName", &tags::ClinicalTrialSiteName);
		tag_to_elem.insert(0x00120031, &tags::ClinicalTrialSiteName);

		ident_to_elem.insert("ClinicalTrialSubjectID", &tags::ClinicalTrialSubjectID);
		tag_to_elem.insert(0x00120040, &tags::ClinicalTrialSubjectID);

		ident_to_elem.insert("ClinicalTrialSubjectReadingID", &tags::ClinicalTrialSubjectReadingID);
		tag_to_elem.insert(0x00120042, &tags::ClinicalTrialSubjectReadingID);

		ident_to_elem.insert("ClinicalTrialTimePointID", &tags::ClinicalTrialTimePointID);
		tag_to_elem.insert(0x00120050, &tags::ClinicalTrialTimePointID);

		ident_to_elem.insert("ClinicalTrialTimePointDescription", &tags::ClinicalTrialTimePointDescription);
		tag_to_elem.insert(0x00120051, &tags::ClinicalTrialTimePointDescription);

		ident_to_elem.insert("ClinicalTrialCoordinatingCenterName", &tags::ClinicalTrialCoordinatingCenterName);
		tag_to_elem.insert(0x00120060, &tags::ClinicalTrialCoordinatingCenterName);

		ident_to_elem.insert("PatientIdentityRemoved", &tags::PatientIdentityRemoved);
		tag_to_elem.insert(0x00120062, &tags::PatientIdentityRemoved);

		ident_to_elem.insert("DeidentificationMethod", &tags::DeidentificationMethod);
		tag_to_elem.insert(0x00120063, &tags::DeidentificationMethod);

		ident_to_elem.insert("DeidentificationMethodCodeSequence", &tags::DeidentificationMethodCodeSequence);
		tag_to_elem.insert(0x00120064, &tags::DeidentificationMethodCodeSequence);

		ident_to_elem.insert("ClinicalTrialSeriesID", &tags::ClinicalTrialSeriesID);
		tag_to_elem.insert(0x00120071, &tags::ClinicalTrialSeriesID);

		ident_to_elem.insert("ClinicalTrialSeriesDescription", &tags::ClinicalTrialSeriesDescription);
		tag_to_elem.insert(0x00120072, &tags::ClinicalTrialSeriesDescription);

		ident_to_elem.insert("ClinicalTrialProtocolEthicsCommitteeName", &tags::ClinicalTrialProtocolEthicsCommitteeName);
		tag_to_elem.insert(0x00120081, &tags::ClinicalTrialProtocolEthicsCommitteeName);

		ident_to_elem.insert("ClinicalTrialProtocolEthicsCommitteeApprovalNumber", &tags::ClinicalTrialProtocolEthicsCommitteeApprovalNumber);
		tag_to_elem.insert(0x00120082, &tags::ClinicalTrialProtocolEthicsCommitteeApprovalNumber);

		ident_to_elem.insert("ConsentForClinicalTrialUseSequence", &tags::ConsentForClinicalTrialUseSequence);
		tag_to_elem.insert(0x00120083, &tags::ConsentForClinicalTrialUseSequence);

		ident_to_elem.insert("DistributionType", &tags::DistributionType);
		tag_to_elem.insert(0x00120084, &tags::DistributionType);

		ident_to_elem.insert("ConsentForDistributionFlag", &tags::ConsentForDistributionFlag);
		tag_to_elem.insert(0x00120085, &tags::ConsentForDistributionFlag);

		ident_to_elem.insert("EthicsCommitteeApprovalEffectivenessStartDate", &tags::EthicsCommitteeApprovalEffectivenessStartDate);
		tag_to_elem.insert(0x00120086, &tags::EthicsCommitteeApprovalEffectivenessStartDate);

		ident_to_elem.insert("EthicsCommitteeApprovalEffectivenessEndDate", &tags::EthicsCommitteeApprovalEffectivenessEndDate);
		tag_to_elem.insert(0x00120087, &tags::EthicsCommitteeApprovalEffectivenessEndDate);

		ident_to_elem.insert("CADFileFormat", &tags::CADFileFormat);
		tag_to_elem.insert(0x00140023, &tags::CADFileFormat);

		ident_to_elem.insert("ComponentReferenceSystem", &tags::ComponentReferenceSystem);
		tag_to_elem.insert(0x00140024, &tags::ComponentReferenceSystem);

		ident_to_elem.insert("ComponentManufacturingProcedure", &tags::ComponentManufacturingProcedure);
		tag_to_elem.insert(0x00140025, &tags::ComponentManufacturingProcedure);

		ident_to_elem.insert("ComponentManufacturer", &tags::ComponentManufacturer);
		tag_to_elem.insert(0x00140028, &tags::ComponentManufacturer);

		ident_to_elem.insert("MaterialThickness", &tags::MaterialThickness);
		tag_to_elem.insert(0x00140030, &tags::MaterialThickness);

		ident_to_elem.insert("MaterialPipeDiameter", &tags::MaterialPipeDiameter);
		tag_to_elem.insert(0x00140032, &tags::MaterialPipeDiameter);

		ident_to_elem.insert("MaterialIsolationDiameter", &tags::MaterialIsolationDiameter);
		tag_to_elem.insert(0x00140034, &tags::MaterialIsolationDiameter);

		ident_to_elem.insert("MaterialGrade", &tags::MaterialGrade);
		tag_to_elem.insert(0x00140042, &tags::MaterialGrade);

		ident_to_elem.insert("MaterialPropertiesDescription", &tags::MaterialPropertiesDescription);
		tag_to_elem.insert(0x00140044, &tags::MaterialPropertiesDescription);

		ident_to_elem.insert("MaterialPropertiesFileFormatRetired", &tags::MaterialPropertiesFileFormatRetired);
		tag_to_elem.insert(0x00140045, &tags::MaterialPropertiesFileFormatRetired);

		ident_to_elem.insert("MaterialNotes", &tags::MaterialNotes);
		tag_to_elem.insert(0x00140046, &tags::MaterialNotes);

		ident_to_elem.insert("ComponentShape", &tags::ComponentShape);
		tag_to_elem.insert(0x00140050, &tags::ComponentShape);

		ident_to_elem.insert("CurvatureType", &tags::CurvatureType);
		tag_to_elem.insert(0x00140052, &tags::CurvatureType);

		ident_to_elem.insert("OuterDiameter", &tags::OuterDiameter);
		tag_to_elem.insert(0x00140054, &tags::OuterDiameter);

		ident_to_elem.insert("InnerDiameter", &tags::InnerDiameter);
		tag_to_elem.insert(0x00140056, &tags::InnerDiameter);

		ident_to_elem.insert("ComponentWelderIDs", &tags::ComponentWelderIDs);
		tag_to_elem.insert(0x00140100, &tags::ComponentWelderIDs);

		ident_to_elem.insert("SecondaryApprovalStatus", &tags::SecondaryApprovalStatus);
		tag_to_elem.insert(0x00140101, &tags::SecondaryApprovalStatus);

		ident_to_elem.insert("SecondaryReviewDate", &tags::SecondaryReviewDate);
		tag_to_elem.insert(0x00140102, &tags::SecondaryReviewDate);

		ident_to_elem.insert("SecondaryReviewTime", &tags::SecondaryReviewTime);
		tag_to_elem.insert(0x00140103, &tags::SecondaryReviewTime);

		ident_to_elem.insert("SecondaryReviewerName", &tags::SecondaryReviewerName);
		tag_to_elem.insert(0x00140104, &tags::SecondaryReviewerName);

		ident_to_elem.insert("RepairID", &tags::RepairID);
		tag_to_elem.insert(0x00140105, &tags::RepairID);

		ident_to_elem.insert("MultipleComponentApprovalSequence", &tags::MultipleComponentApprovalSequence);
		tag_to_elem.insert(0x00140106, &tags::MultipleComponentApprovalSequence);

		ident_to_elem.insert("OtherApprovalStatus", &tags::OtherApprovalStatus);
		tag_to_elem.insert(0x00140107, &tags::OtherApprovalStatus);

		ident_to_elem.insert("OtherSecondaryApprovalStatus", &tags::OtherSecondaryApprovalStatus);
		tag_to_elem.insert(0x00140108, &tags::OtherSecondaryApprovalStatus);

		ident_to_elem.insert("ActualEnvironmentalConditions", &tags::ActualEnvironmentalConditions);
		tag_to_elem.insert(0x00141010, &tags::ActualEnvironmentalConditions);

		ident_to_elem.insert("ExpiryDate", &tags::ExpiryDate);
		tag_to_elem.insert(0x00141020, &tags::ExpiryDate);

		ident_to_elem.insert("EnvironmentalConditions", &tags::EnvironmentalConditions);
		tag_to_elem.insert(0x00141040, &tags::EnvironmentalConditions);

		ident_to_elem.insert("EvaluatorSequence", &tags::EvaluatorSequence);
		tag_to_elem.insert(0x00142002, &tags::EvaluatorSequence);

		ident_to_elem.insert("EvaluatorNumber", &tags::EvaluatorNumber);
		tag_to_elem.insert(0x00142004, &tags::EvaluatorNumber);

		ident_to_elem.insert("EvaluatorName", &tags::EvaluatorName);
		tag_to_elem.insert(0x00142006, &tags::EvaluatorName);

		ident_to_elem.insert("EvaluationAttempt", &tags::EvaluationAttempt);
		tag_to_elem.insert(0x00142008, &tags::EvaluationAttempt);

		ident_to_elem.insert("IndicationSequence", &tags::IndicationSequence);
		tag_to_elem.insert(0x00142012, &tags::IndicationSequence);

		ident_to_elem.insert("IndicationNumber", &tags::IndicationNumber);
		tag_to_elem.insert(0x00142014, &tags::IndicationNumber);

		ident_to_elem.insert("IndicationLabel", &tags::IndicationLabel);
		tag_to_elem.insert(0x00142016, &tags::IndicationLabel);

		ident_to_elem.insert("IndicationDescription", &tags::IndicationDescription);
		tag_to_elem.insert(0x00142018, &tags::IndicationDescription);

		ident_to_elem.insert("IndicationType", &tags::IndicationType);
		tag_to_elem.insert(0x0014201A, &tags::IndicationType);

		ident_to_elem.insert("IndicationDisposition", &tags::IndicationDisposition);
		tag_to_elem.insert(0x0014201C, &tags::IndicationDisposition);

		ident_to_elem.insert("IndicationROISequence", &tags::IndicationROISequence);
		tag_to_elem.insert(0x0014201E, &tags::IndicationROISequence);

		ident_to_elem.insert("IndicationPhysicalPropertySequence", &tags::IndicationPhysicalPropertySequence);
		tag_to_elem.insert(0x00142030, &tags::IndicationPhysicalPropertySequence);

		ident_to_elem.insert("PropertyLabel", &tags::PropertyLabel);
		tag_to_elem.insert(0x00142032, &tags::PropertyLabel);

		ident_to_elem.insert("CoordinateSystemNumberOfAxes", &tags::CoordinateSystemNumberOfAxes);
		tag_to_elem.insert(0x00142202, &tags::CoordinateSystemNumberOfAxes);

		ident_to_elem.insert("CoordinateSystemAxesSequence", &tags::CoordinateSystemAxesSequence);
		tag_to_elem.insert(0x00142204, &tags::CoordinateSystemAxesSequence);

		ident_to_elem.insert("CoordinateSystemAxisDescription", &tags::CoordinateSystemAxisDescription);
		tag_to_elem.insert(0x00142206, &tags::CoordinateSystemAxisDescription);

		ident_to_elem.insert("CoordinateSystemDataSetMapping", &tags::CoordinateSystemDataSetMapping);
		tag_to_elem.insert(0x00142208, &tags::CoordinateSystemDataSetMapping);

		ident_to_elem.insert("CoordinateSystemAxisNumber", &tags::CoordinateSystemAxisNumber);
		tag_to_elem.insert(0x0014220A, &tags::CoordinateSystemAxisNumber);

		ident_to_elem.insert("CoordinateSystemAxisType", &tags::CoordinateSystemAxisType);
		tag_to_elem.insert(0x0014220C, &tags::CoordinateSystemAxisType);

		ident_to_elem.insert("CoordinateSystemAxisUnits", &tags::CoordinateSystemAxisUnits);
		tag_to_elem.insert(0x0014220E, &tags::CoordinateSystemAxisUnits);

		ident_to_elem.insert("CoordinateSystemAxisValues", &tags::CoordinateSystemAxisValues);
		tag_to_elem.insert(0x00142210, &tags::CoordinateSystemAxisValues);

		ident_to_elem.insert("CoordinateSystemTransformSequence", &tags::CoordinateSystemTransformSequence);
		tag_to_elem.insert(0x00142220, &tags::CoordinateSystemTransformSequence);

		ident_to_elem.insert("TransformDescription", &tags::TransformDescription);
		tag_to_elem.insert(0x00142222, &tags::TransformDescription);

		ident_to_elem.insert("TransformNumberOfAxes", &tags::TransformNumberOfAxes);
		tag_to_elem.insert(0x00142224, &tags::TransformNumberOfAxes);

		ident_to_elem.insert("TransformOrderOfAxes", &tags::TransformOrderOfAxes);
		tag_to_elem.insert(0x00142226, &tags::TransformOrderOfAxes);

		ident_to_elem.insert("TransformedAxisUnits", &tags::TransformedAxisUnits);
		tag_to_elem.insert(0x00142228, &tags::TransformedAxisUnits);

		ident_to_elem.insert("CoordinateSystemTransformRotationAndScaleMatrix", &tags::CoordinateSystemTransformRotationAndScaleMatrix);
		tag_to_elem.insert(0x0014222A, &tags::CoordinateSystemTransformRotationAndScaleMatrix);

		ident_to_elem.insert("CoordinateSystemTransformTranslationMatrix", &tags::CoordinateSystemTransformTranslationMatrix);
		tag_to_elem.insert(0x0014222C, &tags::CoordinateSystemTransformTranslationMatrix);

		ident_to_elem.insert("InternalDetectorFrameTime", &tags::InternalDetectorFrameTime);
		tag_to_elem.insert(0x00143011, &tags::InternalDetectorFrameTime);

		ident_to_elem.insert("NumberOfFramesIntegrated", &tags::NumberOfFramesIntegrated);
		tag_to_elem.insert(0x00143012, &tags::NumberOfFramesIntegrated);

		ident_to_elem.insert("DetectorTemperatureSequence", &tags::DetectorTemperatureSequence);
		tag_to_elem.insert(0x00143020, &tags::DetectorTemperatureSequence);

		ident_to_elem.insert("SensorName", &tags::SensorName);
		tag_to_elem.insert(0x00143022, &tags::SensorName);

		ident_to_elem.insert("HorizontalOffsetOfSensor", &tags::HorizontalOffsetOfSensor);
		tag_to_elem.insert(0x00143024, &tags::HorizontalOffsetOfSensor);

		ident_to_elem.insert("VerticalOffsetOfSensor", &tags::VerticalOffsetOfSensor);
		tag_to_elem.insert(0x00143026, &tags::VerticalOffsetOfSensor);

		ident_to_elem.insert("SensorTemperature", &tags::SensorTemperature);
		tag_to_elem.insert(0x00143028, &tags::SensorTemperature);

		ident_to_elem.insert("DarkCurrentSequence", &tags::DarkCurrentSequence);
		tag_to_elem.insert(0x00143040, &tags::DarkCurrentSequence);

		ident_to_elem.insert("DarkCurrentCounts", &tags::DarkCurrentCounts);
		tag_to_elem.insert(0x00143050, &tags::DarkCurrentCounts);

		ident_to_elem.insert("GainCorrectionReferenceSequence", &tags::GainCorrectionReferenceSequence);
		tag_to_elem.insert(0x00143060, &tags::GainCorrectionReferenceSequence);

		ident_to_elem.insert("AirCounts", &tags::AirCounts);
		tag_to_elem.insert(0x00143070, &tags::AirCounts);

		ident_to_elem.insert("KVUsedInGainCalibration", &tags::KVUsedInGainCalibration);
		tag_to_elem.insert(0x00143071, &tags::KVUsedInGainCalibration);

		ident_to_elem.insert("MAUsedInGainCalibration", &tags::MAUsedInGainCalibration);
		tag_to_elem.insert(0x00143072, &tags::MAUsedInGainCalibration);

		ident_to_elem.insert("NumberOfFramesUsedForIntegration", &tags::NumberOfFramesUsedForIntegration);
		tag_to_elem.insert(0x00143073, &tags::NumberOfFramesUsedForIntegration);

		ident_to_elem.insert("FilterMaterialUsedInGainCalibration", &tags::FilterMaterialUsedInGainCalibration);
		tag_to_elem.insert(0x00143074, &tags::FilterMaterialUsedInGainCalibration);

		ident_to_elem.insert("FilterThicknessUsedInGainCalibration", &tags::FilterThicknessUsedInGainCalibration);
		tag_to_elem.insert(0x00143075, &tags::FilterThicknessUsedInGainCalibration);

		ident_to_elem.insert("DateOfGainCalibration", &tags::DateOfGainCalibration);
		tag_to_elem.insert(0x00143076, &tags::DateOfGainCalibration);

		ident_to_elem.insert("TimeOfGainCalibration", &tags::TimeOfGainCalibration);
		tag_to_elem.insert(0x00143077, &tags::TimeOfGainCalibration);

		ident_to_elem.insert("BadPixelImage", &tags::BadPixelImage);
		tag_to_elem.insert(0x00143080, &tags::BadPixelImage);

		ident_to_elem.insert("CalibrationNotes", &tags::CalibrationNotes);
		tag_to_elem.insert(0x00143099, &tags::CalibrationNotes);

		ident_to_elem.insert("PulserEquipmentSequence", &tags::PulserEquipmentSequence);
		tag_to_elem.insert(0x00144002, &tags::PulserEquipmentSequence);

		ident_to_elem.insert("PulserType", &tags::PulserType);
		tag_to_elem.insert(0x00144004, &tags::PulserType);

		ident_to_elem.insert("PulserNotes", &tags::PulserNotes);
		tag_to_elem.insert(0x00144006, &tags::PulserNotes);

		ident_to_elem.insert("ReceiverEquipmentSequence", &tags::ReceiverEquipmentSequence);
		tag_to_elem.insert(0x00144008, &tags::ReceiverEquipmentSequence);

		ident_to_elem.insert("AmplifierType", &tags::AmplifierType);
		tag_to_elem.insert(0x0014400A, &tags::AmplifierType);

		ident_to_elem.insert("ReceiverNotes", &tags::ReceiverNotes);
		tag_to_elem.insert(0x0014400C, &tags::ReceiverNotes);

		ident_to_elem.insert("PreAmplifierEquipmentSequence", &tags::PreAmplifierEquipmentSequence);
		tag_to_elem.insert(0x0014400E, &tags::PreAmplifierEquipmentSequence);

		ident_to_elem.insert("PreAmplifierNotes", &tags::PreAmplifierNotes);
		tag_to_elem.insert(0x0014400F, &tags::PreAmplifierNotes);

		ident_to_elem.insert("TransmitTransducerSequence", &tags::TransmitTransducerSequence);
		tag_to_elem.insert(0x00144010, &tags::TransmitTransducerSequence);

		ident_to_elem.insert("ReceiveTransducerSequence", &tags::ReceiveTransducerSequence);
		tag_to_elem.insert(0x00144011, &tags::ReceiveTransducerSequence);

		ident_to_elem.insert("NumberOfElements", &tags::NumberOfElements);
		tag_to_elem.insert(0x00144012, &tags::NumberOfElements);

		ident_to_elem.insert("ElementShape", &tags::ElementShape);
		tag_to_elem.insert(0x00144013, &tags::ElementShape);

		ident_to_elem.insert("ElementDimensionA", &tags::ElementDimensionA);
		tag_to_elem.insert(0x00144014, &tags::ElementDimensionA);

		ident_to_elem.insert("ElementDimensionB", &tags::ElementDimensionB);
		tag_to_elem.insert(0x00144015, &tags::ElementDimensionB);

		ident_to_elem.insert("ElementPitchA", &tags::ElementPitchA);
		tag_to_elem.insert(0x00144016, &tags::ElementPitchA);

		ident_to_elem.insert("MeasuredBeamDimensionA", &tags::MeasuredBeamDimensionA);
		tag_to_elem.insert(0x00144017, &tags::MeasuredBeamDimensionA);

		ident_to_elem.insert("MeasuredBeamDimensionB", &tags::MeasuredBeamDimensionB);
		tag_to_elem.insert(0x00144018, &tags::MeasuredBeamDimensionB);

		ident_to_elem.insert("LocationOfMeasuredBeamDiameter", &tags::LocationOfMeasuredBeamDiameter);
		tag_to_elem.insert(0x00144019, &tags::LocationOfMeasuredBeamDiameter);

		ident_to_elem.insert("NominalFrequency", &tags::NominalFrequency);
		tag_to_elem.insert(0x0014401A, &tags::NominalFrequency);

		ident_to_elem.insert("MeasuredCenterFrequency", &tags::MeasuredCenterFrequency);
		tag_to_elem.insert(0x0014401B, &tags::MeasuredCenterFrequency);

		ident_to_elem.insert("MeasuredBandwidth", &tags::MeasuredBandwidth);
		tag_to_elem.insert(0x0014401C, &tags::MeasuredBandwidth);

		ident_to_elem.insert("ElementPitchB", &tags::ElementPitchB);
		tag_to_elem.insert(0x0014401D, &tags::ElementPitchB);

		ident_to_elem.insert("PulserSettingsSequence", &tags::PulserSettingsSequence);
		tag_to_elem.insert(0x00144020, &tags::PulserSettingsSequence);

		ident_to_elem.insert("PulseWidth", &tags::PulseWidth);
		tag_to_elem.insert(0x00144022, &tags::PulseWidth);

		ident_to_elem.insert("ExcitationFrequency", &tags::ExcitationFrequency);
		tag_to_elem.insert(0x00144024, &tags::ExcitationFrequency);

		ident_to_elem.insert("ModulationType", &tags::ModulationType);
		tag_to_elem.insert(0x00144026, &tags::ModulationType);

		ident_to_elem.insert("Damping", &tags::Damping);
		tag_to_elem.insert(0x00144028, &tags::Damping);

		ident_to_elem.insert("ReceiverSettingsSequence", &tags::ReceiverSettingsSequence);
		tag_to_elem.insert(0x00144030, &tags::ReceiverSettingsSequence);

		ident_to_elem.insert("AcquiredSoundpathLength", &tags::AcquiredSoundpathLength);
		tag_to_elem.insert(0x00144031, &tags::AcquiredSoundpathLength);

		ident_to_elem.insert("AcquisitionCompressionType", &tags::AcquisitionCompressionType);
		tag_to_elem.insert(0x00144032, &tags::AcquisitionCompressionType);

		ident_to_elem.insert("AcquisitionSampleSize", &tags::AcquisitionSampleSize);
		tag_to_elem.insert(0x00144033, &tags::AcquisitionSampleSize);

		ident_to_elem.insert("RectifierSmoothing", &tags::RectifierSmoothing);
		tag_to_elem.insert(0x00144034, &tags::RectifierSmoothing);

		ident_to_elem.insert("DACSequence", &tags::DACSequence);
		tag_to_elem.insert(0x00144035, &tags::DACSequence);

		ident_to_elem.insert("DACType", &tags::DACType);
		tag_to_elem.insert(0x00144036, &tags::DACType);

		ident_to_elem.insert("DACGainPoints", &tags::DACGainPoints);
		tag_to_elem.insert(0x00144038, &tags::DACGainPoints);

		ident_to_elem.insert("DACTimePoints", &tags::DACTimePoints);
		tag_to_elem.insert(0x0014403A, &tags::DACTimePoints);

		ident_to_elem.insert("DACAmplitude", &tags::DACAmplitude);
		tag_to_elem.insert(0x0014403C, &tags::DACAmplitude);

		ident_to_elem.insert("PreAmplifierSettingsSequence", &tags::PreAmplifierSettingsSequence);
		tag_to_elem.insert(0x00144040, &tags::PreAmplifierSettingsSequence);

		ident_to_elem.insert("TransmitTransducerSettingsSequence", &tags::TransmitTransducerSettingsSequence);
		tag_to_elem.insert(0x00144050, &tags::TransmitTransducerSettingsSequence);

		ident_to_elem.insert("ReceiveTransducerSettingsSequence", &tags::ReceiveTransducerSettingsSequence);
		tag_to_elem.insert(0x00144051, &tags::ReceiveTransducerSettingsSequence);

		ident_to_elem.insert("IncidentAngle", &tags::IncidentAngle);
		tag_to_elem.insert(0x00144052, &tags::IncidentAngle);

		ident_to_elem.insert("CouplingTechnique", &tags::CouplingTechnique);
		tag_to_elem.insert(0x00144054, &tags::CouplingTechnique);

		ident_to_elem.insert("CouplingMedium", &tags::CouplingMedium);
		tag_to_elem.insert(0x00144056, &tags::CouplingMedium);

		ident_to_elem.insert("CouplingVelocity", &tags::CouplingVelocity);
		tag_to_elem.insert(0x00144057, &tags::CouplingVelocity);

		ident_to_elem.insert("ProbeCenterLocationX", &tags::ProbeCenterLocationX);
		tag_to_elem.insert(0x00144058, &tags::ProbeCenterLocationX);

		ident_to_elem.insert("ProbeCenterLocationZ", &tags::ProbeCenterLocationZ);
		tag_to_elem.insert(0x00144059, &tags::ProbeCenterLocationZ);

		ident_to_elem.insert("SoundPathLength", &tags::SoundPathLength);
		tag_to_elem.insert(0x0014405A, &tags::SoundPathLength);

		ident_to_elem.insert("DelayLawIdentifier", &tags::DelayLawIdentifier);
		tag_to_elem.insert(0x0014405C, &tags::DelayLawIdentifier);

		ident_to_elem.insert("GateSettingsSequence", &tags::GateSettingsSequence);
		tag_to_elem.insert(0x00144060, &tags::GateSettingsSequence);

		ident_to_elem.insert("GateThreshold", &tags::GateThreshold);
		tag_to_elem.insert(0x00144062, &tags::GateThreshold);

		ident_to_elem.insert("VelocityOfSound", &tags::VelocityOfSound);
		tag_to_elem.insert(0x00144064, &tags::VelocityOfSound);

		ident_to_elem.insert("CalibrationSettingsSequence", &tags::CalibrationSettingsSequence);
		tag_to_elem.insert(0x00144070, &tags::CalibrationSettingsSequence);

		ident_to_elem.insert("CalibrationProcedure", &tags::CalibrationProcedure);
		tag_to_elem.insert(0x00144072, &tags::CalibrationProcedure);

		ident_to_elem.insert("ProcedureVersion", &tags::ProcedureVersion);
		tag_to_elem.insert(0x00144074, &tags::ProcedureVersion);

		ident_to_elem.insert("ProcedureCreationDate", &tags::ProcedureCreationDate);
		tag_to_elem.insert(0x00144076, &tags::ProcedureCreationDate);

		ident_to_elem.insert("ProcedureExpirationDate", &tags::ProcedureExpirationDate);
		tag_to_elem.insert(0x00144078, &tags::ProcedureExpirationDate);

		ident_to_elem.insert("ProcedureLastModifiedDate", &tags::ProcedureLastModifiedDate);
		tag_to_elem.insert(0x0014407A, &tags::ProcedureLastModifiedDate);

		ident_to_elem.insert("CalibrationTime", &tags::CalibrationTime);
		tag_to_elem.insert(0x0014407C, &tags::CalibrationTime);

		ident_to_elem.insert("CalibrationDate", &tags::CalibrationDate);
		tag_to_elem.insert(0x0014407E, &tags::CalibrationDate);

		ident_to_elem.insert("ProbeDriveEquipmentSequence", &tags::ProbeDriveEquipmentSequence);
		tag_to_elem.insert(0x00144080, &tags::ProbeDriveEquipmentSequence);

		ident_to_elem.insert("DriveType", &tags::DriveType);
		tag_to_elem.insert(0x00144081, &tags::DriveType);

		ident_to_elem.insert("ProbeDriveNotes", &tags::ProbeDriveNotes);
		tag_to_elem.insert(0x00144082, &tags::ProbeDriveNotes);

		ident_to_elem.insert("DriveProbeSequence", &tags::DriveProbeSequence);
		tag_to_elem.insert(0x00144083, &tags::DriveProbeSequence);

		ident_to_elem.insert("ProbeInductance", &tags::ProbeInductance);
		tag_to_elem.insert(0x00144084, &tags::ProbeInductance);

		ident_to_elem.insert("ProbeResistance", &tags::ProbeResistance);
		tag_to_elem.insert(0x00144085, &tags::ProbeResistance);

		ident_to_elem.insert("ReceiveProbeSequence", &tags::ReceiveProbeSequence);
		tag_to_elem.insert(0x00144086, &tags::ReceiveProbeSequence);

		ident_to_elem.insert("ProbeDriveSettingsSequence", &tags::ProbeDriveSettingsSequence);
		tag_to_elem.insert(0x00144087, &tags::ProbeDriveSettingsSequence);

		ident_to_elem.insert("BridgeResistors", &tags::BridgeResistors);
		tag_to_elem.insert(0x00144088, &tags::BridgeResistors);

		ident_to_elem.insert("ProbeOrientationAngle", &tags::ProbeOrientationAngle);
		tag_to_elem.insert(0x00144089, &tags::ProbeOrientationAngle);

		ident_to_elem.insert("UserSelectedGainY", &tags::UserSelectedGainY);
		tag_to_elem.insert(0x0014408B, &tags::UserSelectedGainY);

		ident_to_elem.insert("UserSelectedPhase", &tags::UserSelectedPhase);
		tag_to_elem.insert(0x0014408C, &tags::UserSelectedPhase);

		ident_to_elem.insert("UserSelectedOffsetX", &tags::UserSelectedOffsetX);
		tag_to_elem.insert(0x0014408D, &tags::UserSelectedOffsetX);

		ident_to_elem.insert("UserSelectedOffsetY", &tags::UserSelectedOffsetY);
		tag_to_elem.insert(0x0014408E, &tags::UserSelectedOffsetY);

		ident_to_elem.insert("ChannelSettingsSequence", &tags::ChannelSettingsSequence);
		tag_to_elem.insert(0x00144091, &tags::ChannelSettingsSequence);

		ident_to_elem.insert("ChannelThreshold", &tags::ChannelThreshold);
		tag_to_elem.insert(0x00144092, &tags::ChannelThreshold);

		ident_to_elem.insert("ScannerSettingsSequence", &tags::ScannerSettingsSequence);
		tag_to_elem.insert(0x0014409A, &tags::ScannerSettingsSequence);

		ident_to_elem.insert("ScanProcedure", &tags::ScanProcedure);
		tag_to_elem.insert(0x0014409B, &tags::ScanProcedure);

		ident_to_elem.insert("TranslationRateX", &tags::TranslationRateX);
		tag_to_elem.insert(0x0014409C, &tags::TranslationRateX);

		ident_to_elem.insert("TranslationRateY", &tags::TranslationRateY);
		tag_to_elem.insert(0x0014409D, &tags::TranslationRateY);

		ident_to_elem.insert("ChannelOverlap", &tags::ChannelOverlap);
		tag_to_elem.insert(0x0014409F, &tags::ChannelOverlap);

		ident_to_elem.insert("ImageQualityIndicatorType", &tags::ImageQualityIndicatorType);
		tag_to_elem.insert(0x001440A0, &tags::ImageQualityIndicatorType);

		ident_to_elem.insert("ImageQualityIndicatorMaterial", &tags::ImageQualityIndicatorMaterial);
		tag_to_elem.insert(0x001440A1, &tags::ImageQualityIndicatorMaterial);

		ident_to_elem.insert("ImageQualityIndicatorSize", &tags::ImageQualityIndicatorSize);
		tag_to_elem.insert(0x001440A2, &tags::ImageQualityIndicatorSize);

		ident_to_elem.insert("LINACEnergy", &tags::LINACEnergy);
		tag_to_elem.insert(0x00145002, &tags::LINACEnergy);

		ident_to_elem.insert("LINACOutput", &tags::LINACOutput);
		tag_to_elem.insert(0x00145004, &tags::LINACOutput);

		ident_to_elem.insert("ActiveAperture", &tags::ActiveAperture);
		tag_to_elem.insert(0x00145100, &tags::ActiveAperture);

		ident_to_elem.insert("TotalAperture", &tags::TotalAperture);
		tag_to_elem.insert(0x00145101, &tags::TotalAperture);

		ident_to_elem.insert("ApertureElevation", &tags::ApertureElevation);
		tag_to_elem.insert(0x00145102, &tags::ApertureElevation);

		ident_to_elem.insert("MainLobeAngle", &tags::MainLobeAngle);
		tag_to_elem.insert(0x00145103, &tags::MainLobeAngle);

		ident_to_elem.insert("MainRoofAngle", &tags::MainRoofAngle);
		tag_to_elem.insert(0x00145104, &tags::MainRoofAngle);

		ident_to_elem.insert("ConnectorType", &tags::ConnectorType);
		tag_to_elem.insert(0x00145105, &tags::ConnectorType);

		ident_to_elem.insert("WedgeModelNumber", &tags::WedgeModelNumber);
		tag_to_elem.insert(0x00145106, &tags::WedgeModelNumber);

		ident_to_elem.insert("WedgeAngleFloat", &tags::WedgeAngleFloat);
		tag_to_elem.insert(0x00145107, &tags::WedgeAngleFloat);

		ident_to_elem.insert("WedgeRoofAngle", &tags::WedgeRoofAngle);
		tag_to_elem.insert(0x00145108, &tags::WedgeRoofAngle);

		ident_to_elem.insert("WedgeElement1Position", &tags::WedgeElement1Position);
		tag_to_elem.insert(0x00145109, &tags::WedgeElement1Position);

		ident_to_elem.insert("WedgeMaterialVelocity", &tags::WedgeMaterialVelocity);
		tag_to_elem.insert(0x0014510A, &tags::WedgeMaterialVelocity);

		ident_to_elem.insert("WedgeMaterial", &tags::WedgeMaterial);
		tag_to_elem.insert(0x0014510B, &tags::WedgeMaterial);

		ident_to_elem.insert("WedgeOffsetZ", &tags::WedgeOffsetZ);
		tag_to_elem.insert(0x0014510C, &tags::WedgeOffsetZ);

		ident_to_elem.insert("WedgeOriginOffsetX", &tags::WedgeOriginOffsetX);
		tag_to_elem.insert(0x0014510D, &tags::WedgeOriginOffsetX);

		ident_to_elem.insert("WedgeTimeDelay", &tags::WedgeTimeDelay);
		tag_to_elem.insert(0x0014510E, &tags::WedgeTimeDelay);

		ident_to_elem.insert("WedgeName", &tags::WedgeName);
		tag_to_elem.insert(0x0014510F, &tags::WedgeName);

		ident_to_elem.insert("WedgeManufacturerName", &tags::WedgeManufacturerName);
		tag_to_elem.insert(0x00145110, &tags::WedgeManufacturerName);

		ident_to_elem.insert("WedgeDescription", &tags::WedgeDescription);
		tag_to_elem.insert(0x00145111, &tags::WedgeDescription);

		ident_to_elem.insert("NominalBeamAngle", &tags::NominalBeamAngle);
		tag_to_elem.insert(0x00145112, &tags::NominalBeamAngle);

		ident_to_elem.insert("WedgeOffsetX", &tags::WedgeOffsetX);
		tag_to_elem.insert(0x00145113, &tags::WedgeOffsetX);

		ident_to_elem.insert("WedgeOffsetY", &tags::WedgeOffsetY);
		tag_to_elem.insert(0x00145114, &tags::WedgeOffsetY);

		ident_to_elem.insert("WedgeTotalLength", &tags::WedgeTotalLength);
		tag_to_elem.insert(0x00145115, &tags::WedgeTotalLength);

		ident_to_elem.insert("WedgeInContactLength", &tags::WedgeInContactLength);
		tag_to_elem.insert(0x00145116, &tags::WedgeInContactLength);

		ident_to_elem.insert("WedgeFrontGap", &tags::WedgeFrontGap);
		tag_to_elem.insert(0x00145117, &tags::WedgeFrontGap);

		ident_to_elem.insert("WedgeTotalHeight", &tags::WedgeTotalHeight);
		tag_to_elem.insert(0x00145118, &tags::WedgeTotalHeight);

		ident_to_elem.insert("WedgeFrontHeight", &tags::WedgeFrontHeight);
		tag_to_elem.insert(0x00145119, &tags::WedgeFrontHeight);

		ident_to_elem.insert("WedgeRearHeight", &tags::WedgeRearHeight);
		tag_to_elem.insert(0x0014511A, &tags::WedgeRearHeight);

		ident_to_elem.insert("WedgeTotalWidth", &tags::WedgeTotalWidth);
		tag_to_elem.insert(0x0014511B, &tags::WedgeTotalWidth);

		ident_to_elem.insert("WedgeInContactWidth", &tags::WedgeInContactWidth);
		tag_to_elem.insert(0x0014511C, &tags::WedgeInContactWidth);

		ident_to_elem.insert("WedgeChamferHeight", &tags::WedgeChamferHeight);
		tag_to_elem.insert(0x0014511D, &tags::WedgeChamferHeight);

		ident_to_elem.insert("WedgeCurve", &tags::WedgeCurve);
		tag_to_elem.insert(0x0014511E, &tags::WedgeCurve);

		ident_to_elem.insert("RadiusAlongWedge", &tags::RadiusAlongWedge);
		tag_to_elem.insert(0x0014511F, &tags::RadiusAlongWedge);

		ident_to_elem.insert("ContrastBolusAgent", &tags::ContrastBolusAgent);
		tag_to_elem.insert(0x00180010, &tags::ContrastBolusAgent);

		ident_to_elem.insert("ContrastBolusAgentSequence", &tags::ContrastBolusAgentSequence);
		tag_to_elem.insert(0x00180012, &tags::ContrastBolusAgentSequence);

		ident_to_elem.insert("ContrastBolusT1Relaxivity", &tags::ContrastBolusT1Relaxivity);
		tag_to_elem.insert(0x00180013, &tags::ContrastBolusT1Relaxivity);

		ident_to_elem.insert("ContrastBolusAdministrationRouteSequence", &tags::ContrastBolusAdministrationRouteSequence);
		tag_to_elem.insert(0x00180014, &tags::ContrastBolusAdministrationRouteSequence);

		ident_to_elem.insert("BodyPartExamined", &tags::BodyPartExamined);
		tag_to_elem.insert(0x00180015, &tags::BodyPartExamined);

		ident_to_elem.insert("ScanningSequence", &tags::ScanningSequence);
		tag_to_elem.insert(0x00180020, &tags::ScanningSequence);

		ident_to_elem.insert("SequenceVariant", &tags::SequenceVariant);
		tag_to_elem.insert(0x00180021, &tags::SequenceVariant);

		ident_to_elem.insert("ScanOptions", &tags::ScanOptions);
		tag_to_elem.insert(0x00180022, &tags::ScanOptions);

		ident_to_elem.insert("MRAcquisitionType", &tags::MRAcquisitionType);
		tag_to_elem.insert(0x00180023, &tags::MRAcquisitionType);

		ident_to_elem.insert("SequenceName", &tags::SequenceName);
		tag_to_elem.insert(0x00180024, &tags::SequenceName);

		ident_to_elem.insert("AngioFlag", &tags::AngioFlag);
		tag_to_elem.insert(0x00180025, &tags::AngioFlag);

		ident_to_elem.insert("InterventionDrugInformationSequence", &tags::InterventionDrugInformationSequence);
		tag_to_elem.insert(0x00180026, &tags::InterventionDrugInformationSequence);

		ident_to_elem.insert("InterventionDrugStopTime", &tags::InterventionDrugStopTime);
		tag_to_elem.insert(0x00180027, &tags::InterventionDrugStopTime);

		ident_to_elem.insert("InterventionDrugDose", &tags::InterventionDrugDose);
		tag_to_elem.insert(0x00180028, &tags::InterventionDrugDose);

		ident_to_elem.insert("InterventionDrugCodeSequence", &tags::InterventionDrugCodeSequence);
		tag_to_elem.insert(0x00180029, &tags::InterventionDrugCodeSequence);

		ident_to_elem.insert("AdditionalDrugSequence", &tags::AdditionalDrugSequence);
		tag_to_elem.insert(0x0018002A, &tags::AdditionalDrugSequence);

		ident_to_elem.insert("Radionuclide", &tags::Radionuclide);
		tag_to_elem.insert(0x00180030, &tags::Radionuclide);

		ident_to_elem.insert("Radiopharmaceutical", &tags::Radiopharmaceutical);
		tag_to_elem.insert(0x00180031, &tags::Radiopharmaceutical);

		ident_to_elem.insert("EnergyWindowCenterline", &tags::EnergyWindowCenterline);
		tag_to_elem.insert(0x00180032, &tags::EnergyWindowCenterline);

		ident_to_elem.insert("EnergyWindowTotalWidth", &tags::EnergyWindowTotalWidth);
		tag_to_elem.insert(0x00180033, &tags::EnergyWindowTotalWidth);

		ident_to_elem.insert("InterventionDrugName", &tags::InterventionDrugName);
		tag_to_elem.insert(0x00180034, &tags::InterventionDrugName);

		ident_to_elem.insert("InterventionDrugStartTime", &tags::InterventionDrugStartTime);
		tag_to_elem.insert(0x00180035, &tags::InterventionDrugStartTime);

		ident_to_elem.insert("InterventionSequence", &tags::InterventionSequence);
		tag_to_elem.insert(0x00180036, &tags::InterventionSequence);

		ident_to_elem.insert("TherapyType", &tags::TherapyType);
		tag_to_elem.insert(0x00180037, &tags::TherapyType);

		ident_to_elem.insert("InterventionStatus", &tags::InterventionStatus);
		tag_to_elem.insert(0x00180038, &tags::InterventionStatus);

		ident_to_elem.insert("TherapyDescription", &tags::TherapyDescription);
		tag_to_elem.insert(0x00180039, &tags::TherapyDescription);

		ident_to_elem.insert("InterventionDescription", &tags::InterventionDescription);
		tag_to_elem.insert(0x0018003A, &tags::InterventionDescription);

		ident_to_elem.insert("CineRate", &tags::CineRate);
		tag_to_elem.insert(0x00180040, &tags::CineRate);

		ident_to_elem.insert("InitialCineRunState", &tags::InitialCineRunState);
		tag_to_elem.insert(0x00180042, &tags::InitialCineRunState);

		ident_to_elem.insert("SliceThickness", &tags::SliceThickness);
		tag_to_elem.insert(0x00180050, &tags::SliceThickness);

		ident_to_elem.insert("KVP", &tags::KVP);
		tag_to_elem.insert(0x00180060, &tags::KVP);

		ident_to_elem.insert("CountsAccumulated", &tags::CountsAccumulated);
		tag_to_elem.insert(0x00180070, &tags::CountsAccumulated);

		ident_to_elem.insert("AcquisitionTerminationCondition", &tags::AcquisitionTerminationCondition);
		tag_to_elem.insert(0x00180071, &tags::AcquisitionTerminationCondition);

		ident_to_elem.insert("EffectiveDuration", &tags::EffectiveDuration);
		tag_to_elem.insert(0x00180072, &tags::EffectiveDuration);

		ident_to_elem.insert("AcquisitionStartCondition", &tags::AcquisitionStartCondition);
		tag_to_elem.insert(0x00180073, &tags::AcquisitionStartCondition);

		ident_to_elem.insert("AcquisitionStartConditionData", &tags::AcquisitionStartConditionData);
		tag_to_elem.insert(0x00180074, &tags::AcquisitionStartConditionData);

		ident_to_elem.insert("AcquisitionTerminationConditionData", &tags::AcquisitionTerminationConditionData);
		tag_to_elem.insert(0x00180075, &tags::AcquisitionTerminationConditionData);

		ident_to_elem.insert("RepetitionTime", &tags::RepetitionTime);
		tag_to_elem.insert(0x00180080, &tags::RepetitionTime);

		ident_to_elem.insert("EchoTime", &tags::EchoTime);
		tag_to_elem.insert(0x00180081, &tags::EchoTime);

		ident_to_elem.insert("InversionTime", &tags::InversionTime);
		tag_to_elem.insert(0x00180082, &tags::InversionTime);

		ident_to_elem.insert("NumberOfAverages", &tags::NumberOfAverages);
		tag_to_elem.insert(0x00180083, &tags::NumberOfAverages);

		ident_to_elem.insert("ImagingFrequency", &tags::ImagingFrequency);
		tag_to_elem.insert(0x00180084, &tags::ImagingFrequency);

		ident_to_elem.insert("ImagedNucleus", &tags::ImagedNucleus);
		tag_to_elem.insert(0x00180085, &tags::ImagedNucleus);

		ident_to_elem.insert("EchoNumbers", &tags::EchoNumbers);
		tag_to_elem.insert(0x00180086, &tags::EchoNumbers);

		ident_to_elem.insert("MagneticFieldStrength", &tags::MagneticFieldStrength);
		tag_to_elem.insert(0x00180087, &tags::MagneticFieldStrength);

		ident_to_elem.insert("SpacingBetweenSlices", &tags::SpacingBetweenSlices);
		tag_to_elem.insert(0x00180088, &tags::SpacingBetweenSlices);

		ident_to_elem.insert("NumberOfPhaseEncodingSteps", &tags::NumberOfPhaseEncodingSteps);
		tag_to_elem.insert(0x00180089, &tags::NumberOfPhaseEncodingSteps);

		ident_to_elem.insert("DataCollectionDiameter", &tags::DataCollectionDiameter);
		tag_to_elem.insert(0x00180090, &tags::DataCollectionDiameter);

		ident_to_elem.insert("EchoTrainLength", &tags::EchoTrainLength);
		tag_to_elem.insert(0x00180091, &tags::EchoTrainLength);

		ident_to_elem.insert("PercentSampling", &tags::PercentSampling);
		tag_to_elem.insert(0x00180093, &tags::PercentSampling);

		ident_to_elem.insert("PercentPhaseFieldOfView", &tags::PercentPhaseFieldOfView);
		tag_to_elem.insert(0x00180094, &tags::PercentPhaseFieldOfView);

		ident_to_elem.insert("PixelBandwidth", &tags::PixelBandwidth);
		tag_to_elem.insert(0x00180095, &tags::PixelBandwidth);

		ident_to_elem.insert("DeviceSerialNumber", &tags::DeviceSerialNumber);
		tag_to_elem.insert(0x00181000, &tags::DeviceSerialNumber);

		ident_to_elem.insert("DeviceUID", &tags::DeviceUID);
		tag_to_elem.insert(0x00181002, &tags::DeviceUID);

		ident_to_elem.insert("DeviceID", &tags::DeviceID);
		tag_to_elem.insert(0x00181003, &tags::DeviceID);

		ident_to_elem.insert("PlateID", &tags::PlateID);
		tag_to_elem.insert(0x00181004, &tags::PlateID);

		ident_to_elem.insert("GeneratorID", &tags::GeneratorID);
		tag_to_elem.insert(0x00181005, &tags::GeneratorID);

		ident_to_elem.insert("GridID", &tags::GridID);
		tag_to_elem.insert(0x00181006, &tags::GridID);

		ident_to_elem.insert("CassetteID", &tags::CassetteID);
		tag_to_elem.insert(0x00181007, &tags::CassetteID);

		ident_to_elem.insert("GantryID", &tags::GantryID);
		tag_to_elem.insert(0x00181008, &tags::GantryID);

		ident_to_elem.insert("UniqueDeviceIdentifier", &tags::UniqueDeviceIdentifier);
		tag_to_elem.insert(0x00181009, &tags::UniqueDeviceIdentifier);

		ident_to_elem.insert("UDISequence", &tags::UDISequence);
		tag_to_elem.insert(0x0018100A, &tags::UDISequence);

		ident_to_elem.insert("SecondaryCaptureDeviceID", &tags::SecondaryCaptureDeviceID);
		tag_to_elem.insert(0x00181010, &tags::SecondaryCaptureDeviceID);

		ident_to_elem.insert("HardcopyCreationDeviceID", &tags::HardcopyCreationDeviceID);
		tag_to_elem.insert(0x00181011, &tags::HardcopyCreationDeviceID);

		ident_to_elem.insert("DateOfSecondaryCapture", &tags::DateOfSecondaryCapture);
		tag_to_elem.insert(0x00181012, &tags::DateOfSecondaryCapture);

		ident_to_elem.insert("TimeOfSecondaryCapture", &tags::TimeOfSecondaryCapture);
		tag_to_elem.insert(0x00181014, &tags::TimeOfSecondaryCapture);

		ident_to_elem.insert("SecondaryCaptureDeviceManufacturer", &tags::SecondaryCaptureDeviceManufacturer);
		tag_to_elem.insert(0x00181016, &tags::SecondaryCaptureDeviceManufacturer);

		ident_to_elem.insert("HardcopyDeviceManufacturer", &tags::HardcopyDeviceManufacturer);
		tag_to_elem.insert(0x00181017, &tags::HardcopyDeviceManufacturer);

		ident_to_elem.insert("SecondaryCaptureDeviceManufacturerModelName", &tags::SecondaryCaptureDeviceManufacturerModelName);
		tag_to_elem.insert(0x00181018, &tags::SecondaryCaptureDeviceManufacturerModelName);

		ident_to_elem.insert("SecondaryCaptureDeviceSoftwareVersions", &tags::SecondaryCaptureDeviceSoftwareVersions);
		tag_to_elem.insert(0x00181019, &tags::SecondaryCaptureDeviceSoftwareVersions);

		ident_to_elem.insert("HardcopyDeviceSoftwareVersion", &tags::HardcopyDeviceSoftwareVersion);
		tag_to_elem.insert(0x0018101A, &tags::HardcopyDeviceSoftwareVersion);

		ident_to_elem.insert("HardcopyDeviceManufacturerModelName", &tags::HardcopyDeviceManufacturerModelName);
		tag_to_elem.insert(0x0018101B, &tags::HardcopyDeviceManufacturerModelName);

		ident_to_elem.insert("SoftwareVersions", &tags::SoftwareVersions);
		tag_to_elem.insert(0x00181020, &tags::SoftwareVersions);

		ident_to_elem.insert("VideoImageFormatAcquired", &tags::VideoImageFormatAcquired);
		tag_to_elem.insert(0x00181022, &tags::VideoImageFormatAcquired);

		ident_to_elem.insert("DigitalImageFormatAcquired", &tags::DigitalImageFormatAcquired);
		tag_to_elem.insert(0x00181023, &tags::DigitalImageFormatAcquired);

		ident_to_elem.insert("ProtocolName", &tags::ProtocolName);
		tag_to_elem.insert(0x00181030, &tags::ProtocolName);

		ident_to_elem.insert("ContrastBolusRoute", &tags::ContrastBolusRoute);
		tag_to_elem.insert(0x00181040, &tags::ContrastBolusRoute);

		ident_to_elem.insert("ContrastBolusVolume", &tags::ContrastBolusVolume);
		tag_to_elem.insert(0x00181041, &tags::ContrastBolusVolume);

		ident_to_elem.insert("ContrastBolusStartTime", &tags::ContrastBolusStartTime);
		tag_to_elem.insert(0x00181042, &tags::ContrastBolusStartTime);

		ident_to_elem.insert("ContrastBolusStopTime", &tags::ContrastBolusStopTime);
		tag_to_elem.insert(0x00181043, &tags::ContrastBolusStopTime);

		ident_to_elem.insert("ContrastBolusTotalDose", &tags::ContrastBolusTotalDose);
		tag_to_elem.insert(0x00181044, &tags::ContrastBolusTotalDose);

		ident_to_elem.insert("SyringeCounts", &tags::SyringeCounts);
		tag_to_elem.insert(0x00181045, &tags::SyringeCounts);

		ident_to_elem.insert("ContrastFlowRate", &tags::ContrastFlowRate);
		tag_to_elem.insert(0x00181046, &tags::ContrastFlowRate);

		ident_to_elem.insert("ContrastFlowDuration", &tags::ContrastFlowDuration);
		tag_to_elem.insert(0x00181047, &tags::ContrastFlowDuration);

		ident_to_elem.insert("ContrastBolusIngredient", &tags::ContrastBolusIngredient);
		tag_to_elem.insert(0x00181048, &tags::ContrastBolusIngredient);

		ident_to_elem.insert("ContrastBolusIngredientConcentration", &tags::ContrastBolusIngredientConcentration);
		tag_to_elem.insert(0x00181049, &tags::ContrastBolusIngredientConcentration);

		ident_to_elem.insert("SpatialResolution", &tags::SpatialResolution);
		tag_to_elem.insert(0x00181050, &tags::SpatialResolution);

		ident_to_elem.insert("TriggerTime", &tags::TriggerTime);
		tag_to_elem.insert(0x00181060, &tags::TriggerTime);

		ident_to_elem.insert("TriggerSourceOrType", &tags::TriggerSourceOrType);
		tag_to_elem.insert(0x00181061, &tags::TriggerSourceOrType);

		ident_to_elem.insert("NominalInterval", &tags::NominalInterval);
		tag_to_elem.insert(0x00181062, &tags::NominalInterval);

		ident_to_elem.insert("FrameTime", &tags::FrameTime);
		tag_to_elem.insert(0x00181063, &tags::FrameTime);

		ident_to_elem.insert("CardiacFramingType", &tags::CardiacFramingType);
		tag_to_elem.insert(0x00181064, &tags::CardiacFramingType);

		ident_to_elem.insert("FrameTimeVector", &tags::FrameTimeVector);
		tag_to_elem.insert(0x00181065, &tags::FrameTimeVector);

		ident_to_elem.insert("FrameDelay", &tags::FrameDelay);
		tag_to_elem.insert(0x00181066, &tags::FrameDelay);

		ident_to_elem.insert("ImageTriggerDelay", &tags::ImageTriggerDelay);
		tag_to_elem.insert(0x00181067, &tags::ImageTriggerDelay);

		ident_to_elem.insert("MultiplexGroupTimeOffset", &tags::MultiplexGroupTimeOffset);
		tag_to_elem.insert(0x00181068, &tags::MultiplexGroupTimeOffset);

		ident_to_elem.insert("TriggerTimeOffset", &tags::TriggerTimeOffset);
		tag_to_elem.insert(0x00181069, &tags::TriggerTimeOffset);

		ident_to_elem.insert("SynchronizationTrigger", &tags::SynchronizationTrigger);
		tag_to_elem.insert(0x0018106A, &tags::SynchronizationTrigger);

		ident_to_elem.insert("SynchronizationChannel", &tags::SynchronizationChannel);
		tag_to_elem.insert(0x0018106C, &tags::SynchronizationChannel);

		ident_to_elem.insert("TriggerSamplePosition", &tags::TriggerSamplePosition);
		tag_to_elem.insert(0x0018106E, &tags::TriggerSamplePosition);

		ident_to_elem.insert("RadiopharmaceuticalRoute", &tags::RadiopharmaceuticalRoute);
		tag_to_elem.insert(0x00181070, &tags::RadiopharmaceuticalRoute);

		ident_to_elem.insert("RadiopharmaceuticalVolume", &tags::RadiopharmaceuticalVolume);
		tag_to_elem.insert(0x00181071, &tags::RadiopharmaceuticalVolume);

		ident_to_elem.insert("RadiopharmaceuticalStartTime", &tags::RadiopharmaceuticalStartTime);
		tag_to_elem.insert(0x00181072, &tags::RadiopharmaceuticalStartTime);

		ident_to_elem.insert("RadiopharmaceuticalStopTime", &tags::RadiopharmaceuticalStopTime);
		tag_to_elem.insert(0x00181073, &tags::RadiopharmaceuticalStopTime);

		ident_to_elem.insert("RadionuclideTotalDose", &tags::RadionuclideTotalDose);
		tag_to_elem.insert(0x00181074, &tags::RadionuclideTotalDose);

		ident_to_elem.insert("RadionuclideHalfLife", &tags::RadionuclideHalfLife);
		tag_to_elem.insert(0x00181075, &tags::RadionuclideHalfLife);

		ident_to_elem.insert("RadionuclidePositronFraction", &tags::RadionuclidePositronFraction);
		tag_to_elem.insert(0x00181076, &tags::RadionuclidePositronFraction);

		ident_to_elem.insert("RadiopharmaceuticalSpecificActivity", &tags::RadiopharmaceuticalSpecificActivity);
		tag_to_elem.insert(0x00181077, &tags::RadiopharmaceuticalSpecificActivity);

		ident_to_elem.insert("RadiopharmaceuticalStartDateTime", &tags::RadiopharmaceuticalStartDateTime);
		tag_to_elem.insert(0x00181078, &tags::RadiopharmaceuticalStartDateTime);

		ident_to_elem.insert("RadiopharmaceuticalStopDateTime", &tags::RadiopharmaceuticalStopDateTime);
		tag_to_elem.insert(0x00181079, &tags::RadiopharmaceuticalStopDateTime);

		ident_to_elem.insert("BeatRejectionFlag", &tags::BeatRejectionFlag);
		tag_to_elem.insert(0x00181080, &tags::BeatRejectionFlag);

		ident_to_elem.insert("LowRRValue", &tags::LowRRValue);
		tag_to_elem.insert(0x00181081, &tags::LowRRValue);

		ident_to_elem.insert("HighRRValue", &tags::HighRRValue);
		tag_to_elem.insert(0x00181082, &tags::HighRRValue);

		ident_to_elem.insert("IntervalsAcquired", &tags::IntervalsAcquired);
		tag_to_elem.insert(0x00181083, &tags::IntervalsAcquired);

		ident_to_elem.insert("IntervalsRejected", &tags::IntervalsRejected);
		tag_to_elem.insert(0x00181084, &tags::IntervalsRejected);

		ident_to_elem.insert("PVCRejection", &tags::PVCRejection);
		tag_to_elem.insert(0x00181085, &tags::PVCRejection);

		ident_to_elem.insert("SkipBeats", &tags::SkipBeats);
		tag_to_elem.insert(0x00181086, &tags::SkipBeats);

		ident_to_elem.insert("HeartRate", &tags::HeartRate);
		tag_to_elem.insert(0x00181088, &tags::HeartRate);

		ident_to_elem.insert("CardiacNumberOfImages", &tags::CardiacNumberOfImages);
		tag_to_elem.insert(0x00181090, &tags::CardiacNumberOfImages);

		ident_to_elem.insert("TriggerWindow", &tags::TriggerWindow);
		tag_to_elem.insert(0x00181094, &tags::TriggerWindow);

		ident_to_elem.insert("ReconstructionDiameter", &tags::ReconstructionDiameter);
		tag_to_elem.insert(0x00181100, &tags::ReconstructionDiameter);

		ident_to_elem.insert("DistanceSourceToDetector", &tags::DistanceSourceToDetector);
		tag_to_elem.insert(0x00181110, &tags::DistanceSourceToDetector);

		ident_to_elem.insert("DistanceSourceToPatient", &tags::DistanceSourceToPatient);
		tag_to_elem.insert(0x00181111, &tags::DistanceSourceToPatient);

		ident_to_elem.insert("EstimatedRadiographicMagnificationFactor", &tags::EstimatedRadiographicMagnificationFactor);
		tag_to_elem.insert(0x00181114, &tags::EstimatedRadiographicMagnificationFactor);

		ident_to_elem.insert("GantryDetectorTilt", &tags::GantryDetectorTilt);
		tag_to_elem.insert(0x00181120, &tags::GantryDetectorTilt);

		ident_to_elem.insert("GantryDetectorSlew", &tags::GantryDetectorSlew);
		tag_to_elem.insert(0x00181121, &tags::GantryDetectorSlew);

		ident_to_elem.insert("TableHeight", &tags::TableHeight);
		tag_to_elem.insert(0x00181130, &tags::TableHeight);

		ident_to_elem.insert("TableTraverse", &tags::TableTraverse);
		tag_to_elem.insert(0x00181131, &tags::TableTraverse);

		ident_to_elem.insert("TableMotion", &tags::TableMotion);
		tag_to_elem.insert(0x00181134, &tags::TableMotion);

		ident_to_elem.insert("TableVerticalIncrement", &tags::TableVerticalIncrement);
		tag_to_elem.insert(0x00181135, &tags::TableVerticalIncrement);

		ident_to_elem.insert("TableLateralIncrement", &tags::TableLateralIncrement);
		tag_to_elem.insert(0x00181136, &tags::TableLateralIncrement);

		ident_to_elem.insert("TableLongitudinalIncrement", &tags::TableLongitudinalIncrement);
		tag_to_elem.insert(0x00181137, &tags::TableLongitudinalIncrement);

		ident_to_elem.insert("TableAngle", &tags::TableAngle);
		tag_to_elem.insert(0x00181138, &tags::TableAngle);

		ident_to_elem.insert("TableType", &tags::TableType);
		tag_to_elem.insert(0x0018113A, &tags::TableType);

		ident_to_elem.insert("RotationDirection", &tags::RotationDirection);
		tag_to_elem.insert(0x00181140, &tags::RotationDirection);

		ident_to_elem.insert("AngularPosition", &tags::AngularPosition);
		tag_to_elem.insert(0x00181141, &tags::AngularPosition);

		ident_to_elem.insert("RadialPosition", &tags::RadialPosition);
		tag_to_elem.insert(0x00181142, &tags::RadialPosition);

		ident_to_elem.insert("ScanArc", &tags::ScanArc);
		tag_to_elem.insert(0x00181143, &tags::ScanArc);

		ident_to_elem.insert("AngularStep", &tags::AngularStep);
		tag_to_elem.insert(0x00181144, &tags::AngularStep);

		ident_to_elem.insert("CenterOfRotationOffset", &tags::CenterOfRotationOffset);
		tag_to_elem.insert(0x00181145, &tags::CenterOfRotationOffset);

		ident_to_elem.insert("RotationOffset", &tags::RotationOffset);
		tag_to_elem.insert(0x00181146, &tags::RotationOffset);

		ident_to_elem.insert("FieldOfViewShape", &tags::FieldOfViewShape);
		tag_to_elem.insert(0x00181147, &tags::FieldOfViewShape);

		ident_to_elem.insert("FieldOfViewDimensions", &tags::FieldOfViewDimensions);
		tag_to_elem.insert(0x00181149, &tags::FieldOfViewDimensions);

		ident_to_elem.insert("ExposureTime", &tags::ExposureTime);
		tag_to_elem.insert(0x00181150, &tags::ExposureTime);

		ident_to_elem.insert("XRayTubeCurrent", &tags::XRayTubeCurrent);
		tag_to_elem.insert(0x00181151, &tags::XRayTubeCurrent);

		ident_to_elem.insert("Exposure", &tags::Exposure);
		tag_to_elem.insert(0x00181152, &tags::Exposure);

		ident_to_elem.insert("ExposureInuAs", &tags::ExposureInuAs);
		tag_to_elem.insert(0x00181153, &tags::ExposureInuAs);

		ident_to_elem.insert("AveragePulseWidth", &tags::AveragePulseWidth);
		tag_to_elem.insert(0x00181154, &tags::AveragePulseWidth);

		ident_to_elem.insert("RadiationSetting", &tags::RadiationSetting);
		tag_to_elem.insert(0x00181155, &tags::RadiationSetting);

		ident_to_elem.insert("RectificationType", &tags::RectificationType);
		tag_to_elem.insert(0x00181156, &tags::RectificationType);

		ident_to_elem.insert("RadiationMode", &tags::RadiationMode);
		tag_to_elem.insert(0x0018115A, &tags::RadiationMode);

		ident_to_elem.insert("ImageAndFluoroscopyAreaDoseProduct", &tags::ImageAndFluoroscopyAreaDoseProduct);
		tag_to_elem.insert(0x0018115E, &tags::ImageAndFluoroscopyAreaDoseProduct);

		ident_to_elem.insert("FilterType", &tags::FilterType);
		tag_to_elem.insert(0x00181160, &tags::FilterType);

		ident_to_elem.insert("TypeOfFilters", &tags::TypeOfFilters);
		tag_to_elem.insert(0x00181161, &tags::TypeOfFilters);

		ident_to_elem.insert("IntensifierSize", &tags::IntensifierSize);
		tag_to_elem.insert(0x00181162, &tags::IntensifierSize);

		ident_to_elem.insert("ImagerPixelSpacing", &tags::ImagerPixelSpacing);
		tag_to_elem.insert(0x00181164, &tags::ImagerPixelSpacing);

		ident_to_elem.insert("Grid", &tags::Grid);
		tag_to_elem.insert(0x00181166, &tags::Grid);

		ident_to_elem.insert("GeneratorPower", &tags::GeneratorPower);
		tag_to_elem.insert(0x00181170, &tags::GeneratorPower);

		ident_to_elem.insert("CollimatorGridName", &tags::CollimatorGridName);
		tag_to_elem.insert(0x00181180, &tags::CollimatorGridName);

		ident_to_elem.insert("CollimatorType", &tags::CollimatorType);
		tag_to_elem.insert(0x00181181, &tags::CollimatorType);

		ident_to_elem.insert("FocalDistance", &tags::FocalDistance);
		tag_to_elem.insert(0x00181182, &tags::FocalDistance);

		ident_to_elem.insert("XFocusCenter", &tags::XFocusCenter);
		tag_to_elem.insert(0x00181183, &tags::XFocusCenter);

		ident_to_elem.insert("YFocusCenter", &tags::YFocusCenter);
		tag_to_elem.insert(0x00181184, &tags::YFocusCenter);

		ident_to_elem.insert("FocalSpots", &tags::FocalSpots);
		tag_to_elem.insert(0x00181190, &tags::FocalSpots);

		ident_to_elem.insert("AnodeTargetMaterial", &tags::AnodeTargetMaterial);
		tag_to_elem.insert(0x00181191, &tags::AnodeTargetMaterial);

		ident_to_elem.insert("BodyPartThickness", &tags::BodyPartThickness);
		tag_to_elem.insert(0x001811A0, &tags::BodyPartThickness);

		ident_to_elem.insert("CompressionForce", &tags::CompressionForce);
		tag_to_elem.insert(0x001811A2, &tags::CompressionForce);

		ident_to_elem.insert("PaddleDescription", &tags::PaddleDescription);
		tag_to_elem.insert(0x001811A4, &tags::PaddleDescription);

		ident_to_elem.insert("DateOfLastCalibration", &tags::DateOfLastCalibration);
		tag_to_elem.insert(0x00181200, &tags::DateOfLastCalibration);

		ident_to_elem.insert("TimeOfLastCalibration", &tags::TimeOfLastCalibration);
		tag_to_elem.insert(0x00181201, &tags::TimeOfLastCalibration);

		ident_to_elem.insert("DateTimeOfLastCalibration", &tags::DateTimeOfLastCalibration);
		tag_to_elem.insert(0x00181202, &tags::DateTimeOfLastCalibration);

		ident_to_elem.insert("ConvolutionKernel", &tags::ConvolutionKernel);
		tag_to_elem.insert(0x00181210, &tags::ConvolutionKernel);

		ident_to_elem.insert("UpperLowerPixelValues", &tags::UpperLowerPixelValues);
		tag_to_elem.insert(0x00181240, &tags::UpperLowerPixelValues);

		ident_to_elem.insert("ActualFrameDuration", &tags::ActualFrameDuration);
		tag_to_elem.insert(0x00181242, &tags::ActualFrameDuration);

		ident_to_elem.insert("CountRate", &tags::CountRate);
		tag_to_elem.insert(0x00181243, &tags::CountRate);

		ident_to_elem.insert("PreferredPlaybackSequencing", &tags::PreferredPlaybackSequencing);
		tag_to_elem.insert(0x00181244, &tags::PreferredPlaybackSequencing);

		ident_to_elem.insert("ReceiveCoilName", &tags::ReceiveCoilName);
		tag_to_elem.insert(0x00181250, &tags::ReceiveCoilName);

		ident_to_elem.insert("TransmitCoilName", &tags::TransmitCoilName);
		tag_to_elem.insert(0x00181251, &tags::TransmitCoilName);

		ident_to_elem.insert("PlateType", &tags::PlateType);
		tag_to_elem.insert(0x00181260, &tags::PlateType);

		ident_to_elem.insert("PhosphorType", &tags::PhosphorType);
		tag_to_elem.insert(0x00181261, &tags::PhosphorType);

		ident_to_elem.insert("WaterEquivalentDiameter", &tags::WaterEquivalentDiameter);
		tag_to_elem.insert(0x00181271, &tags::WaterEquivalentDiameter);

		ident_to_elem.insert("WaterEquivalentDiameterCalculationMethodCodeSequence", &tags::WaterEquivalentDiameterCalculationMethodCodeSequence);
		tag_to_elem.insert(0x00181272, &tags::WaterEquivalentDiameterCalculationMethodCodeSequence);

		ident_to_elem.insert("ScanVelocity", &tags::ScanVelocity);
		tag_to_elem.insert(0x00181300, &tags::ScanVelocity);

		ident_to_elem.insert("WholeBodyTechnique", &tags::WholeBodyTechnique);
		tag_to_elem.insert(0x00181301, &tags::WholeBodyTechnique);

		ident_to_elem.insert("ScanLength", &tags::ScanLength);
		tag_to_elem.insert(0x00181302, &tags::ScanLength);

		ident_to_elem.insert("AcquisitionMatrix", &tags::AcquisitionMatrix);
		tag_to_elem.insert(0x00181310, &tags::AcquisitionMatrix);

		ident_to_elem.insert("InPlanePhaseEncodingDirection", &tags::InPlanePhaseEncodingDirection);
		tag_to_elem.insert(0x00181312, &tags::InPlanePhaseEncodingDirection);

		ident_to_elem.insert("FlipAngle", &tags::FlipAngle);
		tag_to_elem.insert(0x00181314, &tags::FlipAngle);

		ident_to_elem.insert("VariableFlipAngleFlag", &tags::VariableFlipAngleFlag);
		tag_to_elem.insert(0x00181315, &tags::VariableFlipAngleFlag);

		ident_to_elem.insert("SAR", &tags::SAR);
		tag_to_elem.insert(0x00181316, &tags::SAR);

		ident_to_elem.insert("dBdt", &tags::dBdt);
		tag_to_elem.insert(0x00181318, &tags::dBdt);

		ident_to_elem.insert("B1rms", &tags::B1rms);
		tag_to_elem.insert(0x00181320, &tags::B1rms);

		ident_to_elem.insert("AcquisitionDeviceProcessingDescription", &tags::AcquisitionDeviceProcessingDescription);
		tag_to_elem.insert(0x00181400, &tags::AcquisitionDeviceProcessingDescription);

		ident_to_elem.insert("AcquisitionDeviceProcessingCode", &tags::AcquisitionDeviceProcessingCode);
		tag_to_elem.insert(0x00181401, &tags::AcquisitionDeviceProcessingCode);

		ident_to_elem.insert("CassetteOrientation", &tags::CassetteOrientation);
		tag_to_elem.insert(0x00181402, &tags::CassetteOrientation);

		ident_to_elem.insert("CassetteSize", &tags::CassetteSize);
		tag_to_elem.insert(0x00181403, &tags::CassetteSize);

		ident_to_elem.insert("ExposuresOnPlate", &tags::ExposuresOnPlate);
		tag_to_elem.insert(0x00181404, &tags::ExposuresOnPlate);

		ident_to_elem.insert("RelativeXRayExposure", &tags::RelativeXRayExposure);
		tag_to_elem.insert(0x00181405, &tags::RelativeXRayExposure);

		ident_to_elem.insert("ExposureIndex", &tags::ExposureIndex);
		tag_to_elem.insert(0x00181411, &tags::ExposureIndex);

		ident_to_elem.insert("TargetExposureIndex", &tags::TargetExposureIndex);
		tag_to_elem.insert(0x00181412, &tags::TargetExposureIndex);

		ident_to_elem.insert("DeviationIndex", &tags::DeviationIndex);
		tag_to_elem.insert(0x00181413, &tags::DeviationIndex);

		ident_to_elem.insert("ColumnAngulation", &tags::ColumnAngulation);
		tag_to_elem.insert(0x00181450, &tags::ColumnAngulation);

		ident_to_elem.insert("TomoLayerHeight", &tags::TomoLayerHeight);
		tag_to_elem.insert(0x00181460, &tags::TomoLayerHeight);

		ident_to_elem.insert("TomoAngle", &tags::TomoAngle);
		tag_to_elem.insert(0x00181470, &tags::TomoAngle);

		ident_to_elem.insert("TomoTime", &tags::TomoTime);
		tag_to_elem.insert(0x00181480, &tags::TomoTime);

		ident_to_elem.insert("TomoType", &tags::TomoType);
		tag_to_elem.insert(0x00181490, &tags::TomoType);

		ident_to_elem.insert("TomoClass", &tags::TomoClass);
		tag_to_elem.insert(0x00181491, &tags::TomoClass);

		ident_to_elem.insert("NumberOfTomosynthesisSourceImages", &tags::NumberOfTomosynthesisSourceImages);
		tag_to_elem.insert(0x00181495, &tags::NumberOfTomosynthesisSourceImages);

		ident_to_elem.insert("PositionerMotion", &tags::PositionerMotion);
		tag_to_elem.insert(0x00181500, &tags::PositionerMotion);

		ident_to_elem.insert("PositionerType", &tags::PositionerType);
		tag_to_elem.insert(0x00181508, &tags::PositionerType);

		ident_to_elem.insert("PositionerPrimaryAngle", &tags::PositionerPrimaryAngle);
		tag_to_elem.insert(0x00181510, &tags::PositionerPrimaryAngle);

		ident_to_elem.insert("PositionerSecondaryAngle", &tags::PositionerSecondaryAngle);
		tag_to_elem.insert(0x00181511, &tags::PositionerSecondaryAngle);

		ident_to_elem.insert("PositionerPrimaryAngleIncrement", &tags::PositionerPrimaryAngleIncrement);
		tag_to_elem.insert(0x00181520, &tags::PositionerPrimaryAngleIncrement);

		ident_to_elem.insert("PositionerSecondaryAngleIncrement", &tags::PositionerSecondaryAngleIncrement);
		tag_to_elem.insert(0x00181521, &tags::PositionerSecondaryAngleIncrement);

		ident_to_elem.insert("DetectorPrimaryAngle", &tags::DetectorPrimaryAngle);
		tag_to_elem.insert(0x00181530, &tags::DetectorPrimaryAngle);

		ident_to_elem.insert("DetectorSecondaryAngle", &tags::DetectorSecondaryAngle);
		tag_to_elem.insert(0x00181531, &tags::DetectorSecondaryAngle);

		ident_to_elem.insert("ShutterShape", &tags::ShutterShape);
		tag_to_elem.insert(0x00181600, &tags::ShutterShape);

		ident_to_elem.insert("ShutterLeftVerticalEdge", &tags::ShutterLeftVerticalEdge);
		tag_to_elem.insert(0x00181602, &tags::ShutterLeftVerticalEdge);

		ident_to_elem.insert("ShutterRightVerticalEdge", &tags::ShutterRightVerticalEdge);
		tag_to_elem.insert(0x00181604, &tags::ShutterRightVerticalEdge);

		ident_to_elem.insert("ShutterUpperHorizontalEdge", &tags::ShutterUpperHorizontalEdge);
		tag_to_elem.insert(0x00181606, &tags::ShutterUpperHorizontalEdge);

		ident_to_elem.insert("ShutterLowerHorizontalEdge", &tags::ShutterLowerHorizontalEdge);
		tag_to_elem.insert(0x00181608, &tags::ShutterLowerHorizontalEdge);

		ident_to_elem.insert("CenterOfCircularShutter", &tags::CenterOfCircularShutter);
		tag_to_elem.insert(0x00181610, &tags::CenterOfCircularShutter);

		ident_to_elem.insert("RadiusOfCircularShutter", &tags::RadiusOfCircularShutter);
		tag_to_elem.insert(0x00181612, &tags::RadiusOfCircularShutter);

		ident_to_elem.insert("VerticesOfThePolygonalShutter", &tags::VerticesOfThePolygonalShutter);
		tag_to_elem.insert(0x00181620, &tags::VerticesOfThePolygonalShutter);

		ident_to_elem.insert("ShutterPresentationValue", &tags::ShutterPresentationValue);
		tag_to_elem.insert(0x00181622, &tags::ShutterPresentationValue);

		ident_to_elem.insert("ShutterOverlayGroup", &tags::ShutterOverlayGroup);
		tag_to_elem.insert(0x00181623, &tags::ShutterOverlayGroup);

		ident_to_elem.insert("ShutterPresentationColorCIELabValue", &tags::ShutterPresentationColorCIELabValue);
		tag_to_elem.insert(0x00181624, &tags::ShutterPresentationColorCIELabValue);

		ident_to_elem.insert("CollimatorShape", &tags::CollimatorShape);
		tag_to_elem.insert(0x00181700, &tags::CollimatorShape);

		ident_to_elem.insert("CollimatorLeftVerticalEdge", &tags::CollimatorLeftVerticalEdge);
		tag_to_elem.insert(0x00181702, &tags::CollimatorLeftVerticalEdge);

		ident_to_elem.insert("CollimatorRightVerticalEdge", &tags::CollimatorRightVerticalEdge);
		tag_to_elem.insert(0x00181704, &tags::CollimatorRightVerticalEdge);

		ident_to_elem.insert("CollimatorUpperHorizontalEdge", &tags::CollimatorUpperHorizontalEdge);
		tag_to_elem.insert(0x00181706, &tags::CollimatorUpperHorizontalEdge);

		ident_to_elem.insert("CollimatorLowerHorizontalEdge", &tags::CollimatorLowerHorizontalEdge);
		tag_to_elem.insert(0x00181708, &tags::CollimatorLowerHorizontalEdge);

		ident_to_elem.insert("CenterOfCircularCollimator", &tags::CenterOfCircularCollimator);
		tag_to_elem.insert(0x00181710, &tags::CenterOfCircularCollimator);

		ident_to_elem.insert("RadiusOfCircularCollimator", &tags::RadiusOfCircularCollimator);
		tag_to_elem.insert(0x00181712, &tags::RadiusOfCircularCollimator);

		ident_to_elem.insert("VerticesOfThePolygonalCollimator", &tags::VerticesOfThePolygonalCollimator);
		tag_to_elem.insert(0x00181720, &tags::VerticesOfThePolygonalCollimator);

		ident_to_elem.insert("AcquisitionTimeSynchronized", &tags::AcquisitionTimeSynchronized);
		tag_to_elem.insert(0x00181800, &tags::AcquisitionTimeSynchronized);

		ident_to_elem.insert("TimeSource", &tags::TimeSource);
		tag_to_elem.insert(0x00181801, &tags::TimeSource);

		ident_to_elem.insert("TimeDistributionProtocol", &tags::TimeDistributionProtocol);
		tag_to_elem.insert(0x00181802, &tags::TimeDistributionProtocol);

		ident_to_elem.insert("NTPSourceAddress", &tags::NTPSourceAddress);
		tag_to_elem.insert(0x00181803, &tags::NTPSourceAddress);

		ident_to_elem.insert("PageNumberVector", &tags::PageNumberVector);
		tag_to_elem.insert(0x00182001, &tags::PageNumberVector);

		ident_to_elem.insert("FrameLabelVector", &tags::FrameLabelVector);
		tag_to_elem.insert(0x00182002, &tags::FrameLabelVector);

		ident_to_elem.insert("FramePrimaryAngleVector", &tags::FramePrimaryAngleVector);
		tag_to_elem.insert(0x00182003, &tags::FramePrimaryAngleVector);

		ident_to_elem.insert("FrameSecondaryAngleVector", &tags::FrameSecondaryAngleVector);
		tag_to_elem.insert(0x00182004, &tags::FrameSecondaryAngleVector);

		ident_to_elem.insert("SliceLocationVector", &tags::SliceLocationVector);
		tag_to_elem.insert(0x00182005, &tags::SliceLocationVector);

		ident_to_elem.insert("DisplayWindowLabelVector", &tags::DisplayWindowLabelVector);
		tag_to_elem.insert(0x00182006, &tags::DisplayWindowLabelVector);

		ident_to_elem.insert("NominalScannedPixelSpacing", &tags::NominalScannedPixelSpacing);
		tag_to_elem.insert(0x00182010, &tags::NominalScannedPixelSpacing);

		ident_to_elem.insert("DigitizingDeviceTransportDirection", &tags::DigitizingDeviceTransportDirection);
		tag_to_elem.insert(0x00182020, &tags::DigitizingDeviceTransportDirection);

		ident_to_elem.insert("RotationOfScannedFilm", &tags::RotationOfScannedFilm);
		tag_to_elem.insert(0x00182030, &tags::RotationOfScannedFilm);

		ident_to_elem.insert("BiopsyTargetSequence", &tags::BiopsyTargetSequence);
		tag_to_elem.insert(0x00182041, &tags::BiopsyTargetSequence);

		ident_to_elem.insert("TargetUID", &tags::TargetUID);
		tag_to_elem.insert(0x00182042, &tags::TargetUID);

		ident_to_elem.insert("LocalizingCursorPosition", &tags::LocalizingCursorPosition);
		tag_to_elem.insert(0x00182043, &tags::LocalizingCursorPosition);

		ident_to_elem.insert("CalculatedTargetPosition", &tags::CalculatedTargetPosition);
		tag_to_elem.insert(0x00182044, &tags::CalculatedTargetPosition);

		ident_to_elem.insert("TargetLabel", &tags::TargetLabel);
		tag_to_elem.insert(0x00182045, &tags::TargetLabel);

		ident_to_elem.insert("DisplayedZValue", &tags::DisplayedZValue);
		tag_to_elem.insert(0x00182046, &tags::DisplayedZValue);

		ident_to_elem.insert("IVUSAcquisition", &tags::IVUSAcquisition);
		tag_to_elem.insert(0x00183100, &tags::IVUSAcquisition);

		ident_to_elem.insert("IVUSPullbackRate", &tags::IVUSPullbackRate);
		tag_to_elem.insert(0x00183101, &tags::IVUSPullbackRate);

		ident_to_elem.insert("IVUSGatedRate", &tags::IVUSGatedRate);
		tag_to_elem.insert(0x00183102, &tags::IVUSGatedRate);

		ident_to_elem.insert("IVUSPullbackStartFrameNumber", &tags::IVUSPullbackStartFrameNumber);
		tag_to_elem.insert(0x00183103, &tags::IVUSPullbackStartFrameNumber);

		ident_to_elem.insert("IVUSPullbackStopFrameNumber", &tags::IVUSPullbackStopFrameNumber);
		tag_to_elem.insert(0x00183104, &tags::IVUSPullbackStopFrameNumber);

		ident_to_elem.insert("LesionNumber", &tags::LesionNumber);
		tag_to_elem.insert(0x00183105, &tags::LesionNumber);

		ident_to_elem.insert("AcquisitionComments", &tags::AcquisitionComments);
		tag_to_elem.insert(0x00184000, &tags::AcquisitionComments);

		ident_to_elem.insert("OutputPower", &tags::OutputPower);
		tag_to_elem.insert(0x00185000, &tags::OutputPower);

		ident_to_elem.insert("TransducerData", &tags::TransducerData);
		tag_to_elem.insert(0x00185010, &tags::TransducerData);

		ident_to_elem.insert("FocusDepth", &tags::FocusDepth);
		tag_to_elem.insert(0x00185012, &tags::FocusDepth);

		ident_to_elem.insert("ProcessingFunction", &tags::ProcessingFunction);
		tag_to_elem.insert(0x00185020, &tags::ProcessingFunction);

		ident_to_elem.insert("PostprocessingFunction", &tags::PostprocessingFunction);
		tag_to_elem.insert(0x00185021, &tags::PostprocessingFunction);

		ident_to_elem.insert("MechanicalIndex", &tags::MechanicalIndex);
		tag_to_elem.insert(0x00185022, &tags::MechanicalIndex);

		ident_to_elem.insert("BoneThermalIndex", &tags::BoneThermalIndex);
		tag_to_elem.insert(0x00185024, &tags::BoneThermalIndex);

		ident_to_elem.insert("CranialThermalIndex", &tags::CranialThermalIndex);
		tag_to_elem.insert(0x00185026, &tags::CranialThermalIndex);

		ident_to_elem.insert("SoftTissueThermalIndex", &tags::SoftTissueThermalIndex);
		tag_to_elem.insert(0x00185027, &tags::SoftTissueThermalIndex);

		ident_to_elem.insert("SoftTissueFocusThermalIndex", &tags::SoftTissueFocusThermalIndex);
		tag_to_elem.insert(0x00185028, &tags::SoftTissueFocusThermalIndex);

		ident_to_elem.insert("SoftTissueSurfaceThermalIndex", &tags::SoftTissueSurfaceThermalIndex);
		tag_to_elem.insert(0x00185029, &tags::SoftTissueSurfaceThermalIndex);

		ident_to_elem.insert("DynamicRange", &tags::DynamicRange);
		tag_to_elem.insert(0x00185030, &tags::DynamicRange);

		ident_to_elem.insert("TotalGain", &tags::TotalGain);
		tag_to_elem.insert(0x00185040, &tags::TotalGain);

		ident_to_elem.insert("DepthOfScanField", &tags::DepthOfScanField);
		tag_to_elem.insert(0x00185050, &tags::DepthOfScanField);

		ident_to_elem.insert("PatientPosition", &tags::PatientPosition);
		tag_to_elem.insert(0x00185100, &tags::PatientPosition);

		ident_to_elem.insert("ViewPosition", &tags::ViewPosition);
		tag_to_elem.insert(0x00185101, &tags::ViewPosition);

		ident_to_elem.insert("ProjectionEponymousNameCodeSequence", &tags::ProjectionEponymousNameCodeSequence);
		tag_to_elem.insert(0x00185104, &tags::ProjectionEponymousNameCodeSequence);

		ident_to_elem.insert("ImageTransformationMatrix", &tags::ImageTransformationMatrix);
		tag_to_elem.insert(0x00185210, &tags::ImageTransformationMatrix);

		ident_to_elem.insert("ImageTranslationVector", &tags::ImageTranslationVector);
		tag_to_elem.insert(0x00185212, &tags::ImageTranslationVector);

		ident_to_elem.insert("Sensitivity", &tags::Sensitivity);
		tag_to_elem.insert(0x00186000, &tags::Sensitivity);

		ident_to_elem.insert("SequenceOfUltrasoundRegions", &tags::SequenceOfUltrasoundRegions);
		tag_to_elem.insert(0x00186011, &tags::SequenceOfUltrasoundRegions);

		ident_to_elem.insert("RegionSpatialFormat", &tags::RegionSpatialFormat);
		tag_to_elem.insert(0x00186012, &tags::RegionSpatialFormat);

		ident_to_elem.insert("RegionDataType", &tags::RegionDataType);
		tag_to_elem.insert(0x00186014, &tags::RegionDataType);

		ident_to_elem.insert("RegionFlags", &tags::RegionFlags);
		tag_to_elem.insert(0x00186016, &tags::RegionFlags);

		ident_to_elem.insert("RegionLocationMinX0", &tags::RegionLocationMinX0);
		tag_to_elem.insert(0x00186018, &tags::RegionLocationMinX0);

		ident_to_elem.insert("RegionLocationMinY0", &tags::RegionLocationMinY0);
		tag_to_elem.insert(0x0018601A, &tags::RegionLocationMinY0);

		ident_to_elem.insert("RegionLocationMaxX1", &tags::RegionLocationMaxX1);
		tag_to_elem.insert(0x0018601C, &tags::RegionLocationMaxX1);

		ident_to_elem.insert("RegionLocationMaxY1", &tags::RegionLocationMaxY1);
		tag_to_elem.insert(0x0018601E, &tags::RegionLocationMaxY1);

		ident_to_elem.insert("ReferencePixelX0", &tags::ReferencePixelX0);
		tag_to_elem.insert(0x00186020, &tags::ReferencePixelX0);

		ident_to_elem.insert("ReferencePixelY0", &tags::ReferencePixelY0);
		tag_to_elem.insert(0x00186022, &tags::ReferencePixelY0);

		ident_to_elem.insert("PhysicalUnitsXDirection", &tags::PhysicalUnitsXDirection);
		tag_to_elem.insert(0x00186024, &tags::PhysicalUnitsXDirection);

		ident_to_elem.insert("PhysicalUnitsYDirection", &tags::PhysicalUnitsYDirection);
		tag_to_elem.insert(0x00186026, &tags::PhysicalUnitsYDirection);

		ident_to_elem.insert("ReferencePixelPhysicalValueX", &tags::ReferencePixelPhysicalValueX);
		tag_to_elem.insert(0x00186028, &tags::ReferencePixelPhysicalValueX);

		ident_to_elem.insert("ReferencePixelPhysicalValueY", &tags::ReferencePixelPhysicalValueY);
		tag_to_elem.insert(0x0018602A, &tags::ReferencePixelPhysicalValueY);

		ident_to_elem.insert("PhysicalDeltaX", &tags::PhysicalDeltaX);
		tag_to_elem.insert(0x0018602C, &tags::PhysicalDeltaX);

		ident_to_elem.insert("PhysicalDeltaY", &tags::PhysicalDeltaY);
		tag_to_elem.insert(0x0018602E, &tags::PhysicalDeltaY);

		ident_to_elem.insert("TransducerFrequency", &tags::TransducerFrequency);
		tag_to_elem.insert(0x00186030, &tags::TransducerFrequency);

		ident_to_elem.insert("TransducerType", &tags::TransducerType);
		tag_to_elem.insert(0x00186031, &tags::TransducerType);

		ident_to_elem.insert("PulseRepetitionFrequency", &tags::PulseRepetitionFrequency);
		tag_to_elem.insert(0x00186032, &tags::PulseRepetitionFrequency);

		ident_to_elem.insert("DopplerCorrectionAngle", &tags::DopplerCorrectionAngle);
		tag_to_elem.insert(0x00186034, &tags::DopplerCorrectionAngle);

		ident_to_elem.insert("SteeringAngle", &tags::SteeringAngle);
		tag_to_elem.insert(0x00186036, &tags::SteeringAngle);

		ident_to_elem.insert("DopplerSampleVolumeXPositionRetired", &tags::DopplerSampleVolumeXPositionRetired);
		tag_to_elem.insert(0x00186038, &tags::DopplerSampleVolumeXPositionRetired);

		ident_to_elem.insert("DopplerSampleVolumeXPosition", &tags::DopplerSampleVolumeXPosition);
		tag_to_elem.insert(0x00186039, &tags::DopplerSampleVolumeXPosition);

		ident_to_elem.insert("DopplerSampleVolumeYPositionRetired", &tags::DopplerSampleVolumeYPositionRetired);
		tag_to_elem.insert(0x0018603A, &tags::DopplerSampleVolumeYPositionRetired);

		ident_to_elem.insert("DopplerSampleVolumeYPosition", &tags::DopplerSampleVolumeYPosition);
		tag_to_elem.insert(0x0018603B, &tags::DopplerSampleVolumeYPosition);

		ident_to_elem.insert("TMLinePositionX0Retired", &tags::TMLinePositionX0Retired);
		tag_to_elem.insert(0x0018603C, &tags::TMLinePositionX0Retired);

		ident_to_elem.insert("TMLinePositionX0", &tags::TMLinePositionX0);
		tag_to_elem.insert(0x0018603D, &tags::TMLinePositionX0);

		ident_to_elem.insert("TMLinePositionY0Retired", &tags::TMLinePositionY0Retired);
		tag_to_elem.insert(0x0018603E, &tags::TMLinePositionY0Retired);

		ident_to_elem.insert("TMLinePositionY0", &tags::TMLinePositionY0);
		tag_to_elem.insert(0x0018603F, &tags::TMLinePositionY0);

		ident_to_elem.insert("TMLinePositionX1Retired", &tags::TMLinePositionX1Retired);
		tag_to_elem.insert(0x00186040, &tags::TMLinePositionX1Retired);

		ident_to_elem.insert("TMLinePositionX1", &tags::TMLinePositionX1);
		tag_to_elem.insert(0x00186041, &tags::TMLinePositionX1);

		ident_to_elem.insert("TMLinePositionY1Retired", &tags::TMLinePositionY1Retired);
		tag_to_elem.insert(0x00186042, &tags::TMLinePositionY1Retired);

		ident_to_elem.insert("TMLinePositionY1", &tags::TMLinePositionY1);
		tag_to_elem.insert(0x00186043, &tags::TMLinePositionY1);

		ident_to_elem.insert("PixelComponentOrganization", &tags::PixelComponentOrganization);
		tag_to_elem.insert(0x00186044, &tags::PixelComponentOrganization);

		ident_to_elem.insert("PixelComponentMask", &tags::PixelComponentMask);
		tag_to_elem.insert(0x00186046, &tags::PixelComponentMask);

		ident_to_elem.insert("PixelComponentRangeStart", &tags::PixelComponentRangeStart);
		tag_to_elem.insert(0x00186048, &tags::PixelComponentRangeStart);

		ident_to_elem.insert("PixelComponentRangeStop", &tags::PixelComponentRangeStop);
		tag_to_elem.insert(0x0018604A, &tags::PixelComponentRangeStop);

		ident_to_elem.insert("PixelComponentPhysicalUnits", &tags::PixelComponentPhysicalUnits);
		tag_to_elem.insert(0x0018604C, &tags::PixelComponentPhysicalUnits);

		ident_to_elem.insert("PixelComponentDataType", &tags::PixelComponentDataType);
		tag_to_elem.insert(0x0018604E, &tags::PixelComponentDataType);

		ident_to_elem.insert("NumberOfTableBreakPoints", &tags::NumberOfTableBreakPoints);
		tag_to_elem.insert(0x00186050, &tags::NumberOfTableBreakPoints);

		ident_to_elem.insert("TableOfXBreakPoints", &tags::TableOfXBreakPoints);
		tag_to_elem.insert(0x00186052, &tags::TableOfXBreakPoints);

		ident_to_elem.insert("TableOfYBreakPoints", &tags::TableOfYBreakPoints);
		tag_to_elem.insert(0x00186054, &tags::TableOfYBreakPoints);

		ident_to_elem.insert("NumberOfTableEntries", &tags::NumberOfTableEntries);
		tag_to_elem.insert(0x00186056, &tags::NumberOfTableEntries);

		ident_to_elem.insert("TableOfPixelValues", &tags::TableOfPixelValues);
		tag_to_elem.insert(0x00186058, &tags::TableOfPixelValues);

		ident_to_elem.insert("TableOfParameterValues", &tags::TableOfParameterValues);
		tag_to_elem.insert(0x0018605A, &tags::TableOfParameterValues);

		ident_to_elem.insert("RWaveTimeVector", &tags::RWaveTimeVector);
		tag_to_elem.insert(0x00186060, &tags::RWaveTimeVector);

		ident_to_elem.insert("DetectorConditionsNominalFlag", &tags::DetectorConditionsNominalFlag);
		tag_to_elem.insert(0x00187000, &tags::DetectorConditionsNominalFlag);

		ident_to_elem.insert("DetectorTemperature", &tags::DetectorTemperature);
		tag_to_elem.insert(0x00187001, &tags::DetectorTemperature);

		ident_to_elem.insert("DetectorType", &tags::DetectorType);
		tag_to_elem.insert(0x00187004, &tags::DetectorType);

		ident_to_elem.insert("DetectorConfiguration", &tags::DetectorConfiguration);
		tag_to_elem.insert(0x00187005, &tags::DetectorConfiguration);

		ident_to_elem.insert("DetectorDescription", &tags::DetectorDescription);
		tag_to_elem.insert(0x00187006, &tags::DetectorDescription);

		ident_to_elem.insert("DetectorMode", &tags::DetectorMode);
		tag_to_elem.insert(0x00187008, &tags::DetectorMode);

		ident_to_elem.insert("DetectorID", &tags::DetectorID);
		tag_to_elem.insert(0x0018700A, &tags::DetectorID);

		ident_to_elem.insert("DateOfLastDetectorCalibration", &tags::DateOfLastDetectorCalibration);
		tag_to_elem.insert(0x0018700C, &tags::DateOfLastDetectorCalibration);

		ident_to_elem.insert("TimeOfLastDetectorCalibration", &tags::TimeOfLastDetectorCalibration);
		tag_to_elem.insert(0x0018700E, &tags::TimeOfLastDetectorCalibration);

		ident_to_elem.insert("ExposuresOnDetectorSinceLastCalibration", &tags::ExposuresOnDetectorSinceLastCalibration);
		tag_to_elem.insert(0x00187010, &tags::ExposuresOnDetectorSinceLastCalibration);

		ident_to_elem.insert("ExposuresOnDetectorSinceManufactured", &tags::ExposuresOnDetectorSinceManufactured);
		tag_to_elem.insert(0x00187011, &tags::ExposuresOnDetectorSinceManufactured);

		ident_to_elem.insert("DetectorTimeSinceLastExposure", &tags::DetectorTimeSinceLastExposure);
		tag_to_elem.insert(0x00187012, &tags::DetectorTimeSinceLastExposure);

		ident_to_elem.insert("DetectorActiveTime", &tags::DetectorActiveTime);
		tag_to_elem.insert(0x00187014, &tags::DetectorActiveTime);

		ident_to_elem.insert("DetectorActivationOffsetFromExposure", &tags::DetectorActivationOffsetFromExposure);
		tag_to_elem.insert(0x00187016, &tags::DetectorActivationOffsetFromExposure);

		ident_to_elem.insert("DetectorBinning", &tags::DetectorBinning);
		tag_to_elem.insert(0x0018701A, &tags::DetectorBinning);

		ident_to_elem.insert("DetectorElementPhysicalSize", &tags::DetectorElementPhysicalSize);
		tag_to_elem.insert(0x00187020, &tags::DetectorElementPhysicalSize);

		ident_to_elem.insert("DetectorElementSpacing", &tags::DetectorElementSpacing);
		tag_to_elem.insert(0x00187022, &tags::DetectorElementSpacing);

		ident_to_elem.insert("DetectorActiveShape", &tags::DetectorActiveShape);
		tag_to_elem.insert(0x00187024, &tags::DetectorActiveShape);

		ident_to_elem.insert("DetectorActiveDimensions", &tags::DetectorActiveDimensions);
		tag_to_elem.insert(0x00187026, &tags::DetectorActiveDimensions);

		ident_to_elem.insert("DetectorActiveOrigin", &tags::DetectorActiveOrigin);
		tag_to_elem.insert(0x00187028, &tags::DetectorActiveOrigin);

		ident_to_elem.insert("DetectorManufacturerName", &tags::DetectorManufacturerName);
		tag_to_elem.insert(0x0018702A, &tags::DetectorManufacturerName);

		ident_to_elem.insert("DetectorManufacturerModelName", &tags::DetectorManufacturerModelName);
		tag_to_elem.insert(0x0018702B, &tags::DetectorManufacturerModelName);

		ident_to_elem.insert("FieldOfViewOrigin", &tags::FieldOfViewOrigin);
		tag_to_elem.insert(0x00187030, &tags::FieldOfViewOrigin);

		ident_to_elem.insert("FieldOfViewRotation", &tags::FieldOfViewRotation);
		tag_to_elem.insert(0x00187032, &tags::FieldOfViewRotation);

		ident_to_elem.insert("FieldOfViewHorizontalFlip", &tags::FieldOfViewHorizontalFlip);
		tag_to_elem.insert(0x00187034, &tags::FieldOfViewHorizontalFlip);

		ident_to_elem.insert("PixelDataAreaOriginRelativeToFOV", &tags::PixelDataAreaOriginRelativeToFOV);
		tag_to_elem.insert(0x00187036, &tags::PixelDataAreaOriginRelativeToFOV);

		ident_to_elem.insert("PixelDataAreaRotationAngleRelativeToFOV", &tags::PixelDataAreaRotationAngleRelativeToFOV);
		tag_to_elem.insert(0x00187038, &tags::PixelDataAreaRotationAngleRelativeToFOV);

		ident_to_elem.insert("GridAbsorbingMaterial", &tags::GridAbsorbingMaterial);
		tag_to_elem.insert(0x00187040, &tags::GridAbsorbingMaterial);

		ident_to_elem.insert("GridSpacingMaterial", &tags::GridSpacingMaterial);
		tag_to_elem.insert(0x00187041, &tags::GridSpacingMaterial);

		ident_to_elem.insert("GridThickness", &tags::GridThickness);
		tag_to_elem.insert(0x00187042, &tags::GridThickness);

		ident_to_elem.insert("GridPitch", &tags::GridPitch);
		tag_to_elem.insert(0x00187044, &tags::GridPitch);

		ident_to_elem.insert("GridAspectRatio", &tags::GridAspectRatio);
		tag_to_elem.insert(0x00187046, &tags::GridAspectRatio);

		ident_to_elem.insert("GridPeriod", &tags::GridPeriod);
		tag_to_elem.insert(0x00187048, &tags::GridPeriod);

		ident_to_elem.insert("GridFocalDistance", &tags::GridFocalDistance);
		tag_to_elem.insert(0x0018704C, &tags::GridFocalDistance);

		ident_to_elem.insert("FilterMaterial", &tags::FilterMaterial);
		tag_to_elem.insert(0x00187050, &tags::FilterMaterial);

		ident_to_elem.insert("FilterThicknessMinimum", &tags::FilterThicknessMinimum);
		tag_to_elem.insert(0x00187052, &tags::FilterThicknessMinimum);

		ident_to_elem.insert("FilterThicknessMaximum", &tags::FilterThicknessMaximum);
		tag_to_elem.insert(0x00187054, &tags::FilterThicknessMaximum);

		ident_to_elem.insert("FilterBeamPathLengthMinimum", &tags::FilterBeamPathLengthMinimum);
		tag_to_elem.insert(0x00187056, &tags::FilterBeamPathLengthMinimum);

		ident_to_elem.insert("FilterBeamPathLengthMaximum", &tags::FilterBeamPathLengthMaximum);
		tag_to_elem.insert(0x00187058, &tags::FilterBeamPathLengthMaximum);

		ident_to_elem.insert("ExposureControlMode", &tags::ExposureControlMode);
		tag_to_elem.insert(0x00187060, &tags::ExposureControlMode);

		ident_to_elem.insert("ExposureControlModeDescription", &tags::ExposureControlModeDescription);
		tag_to_elem.insert(0x00187062, &tags::ExposureControlModeDescription);

		ident_to_elem.insert("ExposureStatus", &tags::ExposureStatus);
		tag_to_elem.insert(0x00187064, &tags::ExposureStatus);

		ident_to_elem.insert("PhototimerSetting", &tags::PhototimerSetting);
		tag_to_elem.insert(0x00187065, &tags::PhototimerSetting);

		ident_to_elem.insert("ExposureTimeInuS", &tags::ExposureTimeInuS);
		tag_to_elem.insert(0x00188150, &tags::ExposureTimeInuS);

		ident_to_elem.insert("XRayTubeCurrentInuA", &tags::XRayTubeCurrentInuA);
		tag_to_elem.insert(0x00188151, &tags::XRayTubeCurrentInuA);

		ident_to_elem.insert("ContentQualification", &tags::ContentQualification);
		tag_to_elem.insert(0x00189004, &tags::ContentQualification);

		ident_to_elem.insert("PulseSequenceName", &tags::PulseSequenceName);
		tag_to_elem.insert(0x00189005, &tags::PulseSequenceName);

		ident_to_elem.insert("MRImagingModifierSequence", &tags::MRImagingModifierSequence);
		tag_to_elem.insert(0x00189006, &tags::MRImagingModifierSequence);

		ident_to_elem.insert("EchoPulseSequence", &tags::EchoPulseSequence);
		tag_to_elem.insert(0x00189008, &tags::EchoPulseSequence);

		ident_to_elem.insert("InversionRecovery", &tags::InversionRecovery);
		tag_to_elem.insert(0x00189009, &tags::InversionRecovery);

		ident_to_elem.insert("FlowCompensation", &tags::FlowCompensation);
		tag_to_elem.insert(0x00189010, &tags::FlowCompensation);

		ident_to_elem.insert("MultipleSpinEcho", &tags::MultipleSpinEcho);
		tag_to_elem.insert(0x00189011, &tags::MultipleSpinEcho);

		ident_to_elem.insert("MultiPlanarExcitation", &tags::MultiPlanarExcitation);
		tag_to_elem.insert(0x00189012, &tags::MultiPlanarExcitation);

		ident_to_elem.insert("PhaseContrast", &tags::PhaseContrast);
		tag_to_elem.insert(0x00189014, &tags::PhaseContrast);

		ident_to_elem.insert("TimeOfFlightContrast", &tags::TimeOfFlightContrast);
		tag_to_elem.insert(0x00189015, &tags::TimeOfFlightContrast);

		ident_to_elem.insert("Spoiling", &tags::Spoiling);
		tag_to_elem.insert(0x00189016, &tags::Spoiling);

		ident_to_elem.insert("SteadyStatePulseSequence", &tags::SteadyStatePulseSequence);
		tag_to_elem.insert(0x00189017, &tags::SteadyStatePulseSequence);

		ident_to_elem.insert("EchoPlanarPulseSequence", &tags::EchoPlanarPulseSequence);
		tag_to_elem.insert(0x00189018, &tags::EchoPlanarPulseSequence);

		ident_to_elem.insert("TagAngleFirstAxis", &tags::TagAngleFirstAxis);
		tag_to_elem.insert(0x00189019, &tags::TagAngleFirstAxis);

		ident_to_elem.insert("MagnetizationTransfer", &tags::MagnetizationTransfer);
		tag_to_elem.insert(0x00189020, &tags::MagnetizationTransfer);

		ident_to_elem.insert("T2Preparation", &tags::T2Preparation);
		tag_to_elem.insert(0x00189021, &tags::T2Preparation);

		ident_to_elem.insert("BloodSignalNulling", &tags::BloodSignalNulling);
		tag_to_elem.insert(0x00189022, &tags::BloodSignalNulling);

		ident_to_elem.insert("SaturationRecovery", &tags::SaturationRecovery);
		tag_to_elem.insert(0x00189024, &tags::SaturationRecovery);

		ident_to_elem.insert("SpectrallySelectedSuppression", &tags::SpectrallySelectedSuppression);
		tag_to_elem.insert(0x00189025, &tags::SpectrallySelectedSuppression);

		ident_to_elem.insert("SpectrallySelectedExcitation", &tags::SpectrallySelectedExcitation);
		tag_to_elem.insert(0x00189026, &tags::SpectrallySelectedExcitation);

		ident_to_elem.insert("SpatialPresaturation", &tags::SpatialPresaturation);
		tag_to_elem.insert(0x00189027, &tags::SpatialPresaturation);

		ident_to_elem.insert("Tagging", &tags::Tagging);
		tag_to_elem.insert(0x00189028, &tags::Tagging);

		ident_to_elem.insert("OversamplingPhase", &tags::OversamplingPhase);
		tag_to_elem.insert(0x00189029, &tags::OversamplingPhase);

		ident_to_elem.insert("TagSpacingFirstDimension", &tags::TagSpacingFirstDimension);
		tag_to_elem.insert(0x00189030, &tags::TagSpacingFirstDimension);

		ident_to_elem.insert("GeometryOfKSpaceTraversal", &tags::GeometryOfKSpaceTraversal);
		tag_to_elem.insert(0x00189032, &tags::GeometryOfKSpaceTraversal);

		ident_to_elem.insert("SegmentedKSpaceTraversal", &tags::SegmentedKSpaceTraversal);
		tag_to_elem.insert(0x00189033, &tags::SegmentedKSpaceTraversal);

		ident_to_elem.insert("RectilinearPhaseEncodeReordering", &tags::RectilinearPhaseEncodeReordering);
		tag_to_elem.insert(0x00189034, &tags::RectilinearPhaseEncodeReordering);

		ident_to_elem.insert("TagThickness", &tags::TagThickness);
		tag_to_elem.insert(0x00189035, &tags::TagThickness);

		ident_to_elem.insert("PartialFourierDirection", &tags::PartialFourierDirection);
		tag_to_elem.insert(0x00189036, &tags::PartialFourierDirection);

		ident_to_elem.insert("CardiacSynchronizationTechnique", &tags::CardiacSynchronizationTechnique);
		tag_to_elem.insert(0x00189037, &tags::CardiacSynchronizationTechnique);

		ident_to_elem.insert("ReceiveCoilManufacturerName", &tags::ReceiveCoilManufacturerName);
		tag_to_elem.insert(0x00189041, &tags::ReceiveCoilManufacturerName);

		ident_to_elem.insert("MRReceiveCoilSequence", &tags::MRReceiveCoilSequence);
		tag_to_elem.insert(0x00189042, &tags::MRReceiveCoilSequence);

		ident_to_elem.insert("ReceiveCoilType", &tags::ReceiveCoilType);
		tag_to_elem.insert(0x00189043, &tags::ReceiveCoilType);

		ident_to_elem.insert("QuadratureReceiveCoil", &tags::QuadratureReceiveCoil);
		tag_to_elem.insert(0x00189044, &tags::QuadratureReceiveCoil);

		ident_to_elem.insert("MultiCoilDefinitionSequence", &tags::MultiCoilDefinitionSequence);
		tag_to_elem.insert(0x00189045, &tags::MultiCoilDefinitionSequence);

		ident_to_elem.insert("MultiCoilConfiguration", &tags::MultiCoilConfiguration);
		tag_to_elem.insert(0x00189046, &tags::MultiCoilConfiguration);

		ident_to_elem.insert("MultiCoilElementName", &tags::MultiCoilElementName);
		tag_to_elem.insert(0x00189047, &tags::MultiCoilElementName);

		ident_to_elem.insert("MultiCoilElementUsed", &tags::MultiCoilElementUsed);
		tag_to_elem.insert(0x00189048, &tags::MultiCoilElementUsed);

		ident_to_elem.insert("MRTransmitCoilSequence", &tags::MRTransmitCoilSequence);
		tag_to_elem.insert(0x00189049, &tags::MRTransmitCoilSequence);

		ident_to_elem.insert("TransmitCoilManufacturerName", &tags::TransmitCoilManufacturerName);
		tag_to_elem.insert(0x00189050, &tags::TransmitCoilManufacturerName);

		ident_to_elem.insert("TransmitCoilType", &tags::TransmitCoilType);
		tag_to_elem.insert(0x00189051, &tags::TransmitCoilType);

		ident_to_elem.insert("SpectralWidth", &tags::SpectralWidth);
		tag_to_elem.insert(0x00189052, &tags::SpectralWidth);

		ident_to_elem.insert("ChemicalShiftReference", &tags::ChemicalShiftReference);
		tag_to_elem.insert(0x00189053, &tags::ChemicalShiftReference);

		ident_to_elem.insert("VolumeLocalizationTechnique", &tags::VolumeLocalizationTechnique);
		tag_to_elem.insert(0x00189054, &tags::VolumeLocalizationTechnique);

		ident_to_elem.insert("MRAcquisitionFrequencyEncodingSteps", &tags::MRAcquisitionFrequencyEncodingSteps);
		tag_to_elem.insert(0x00189058, &tags::MRAcquisitionFrequencyEncodingSteps);

		ident_to_elem.insert("Decoupling", &tags::Decoupling);
		tag_to_elem.insert(0x00189059, &tags::Decoupling);

		ident_to_elem.insert("DecoupledNucleus", &tags::DecoupledNucleus);
		tag_to_elem.insert(0x00189060, &tags::DecoupledNucleus);

		ident_to_elem.insert("DecouplingFrequency", &tags::DecouplingFrequency);
		tag_to_elem.insert(0x00189061, &tags::DecouplingFrequency);

		ident_to_elem.insert("DecouplingMethod", &tags::DecouplingMethod);
		tag_to_elem.insert(0x00189062, &tags::DecouplingMethod);

		ident_to_elem.insert("DecouplingChemicalShiftReference", &tags::DecouplingChemicalShiftReference);
		tag_to_elem.insert(0x00189063, &tags::DecouplingChemicalShiftReference);

		ident_to_elem.insert("KSpaceFiltering", &tags::KSpaceFiltering);
		tag_to_elem.insert(0x00189064, &tags::KSpaceFiltering);

		ident_to_elem.insert("TimeDomainFiltering", &tags::TimeDomainFiltering);
		tag_to_elem.insert(0x00189065, &tags::TimeDomainFiltering);

		ident_to_elem.insert("NumberOfZeroFills", &tags::NumberOfZeroFills);
		tag_to_elem.insert(0x00189066, &tags::NumberOfZeroFills);

		ident_to_elem.insert("BaselineCorrection", &tags::BaselineCorrection);
		tag_to_elem.insert(0x00189067, &tags::BaselineCorrection);

		ident_to_elem.insert("ParallelReductionFactorInPlane", &tags::ParallelReductionFactorInPlane);
		tag_to_elem.insert(0x00189069, &tags::ParallelReductionFactorInPlane);

		ident_to_elem.insert("CardiacRRIntervalSpecified", &tags::CardiacRRIntervalSpecified);
		tag_to_elem.insert(0x00189070, &tags::CardiacRRIntervalSpecified);

		ident_to_elem.insert("AcquisitionDuration", &tags::AcquisitionDuration);
		tag_to_elem.insert(0x00189073, &tags::AcquisitionDuration);

		ident_to_elem.insert("FrameAcquisitionDateTime", &tags::FrameAcquisitionDateTime);
		tag_to_elem.insert(0x00189074, &tags::FrameAcquisitionDateTime);

		ident_to_elem.insert("DiffusionDirectionality", &tags::DiffusionDirectionality);
		tag_to_elem.insert(0x00189075, &tags::DiffusionDirectionality);

		ident_to_elem.insert("DiffusionGradientDirectionSequence", &tags::DiffusionGradientDirectionSequence);
		tag_to_elem.insert(0x00189076, &tags::DiffusionGradientDirectionSequence);

		ident_to_elem.insert("ParallelAcquisition", &tags::ParallelAcquisition);
		tag_to_elem.insert(0x00189077, &tags::ParallelAcquisition);

		ident_to_elem.insert("ParallelAcquisitionTechnique", &tags::ParallelAcquisitionTechnique);
		tag_to_elem.insert(0x00189078, &tags::ParallelAcquisitionTechnique);

		ident_to_elem.insert("InversionTimes", &tags::InversionTimes);
		tag_to_elem.insert(0x00189079, &tags::InversionTimes);

		ident_to_elem.insert("MetaboliteMapDescription", &tags::MetaboliteMapDescription);
		tag_to_elem.insert(0x00189080, &tags::MetaboliteMapDescription);

		ident_to_elem.insert("PartialFourier", &tags::PartialFourier);
		tag_to_elem.insert(0x00189081, &tags::PartialFourier);

		ident_to_elem.insert("EffectiveEchoTime", &tags::EffectiveEchoTime);
		tag_to_elem.insert(0x00189082, &tags::EffectiveEchoTime);

		ident_to_elem.insert("MetaboliteMapCodeSequence", &tags::MetaboliteMapCodeSequence);
		tag_to_elem.insert(0x00189083, &tags::MetaboliteMapCodeSequence);

		ident_to_elem.insert("ChemicalShiftSequence", &tags::ChemicalShiftSequence);
		tag_to_elem.insert(0x00189084, &tags::ChemicalShiftSequence);

		ident_to_elem.insert("CardiacSignalSource", &tags::CardiacSignalSource);
		tag_to_elem.insert(0x00189085, &tags::CardiacSignalSource);

		ident_to_elem.insert("DiffusionBValue", &tags::DiffusionBValue);
		tag_to_elem.insert(0x00189087, &tags::DiffusionBValue);

		ident_to_elem.insert("DiffusionGradientOrientation", &tags::DiffusionGradientOrientation);
		tag_to_elem.insert(0x00189089, &tags::DiffusionGradientOrientation);

		ident_to_elem.insert("VelocityEncodingDirection", &tags::VelocityEncodingDirection);
		tag_to_elem.insert(0x00189090, &tags::VelocityEncodingDirection);

		ident_to_elem.insert("VelocityEncodingMinimumValue", &tags::VelocityEncodingMinimumValue);
		tag_to_elem.insert(0x00189091, &tags::VelocityEncodingMinimumValue);

		ident_to_elem.insert("VelocityEncodingAcquisitionSequence", &tags::VelocityEncodingAcquisitionSequence);
		tag_to_elem.insert(0x00189092, &tags::VelocityEncodingAcquisitionSequence);

		ident_to_elem.insert("NumberOfKSpaceTrajectories", &tags::NumberOfKSpaceTrajectories);
		tag_to_elem.insert(0x00189093, &tags::NumberOfKSpaceTrajectories);

		ident_to_elem.insert("CoverageOfKSpace", &tags::CoverageOfKSpace);
		tag_to_elem.insert(0x00189094, &tags::CoverageOfKSpace);

		ident_to_elem.insert("SpectroscopyAcquisitionPhaseRows", &tags::SpectroscopyAcquisitionPhaseRows);
		tag_to_elem.insert(0x00189095, &tags::SpectroscopyAcquisitionPhaseRows);

		ident_to_elem.insert("ParallelReductionFactorInPlaneRetired", &tags::ParallelReductionFactorInPlaneRetired);
		tag_to_elem.insert(0x00189096, &tags::ParallelReductionFactorInPlaneRetired);

		ident_to_elem.insert("TransmitterFrequency", &tags::TransmitterFrequency);
		tag_to_elem.insert(0x00189098, &tags::TransmitterFrequency);

		ident_to_elem.insert("ResonantNucleus", &tags::ResonantNucleus);
		tag_to_elem.insert(0x00189100, &tags::ResonantNucleus);

		ident_to_elem.insert("FrequencyCorrection", &tags::FrequencyCorrection);
		tag_to_elem.insert(0x00189101, &tags::FrequencyCorrection);

		ident_to_elem.insert("MRSpectroscopyFOVGeometrySequence", &tags::MRSpectroscopyFOVGeometrySequence);
		tag_to_elem.insert(0x00189103, &tags::MRSpectroscopyFOVGeometrySequence);

		ident_to_elem.insert("SlabThickness", &tags::SlabThickness);
		tag_to_elem.insert(0x00189104, &tags::SlabThickness);

		ident_to_elem.insert("SlabOrientation", &tags::SlabOrientation);
		tag_to_elem.insert(0x00189105, &tags::SlabOrientation);

		ident_to_elem.insert("MidSlabPosition", &tags::MidSlabPosition);
		tag_to_elem.insert(0x00189106, &tags::MidSlabPosition);

		ident_to_elem.insert("MRSpatialSaturationSequence", &tags::MRSpatialSaturationSequence);
		tag_to_elem.insert(0x00189107, &tags::MRSpatialSaturationSequence);

		ident_to_elem.insert("MRTimingAndRelatedParametersSequence", &tags::MRTimingAndRelatedParametersSequence);
		tag_to_elem.insert(0x00189112, &tags::MRTimingAndRelatedParametersSequence);

		ident_to_elem.insert("MREchoSequence", &tags::MREchoSequence);
		tag_to_elem.insert(0x00189114, &tags::MREchoSequence);

		ident_to_elem.insert("MRModifierSequence", &tags::MRModifierSequence);
		tag_to_elem.insert(0x00189115, &tags::MRModifierSequence);

		ident_to_elem.insert("MRDiffusionSequence", &tags::MRDiffusionSequence);
		tag_to_elem.insert(0x00189117, &tags::MRDiffusionSequence);

		ident_to_elem.insert("CardiacSynchronizationSequence", &tags::CardiacSynchronizationSequence);
		tag_to_elem.insert(0x00189118, &tags::CardiacSynchronizationSequence);

		ident_to_elem.insert("MRAveragesSequence", &tags::MRAveragesSequence);
		tag_to_elem.insert(0x00189119, &tags::MRAveragesSequence);

		ident_to_elem.insert("MRFOVGeometrySequence", &tags::MRFOVGeometrySequence);
		tag_to_elem.insert(0x00189125, &tags::MRFOVGeometrySequence);

		ident_to_elem.insert("VolumeLocalizationSequence", &tags::VolumeLocalizationSequence);
		tag_to_elem.insert(0x00189126, &tags::VolumeLocalizationSequence);

		ident_to_elem.insert("SpectroscopyAcquisitionDataColumns", &tags::SpectroscopyAcquisitionDataColumns);
		tag_to_elem.insert(0x00189127, &tags::SpectroscopyAcquisitionDataColumns);

		ident_to_elem.insert("DiffusionAnisotropyType", &tags::DiffusionAnisotropyType);
		tag_to_elem.insert(0x00189147, &tags::DiffusionAnisotropyType);

		ident_to_elem.insert("FrameReferenceDateTime", &tags::FrameReferenceDateTime);
		tag_to_elem.insert(0x00189151, &tags::FrameReferenceDateTime);

		ident_to_elem.insert("MRMetaboliteMapSequence", &tags::MRMetaboliteMapSequence);
		tag_to_elem.insert(0x00189152, &tags::MRMetaboliteMapSequence);

		ident_to_elem.insert("ParallelReductionFactorOutOfPlane", &tags::ParallelReductionFactorOutOfPlane);
		tag_to_elem.insert(0x00189155, &tags::ParallelReductionFactorOutOfPlane);

		ident_to_elem.insert("SpectroscopyAcquisitionOutOfPlanePhaseSteps", &tags::SpectroscopyAcquisitionOutOfPlanePhaseSteps);
		tag_to_elem.insert(0x00189159, &tags::SpectroscopyAcquisitionOutOfPlanePhaseSteps);

		ident_to_elem.insert("BulkMotionStatus", &tags::BulkMotionStatus);
		tag_to_elem.insert(0x00189166, &tags::BulkMotionStatus);

		ident_to_elem.insert("ParallelReductionFactorSecondInPlane", &tags::ParallelReductionFactorSecondInPlane);
		tag_to_elem.insert(0x00189168, &tags::ParallelReductionFactorSecondInPlane);

		ident_to_elem.insert("CardiacBeatRejectionTechnique", &tags::CardiacBeatRejectionTechnique);
		tag_to_elem.insert(0x00189169, &tags::CardiacBeatRejectionTechnique);

		ident_to_elem.insert("RespiratoryMotionCompensationTechnique", &tags::RespiratoryMotionCompensationTechnique);
		tag_to_elem.insert(0x00189170, &tags::RespiratoryMotionCompensationTechnique);

		ident_to_elem.insert("RespiratorySignalSource", &tags::RespiratorySignalSource);
		tag_to_elem.insert(0x00189171, &tags::RespiratorySignalSource);

		ident_to_elem.insert("BulkMotionCompensationTechnique", &tags::BulkMotionCompensationTechnique);
		tag_to_elem.insert(0x00189172, &tags::BulkMotionCompensationTechnique);

		ident_to_elem.insert("BulkMotionSignalSource", &tags::BulkMotionSignalSource);
		tag_to_elem.insert(0x00189173, &tags::BulkMotionSignalSource);

		ident_to_elem.insert("ApplicableSafetyStandardAgency", &tags::ApplicableSafetyStandardAgency);
		tag_to_elem.insert(0x00189174, &tags::ApplicableSafetyStandardAgency);

		ident_to_elem.insert("ApplicableSafetyStandardDescription", &tags::ApplicableSafetyStandardDescription);
		tag_to_elem.insert(0x00189175, &tags::ApplicableSafetyStandardDescription);

		ident_to_elem.insert("OperatingModeSequence", &tags::OperatingModeSequence);
		tag_to_elem.insert(0x00189176, &tags::OperatingModeSequence);

		ident_to_elem.insert("OperatingModeType", &tags::OperatingModeType);
		tag_to_elem.insert(0x00189177, &tags::OperatingModeType);

		ident_to_elem.insert("OperatingMode", &tags::OperatingMode);
		tag_to_elem.insert(0x00189178, &tags::OperatingMode);

		ident_to_elem.insert("SpecificAbsorptionRateDefinition", &tags::SpecificAbsorptionRateDefinition);
		tag_to_elem.insert(0x00189179, &tags::SpecificAbsorptionRateDefinition);

		ident_to_elem.insert("GradientOutputType", &tags::GradientOutputType);
		tag_to_elem.insert(0x00189180, &tags::GradientOutputType);

		ident_to_elem.insert("SpecificAbsorptionRateValue", &tags::SpecificAbsorptionRateValue);
		tag_to_elem.insert(0x00189181, &tags::SpecificAbsorptionRateValue);

		ident_to_elem.insert("GradientOutput", &tags::GradientOutput);
		tag_to_elem.insert(0x00189182, &tags::GradientOutput);

		ident_to_elem.insert("FlowCompensationDirection", &tags::FlowCompensationDirection);
		tag_to_elem.insert(0x00189183, &tags::FlowCompensationDirection);

		ident_to_elem.insert("TaggingDelay", &tags::TaggingDelay);
		tag_to_elem.insert(0x00189184, &tags::TaggingDelay);

		ident_to_elem.insert("RespiratoryMotionCompensationTechniqueDescription", &tags::RespiratoryMotionCompensationTechniqueDescription);
		tag_to_elem.insert(0x00189185, &tags::RespiratoryMotionCompensationTechniqueDescription);

		ident_to_elem.insert("RespiratorySignalSourceID", &tags::RespiratorySignalSourceID);
		tag_to_elem.insert(0x00189186, &tags::RespiratorySignalSourceID);

		ident_to_elem.insert("ChemicalShiftMinimumIntegrationLimitInHz", &tags::ChemicalShiftMinimumIntegrationLimitInHz);
		tag_to_elem.insert(0x00189195, &tags::ChemicalShiftMinimumIntegrationLimitInHz);

		ident_to_elem.insert("ChemicalShiftMaximumIntegrationLimitInHz", &tags::ChemicalShiftMaximumIntegrationLimitInHz);
		tag_to_elem.insert(0x00189196, &tags::ChemicalShiftMaximumIntegrationLimitInHz);

		ident_to_elem.insert("MRVelocityEncodingSequence", &tags::MRVelocityEncodingSequence);
		tag_to_elem.insert(0x00189197, &tags::MRVelocityEncodingSequence);

		ident_to_elem.insert("FirstOrderPhaseCorrection", &tags::FirstOrderPhaseCorrection);
		tag_to_elem.insert(0x00189198, &tags::FirstOrderPhaseCorrection);

		ident_to_elem.insert("WaterReferencedPhaseCorrection", &tags::WaterReferencedPhaseCorrection);
		tag_to_elem.insert(0x00189199, &tags::WaterReferencedPhaseCorrection);

		ident_to_elem.insert("MRSpectroscopyAcquisitionType", &tags::MRSpectroscopyAcquisitionType);
		tag_to_elem.insert(0x00189200, &tags::MRSpectroscopyAcquisitionType);

		ident_to_elem.insert("RespiratoryCyclePosition", &tags::RespiratoryCyclePosition);
		tag_to_elem.insert(0x00189214, &tags::RespiratoryCyclePosition);

		ident_to_elem.insert("VelocityEncodingMaximumValue", &tags::VelocityEncodingMaximumValue);
		tag_to_elem.insert(0x00189217, &tags::VelocityEncodingMaximumValue);

		ident_to_elem.insert("TagSpacingSecondDimension", &tags::TagSpacingSecondDimension);
		tag_to_elem.insert(0x00189218, &tags::TagSpacingSecondDimension);

		ident_to_elem.insert("TagAngleSecondAxis", &tags::TagAngleSecondAxis);
		tag_to_elem.insert(0x00189219, &tags::TagAngleSecondAxis);

		ident_to_elem.insert("FrameAcquisitionDuration", &tags::FrameAcquisitionDuration);
		tag_to_elem.insert(0x00189220, &tags::FrameAcquisitionDuration);

		ident_to_elem.insert("MRImageFrameTypeSequence", &tags::MRImageFrameTypeSequence);
		tag_to_elem.insert(0x00189226, &tags::MRImageFrameTypeSequence);

		ident_to_elem.insert("MRSpectroscopyFrameTypeSequence", &tags::MRSpectroscopyFrameTypeSequence);
		tag_to_elem.insert(0x00189227, &tags::MRSpectroscopyFrameTypeSequence);

		ident_to_elem.insert("MRAcquisitionPhaseEncodingStepsInPlane", &tags::MRAcquisitionPhaseEncodingStepsInPlane);
		tag_to_elem.insert(0x00189231, &tags::MRAcquisitionPhaseEncodingStepsInPlane);

		ident_to_elem.insert("MRAcquisitionPhaseEncodingStepsOutOfPlane", &tags::MRAcquisitionPhaseEncodingStepsOutOfPlane);
		tag_to_elem.insert(0x00189232, &tags::MRAcquisitionPhaseEncodingStepsOutOfPlane);

		ident_to_elem.insert("SpectroscopyAcquisitionPhaseColumns", &tags::SpectroscopyAcquisitionPhaseColumns);
		tag_to_elem.insert(0x00189234, &tags::SpectroscopyAcquisitionPhaseColumns);

		ident_to_elem.insert("CardiacCyclePosition", &tags::CardiacCyclePosition);
		tag_to_elem.insert(0x00189236, &tags::CardiacCyclePosition);

		ident_to_elem.insert("SpecificAbsorptionRateSequence", &tags::SpecificAbsorptionRateSequence);
		tag_to_elem.insert(0x00189239, &tags::SpecificAbsorptionRateSequence);

		ident_to_elem.insert("RFEchoTrainLength", &tags::RFEchoTrainLength);
		tag_to_elem.insert(0x00189240, &tags::RFEchoTrainLength);

		ident_to_elem.insert("GradientEchoTrainLength", &tags::GradientEchoTrainLength);
		tag_to_elem.insert(0x00189241, &tags::GradientEchoTrainLength);

		ident_to_elem.insert("ArterialSpinLabelingContrast", &tags::ArterialSpinLabelingContrast);
		tag_to_elem.insert(0x00189250, &tags::ArterialSpinLabelingContrast);

		ident_to_elem.insert("MRArterialSpinLabelingSequence", &tags::MRArterialSpinLabelingSequence);
		tag_to_elem.insert(0x00189251, &tags::MRArterialSpinLabelingSequence);

		ident_to_elem.insert("ASLTechniqueDescription", &tags::ASLTechniqueDescription);
		tag_to_elem.insert(0x00189252, &tags::ASLTechniqueDescription);

		ident_to_elem.insert("ASLSlabNumber", &tags::ASLSlabNumber);
		tag_to_elem.insert(0x00189253, &tags::ASLSlabNumber);

		ident_to_elem.insert("ASLSlabThickness", &tags::ASLSlabThickness);
		tag_to_elem.insert(0x00189254, &tags::ASLSlabThickness);

		ident_to_elem.insert("ASLSlabOrientation", &tags::ASLSlabOrientation);
		tag_to_elem.insert(0x00189255, &tags::ASLSlabOrientation);

		ident_to_elem.insert("ASLMidSlabPosition", &tags::ASLMidSlabPosition);
		tag_to_elem.insert(0x00189256, &tags::ASLMidSlabPosition);

		ident_to_elem.insert("ASLContext", &tags::ASLContext);
		tag_to_elem.insert(0x00189257, &tags::ASLContext);

		ident_to_elem.insert("ASLPulseTrainDuration", &tags::ASLPulseTrainDuration);
		tag_to_elem.insert(0x00189258, &tags::ASLPulseTrainDuration);

		ident_to_elem.insert("ASLCrusherFlag", &tags::ASLCrusherFlag);
		tag_to_elem.insert(0x00189259, &tags::ASLCrusherFlag);

		ident_to_elem.insert("ASLCrusherFlowLimit", &tags::ASLCrusherFlowLimit);
		tag_to_elem.insert(0x0018925A, &tags::ASLCrusherFlowLimit);

		ident_to_elem.insert("ASLCrusherDescription", &tags::ASLCrusherDescription);
		tag_to_elem.insert(0x0018925B, &tags::ASLCrusherDescription);

		ident_to_elem.insert("ASLBolusCutoffFlag", &tags::ASLBolusCutoffFlag);
		tag_to_elem.insert(0x0018925C, &tags::ASLBolusCutoffFlag);

		ident_to_elem.insert("ASLBolusCutoffTimingSequence", &tags::ASLBolusCutoffTimingSequence);
		tag_to_elem.insert(0x0018925D, &tags::ASLBolusCutoffTimingSequence);

		ident_to_elem.insert("ASLBolusCutoffTechnique", &tags::ASLBolusCutoffTechnique);
		tag_to_elem.insert(0x0018925E, &tags::ASLBolusCutoffTechnique);

		ident_to_elem.insert("ASLBolusCutoffDelayTime", &tags::ASLBolusCutoffDelayTime);
		tag_to_elem.insert(0x0018925F, &tags::ASLBolusCutoffDelayTime);

		ident_to_elem.insert("ASLSlabSequence", &tags::ASLSlabSequence);
		tag_to_elem.insert(0x00189260, &tags::ASLSlabSequence);

		ident_to_elem.insert("ChemicalShiftMinimumIntegrationLimitInppm", &tags::ChemicalShiftMinimumIntegrationLimitInppm);
		tag_to_elem.insert(0x00189295, &tags::ChemicalShiftMinimumIntegrationLimitInppm);

		ident_to_elem.insert("ChemicalShiftMaximumIntegrationLimitInppm", &tags::ChemicalShiftMaximumIntegrationLimitInppm);
		tag_to_elem.insert(0x00189296, &tags::ChemicalShiftMaximumIntegrationLimitInppm);

		ident_to_elem.insert("WaterReferenceAcquisition", &tags::WaterReferenceAcquisition);
		tag_to_elem.insert(0x00189297, &tags::WaterReferenceAcquisition);

		ident_to_elem.insert("EchoPeakPosition", &tags::EchoPeakPosition);
		tag_to_elem.insert(0x00189298, &tags::EchoPeakPosition);

		ident_to_elem.insert("CTAcquisitionTypeSequence", &tags::CTAcquisitionTypeSequence);
		tag_to_elem.insert(0x00189301, &tags::CTAcquisitionTypeSequence);

		ident_to_elem.insert("AcquisitionType", &tags::AcquisitionType);
		tag_to_elem.insert(0x00189302, &tags::AcquisitionType);

		ident_to_elem.insert("TubeAngle", &tags::TubeAngle);
		tag_to_elem.insert(0x00189303, &tags::TubeAngle);

		ident_to_elem.insert("CTAcquisitionDetailsSequence", &tags::CTAcquisitionDetailsSequence);
		tag_to_elem.insert(0x00189304, &tags::CTAcquisitionDetailsSequence);

		ident_to_elem.insert("RevolutionTime", &tags::RevolutionTime);
		tag_to_elem.insert(0x00189305, &tags::RevolutionTime);

		ident_to_elem.insert("SingleCollimationWidth", &tags::SingleCollimationWidth);
		tag_to_elem.insert(0x00189306, &tags::SingleCollimationWidth);

		ident_to_elem.insert("TotalCollimationWidth", &tags::TotalCollimationWidth);
		tag_to_elem.insert(0x00189307, &tags::TotalCollimationWidth);

		ident_to_elem.insert("CTTableDynamicsSequence", &tags::CTTableDynamicsSequence);
		tag_to_elem.insert(0x00189308, &tags::CTTableDynamicsSequence);

		ident_to_elem.insert("TableSpeed", &tags::TableSpeed);
		tag_to_elem.insert(0x00189309, &tags::TableSpeed);

		ident_to_elem.insert("TableFeedPerRotation", &tags::TableFeedPerRotation);
		tag_to_elem.insert(0x00189310, &tags::TableFeedPerRotation);

		ident_to_elem.insert("SpiralPitchFactor", &tags::SpiralPitchFactor);
		tag_to_elem.insert(0x00189311, &tags::SpiralPitchFactor);

		ident_to_elem.insert("CTGeometrySequence", &tags::CTGeometrySequence);
		tag_to_elem.insert(0x00189312, &tags::CTGeometrySequence);

		ident_to_elem.insert("DataCollectionCenterPatient", &tags::DataCollectionCenterPatient);
		tag_to_elem.insert(0x00189313, &tags::DataCollectionCenterPatient);

		ident_to_elem.insert("CTReconstructionSequence", &tags::CTReconstructionSequence);
		tag_to_elem.insert(0x00189314, &tags::CTReconstructionSequence);

		ident_to_elem.insert("ReconstructionAlgorithm", &tags::ReconstructionAlgorithm);
		tag_to_elem.insert(0x00189315, &tags::ReconstructionAlgorithm);

		ident_to_elem.insert("ConvolutionKernelGroup", &tags::ConvolutionKernelGroup);
		tag_to_elem.insert(0x00189316, &tags::ConvolutionKernelGroup);

		ident_to_elem.insert("ReconstructionFieldOfView", &tags::ReconstructionFieldOfView);
		tag_to_elem.insert(0x00189317, &tags::ReconstructionFieldOfView);

		ident_to_elem.insert("ReconstructionTargetCenterPatient", &tags::ReconstructionTargetCenterPatient);
		tag_to_elem.insert(0x00189318, &tags::ReconstructionTargetCenterPatient);

		ident_to_elem.insert("ReconstructionAngle", &tags::ReconstructionAngle);
		tag_to_elem.insert(0x00189319, &tags::ReconstructionAngle);

		ident_to_elem.insert("ImageFilter", &tags::ImageFilter);
		tag_to_elem.insert(0x00189320, &tags::ImageFilter);

		ident_to_elem.insert("CTExposureSequence", &tags::CTExposureSequence);
		tag_to_elem.insert(0x00189321, &tags::CTExposureSequence);

		ident_to_elem.insert("ReconstructionPixelSpacing", &tags::ReconstructionPixelSpacing);
		tag_to_elem.insert(0x00189322, &tags::ReconstructionPixelSpacing);

		ident_to_elem.insert("ExposureModulationType", &tags::ExposureModulationType);
		tag_to_elem.insert(0x00189323, &tags::ExposureModulationType);

		ident_to_elem.insert("EstimatedDoseSaving", &tags::EstimatedDoseSaving);
		tag_to_elem.insert(0x00189324, &tags::EstimatedDoseSaving);

		ident_to_elem.insert("CTXRayDetailsSequence", &tags::CTXRayDetailsSequence);
		tag_to_elem.insert(0x00189325, &tags::CTXRayDetailsSequence);

		ident_to_elem.insert("CTPositionSequence", &tags::CTPositionSequence);
		tag_to_elem.insert(0x00189326, &tags::CTPositionSequence);

		ident_to_elem.insert("TablePosition", &tags::TablePosition);
		tag_to_elem.insert(0x00189327, &tags::TablePosition);

		ident_to_elem.insert("ExposureTimeInms", &tags::ExposureTimeInms);
		tag_to_elem.insert(0x00189328, &tags::ExposureTimeInms);

		ident_to_elem.insert("CTImageFrameTypeSequence", &tags::CTImageFrameTypeSequence);
		tag_to_elem.insert(0x00189329, &tags::CTImageFrameTypeSequence);

		ident_to_elem.insert("XRayTubeCurrentInmA", &tags::XRayTubeCurrentInmA);
		tag_to_elem.insert(0x00189330, &tags::XRayTubeCurrentInmA);

		ident_to_elem.insert("ExposureInmAs", &tags::ExposureInmAs);
		tag_to_elem.insert(0x00189332, &tags::ExposureInmAs);

		ident_to_elem.insert("ConstantVolumeFlag", &tags::ConstantVolumeFlag);
		tag_to_elem.insert(0x00189333, &tags::ConstantVolumeFlag);

		ident_to_elem.insert("FluoroscopyFlag", &tags::FluoroscopyFlag);
		tag_to_elem.insert(0x00189334, &tags::FluoroscopyFlag);

		ident_to_elem.insert("DistanceSourceToDataCollectionCenter", &tags::DistanceSourceToDataCollectionCenter);
		tag_to_elem.insert(0x00189335, &tags::DistanceSourceToDataCollectionCenter);

		ident_to_elem.insert("ContrastBolusAgentNumber", &tags::ContrastBolusAgentNumber);
		tag_to_elem.insert(0x00189337, &tags::ContrastBolusAgentNumber);

		ident_to_elem.insert("ContrastBolusIngredientCodeSequence", &tags::ContrastBolusIngredientCodeSequence);
		tag_to_elem.insert(0x00189338, &tags::ContrastBolusIngredientCodeSequence);

		ident_to_elem.insert("ContrastAdministrationProfileSequence", &tags::ContrastAdministrationProfileSequence);
		tag_to_elem.insert(0x00189340, &tags::ContrastAdministrationProfileSequence);

		ident_to_elem.insert("ContrastBolusUsageSequence", &tags::ContrastBolusUsageSequence);
		tag_to_elem.insert(0x00189341, &tags::ContrastBolusUsageSequence);

		ident_to_elem.insert("ContrastBolusAgentAdministered", &tags::ContrastBolusAgentAdministered);
		tag_to_elem.insert(0x00189342, &tags::ContrastBolusAgentAdministered);

		ident_to_elem.insert("ContrastBolusAgentDetected", &tags::ContrastBolusAgentDetected);
		tag_to_elem.insert(0x00189343, &tags::ContrastBolusAgentDetected);

		ident_to_elem.insert("ContrastBolusAgentPhase", &tags::ContrastBolusAgentPhase);
		tag_to_elem.insert(0x00189344, &tags::ContrastBolusAgentPhase);

		ident_to_elem.insert("CTDIvol", &tags::CTDIvol);
		tag_to_elem.insert(0x00189345, &tags::CTDIvol);

		ident_to_elem.insert("CTDIPhantomTypeCodeSequence", &tags::CTDIPhantomTypeCodeSequence);
		tag_to_elem.insert(0x00189346, &tags::CTDIPhantomTypeCodeSequence);

		ident_to_elem.insert("CalciumScoringMassFactorPatient", &tags::CalciumScoringMassFactorPatient);
		tag_to_elem.insert(0x00189351, &tags::CalciumScoringMassFactorPatient);

		ident_to_elem.insert("CalciumScoringMassFactorDevice", &tags::CalciumScoringMassFactorDevice);
		tag_to_elem.insert(0x00189352, &tags::CalciumScoringMassFactorDevice);

		ident_to_elem.insert("EnergyWeightingFactor", &tags::EnergyWeightingFactor);
		tag_to_elem.insert(0x00189353, &tags::EnergyWeightingFactor);

		ident_to_elem.insert("CTAdditionalXRaySourceSequence", &tags::CTAdditionalXRaySourceSequence);
		tag_to_elem.insert(0x00189360, &tags::CTAdditionalXRaySourceSequence);

		ident_to_elem.insert("ProjectionPixelCalibrationSequence", &tags::ProjectionPixelCalibrationSequence);
		tag_to_elem.insert(0x00189401, &tags::ProjectionPixelCalibrationSequence);

		ident_to_elem.insert("DistanceSourceToIsocenter", &tags::DistanceSourceToIsocenter);
		tag_to_elem.insert(0x00189402, &tags::DistanceSourceToIsocenter);

		ident_to_elem.insert("DistanceObjectToTableTop", &tags::DistanceObjectToTableTop);
		tag_to_elem.insert(0x00189403, &tags::DistanceObjectToTableTop);

		ident_to_elem.insert("ObjectPixelSpacingInCenterOfBeam", &tags::ObjectPixelSpacingInCenterOfBeam);
		tag_to_elem.insert(0x00189404, &tags::ObjectPixelSpacingInCenterOfBeam);

		ident_to_elem.insert("PositionerPositionSequence", &tags::PositionerPositionSequence);
		tag_to_elem.insert(0x00189405, &tags::PositionerPositionSequence);

		ident_to_elem.insert("TablePositionSequence", &tags::TablePositionSequence);
		tag_to_elem.insert(0x00189406, &tags::TablePositionSequence);

		ident_to_elem.insert("CollimatorShapeSequence", &tags::CollimatorShapeSequence);
		tag_to_elem.insert(0x00189407, &tags::CollimatorShapeSequence);

		ident_to_elem.insert("PlanesInAcquisition", &tags::PlanesInAcquisition);
		tag_to_elem.insert(0x00189410, &tags::PlanesInAcquisition);

		ident_to_elem.insert("XAXRFFrameCharacteristicsSequence", &tags::XAXRFFrameCharacteristicsSequence);
		tag_to_elem.insert(0x00189412, &tags::XAXRFFrameCharacteristicsSequence);

		ident_to_elem.insert("FrameAcquisitionSequence", &tags::FrameAcquisitionSequence);
		tag_to_elem.insert(0x00189417, &tags::FrameAcquisitionSequence);

		ident_to_elem.insert("XRayReceptorType", &tags::XRayReceptorType);
		tag_to_elem.insert(0x00189420, &tags::XRayReceptorType);

		ident_to_elem.insert("AcquisitionProtocolName", &tags::AcquisitionProtocolName);
		tag_to_elem.insert(0x00189423, &tags::AcquisitionProtocolName);

		ident_to_elem.insert("AcquisitionProtocolDescription", &tags::AcquisitionProtocolDescription);
		tag_to_elem.insert(0x00189424, &tags::AcquisitionProtocolDescription);

		ident_to_elem.insert("ContrastBolusIngredientOpaque", &tags::ContrastBolusIngredientOpaque);
		tag_to_elem.insert(0x00189425, &tags::ContrastBolusIngredientOpaque);

		ident_to_elem.insert("DistanceReceptorPlaneToDetectorHousing", &tags::DistanceReceptorPlaneToDetectorHousing);
		tag_to_elem.insert(0x00189426, &tags::DistanceReceptorPlaneToDetectorHousing);

		ident_to_elem.insert("IntensifierActiveShape", &tags::IntensifierActiveShape);
		tag_to_elem.insert(0x00189427, &tags::IntensifierActiveShape);

		ident_to_elem.insert("IntensifierActiveDimensions", &tags::IntensifierActiveDimensions);
		tag_to_elem.insert(0x00189428, &tags::IntensifierActiveDimensions);

		ident_to_elem.insert("PhysicalDetectorSize", &tags::PhysicalDetectorSize);
		tag_to_elem.insert(0x00189429, &tags::PhysicalDetectorSize);

		ident_to_elem.insert("PositionOfIsocenterProjection", &tags::PositionOfIsocenterProjection);
		tag_to_elem.insert(0x00189430, &tags::PositionOfIsocenterProjection);

		ident_to_elem.insert("FieldOfViewSequence", &tags::FieldOfViewSequence);
		tag_to_elem.insert(0x00189432, &tags::FieldOfViewSequence);

		ident_to_elem.insert("FieldOfViewDescription", &tags::FieldOfViewDescription);
		tag_to_elem.insert(0x00189433, &tags::FieldOfViewDescription);

		ident_to_elem.insert("ExposureControlSensingRegionsSequence", &tags::ExposureControlSensingRegionsSequence);
		tag_to_elem.insert(0x00189434, &tags::ExposureControlSensingRegionsSequence);

		ident_to_elem.insert("ExposureControlSensingRegionShape", &tags::ExposureControlSensingRegionShape);
		tag_to_elem.insert(0x00189435, &tags::ExposureControlSensingRegionShape);

		ident_to_elem.insert("ExposureControlSensingRegionLeftVerticalEdge", &tags::ExposureControlSensingRegionLeftVerticalEdge);
		tag_to_elem.insert(0x00189436, &tags::ExposureControlSensingRegionLeftVerticalEdge);

		ident_to_elem.insert("ExposureControlSensingRegionRightVerticalEdge", &tags::ExposureControlSensingRegionRightVerticalEdge);
		tag_to_elem.insert(0x00189437, &tags::ExposureControlSensingRegionRightVerticalEdge);

		ident_to_elem.insert("ExposureControlSensingRegionUpperHorizontalEdge", &tags::ExposureControlSensingRegionUpperHorizontalEdge);
		tag_to_elem.insert(0x00189438, &tags::ExposureControlSensingRegionUpperHorizontalEdge);

		ident_to_elem.insert("ExposureControlSensingRegionLowerHorizontalEdge", &tags::ExposureControlSensingRegionLowerHorizontalEdge);
		tag_to_elem.insert(0x00189439, &tags::ExposureControlSensingRegionLowerHorizontalEdge);

		ident_to_elem.insert("CenterOfCircularExposureControlSensingRegion", &tags::CenterOfCircularExposureControlSensingRegion);
		tag_to_elem.insert(0x00189440, &tags::CenterOfCircularExposureControlSensingRegion);

		ident_to_elem.insert("RadiusOfCircularExposureControlSensingRegion", &tags::RadiusOfCircularExposureControlSensingRegion);
		tag_to_elem.insert(0x00189441, &tags::RadiusOfCircularExposureControlSensingRegion);

		ident_to_elem.insert("VerticesOfThePolygonalExposureControlSensingRegion", &tags::VerticesOfThePolygonalExposureControlSensingRegion);
		tag_to_elem.insert(0x00189442, &tags::VerticesOfThePolygonalExposureControlSensingRegion);

		ident_to_elem.insert("ColumnAngulationPatient", &tags::ColumnAngulationPatient);
		tag_to_elem.insert(0x00189447, &tags::ColumnAngulationPatient);

		ident_to_elem.insert("BeamAngle", &tags::BeamAngle);
		tag_to_elem.insert(0x00189449, &tags::BeamAngle);

		ident_to_elem.insert("FrameDetectorParametersSequence", &tags::FrameDetectorParametersSequence);
		tag_to_elem.insert(0x00189451, &tags::FrameDetectorParametersSequence);

		ident_to_elem.insert("CalculatedAnatomyThickness", &tags::CalculatedAnatomyThickness);
		tag_to_elem.insert(0x00189452, &tags::CalculatedAnatomyThickness);

		ident_to_elem.insert("CalibrationSequence", &tags::CalibrationSequence);
		tag_to_elem.insert(0x00189455, &tags::CalibrationSequence);

		ident_to_elem.insert("ObjectThicknessSequence", &tags::ObjectThicknessSequence);
		tag_to_elem.insert(0x00189456, &tags::ObjectThicknessSequence);

		ident_to_elem.insert("PlaneIdentification", &tags::PlaneIdentification);
		tag_to_elem.insert(0x00189457, &tags::PlaneIdentification);

		ident_to_elem.insert("FieldOfViewDimensionsInFloat", &tags::FieldOfViewDimensionsInFloat);
		tag_to_elem.insert(0x00189461, &tags::FieldOfViewDimensionsInFloat);

		ident_to_elem.insert("IsocenterReferenceSystemSequence", &tags::IsocenterReferenceSystemSequence);
		tag_to_elem.insert(0x00189462, &tags::IsocenterReferenceSystemSequence);

		ident_to_elem.insert("PositionerIsocenterPrimaryAngle", &tags::PositionerIsocenterPrimaryAngle);
		tag_to_elem.insert(0x00189463, &tags::PositionerIsocenterPrimaryAngle);

		ident_to_elem.insert("PositionerIsocenterSecondaryAngle", &tags::PositionerIsocenterSecondaryAngle);
		tag_to_elem.insert(0x00189464, &tags::PositionerIsocenterSecondaryAngle);

		ident_to_elem.insert("PositionerIsocenterDetectorRotationAngle", &tags::PositionerIsocenterDetectorRotationAngle);
		tag_to_elem.insert(0x00189465, &tags::PositionerIsocenterDetectorRotationAngle);

		ident_to_elem.insert("TableXPositionToIsocenter", &tags::TableXPositionToIsocenter);
		tag_to_elem.insert(0x00189466, &tags::TableXPositionToIsocenter);

		ident_to_elem.insert("TableYPositionToIsocenter", &tags::TableYPositionToIsocenter);
		tag_to_elem.insert(0x00189467, &tags::TableYPositionToIsocenter);

		ident_to_elem.insert("TableZPositionToIsocenter", &tags::TableZPositionToIsocenter);
		tag_to_elem.insert(0x00189468, &tags::TableZPositionToIsocenter);

		ident_to_elem.insert("TableHorizontalRotationAngle", &tags::TableHorizontalRotationAngle);
		tag_to_elem.insert(0x00189469, &tags::TableHorizontalRotationAngle);

		ident_to_elem.insert("TableHeadTiltAngle", &tags::TableHeadTiltAngle);
		tag_to_elem.insert(0x00189470, &tags::TableHeadTiltAngle);

		ident_to_elem.insert("TableCradleTiltAngle", &tags::TableCradleTiltAngle);
		tag_to_elem.insert(0x00189471, &tags::TableCradleTiltAngle);

		ident_to_elem.insert("FrameDisplayShutterSequence", &tags::FrameDisplayShutterSequence);
		tag_to_elem.insert(0x00189472, &tags::FrameDisplayShutterSequence);

		ident_to_elem.insert("AcquiredImageAreaDoseProduct", &tags::AcquiredImageAreaDoseProduct);
		tag_to_elem.insert(0x00189473, &tags::AcquiredImageAreaDoseProduct);

		ident_to_elem.insert("CArmPositionerTabletopRelationship", &tags::CArmPositionerTabletopRelationship);
		tag_to_elem.insert(0x00189474, &tags::CArmPositionerTabletopRelationship);

		ident_to_elem.insert("XRayGeometrySequence", &tags::XRayGeometrySequence);
		tag_to_elem.insert(0x00189476, &tags::XRayGeometrySequence);

		ident_to_elem.insert("IrradiationEventIdentificationSequence", &tags::IrradiationEventIdentificationSequence);
		tag_to_elem.insert(0x00189477, &tags::IrradiationEventIdentificationSequence);

		ident_to_elem.insert("XRay3DFrameTypeSequence", &tags::XRay3DFrameTypeSequence);
		tag_to_elem.insert(0x00189504, &tags::XRay3DFrameTypeSequence);

		ident_to_elem.insert("ContributingSourcesSequence", &tags::ContributingSourcesSequence);
		tag_to_elem.insert(0x00189506, &tags::ContributingSourcesSequence);

		ident_to_elem.insert("XRay3DAcquisitionSequence", &tags::XRay3DAcquisitionSequence);
		tag_to_elem.insert(0x00189507, &tags::XRay3DAcquisitionSequence);

		ident_to_elem.insert("PrimaryPositionerScanArc", &tags::PrimaryPositionerScanArc);
		tag_to_elem.insert(0x00189508, &tags::PrimaryPositionerScanArc);

		ident_to_elem.insert("SecondaryPositionerScanArc", &tags::SecondaryPositionerScanArc);
		tag_to_elem.insert(0x00189509, &tags::SecondaryPositionerScanArc);

		ident_to_elem.insert("PrimaryPositionerScanStartAngle", &tags::PrimaryPositionerScanStartAngle);
		tag_to_elem.insert(0x00189510, &tags::PrimaryPositionerScanStartAngle);

		ident_to_elem.insert("SecondaryPositionerScanStartAngle", &tags::SecondaryPositionerScanStartAngle);
		tag_to_elem.insert(0x00189511, &tags::SecondaryPositionerScanStartAngle);

		ident_to_elem.insert("PrimaryPositionerIncrement", &tags::PrimaryPositionerIncrement);
		tag_to_elem.insert(0x00189514, &tags::PrimaryPositionerIncrement);

		ident_to_elem.insert("SecondaryPositionerIncrement", &tags::SecondaryPositionerIncrement);
		tag_to_elem.insert(0x00189515, &tags::SecondaryPositionerIncrement);

		ident_to_elem.insert("StartAcquisitionDateTime", &tags::StartAcquisitionDateTime);
		tag_to_elem.insert(0x00189516, &tags::StartAcquisitionDateTime);

		ident_to_elem.insert("EndAcquisitionDateTime", &tags::EndAcquisitionDateTime);
		tag_to_elem.insert(0x00189517, &tags::EndAcquisitionDateTime);

		ident_to_elem.insert("PrimaryPositionerIncrementSign", &tags::PrimaryPositionerIncrementSign);
		tag_to_elem.insert(0x00189518, &tags::PrimaryPositionerIncrementSign);

		ident_to_elem.insert("SecondaryPositionerIncrementSign", &tags::SecondaryPositionerIncrementSign);
		tag_to_elem.insert(0x00189519, &tags::SecondaryPositionerIncrementSign);

		ident_to_elem.insert("ApplicationName", &tags::ApplicationName);
		tag_to_elem.insert(0x00189524, &tags::ApplicationName);

		ident_to_elem.insert("ApplicationVersion", &tags::ApplicationVersion);
		tag_to_elem.insert(0x00189525, &tags::ApplicationVersion);

		ident_to_elem.insert("ApplicationManufacturer", &tags::ApplicationManufacturer);
		tag_to_elem.insert(0x00189526, &tags::ApplicationManufacturer);

		ident_to_elem.insert("AlgorithmType", &tags::AlgorithmType);
		tag_to_elem.insert(0x00189527, &tags::AlgorithmType);

		ident_to_elem.insert("AlgorithmDescription", &tags::AlgorithmDescription);
		tag_to_elem.insert(0x00189528, &tags::AlgorithmDescription);

		ident_to_elem.insert("XRay3DReconstructionSequence", &tags::XRay3DReconstructionSequence);
		tag_to_elem.insert(0x00189530, &tags::XRay3DReconstructionSequence);

		ident_to_elem.insert("ReconstructionDescription", &tags::ReconstructionDescription);
		tag_to_elem.insert(0x00189531, &tags::ReconstructionDescription);

		ident_to_elem.insert("PerProjectionAcquisitionSequence", &tags::PerProjectionAcquisitionSequence);
		tag_to_elem.insert(0x00189538, &tags::PerProjectionAcquisitionSequence);

		ident_to_elem.insert("DetectorPositionSequence", &tags::DetectorPositionSequence);
		tag_to_elem.insert(0x00189541, &tags::DetectorPositionSequence);

		ident_to_elem.insert("XRayAcquisitionDoseSequence", &tags::XRayAcquisitionDoseSequence);
		tag_to_elem.insert(0x00189542, &tags::XRayAcquisitionDoseSequence);

		ident_to_elem.insert("XRaySourceIsocenterPrimaryAngle", &tags::XRaySourceIsocenterPrimaryAngle);
		tag_to_elem.insert(0x00189543, &tags::XRaySourceIsocenterPrimaryAngle);

		ident_to_elem.insert("XRaySourceIsocenterSecondaryAngle", &tags::XRaySourceIsocenterSecondaryAngle);
		tag_to_elem.insert(0x00189544, &tags::XRaySourceIsocenterSecondaryAngle);

		ident_to_elem.insert("BreastSupportIsocenterPrimaryAngle", &tags::BreastSupportIsocenterPrimaryAngle);
		tag_to_elem.insert(0x00189545, &tags::BreastSupportIsocenterPrimaryAngle);

		ident_to_elem.insert("BreastSupportIsocenterSecondaryAngle", &tags::BreastSupportIsocenterSecondaryAngle);
		tag_to_elem.insert(0x00189546, &tags::BreastSupportIsocenterSecondaryAngle);

		ident_to_elem.insert("BreastSupportXPositionToIsocenter", &tags::BreastSupportXPositionToIsocenter);
		tag_to_elem.insert(0x00189547, &tags::BreastSupportXPositionToIsocenter);

		ident_to_elem.insert("BreastSupportYPositionToIsocenter", &tags::BreastSupportYPositionToIsocenter);
		tag_to_elem.insert(0x00189548, &tags::BreastSupportYPositionToIsocenter);

		ident_to_elem.insert("BreastSupportZPositionToIsocenter", &tags::BreastSupportZPositionToIsocenter);
		tag_to_elem.insert(0x00189549, &tags::BreastSupportZPositionToIsocenter);

		ident_to_elem.insert("DetectorIsocenterPrimaryAngle", &tags::DetectorIsocenterPrimaryAngle);
		tag_to_elem.insert(0x00189550, &tags::DetectorIsocenterPrimaryAngle);

		ident_to_elem.insert("DetectorIsocenterSecondaryAngle", &tags::DetectorIsocenterSecondaryAngle);
		tag_to_elem.insert(0x00189551, &tags::DetectorIsocenterSecondaryAngle);

		ident_to_elem.insert("DetectorXPositionToIsocenter", &tags::DetectorXPositionToIsocenter);
		tag_to_elem.insert(0x00189552, &tags::DetectorXPositionToIsocenter);

		ident_to_elem.insert("DetectorYPositionToIsocenter", &tags::DetectorYPositionToIsocenter);
		tag_to_elem.insert(0x00189553, &tags::DetectorYPositionToIsocenter);

		ident_to_elem.insert("DetectorZPositionToIsocenter", &tags::DetectorZPositionToIsocenter);
		tag_to_elem.insert(0x00189554, &tags::DetectorZPositionToIsocenter);

		ident_to_elem.insert("XRayGridSequence", &tags::XRayGridSequence);
		tag_to_elem.insert(0x00189555, &tags::XRayGridSequence);

		ident_to_elem.insert("XRayFilterSequence", &tags::XRayFilterSequence);
		tag_to_elem.insert(0x00189556, &tags::XRayFilterSequence);

		ident_to_elem.insert("DetectorActiveAreaTLHCPosition", &tags::DetectorActiveAreaTLHCPosition);
		tag_to_elem.insert(0x00189557, &tags::DetectorActiveAreaTLHCPosition);

		ident_to_elem.insert("DetectorActiveAreaOrientation", &tags::DetectorActiveAreaOrientation);
		tag_to_elem.insert(0x00189558, &tags::DetectorActiveAreaOrientation);

		ident_to_elem.insert("PositionerPrimaryAngleDirection", &tags::PositionerPrimaryAngleDirection);
		tag_to_elem.insert(0x00189559, &tags::PositionerPrimaryAngleDirection);

		ident_to_elem.insert("DiffusionBMatrixSequence", &tags::DiffusionBMatrixSequence);
		tag_to_elem.insert(0x00189601, &tags::DiffusionBMatrixSequence);

		ident_to_elem.insert("DiffusionBValueXX", &tags::DiffusionBValueXX);
		tag_to_elem.insert(0x00189602, &tags::DiffusionBValueXX);

		ident_to_elem.insert("DiffusionBValueXY", &tags::DiffusionBValueXY);
		tag_to_elem.insert(0x00189603, &tags::DiffusionBValueXY);

		ident_to_elem.insert("DiffusionBValueXZ", &tags::DiffusionBValueXZ);
		tag_to_elem.insert(0x00189604, &tags::DiffusionBValueXZ);

		ident_to_elem.insert("DiffusionBValueYY", &tags::DiffusionBValueYY);
		tag_to_elem.insert(0x00189605, &tags::DiffusionBValueYY);

		ident_to_elem.insert("DiffusionBValueYZ", &tags::DiffusionBValueYZ);
		tag_to_elem.insert(0x00189606, &tags::DiffusionBValueYZ);

		ident_to_elem.insert("DiffusionBValueZZ", &tags::DiffusionBValueZZ);
		tag_to_elem.insert(0x00189607, &tags::DiffusionBValueZZ);

		ident_to_elem.insert("FunctionalMRSequence", &tags::FunctionalMRSequence);
		tag_to_elem.insert(0x00189621, &tags::FunctionalMRSequence);

		ident_to_elem.insert("FunctionalSettlingPhaseFramesPresent", &tags::FunctionalSettlingPhaseFramesPresent);
		tag_to_elem.insert(0x00189622, &tags::FunctionalSettlingPhaseFramesPresent);

		ident_to_elem.insert("FunctionalSyncPulse", &tags::FunctionalSyncPulse);
		tag_to_elem.insert(0x00189623, &tags::FunctionalSyncPulse);

		ident_to_elem.insert("SettlingPhaseFrame", &tags::SettlingPhaseFrame);
		tag_to_elem.insert(0x00189624, &tags::SettlingPhaseFrame);

		ident_to_elem.insert("DecayCorrectionDateTime", &tags::DecayCorrectionDateTime);
		tag_to_elem.insert(0x00189701, &tags::DecayCorrectionDateTime);

		ident_to_elem.insert("StartDensityThreshold", &tags::StartDensityThreshold);
		tag_to_elem.insert(0x00189715, &tags::StartDensityThreshold);

		ident_to_elem.insert("StartRelativeDensityDifferenceThreshold", &tags::StartRelativeDensityDifferenceThreshold);
		tag_to_elem.insert(0x00189716, &tags::StartRelativeDensityDifferenceThreshold);

		ident_to_elem.insert("StartCardiacTriggerCountThreshold", &tags::StartCardiacTriggerCountThreshold);
		tag_to_elem.insert(0x00189717, &tags::StartCardiacTriggerCountThreshold);

		ident_to_elem.insert("StartRespiratoryTriggerCountThreshold", &tags::StartRespiratoryTriggerCountThreshold);
		tag_to_elem.insert(0x00189718, &tags::StartRespiratoryTriggerCountThreshold);

		ident_to_elem.insert("TerminationCountsThreshold", &tags::TerminationCountsThreshold);
		tag_to_elem.insert(0x00189719, &tags::TerminationCountsThreshold);

		ident_to_elem.insert("TerminationDensityThreshold", &tags::TerminationDensityThreshold);
		tag_to_elem.insert(0x00189720, &tags::TerminationDensityThreshold);

		ident_to_elem.insert("TerminationRelativeDensityThreshold", &tags::TerminationRelativeDensityThreshold);
		tag_to_elem.insert(0x00189721, &tags::TerminationRelativeDensityThreshold);

		ident_to_elem.insert("TerminationTimeThreshold", &tags::TerminationTimeThreshold);
		tag_to_elem.insert(0x00189722, &tags::TerminationTimeThreshold);

		ident_to_elem.insert("TerminationCardiacTriggerCountThreshold", &tags::TerminationCardiacTriggerCountThreshold);
		tag_to_elem.insert(0x00189723, &tags::TerminationCardiacTriggerCountThreshold);

		ident_to_elem.insert("TerminationRespiratoryTriggerCountThreshold", &tags::TerminationRespiratoryTriggerCountThreshold);
		tag_to_elem.insert(0x00189724, &tags::TerminationRespiratoryTriggerCountThreshold);

		ident_to_elem.insert("DetectorGeometry", &tags::DetectorGeometry);
		tag_to_elem.insert(0x00189725, &tags::DetectorGeometry);

		ident_to_elem.insert("TransverseDetectorSeparation", &tags::TransverseDetectorSeparation);
		tag_to_elem.insert(0x00189726, &tags::TransverseDetectorSeparation);

		ident_to_elem.insert("AxialDetectorDimension", &tags::AxialDetectorDimension);
		tag_to_elem.insert(0x00189727, &tags::AxialDetectorDimension);

		ident_to_elem.insert("RadiopharmaceuticalAgentNumber", &tags::RadiopharmaceuticalAgentNumber);
		tag_to_elem.insert(0x00189729, &tags::RadiopharmaceuticalAgentNumber);

		ident_to_elem.insert("PETFrameAcquisitionSequence", &tags::PETFrameAcquisitionSequence);
		tag_to_elem.insert(0x00189732, &tags::PETFrameAcquisitionSequence);

		ident_to_elem.insert("PETDetectorMotionDetailsSequence", &tags::PETDetectorMotionDetailsSequence);
		tag_to_elem.insert(0x00189733, &tags::PETDetectorMotionDetailsSequence);

		ident_to_elem.insert("PETTableDynamicsSequence", &tags::PETTableDynamicsSequence);
		tag_to_elem.insert(0x00189734, &tags::PETTableDynamicsSequence);

		ident_to_elem.insert("PETPositionSequence", &tags::PETPositionSequence);
		tag_to_elem.insert(0x00189735, &tags::PETPositionSequence);

		ident_to_elem.insert("PETFrameCorrectionFactorsSequence", &tags::PETFrameCorrectionFactorsSequence);
		tag_to_elem.insert(0x00189736, &tags::PETFrameCorrectionFactorsSequence);

		ident_to_elem.insert("RadiopharmaceuticalUsageSequence", &tags::RadiopharmaceuticalUsageSequence);
		tag_to_elem.insert(0x00189737, &tags::RadiopharmaceuticalUsageSequence);

		ident_to_elem.insert("AttenuationCorrectionSource", &tags::AttenuationCorrectionSource);
		tag_to_elem.insert(0x00189738, &tags::AttenuationCorrectionSource);

		ident_to_elem.insert("NumberOfIterations", &tags::NumberOfIterations);
		tag_to_elem.insert(0x00189739, &tags::NumberOfIterations);

		ident_to_elem.insert("NumberOfSubsets", &tags::NumberOfSubsets);
		tag_to_elem.insert(0x00189740, &tags::NumberOfSubsets);

		ident_to_elem.insert("PETReconstructionSequence", &tags::PETReconstructionSequence);
		tag_to_elem.insert(0x00189749, &tags::PETReconstructionSequence);

		ident_to_elem.insert("PETFrameTypeSequence", &tags::PETFrameTypeSequence);
		tag_to_elem.insert(0x00189751, &tags::PETFrameTypeSequence);

		ident_to_elem.insert("TimeOfFlightInformationUsed", &tags::TimeOfFlightInformationUsed);
		tag_to_elem.insert(0x00189755, &tags::TimeOfFlightInformationUsed);

		ident_to_elem.insert("ReconstructionType", &tags::ReconstructionType);
		tag_to_elem.insert(0x00189756, &tags::ReconstructionType);

		ident_to_elem.insert("DecayCorrected", &tags::DecayCorrected);
		tag_to_elem.insert(0x00189758, &tags::DecayCorrected);

		ident_to_elem.insert("AttenuationCorrected", &tags::AttenuationCorrected);
		tag_to_elem.insert(0x00189759, &tags::AttenuationCorrected);

		ident_to_elem.insert("ScatterCorrected", &tags::ScatterCorrected);
		tag_to_elem.insert(0x00189760, &tags::ScatterCorrected);

		ident_to_elem.insert("DeadTimeCorrected", &tags::DeadTimeCorrected);
		tag_to_elem.insert(0x00189761, &tags::DeadTimeCorrected);

		ident_to_elem.insert("GantryMotionCorrected", &tags::GantryMotionCorrected);
		tag_to_elem.insert(0x00189762, &tags::GantryMotionCorrected);

		ident_to_elem.insert("PatientMotionCorrected", &tags::PatientMotionCorrected);
		tag_to_elem.insert(0x00189763, &tags::PatientMotionCorrected);

		ident_to_elem.insert("CountLossNormalizationCorrected", &tags::CountLossNormalizationCorrected);
		tag_to_elem.insert(0x00189764, &tags::CountLossNormalizationCorrected);

		ident_to_elem.insert("RandomsCorrected", &tags::RandomsCorrected);
		tag_to_elem.insert(0x00189765, &tags::RandomsCorrected);

		ident_to_elem.insert("NonUniformRadialSamplingCorrected", &tags::NonUniformRadialSamplingCorrected);
		tag_to_elem.insert(0x00189766, &tags::NonUniformRadialSamplingCorrected);

		ident_to_elem.insert("SensitivityCalibrated", &tags::SensitivityCalibrated);
		tag_to_elem.insert(0x00189767, &tags::SensitivityCalibrated);

		ident_to_elem.insert("DetectorNormalizationCorrection", &tags::DetectorNormalizationCorrection);
		tag_to_elem.insert(0x00189768, &tags::DetectorNormalizationCorrection);

		ident_to_elem.insert("IterativeReconstructionMethod", &tags::IterativeReconstructionMethod);
		tag_to_elem.insert(0x00189769, &tags::IterativeReconstructionMethod);

		ident_to_elem.insert("AttenuationCorrectionTemporalRelationship", &tags::AttenuationCorrectionTemporalRelationship);
		tag_to_elem.insert(0x00189770, &tags::AttenuationCorrectionTemporalRelationship);

		ident_to_elem.insert("PatientPhysiologicalStateSequence", &tags::PatientPhysiologicalStateSequence);
		tag_to_elem.insert(0x00189771, &tags::PatientPhysiologicalStateSequence);

		ident_to_elem.insert("PatientPhysiologicalStateCodeSequence", &tags::PatientPhysiologicalStateCodeSequence);
		tag_to_elem.insert(0x00189772, &tags::PatientPhysiologicalStateCodeSequence);

		ident_to_elem.insert("DepthsOfFocus", &tags::DepthsOfFocus);
		tag_to_elem.insert(0x00189801, &tags::DepthsOfFocus);

		ident_to_elem.insert("ExcludedIntervalsSequence", &tags::ExcludedIntervalsSequence);
		tag_to_elem.insert(0x00189803, &tags::ExcludedIntervalsSequence);

		ident_to_elem.insert("ExclusionStartDateTime", &tags::ExclusionStartDateTime);
		tag_to_elem.insert(0x00189804, &tags::ExclusionStartDateTime);

		ident_to_elem.insert("ExclusionDuration", &tags::ExclusionDuration);
		tag_to_elem.insert(0x00189805, &tags::ExclusionDuration);

		ident_to_elem.insert("USImageDescriptionSequence", &tags::USImageDescriptionSequence);
		tag_to_elem.insert(0x00189806, &tags::USImageDescriptionSequence);

		ident_to_elem.insert("ImageDataTypeSequence", &tags::ImageDataTypeSequence);
		tag_to_elem.insert(0x00189807, &tags::ImageDataTypeSequence);

		ident_to_elem.insert("DataType", &tags::DataType);
		tag_to_elem.insert(0x00189808, &tags::DataType);

		ident_to_elem.insert("TransducerScanPatternCodeSequence", &tags::TransducerScanPatternCodeSequence);
		tag_to_elem.insert(0x00189809, &tags::TransducerScanPatternCodeSequence);

		ident_to_elem.insert("AliasedDataType", &tags::AliasedDataType);
		tag_to_elem.insert(0x0018980B, &tags::AliasedDataType);

		ident_to_elem.insert("PositionMeasuringDeviceUsed", &tags::PositionMeasuringDeviceUsed);
		tag_to_elem.insert(0x0018980C, &tags::PositionMeasuringDeviceUsed);

		ident_to_elem.insert("TransducerGeometryCodeSequence", &tags::TransducerGeometryCodeSequence);
		tag_to_elem.insert(0x0018980D, &tags::TransducerGeometryCodeSequence);

		ident_to_elem.insert("TransducerBeamSteeringCodeSequence", &tags::TransducerBeamSteeringCodeSequence);
		tag_to_elem.insert(0x0018980E, &tags::TransducerBeamSteeringCodeSequence);

		ident_to_elem.insert("TransducerApplicationCodeSequence", &tags::TransducerApplicationCodeSequence);
		tag_to_elem.insert(0x0018980F, &tags::TransducerApplicationCodeSequence);

		ident_to_elem.insert("ZeroVelocityPixelValue", &tags::ZeroVelocityPixelValue);
		tag_to_elem.insert(0x00189810, &tags::ZeroVelocityPixelValue);

		ident_to_elem.insert("ReferenceLocationLabel", &tags::ReferenceLocationLabel);
		tag_to_elem.insert(0x00189900, &tags::ReferenceLocationLabel);

		ident_to_elem.insert("ReferenceLocationDescription", &tags::ReferenceLocationDescription);
		tag_to_elem.insert(0x00189901, &tags::ReferenceLocationDescription);

		ident_to_elem.insert("ReferenceBasisCodeSequence", &tags::ReferenceBasisCodeSequence);
		tag_to_elem.insert(0x00189902, &tags::ReferenceBasisCodeSequence);

		ident_to_elem.insert("ReferenceGeometryCodeSequence", &tags::ReferenceGeometryCodeSequence);
		tag_to_elem.insert(0x00189903, &tags::ReferenceGeometryCodeSequence);

		ident_to_elem.insert("OffsetDistance", &tags::OffsetDistance);
		tag_to_elem.insert(0x00189904, &tags::OffsetDistance);

		ident_to_elem.insert("OffsetDirection", &tags::OffsetDirection);
		tag_to_elem.insert(0x00189905, &tags::OffsetDirection);

		ident_to_elem.insert("PotentialScheduledProtocolCodeSequence", &tags::PotentialScheduledProtocolCodeSequence);
		tag_to_elem.insert(0x00189906, &tags::PotentialScheduledProtocolCodeSequence);

		ident_to_elem.insert("PotentialRequestedProcedureCodeSequence", &tags::PotentialRequestedProcedureCodeSequence);
		tag_to_elem.insert(0x00189907, &tags::PotentialRequestedProcedureCodeSequence);

		ident_to_elem.insert("PotentialReasonsForProcedure", &tags::PotentialReasonsForProcedure);
		tag_to_elem.insert(0x00189908, &tags::PotentialReasonsForProcedure);

		ident_to_elem.insert("PotentialReasonsForProcedureCodeSequence", &tags::PotentialReasonsForProcedureCodeSequence);
		tag_to_elem.insert(0x00189909, &tags::PotentialReasonsForProcedureCodeSequence);

		ident_to_elem.insert("PotentialDiagnosticTasks", &tags::PotentialDiagnosticTasks);
		tag_to_elem.insert(0x0018990A, &tags::PotentialDiagnosticTasks);

		ident_to_elem.insert("ContraindicationsCodeSequence", &tags::ContraindicationsCodeSequence);
		tag_to_elem.insert(0x0018990B, &tags::ContraindicationsCodeSequence);

		ident_to_elem.insert("ReferencedDefinedProtocolSequence", &tags::ReferencedDefinedProtocolSequence);
		tag_to_elem.insert(0x0018990C, &tags::ReferencedDefinedProtocolSequence);

		ident_to_elem.insert("ReferencedPerformedProtocolSequence", &tags::ReferencedPerformedProtocolSequence);
		tag_to_elem.insert(0x0018990D, &tags::ReferencedPerformedProtocolSequence);

		ident_to_elem.insert("PredecessorProtocolSequence", &tags::PredecessorProtocolSequence);
		tag_to_elem.insert(0x0018990E, &tags::PredecessorProtocolSequence);

		ident_to_elem.insert("ProtocolPlanningInformation", &tags::ProtocolPlanningInformation);
		tag_to_elem.insert(0x0018990F, &tags::ProtocolPlanningInformation);

		ident_to_elem.insert("ProtocolDesignRationale", &tags::ProtocolDesignRationale);
		tag_to_elem.insert(0x00189910, &tags::ProtocolDesignRationale);

		ident_to_elem.insert("PatientSpecificationSequence", &tags::PatientSpecificationSequence);
		tag_to_elem.insert(0x00189911, &tags::PatientSpecificationSequence);

		ident_to_elem.insert("ModelSpecificationSequence", &tags::ModelSpecificationSequence);
		tag_to_elem.insert(0x00189912, &tags::ModelSpecificationSequence);

		ident_to_elem.insert("ParametersSpecificationSequence", &tags::ParametersSpecificationSequence);
		tag_to_elem.insert(0x00189913, &tags::ParametersSpecificationSequence);

		ident_to_elem.insert("InstructionSequence", &tags::InstructionSequence);
		tag_to_elem.insert(0x00189914, &tags::InstructionSequence);

		ident_to_elem.insert("InstructionIndex", &tags::InstructionIndex);
		tag_to_elem.insert(0x00189915, &tags::InstructionIndex);

		ident_to_elem.insert("InstructionText", &tags::InstructionText);
		tag_to_elem.insert(0x00189916, &tags::InstructionText);

		ident_to_elem.insert("InstructionDescription", &tags::InstructionDescription);
		tag_to_elem.insert(0x00189917, &tags::InstructionDescription);

		ident_to_elem.insert("InstructionPerformedFlag", &tags::InstructionPerformedFlag);
		tag_to_elem.insert(0x00189918, &tags::InstructionPerformedFlag);

		ident_to_elem.insert("InstructionPerformedDateTime", &tags::InstructionPerformedDateTime);
		tag_to_elem.insert(0x00189919, &tags::InstructionPerformedDateTime);

		ident_to_elem.insert("InstructionPerformanceComment", &tags::InstructionPerformanceComment);
		tag_to_elem.insert(0x0018991A, &tags::InstructionPerformanceComment);

		ident_to_elem.insert("PatientPositioningInstructionSequence", &tags::PatientPositioningInstructionSequence);
		tag_to_elem.insert(0x0018991B, &tags::PatientPositioningInstructionSequence);

		ident_to_elem.insert("PositioningMethodCodeSequence", &tags::PositioningMethodCodeSequence);
		tag_to_elem.insert(0x0018991C, &tags::PositioningMethodCodeSequence);

		ident_to_elem.insert("PositioningLandmarkSequence", &tags::PositioningLandmarkSequence);
		tag_to_elem.insert(0x0018991D, &tags::PositioningLandmarkSequence);

		ident_to_elem.insert("TargetFrameOfReferenceUID", &tags::TargetFrameOfReferenceUID);
		tag_to_elem.insert(0x0018991E, &tags::TargetFrameOfReferenceUID);

		ident_to_elem.insert("AcquisitionProtocolElementSpecificationSequence", &tags::AcquisitionProtocolElementSpecificationSequence);
		tag_to_elem.insert(0x0018991F, &tags::AcquisitionProtocolElementSpecificationSequence);

		ident_to_elem.insert("AcquisitionProtocolElementSequence", &tags::AcquisitionProtocolElementSequence);
		tag_to_elem.insert(0x00189920, &tags::AcquisitionProtocolElementSequence);

		ident_to_elem.insert("ProtocolElementNumber", &tags::ProtocolElementNumber);
		tag_to_elem.insert(0x00189921, &tags::ProtocolElementNumber);

		ident_to_elem.insert("ProtocolElementName", &tags::ProtocolElementName);
		tag_to_elem.insert(0x00189922, &tags::ProtocolElementName);

		ident_to_elem.insert("ProtocolElementCharacteristicsSummary", &tags::ProtocolElementCharacteristicsSummary);
		tag_to_elem.insert(0x00189923, &tags::ProtocolElementCharacteristicsSummary);

		ident_to_elem.insert("ProtocolElementPurpose", &tags::ProtocolElementPurpose);
		tag_to_elem.insert(0x00189924, &tags::ProtocolElementPurpose);

		ident_to_elem.insert("AcquisitionMotion", &tags::AcquisitionMotion);
		tag_to_elem.insert(0x00189930, &tags::AcquisitionMotion);

		ident_to_elem.insert("AcquisitionStartLocationSequence", &tags::AcquisitionStartLocationSequence);
		tag_to_elem.insert(0x00189931, &tags::AcquisitionStartLocationSequence);

		ident_to_elem.insert("AcquisitionEndLocationSequence", &tags::AcquisitionEndLocationSequence);
		tag_to_elem.insert(0x00189932, &tags::AcquisitionEndLocationSequence);

		ident_to_elem.insert("ReconstructionProtocolElementSpecificationSequence", &tags::ReconstructionProtocolElementSpecificationSequence);
		tag_to_elem.insert(0x00189933, &tags::ReconstructionProtocolElementSpecificationSequence);

		ident_to_elem.insert("ReconstructionProtocolElementSequence", &tags::ReconstructionProtocolElementSequence);
		tag_to_elem.insert(0x00189934, &tags::ReconstructionProtocolElementSequence);

		ident_to_elem.insert("StorageProtocolElementSpecificationSequence", &tags::StorageProtocolElementSpecificationSequence);
		tag_to_elem.insert(0x00189935, &tags::StorageProtocolElementSpecificationSequence);

		ident_to_elem.insert("StorageProtocolElementSequence", &tags::StorageProtocolElementSequence);
		tag_to_elem.insert(0x00189936, &tags::StorageProtocolElementSequence);

		ident_to_elem.insert("RequestedSeriesDescription", &tags::RequestedSeriesDescription);
		tag_to_elem.insert(0x00189937, &tags::RequestedSeriesDescription);

		ident_to_elem.insert("SourceAcquisitionProtocolElementNumber", &tags::SourceAcquisitionProtocolElementNumber);
		tag_to_elem.insert(0x00189938, &tags::SourceAcquisitionProtocolElementNumber);

		ident_to_elem.insert("SourceAcquisitionBeamNumber", &tags::SourceAcquisitionBeamNumber);
		tag_to_elem.insert(0x00189939, &tags::SourceAcquisitionBeamNumber);

		ident_to_elem.insert("SourceReconstructionProtocolElementNumber", &tags::SourceReconstructionProtocolElementNumber);
		tag_to_elem.insert(0x0018993A, &tags::SourceReconstructionProtocolElementNumber);

		ident_to_elem.insert("ReconstructionStartLocationSequence", &tags::ReconstructionStartLocationSequence);
		tag_to_elem.insert(0x0018993B, &tags::ReconstructionStartLocationSequence);

		ident_to_elem.insert("ReconstructionEndLocationSequence", &tags::ReconstructionEndLocationSequence);
		tag_to_elem.insert(0x0018993C, &tags::ReconstructionEndLocationSequence);

		ident_to_elem.insert("ReconstructionAlgorithmSequence", &tags::ReconstructionAlgorithmSequence);
		tag_to_elem.insert(0x0018993D, &tags::ReconstructionAlgorithmSequence);

		ident_to_elem.insert("ReconstructionTargetCenterLocationSequence", &tags::ReconstructionTargetCenterLocationSequence);
		tag_to_elem.insert(0x0018993E, &tags::ReconstructionTargetCenterLocationSequence);

		ident_to_elem.insert("ImageFilterDescription", &tags::ImageFilterDescription);
		tag_to_elem.insert(0x00189941, &tags::ImageFilterDescription);

		ident_to_elem.insert("CTDIvolNotificationTrigger", &tags::CTDIvolNotificationTrigger);
		tag_to_elem.insert(0x00189942, &tags::CTDIvolNotificationTrigger);

		ident_to_elem.insert("DLPNotificationTrigger", &tags::DLPNotificationTrigger);
		tag_to_elem.insert(0x00189943, &tags::DLPNotificationTrigger);

		ident_to_elem.insert("AutoKVPSelectionType", &tags::AutoKVPSelectionType);
		tag_to_elem.insert(0x00189944, &tags::AutoKVPSelectionType);

		ident_to_elem.insert("AutoKVPUpperBound", &tags::AutoKVPUpperBound);
		tag_to_elem.insert(0x00189945, &tags::AutoKVPUpperBound);

		ident_to_elem.insert("AutoKVPLowerBound", &tags::AutoKVPLowerBound);
		tag_to_elem.insert(0x00189946, &tags::AutoKVPLowerBound);

		ident_to_elem.insert("ProtocolDefinedPatientPosition", &tags::ProtocolDefinedPatientPosition);
		tag_to_elem.insert(0x00189947, &tags::ProtocolDefinedPatientPosition);

		ident_to_elem.insert("ContributingEquipmentSequence", &tags::ContributingEquipmentSequence);
		tag_to_elem.insert(0x0018A001, &tags::ContributingEquipmentSequence);

		ident_to_elem.insert("ContributionDateTime", &tags::ContributionDateTime);
		tag_to_elem.insert(0x0018A002, &tags::ContributionDateTime);

		ident_to_elem.insert("ContributionDescription", &tags::ContributionDescription);
		tag_to_elem.insert(0x0018A003, &tags::ContributionDescription);

		ident_to_elem.insert("StudyInstanceUID", &tags::StudyInstanceUID);
		tag_to_elem.insert(0x0020000D, &tags::StudyInstanceUID);

		ident_to_elem.insert("SeriesInstanceUID", &tags::SeriesInstanceUID);
		tag_to_elem.insert(0x0020000E, &tags::SeriesInstanceUID);

		ident_to_elem.insert("StudyID", &tags::StudyID);
		tag_to_elem.insert(0x00200010, &tags::StudyID);

		ident_to_elem.insert("SeriesNumber", &tags::SeriesNumber);
		tag_to_elem.insert(0x00200011, &tags::SeriesNumber);

		ident_to_elem.insert("AcquisitionNumber", &tags::AcquisitionNumber);
		tag_to_elem.insert(0x00200012, &tags::AcquisitionNumber);

		ident_to_elem.insert("InstanceNumber", &tags::InstanceNumber);
		tag_to_elem.insert(0x00200013, &tags::InstanceNumber);

		ident_to_elem.insert("IsotopeNumber", &tags::IsotopeNumber);
		tag_to_elem.insert(0x00200014, &tags::IsotopeNumber);

		ident_to_elem.insert("PhaseNumber", &tags::PhaseNumber);
		tag_to_elem.insert(0x00200015, &tags::PhaseNumber);

		ident_to_elem.insert("IntervalNumber", &tags::IntervalNumber);
		tag_to_elem.insert(0x00200016, &tags::IntervalNumber);

		ident_to_elem.insert("TimeSlotNumber", &tags::TimeSlotNumber);
		tag_to_elem.insert(0x00200017, &tags::TimeSlotNumber);

		ident_to_elem.insert("AngleNumber", &tags::AngleNumber);
		tag_to_elem.insert(0x00200018, &tags::AngleNumber);

		ident_to_elem.insert("ItemNumber", &tags::ItemNumber);
		tag_to_elem.insert(0x00200019, &tags::ItemNumber);

		ident_to_elem.insert("PatientOrientation", &tags::PatientOrientation);
		tag_to_elem.insert(0x00200020, &tags::PatientOrientation);

		ident_to_elem.insert("OverlayNumber", &tags::OverlayNumber);
		tag_to_elem.insert(0x00200022, &tags::OverlayNumber);

		ident_to_elem.insert("CurveNumber", &tags::CurveNumber);
		tag_to_elem.insert(0x00200024, &tags::CurveNumber);

		ident_to_elem.insert("LUTNumber", &tags::LUTNumber);
		tag_to_elem.insert(0x00200026, &tags::LUTNumber);

		ident_to_elem.insert("ImagePosition", &tags::ImagePosition);
		tag_to_elem.insert(0x00200030, &tags::ImagePosition);

		ident_to_elem.insert("ImagePositionPatient", &tags::ImagePositionPatient);
		tag_to_elem.insert(0x00200032, &tags::ImagePositionPatient);

		ident_to_elem.insert("ImageOrientation", &tags::ImageOrientation);
		tag_to_elem.insert(0x00200035, &tags::ImageOrientation);

		ident_to_elem.insert("ImageOrientationPatient", &tags::ImageOrientationPatient);
		tag_to_elem.insert(0x00200037, &tags::ImageOrientationPatient);

		ident_to_elem.insert("Location", &tags::Location);
		tag_to_elem.insert(0x00200050, &tags::Location);

		ident_to_elem.insert("FrameOfReferenceUID", &tags::FrameOfReferenceUID);
		tag_to_elem.insert(0x00200052, &tags::FrameOfReferenceUID);

		ident_to_elem.insert("Laterality", &tags::Laterality);
		tag_to_elem.insert(0x00200060, &tags::Laterality);

		ident_to_elem.insert("ImageLaterality", &tags::ImageLaterality);
		tag_to_elem.insert(0x00200062, &tags::ImageLaterality);

		ident_to_elem.insert("ImageGeometryType", &tags::ImageGeometryType);
		tag_to_elem.insert(0x00200070, &tags::ImageGeometryType);

		ident_to_elem.insert("MaskingImage", &tags::MaskingImage);
		tag_to_elem.insert(0x00200080, &tags::MaskingImage);

		ident_to_elem.insert("ReportNumber", &tags::ReportNumber);
		tag_to_elem.insert(0x002000AA, &tags::ReportNumber);

		ident_to_elem.insert("TemporalPositionIdentifier", &tags::TemporalPositionIdentifier);
		tag_to_elem.insert(0x00200100, &tags::TemporalPositionIdentifier);

		ident_to_elem.insert("NumberOfTemporalPositions", &tags::NumberOfTemporalPositions);
		tag_to_elem.insert(0x00200105, &tags::NumberOfTemporalPositions);

		ident_to_elem.insert("TemporalResolution", &tags::TemporalResolution);
		tag_to_elem.insert(0x00200110, &tags::TemporalResolution);

		ident_to_elem.insert("SynchronizationFrameOfReferenceUID", &tags::SynchronizationFrameOfReferenceUID);
		tag_to_elem.insert(0x00200200, &tags::SynchronizationFrameOfReferenceUID);

		ident_to_elem.insert("SOPInstanceUIDOfConcatenationSource", &tags::SOPInstanceUIDOfConcatenationSource);
		tag_to_elem.insert(0x00200242, &tags::SOPInstanceUIDOfConcatenationSource);

		ident_to_elem.insert("SeriesInStudy", &tags::SeriesInStudy);
		tag_to_elem.insert(0x00201000, &tags::SeriesInStudy);

		ident_to_elem.insert("AcquisitionsInSeries", &tags::AcquisitionsInSeries);
		tag_to_elem.insert(0x00201001, &tags::AcquisitionsInSeries);

		ident_to_elem.insert("ImagesInAcquisition", &tags::ImagesInAcquisition);
		tag_to_elem.insert(0x00201002, &tags::ImagesInAcquisition);

		ident_to_elem.insert("ImagesInSeries", &tags::ImagesInSeries);
		tag_to_elem.insert(0x00201003, &tags::ImagesInSeries);

		ident_to_elem.insert("AcquisitionsInStudy", &tags::AcquisitionsInStudy);
		tag_to_elem.insert(0x00201004, &tags::AcquisitionsInStudy);

		ident_to_elem.insert("ImagesInStudy", &tags::ImagesInStudy);
		tag_to_elem.insert(0x00201005, &tags::ImagesInStudy);

		ident_to_elem.insert("Reference", &tags::Reference);
		tag_to_elem.insert(0x00201020, &tags::Reference);

		ident_to_elem.insert("TargetPositionReferenceIndicator", &tags::TargetPositionReferenceIndicator);
		tag_to_elem.insert(0x0020103F, &tags::TargetPositionReferenceIndicator);

		ident_to_elem.insert("PositionReferenceIndicator", &tags::PositionReferenceIndicator);
		tag_to_elem.insert(0x00201040, &tags::PositionReferenceIndicator);

		ident_to_elem.insert("SliceLocation", &tags::SliceLocation);
		tag_to_elem.insert(0x00201041, &tags::SliceLocation);

		ident_to_elem.insert("OtherStudyNumbers", &tags::OtherStudyNumbers);
		tag_to_elem.insert(0x00201070, &tags::OtherStudyNumbers);

		ident_to_elem.insert("NumberOfPatientRelatedStudies", &tags::NumberOfPatientRelatedStudies);
		tag_to_elem.insert(0x00201200, &tags::NumberOfPatientRelatedStudies);

		ident_to_elem.insert("NumberOfPatientRelatedSeries", &tags::NumberOfPatientRelatedSeries);
		tag_to_elem.insert(0x00201202, &tags::NumberOfPatientRelatedSeries);

		ident_to_elem.insert("NumberOfPatientRelatedInstances", &tags::NumberOfPatientRelatedInstances);
		tag_to_elem.insert(0x00201204, &tags::NumberOfPatientRelatedInstances);

		ident_to_elem.insert("NumberOfStudyRelatedSeries", &tags::NumberOfStudyRelatedSeries);
		tag_to_elem.insert(0x00201206, &tags::NumberOfStudyRelatedSeries);

		ident_to_elem.insert("NumberOfStudyRelatedInstances", &tags::NumberOfStudyRelatedInstances);
		tag_to_elem.insert(0x00201208, &tags::NumberOfStudyRelatedInstances);

		ident_to_elem.insert("NumberOfSeriesRelatedInstances", &tags::NumberOfSeriesRelatedInstances);
		tag_to_elem.insert(0x00201209, &tags::NumberOfSeriesRelatedInstances);

		ident_to_elem.insert("ModifyingDeviceID", &tags::ModifyingDeviceID);
		tag_to_elem.insert(0x00203401, &tags::ModifyingDeviceID);

		ident_to_elem.insert("ModifiedImageID", &tags::ModifiedImageID);
		tag_to_elem.insert(0x00203402, &tags::ModifiedImageID);

		ident_to_elem.insert("ModifiedImageDate", &tags::ModifiedImageDate);
		tag_to_elem.insert(0x00203403, &tags::ModifiedImageDate);

		ident_to_elem.insert("ModifyingDeviceManufacturer", &tags::ModifyingDeviceManufacturer);
		tag_to_elem.insert(0x00203404, &tags::ModifyingDeviceManufacturer);

		ident_to_elem.insert("ModifiedImageTime", &tags::ModifiedImageTime);
		tag_to_elem.insert(0x00203405, &tags::ModifiedImageTime);

		ident_to_elem.insert("ModifiedImageDescription", &tags::ModifiedImageDescription);
		tag_to_elem.insert(0x00203406, &tags::ModifiedImageDescription);

		ident_to_elem.insert("ImageComments", &tags::ImageComments);
		tag_to_elem.insert(0x00204000, &tags::ImageComments);

		ident_to_elem.insert("OriginalImageIdentification", &tags::OriginalImageIdentification);
		tag_to_elem.insert(0x00205000, &tags::OriginalImageIdentification);

		ident_to_elem.insert("OriginalImageIdentificationNomenclature", &tags::OriginalImageIdentificationNomenclature);
		tag_to_elem.insert(0x00205002, &tags::OriginalImageIdentificationNomenclature);

		ident_to_elem.insert("StackID", &tags::StackID);
		tag_to_elem.insert(0x00209056, &tags::StackID);

		ident_to_elem.insert("InStackPositionNumber", &tags::InStackPositionNumber);
		tag_to_elem.insert(0x00209057, &tags::InStackPositionNumber);

		ident_to_elem.insert("FrameAnatomySequence", &tags::FrameAnatomySequence);
		tag_to_elem.insert(0x00209071, &tags::FrameAnatomySequence);

		ident_to_elem.insert("FrameLaterality", &tags::FrameLaterality);
		tag_to_elem.insert(0x00209072, &tags::FrameLaterality);

		ident_to_elem.insert("FrameContentSequence", &tags::FrameContentSequence);
		tag_to_elem.insert(0x00209111, &tags::FrameContentSequence);

		ident_to_elem.insert("PlanePositionSequence", &tags::PlanePositionSequence);
		tag_to_elem.insert(0x00209113, &tags::PlanePositionSequence);

		ident_to_elem.insert("PlaneOrientationSequence", &tags::PlaneOrientationSequence);
		tag_to_elem.insert(0x00209116, &tags::PlaneOrientationSequence);

		ident_to_elem.insert("TemporalPositionIndex", &tags::TemporalPositionIndex);
		tag_to_elem.insert(0x00209128, &tags::TemporalPositionIndex);

		ident_to_elem.insert("NominalCardiacTriggerDelayTime", &tags::NominalCardiacTriggerDelayTime);
		tag_to_elem.insert(0x00209153, &tags::NominalCardiacTriggerDelayTime);

		ident_to_elem.insert("NominalCardiacTriggerTimePriorToRPeak", &tags::NominalCardiacTriggerTimePriorToRPeak);
		tag_to_elem.insert(0x00209154, &tags::NominalCardiacTriggerTimePriorToRPeak);

		ident_to_elem.insert("ActualCardiacTriggerTimePriorToRPeak", &tags::ActualCardiacTriggerTimePriorToRPeak);
		tag_to_elem.insert(0x00209155, &tags::ActualCardiacTriggerTimePriorToRPeak);

		ident_to_elem.insert("FrameAcquisitionNumber", &tags::FrameAcquisitionNumber);
		tag_to_elem.insert(0x00209156, &tags::FrameAcquisitionNumber);

		ident_to_elem.insert("DimensionIndexValues", &tags::DimensionIndexValues);
		tag_to_elem.insert(0x00209157, &tags::DimensionIndexValues);

		ident_to_elem.insert("FrameComments", &tags::FrameComments);
		tag_to_elem.insert(0x00209158, &tags::FrameComments);

		ident_to_elem.insert("ConcatenationUID", &tags::ConcatenationUID);
		tag_to_elem.insert(0x00209161, &tags::ConcatenationUID);

		ident_to_elem.insert("InConcatenationNumber", &tags::InConcatenationNumber);
		tag_to_elem.insert(0x00209162, &tags::InConcatenationNumber);

		ident_to_elem.insert("InConcatenationTotalNumber", &tags::InConcatenationTotalNumber);
		tag_to_elem.insert(0x00209163, &tags::InConcatenationTotalNumber);

		ident_to_elem.insert("DimensionOrganizationUID", &tags::DimensionOrganizationUID);
		tag_to_elem.insert(0x00209164, &tags::DimensionOrganizationUID);

		ident_to_elem.insert("DimensionIndexPointer", &tags::DimensionIndexPointer);
		tag_to_elem.insert(0x00209165, &tags::DimensionIndexPointer);

		ident_to_elem.insert("FunctionalGroupPointer", &tags::FunctionalGroupPointer);
		tag_to_elem.insert(0x00209167, &tags::FunctionalGroupPointer);

		ident_to_elem.insert("UnassignedSharedConvertedAttributesSequence", &tags::UnassignedSharedConvertedAttributesSequence);
		tag_to_elem.insert(0x00209170, &tags::UnassignedSharedConvertedAttributesSequence);

		ident_to_elem.insert("UnassignedPerFrameConvertedAttributesSequence", &tags::UnassignedPerFrameConvertedAttributesSequence);
		tag_to_elem.insert(0x00209171, &tags::UnassignedPerFrameConvertedAttributesSequence);

		ident_to_elem.insert("ConversionSourceAttributesSequence", &tags::ConversionSourceAttributesSequence);
		tag_to_elem.insert(0x00209172, &tags::ConversionSourceAttributesSequence);

		ident_to_elem.insert("DimensionIndexPrivateCreator", &tags::DimensionIndexPrivateCreator);
		tag_to_elem.insert(0x00209213, &tags::DimensionIndexPrivateCreator);

		ident_to_elem.insert("DimensionOrganizationSequence", &tags::DimensionOrganizationSequence);
		tag_to_elem.insert(0x00209221, &tags::DimensionOrganizationSequence);

		ident_to_elem.insert("DimensionIndexSequence", &tags::DimensionIndexSequence);
		tag_to_elem.insert(0x00209222, &tags::DimensionIndexSequence);

		ident_to_elem.insert("ConcatenationFrameOffsetNumber", &tags::ConcatenationFrameOffsetNumber);
		tag_to_elem.insert(0x00209228, &tags::ConcatenationFrameOffsetNumber);

		ident_to_elem.insert("FunctionalGroupPrivateCreator", &tags::FunctionalGroupPrivateCreator);
		tag_to_elem.insert(0x00209238, &tags::FunctionalGroupPrivateCreator);

		ident_to_elem.insert("NominalPercentageOfCardiacPhase", &tags::NominalPercentageOfCardiacPhase);
		tag_to_elem.insert(0x00209241, &tags::NominalPercentageOfCardiacPhase);

		ident_to_elem.insert("NominalPercentageOfRespiratoryPhase", &tags::NominalPercentageOfRespiratoryPhase);
		tag_to_elem.insert(0x00209245, &tags::NominalPercentageOfRespiratoryPhase);

		ident_to_elem.insert("StartingRespiratoryAmplitude", &tags::StartingRespiratoryAmplitude);
		tag_to_elem.insert(0x00209246, &tags::StartingRespiratoryAmplitude);

		ident_to_elem.insert("StartingRespiratoryPhase", &tags::StartingRespiratoryPhase);
		tag_to_elem.insert(0x00209247, &tags::StartingRespiratoryPhase);

		ident_to_elem.insert("EndingRespiratoryAmplitude", &tags::EndingRespiratoryAmplitude);
		tag_to_elem.insert(0x00209248, &tags::EndingRespiratoryAmplitude);

		ident_to_elem.insert("EndingRespiratoryPhase", &tags::EndingRespiratoryPhase);
		tag_to_elem.insert(0x00209249, &tags::EndingRespiratoryPhase);

		ident_to_elem.insert("RespiratoryTriggerType", &tags::RespiratoryTriggerType);
		tag_to_elem.insert(0x00209250, &tags::RespiratoryTriggerType);

		ident_to_elem.insert("RRIntervalTimeNominal", &tags::RRIntervalTimeNominal);
		tag_to_elem.insert(0x00209251, &tags::RRIntervalTimeNominal);

		ident_to_elem.insert("ActualCardiacTriggerDelayTime", &tags::ActualCardiacTriggerDelayTime);
		tag_to_elem.insert(0x00209252, &tags::ActualCardiacTriggerDelayTime);

		ident_to_elem.insert("RespiratorySynchronizationSequence", &tags::RespiratorySynchronizationSequence);
		tag_to_elem.insert(0x00209253, &tags::RespiratorySynchronizationSequence);

		ident_to_elem.insert("RespiratoryIntervalTime", &tags::RespiratoryIntervalTime);
		tag_to_elem.insert(0x00209254, &tags::RespiratoryIntervalTime);

		ident_to_elem.insert("NominalRespiratoryTriggerDelayTime", &tags::NominalRespiratoryTriggerDelayTime);
		tag_to_elem.insert(0x00209255, &tags::NominalRespiratoryTriggerDelayTime);

		ident_to_elem.insert("RespiratoryTriggerDelayThreshold", &tags::RespiratoryTriggerDelayThreshold);
		tag_to_elem.insert(0x00209256, &tags::RespiratoryTriggerDelayThreshold);

		ident_to_elem.insert("ActualRespiratoryTriggerDelayTime", &tags::ActualRespiratoryTriggerDelayTime);
		tag_to_elem.insert(0x00209257, &tags::ActualRespiratoryTriggerDelayTime);

		ident_to_elem.insert("ImagePositionVolume", &tags::ImagePositionVolume);
		tag_to_elem.insert(0x00209301, &tags::ImagePositionVolume);

		ident_to_elem.insert("ImageOrientationVolume", &tags::ImageOrientationVolume);
		tag_to_elem.insert(0x00209302, &tags::ImageOrientationVolume);

		ident_to_elem.insert("UltrasoundAcquisitionGeometry", &tags::UltrasoundAcquisitionGeometry);
		tag_to_elem.insert(0x00209307, &tags::UltrasoundAcquisitionGeometry);

		ident_to_elem.insert("ApexPosition", &tags::ApexPosition);
		tag_to_elem.insert(0x00209308, &tags::ApexPosition);

		ident_to_elem.insert("VolumeToTransducerMappingMatrix", &tags::VolumeToTransducerMappingMatrix);
		tag_to_elem.insert(0x00209309, &tags::VolumeToTransducerMappingMatrix);

		ident_to_elem.insert("VolumeToTableMappingMatrix", &tags::VolumeToTableMappingMatrix);
		tag_to_elem.insert(0x0020930A, &tags::VolumeToTableMappingMatrix);

		ident_to_elem.insert("VolumeToTransducerRelationship", &tags::VolumeToTransducerRelationship);
		tag_to_elem.insert(0x0020930B, &tags::VolumeToTransducerRelationship);

		ident_to_elem.insert("PatientFrameOfReferenceSource", &tags::PatientFrameOfReferenceSource);
		tag_to_elem.insert(0x0020930C, &tags::PatientFrameOfReferenceSource);

		ident_to_elem.insert("TemporalPositionTimeOffset", &tags::TemporalPositionTimeOffset);
		tag_to_elem.insert(0x0020930D, &tags::TemporalPositionTimeOffset);

		ident_to_elem.insert("PlanePositionVolumeSequence", &tags::PlanePositionVolumeSequence);
		tag_to_elem.insert(0x0020930E, &tags::PlanePositionVolumeSequence);

		ident_to_elem.insert("PlaneOrientationVolumeSequence", &tags::PlaneOrientationVolumeSequence);
		tag_to_elem.insert(0x0020930F, &tags::PlaneOrientationVolumeSequence);

		ident_to_elem.insert("TemporalPositionSequence", &tags::TemporalPositionSequence);
		tag_to_elem.insert(0x00209310, &tags::TemporalPositionSequence);

		ident_to_elem.insert("DimensionOrganizationType", &tags::DimensionOrganizationType);
		tag_to_elem.insert(0x00209311, &tags::DimensionOrganizationType);

		ident_to_elem.insert("VolumeFrameOfReferenceUID", &tags::VolumeFrameOfReferenceUID);
		tag_to_elem.insert(0x00209312, &tags::VolumeFrameOfReferenceUID);

		ident_to_elem.insert("TableFrameOfReferenceUID", &tags::TableFrameOfReferenceUID);
		tag_to_elem.insert(0x00209313, &tags::TableFrameOfReferenceUID);

		ident_to_elem.insert("DimensionDescriptionLabel", &tags::DimensionDescriptionLabel);
		tag_to_elem.insert(0x00209421, &tags::DimensionDescriptionLabel);

		ident_to_elem.insert("PatientOrientationInFrameSequence", &tags::PatientOrientationInFrameSequence);
		tag_to_elem.insert(0x00209450, &tags::PatientOrientationInFrameSequence);

		ident_to_elem.insert("FrameLabel", &tags::FrameLabel);
		tag_to_elem.insert(0x00209453, &tags::FrameLabel);

		ident_to_elem.insert("AcquisitionIndex", &tags::AcquisitionIndex);
		tag_to_elem.insert(0x00209518, &tags::AcquisitionIndex);

		ident_to_elem.insert("ContributingSOPInstancesReferenceSequence", &tags::ContributingSOPInstancesReferenceSequence);
		tag_to_elem.insert(0x00209529, &tags::ContributingSOPInstancesReferenceSequence);

		ident_to_elem.insert("ReconstructionIndex", &tags::ReconstructionIndex);
		tag_to_elem.insert(0x00209536, &tags::ReconstructionIndex);

		ident_to_elem.insert("LightPathFilterPassThroughWavelength", &tags::LightPathFilterPassThroughWavelength);
		tag_to_elem.insert(0x00220001, &tags::LightPathFilterPassThroughWavelength);

		ident_to_elem.insert("LightPathFilterPassBand", &tags::LightPathFilterPassBand);
		tag_to_elem.insert(0x00220002, &tags::LightPathFilterPassBand);

		ident_to_elem.insert("ImagePathFilterPassThroughWavelength", &tags::ImagePathFilterPassThroughWavelength);
		tag_to_elem.insert(0x00220003, &tags::ImagePathFilterPassThroughWavelength);

		ident_to_elem.insert("ImagePathFilterPassBand", &tags::ImagePathFilterPassBand);
		tag_to_elem.insert(0x00220004, &tags::ImagePathFilterPassBand);

		ident_to_elem.insert("PatientEyeMovementCommanded", &tags::PatientEyeMovementCommanded);
		tag_to_elem.insert(0x00220005, &tags::PatientEyeMovementCommanded);

		ident_to_elem.insert("PatientEyeMovementCommandCodeSequence", &tags::PatientEyeMovementCommandCodeSequence);
		tag_to_elem.insert(0x00220006, &tags::PatientEyeMovementCommandCodeSequence);

		ident_to_elem.insert("SphericalLensPower", &tags::SphericalLensPower);
		tag_to_elem.insert(0x00220007, &tags::SphericalLensPower);

		ident_to_elem.insert("CylinderLensPower", &tags::CylinderLensPower);
		tag_to_elem.insert(0x00220008, &tags::CylinderLensPower);

		ident_to_elem.insert("CylinderAxis", &tags::CylinderAxis);
		tag_to_elem.insert(0x00220009, &tags::CylinderAxis);

		ident_to_elem.insert("EmmetropicMagnification", &tags::EmmetropicMagnification);
		tag_to_elem.insert(0x0022000A, &tags::EmmetropicMagnification);

		ident_to_elem.insert("IntraOcularPressure", &tags::IntraOcularPressure);
		tag_to_elem.insert(0x0022000B, &tags::IntraOcularPressure);

		ident_to_elem.insert("HorizontalFieldOfView", &tags::HorizontalFieldOfView);
		tag_to_elem.insert(0x0022000C, &tags::HorizontalFieldOfView);

		ident_to_elem.insert("PupilDilated", &tags::PupilDilated);
		tag_to_elem.insert(0x0022000D, &tags::PupilDilated);

		ident_to_elem.insert("DegreeOfDilation", &tags::DegreeOfDilation);
		tag_to_elem.insert(0x0022000E, &tags::DegreeOfDilation);

		ident_to_elem.insert("StereoBaselineAngle", &tags::StereoBaselineAngle);
		tag_to_elem.insert(0x00220010, &tags::StereoBaselineAngle);

		ident_to_elem.insert("StereoBaselineDisplacement", &tags::StereoBaselineDisplacement);
		tag_to_elem.insert(0x00220011, &tags::StereoBaselineDisplacement);

		ident_to_elem.insert("StereoHorizontalPixelOffset", &tags::StereoHorizontalPixelOffset);
		tag_to_elem.insert(0x00220012, &tags::StereoHorizontalPixelOffset);

		ident_to_elem.insert("StereoVerticalPixelOffset", &tags::StereoVerticalPixelOffset);
		tag_to_elem.insert(0x00220013, &tags::StereoVerticalPixelOffset);

		ident_to_elem.insert("StereoRotation", &tags::StereoRotation);
		tag_to_elem.insert(0x00220014, &tags::StereoRotation);

		ident_to_elem.insert("AcquisitionDeviceTypeCodeSequence", &tags::AcquisitionDeviceTypeCodeSequence);
		tag_to_elem.insert(0x00220015, &tags::AcquisitionDeviceTypeCodeSequence);

		ident_to_elem.insert("IlluminationTypeCodeSequence", &tags::IlluminationTypeCodeSequence);
		tag_to_elem.insert(0x00220016, &tags::IlluminationTypeCodeSequence);

		ident_to_elem.insert("LightPathFilterTypeStackCodeSequence", &tags::LightPathFilterTypeStackCodeSequence);
		tag_to_elem.insert(0x00220017, &tags::LightPathFilterTypeStackCodeSequence);

		ident_to_elem.insert("ImagePathFilterTypeStackCodeSequence", &tags::ImagePathFilterTypeStackCodeSequence);
		tag_to_elem.insert(0x00220018, &tags::ImagePathFilterTypeStackCodeSequence);

		ident_to_elem.insert("LensesCodeSequence", &tags::LensesCodeSequence);
		tag_to_elem.insert(0x00220019, &tags::LensesCodeSequence);

		ident_to_elem.insert("ChannelDescriptionCodeSequence", &tags::ChannelDescriptionCodeSequence);
		tag_to_elem.insert(0x0022001A, &tags::ChannelDescriptionCodeSequence);

		ident_to_elem.insert("RefractiveStateSequence", &tags::RefractiveStateSequence);
		tag_to_elem.insert(0x0022001B, &tags::RefractiveStateSequence);

		ident_to_elem.insert("MydriaticAgentCodeSequence", &tags::MydriaticAgentCodeSequence);
		tag_to_elem.insert(0x0022001C, &tags::MydriaticAgentCodeSequence);

		ident_to_elem.insert("RelativeImagePositionCodeSequence", &tags::RelativeImagePositionCodeSequence);
		tag_to_elem.insert(0x0022001D, &tags::RelativeImagePositionCodeSequence);

		ident_to_elem.insert("CameraAngleOfView", &tags::CameraAngleOfView);
		tag_to_elem.insert(0x0022001E, &tags::CameraAngleOfView);

		ident_to_elem.insert("StereoPairsSequence", &tags::StereoPairsSequence);
		tag_to_elem.insert(0x00220020, &tags::StereoPairsSequence);

		ident_to_elem.insert("LeftImageSequence", &tags::LeftImageSequence);
		tag_to_elem.insert(0x00220021, &tags::LeftImageSequence);

		ident_to_elem.insert("RightImageSequence", &tags::RightImageSequence);
		tag_to_elem.insert(0x00220022, &tags::RightImageSequence);

		ident_to_elem.insert("StereoPairsPresent", &tags::StereoPairsPresent);
		tag_to_elem.insert(0x00220028, &tags::StereoPairsPresent);

		ident_to_elem.insert("AxialLengthOfTheEye", &tags::AxialLengthOfTheEye);
		tag_to_elem.insert(0x00220030, &tags::AxialLengthOfTheEye);

		ident_to_elem.insert("OphthalmicFrameLocationSequence", &tags::OphthalmicFrameLocationSequence);
		tag_to_elem.insert(0x00220031, &tags::OphthalmicFrameLocationSequence);

		ident_to_elem.insert("ReferenceCoordinates", &tags::ReferenceCoordinates);
		tag_to_elem.insert(0x00220032, &tags::ReferenceCoordinates);

		ident_to_elem.insert("DepthSpatialResolution", &tags::DepthSpatialResolution);
		tag_to_elem.insert(0x00220035, &tags::DepthSpatialResolution);

		ident_to_elem.insert("MaximumDepthDistortion", &tags::MaximumDepthDistortion);
		tag_to_elem.insert(0x00220036, &tags::MaximumDepthDistortion);

		ident_to_elem.insert("AlongScanSpatialResolution", &tags::AlongScanSpatialResolution);
		tag_to_elem.insert(0x00220037, &tags::AlongScanSpatialResolution);

		ident_to_elem.insert("MaximumAlongScanDistortion", &tags::MaximumAlongScanDistortion);
		tag_to_elem.insert(0x00220038, &tags::MaximumAlongScanDistortion);

		ident_to_elem.insert("OphthalmicImageOrientation", &tags::OphthalmicImageOrientation);
		tag_to_elem.insert(0x00220039, &tags::OphthalmicImageOrientation);

		ident_to_elem.insert("DepthOfTransverseImage", &tags::DepthOfTransverseImage);
		tag_to_elem.insert(0x00220041, &tags::DepthOfTransverseImage);

		ident_to_elem.insert("MydriaticAgentConcentrationUnitsSequence", &tags::MydriaticAgentConcentrationUnitsSequence);
		tag_to_elem.insert(0x00220042, &tags::MydriaticAgentConcentrationUnitsSequence);

		ident_to_elem.insert("AcrossScanSpatialResolution", &tags::AcrossScanSpatialResolution);
		tag_to_elem.insert(0x00220048, &tags::AcrossScanSpatialResolution);

		ident_to_elem.insert("MaximumAcrossScanDistortion", &tags::MaximumAcrossScanDistortion);
		tag_to_elem.insert(0x00220049, &tags::MaximumAcrossScanDistortion);

		ident_to_elem.insert("MydriaticAgentConcentration", &tags::MydriaticAgentConcentration);
		tag_to_elem.insert(0x0022004E, &tags::MydriaticAgentConcentration);

		ident_to_elem.insert("IlluminationWaveLength", &tags::IlluminationWaveLength);
		tag_to_elem.insert(0x00220055, &tags::IlluminationWaveLength);

		ident_to_elem.insert("IlluminationPower", &tags::IlluminationPower);
		tag_to_elem.insert(0x00220056, &tags::IlluminationPower);

		ident_to_elem.insert("IlluminationBandwidth", &tags::IlluminationBandwidth);
		tag_to_elem.insert(0x00220057, &tags::IlluminationBandwidth);

		ident_to_elem.insert("MydriaticAgentSequence", &tags::MydriaticAgentSequence);
		tag_to_elem.insert(0x00220058, &tags::MydriaticAgentSequence);

		ident_to_elem.insert("OphthalmicAxialMeasurementsRightEyeSequence", &tags::OphthalmicAxialMeasurementsRightEyeSequence);
		tag_to_elem.insert(0x00221007, &tags::OphthalmicAxialMeasurementsRightEyeSequence);

		ident_to_elem.insert("OphthalmicAxialMeasurementsLeftEyeSequence", &tags::OphthalmicAxialMeasurementsLeftEyeSequence);
		tag_to_elem.insert(0x00221008, &tags::OphthalmicAxialMeasurementsLeftEyeSequence);

		ident_to_elem.insert("OphthalmicAxialMeasurementsDeviceType", &tags::OphthalmicAxialMeasurementsDeviceType);
		tag_to_elem.insert(0x00221009, &tags::OphthalmicAxialMeasurementsDeviceType);

		ident_to_elem.insert("OphthalmicAxialLengthMeasurementsType", &tags::OphthalmicAxialLengthMeasurementsType);
		tag_to_elem.insert(0x00221010, &tags::OphthalmicAxialLengthMeasurementsType);

		ident_to_elem.insert("OphthalmicAxialLengthSequence", &tags::OphthalmicAxialLengthSequence);
		tag_to_elem.insert(0x00221012, &tags::OphthalmicAxialLengthSequence);

		ident_to_elem.insert("OphthalmicAxialLength", &tags::OphthalmicAxialLength);
		tag_to_elem.insert(0x00221019, &tags::OphthalmicAxialLength);

		ident_to_elem.insert("LensStatusCodeSequence", &tags::LensStatusCodeSequence);
		tag_to_elem.insert(0x00221024, &tags::LensStatusCodeSequence);

		ident_to_elem.insert("VitreousStatusCodeSequence", &tags::VitreousStatusCodeSequence);
		tag_to_elem.insert(0x00221025, &tags::VitreousStatusCodeSequence);

		ident_to_elem.insert("IOLFormulaCodeSequence", &tags::IOLFormulaCodeSequence);
		tag_to_elem.insert(0x00221028, &tags::IOLFormulaCodeSequence);

		ident_to_elem.insert("IOLFormulaDetail", &tags::IOLFormulaDetail);
		tag_to_elem.insert(0x00221029, &tags::IOLFormulaDetail);

		ident_to_elem.insert("KeratometerIndex", &tags::KeratometerIndex);
		tag_to_elem.insert(0x00221033, &tags::KeratometerIndex);

		ident_to_elem.insert("SourceOfOphthalmicAxialLengthCodeSequence", &tags::SourceOfOphthalmicAxialLengthCodeSequence);
		tag_to_elem.insert(0x00221035, &tags::SourceOfOphthalmicAxialLengthCodeSequence);

		ident_to_elem.insert("TargetRefraction", &tags::TargetRefraction);
		tag_to_elem.insert(0x00221037, &tags::TargetRefraction);

		ident_to_elem.insert("RefractiveProcedureOccurred", &tags::RefractiveProcedureOccurred);
		tag_to_elem.insert(0x00221039, &tags::RefractiveProcedureOccurred);

		ident_to_elem.insert("RefractiveSurgeryTypeCodeSequence", &tags::RefractiveSurgeryTypeCodeSequence);
		tag_to_elem.insert(0x00221040, &tags::RefractiveSurgeryTypeCodeSequence);

		ident_to_elem.insert("OphthalmicUltrasoundMethodCodeSequence", &tags::OphthalmicUltrasoundMethodCodeSequence);
		tag_to_elem.insert(0x00221044, &tags::OphthalmicUltrasoundMethodCodeSequence);

		ident_to_elem.insert("OphthalmicAxialLengthMeasurementsSequence", &tags::OphthalmicAxialLengthMeasurementsSequence);
		tag_to_elem.insert(0x00221050, &tags::OphthalmicAxialLengthMeasurementsSequence);

		ident_to_elem.insert("IOLPower", &tags::IOLPower);
		tag_to_elem.insert(0x00221053, &tags::IOLPower);

		ident_to_elem.insert("PredictedRefractiveError", &tags::PredictedRefractiveError);
		tag_to_elem.insert(0x00221054, &tags::PredictedRefractiveError);

		ident_to_elem.insert("OphthalmicAxialLengthVelocity", &tags::OphthalmicAxialLengthVelocity);
		tag_to_elem.insert(0x00221059, &tags::OphthalmicAxialLengthVelocity);

		ident_to_elem.insert("LensStatusDescription", &tags::LensStatusDescription);
		tag_to_elem.insert(0x00221065, &tags::LensStatusDescription);

		ident_to_elem.insert("VitreousStatusDescription", &tags::VitreousStatusDescription);
		tag_to_elem.insert(0x00221066, &tags::VitreousStatusDescription);

		ident_to_elem.insert("IOLPowerSequence", &tags::IOLPowerSequence);
		tag_to_elem.insert(0x00221090, &tags::IOLPowerSequence);

		ident_to_elem.insert("LensConstantSequence", &tags::LensConstantSequence);
		tag_to_elem.insert(0x00221092, &tags::LensConstantSequence);

		ident_to_elem.insert("IOLManufacturer", &tags::IOLManufacturer);
		tag_to_elem.insert(0x00221093, &tags::IOLManufacturer);

		ident_to_elem.insert("LensConstantDescription", &tags::LensConstantDescription);
		tag_to_elem.insert(0x00221094, &tags::LensConstantDescription);

		ident_to_elem.insert("ImplantName", &tags::ImplantName);
		tag_to_elem.insert(0x00221095, &tags::ImplantName);

		ident_to_elem.insert("KeratometryMeasurementTypeCodeSequence", &tags::KeratometryMeasurementTypeCodeSequence);
		tag_to_elem.insert(0x00221096, &tags::KeratometryMeasurementTypeCodeSequence);

		ident_to_elem.insert("ImplantPartNumber", &tags::ImplantPartNumber);
		tag_to_elem.insert(0x00221097, &tags::ImplantPartNumber);

		ident_to_elem.insert("ReferencedOphthalmicAxialMeasurementsSequence", &tags::ReferencedOphthalmicAxialMeasurementsSequence);
		tag_to_elem.insert(0x00221100, &tags::ReferencedOphthalmicAxialMeasurementsSequence);

		ident_to_elem.insert("OphthalmicAxialLengthMeasurementsSegmentNameCodeSequence", &tags::OphthalmicAxialLengthMeasurementsSegmentNameCodeSequence);
		tag_to_elem.insert(0x00221101, &tags::OphthalmicAxialLengthMeasurementsSegmentNameCodeSequence);

		ident_to_elem.insert("RefractiveErrorBeforeRefractiveSurgeryCodeSequence", &tags::RefractiveErrorBeforeRefractiveSurgeryCodeSequence);
		tag_to_elem.insert(0x00221103, &tags::RefractiveErrorBeforeRefractiveSurgeryCodeSequence);

		ident_to_elem.insert("IOLPowerForExactEmmetropia", &tags::IOLPowerForExactEmmetropia);
		tag_to_elem.insert(0x00221121, &tags::IOLPowerForExactEmmetropia);

		ident_to_elem.insert("IOLPowerForExactTargetRefraction", &tags::IOLPowerForExactTargetRefraction);
		tag_to_elem.insert(0x00221122, &tags::IOLPowerForExactTargetRefraction);

		ident_to_elem.insert("AnteriorChamberDepthDefinitionCodeSequence", &tags::AnteriorChamberDepthDefinitionCodeSequence);
		tag_to_elem.insert(0x00221125, &tags::AnteriorChamberDepthDefinitionCodeSequence);

		ident_to_elem.insert("LensThicknessSequence", &tags::LensThicknessSequence);
		tag_to_elem.insert(0x00221127, &tags::LensThicknessSequence);

		ident_to_elem.insert("AnteriorChamberDepthSequence", &tags::AnteriorChamberDepthSequence);
		tag_to_elem.insert(0x00221128, &tags::AnteriorChamberDepthSequence);

		ident_to_elem.insert("LensThickness", &tags::LensThickness);
		tag_to_elem.insert(0x00221130, &tags::LensThickness);

		ident_to_elem.insert("AnteriorChamberDepth", &tags::AnteriorChamberDepth);
		tag_to_elem.insert(0x00221131, &tags::AnteriorChamberDepth);

		ident_to_elem.insert("SourceOfLensThicknessDataCodeSequence", &tags::SourceOfLensThicknessDataCodeSequence);
		tag_to_elem.insert(0x00221132, &tags::SourceOfLensThicknessDataCodeSequence);

		ident_to_elem.insert("SourceOfAnteriorChamberDepthDataCodeSequence", &tags::SourceOfAnteriorChamberDepthDataCodeSequence);
		tag_to_elem.insert(0x00221133, &tags::SourceOfAnteriorChamberDepthDataCodeSequence);

		ident_to_elem.insert("SourceOfRefractiveMeasurementsSequence", &tags::SourceOfRefractiveMeasurementsSequence);
		tag_to_elem.insert(0x00221134, &tags::SourceOfRefractiveMeasurementsSequence);

		ident_to_elem.insert("SourceOfRefractiveMeasurementsCodeSequence", &tags::SourceOfRefractiveMeasurementsCodeSequence);
		tag_to_elem.insert(0x00221135, &tags::SourceOfRefractiveMeasurementsCodeSequence);

		ident_to_elem.insert("OphthalmicAxialLengthMeasurementModified", &tags::OphthalmicAxialLengthMeasurementModified);
		tag_to_elem.insert(0x00221140, &tags::OphthalmicAxialLengthMeasurementModified);

		ident_to_elem.insert("OphthalmicAxialLengthDataSourceCodeSequence", &tags::OphthalmicAxialLengthDataSourceCodeSequence);
		tag_to_elem.insert(0x00221150, &tags::OphthalmicAxialLengthDataSourceCodeSequence);

		ident_to_elem.insert("OphthalmicAxialLengthAcquisitionMethodCodeSequence", &tags::OphthalmicAxialLengthAcquisitionMethodCodeSequence);
		tag_to_elem.insert(0x00221153, &tags::OphthalmicAxialLengthAcquisitionMethodCodeSequence);

		ident_to_elem.insert("SignalToNoiseRatio", &tags::SignalToNoiseRatio);
		tag_to_elem.insert(0x00221155, &tags::SignalToNoiseRatio);

		ident_to_elem.insert("OphthalmicAxialLengthDataSourceDescription", &tags::OphthalmicAxialLengthDataSourceDescription);
		tag_to_elem.insert(0x00221159, &tags::OphthalmicAxialLengthDataSourceDescription);

		ident_to_elem.insert("OphthalmicAxialLengthMeasurementsTotalLengthSequence", &tags::OphthalmicAxialLengthMeasurementsTotalLengthSequence);
		tag_to_elem.insert(0x00221210, &tags::OphthalmicAxialLengthMeasurementsTotalLengthSequence);

		ident_to_elem.insert("OphthalmicAxialLengthMeasurementsSegmentalLengthSequence", &tags::OphthalmicAxialLengthMeasurementsSegmentalLengthSequence);
		tag_to_elem.insert(0x00221211, &tags::OphthalmicAxialLengthMeasurementsSegmentalLengthSequence);

		ident_to_elem.insert("OphthalmicAxialLengthMeasurementsLengthSummationSequence", &tags::OphthalmicAxialLengthMeasurementsLengthSummationSequence);
		tag_to_elem.insert(0x00221212, &tags::OphthalmicAxialLengthMeasurementsLengthSummationSequence);

		ident_to_elem.insert("UltrasoundOphthalmicAxialLengthMeasurementsSequence", &tags::UltrasoundOphthalmicAxialLengthMeasurementsSequence);
		tag_to_elem.insert(0x00221220, &tags::UltrasoundOphthalmicAxialLengthMeasurementsSequence);

		ident_to_elem.insert("OpticalOphthalmicAxialLengthMeasurementsSequence", &tags::OpticalOphthalmicAxialLengthMeasurementsSequence);
		tag_to_elem.insert(0x00221225, &tags::OpticalOphthalmicAxialLengthMeasurementsSequence);

		ident_to_elem.insert("UltrasoundSelectedOphthalmicAxialLengthSequence", &tags::UltrasoundSelectedOphthalmicAxialLengthSequence);
		tag_to_elem.insert(0x00221230, &tags::UltrasoundSelectedOphthalmicAxialLengthSequence);

		ident_to_elem.insert("OphthalmicAxialLengthSelectionMethodCodeSequence", &tags::OphthalmicAxialLengthSelectionMethodCodeSequence);
		tag_to_elem.insert(0x00221250, &tags::OphthalmicAxialLengthSelectionMethodCodeSequence);

		ident_to_elem.insert("OpticalSelectedOphthalmicAxialLengthSequence", &tags::OpticalSelectedOphthalmicAxialLengthSequence);
		tag_to_elem.insert(0x00221255, &tags::OpticalSelectedOphthalmicAxialLengthSequence);

		ident_to_elem.insert("SelectedSegmentalOphthalmicAxialLengthSequence", &tags::SelectedSegmentalOphthalmicAxialLengthSequence);
		tag_to_elem.insert(0x00221257, &tags::SelectedSegmentalOphthalmicAxialLengthSequence);

		ident_to_elem.insert("SelectedTotalOphthalmicAxialLengthSequence", &tags::SelectedTotalOphthalmicAxialLengthSequence);
		tag_to_elem.insert(0x00221260, &tags::SelectedTotalOphthalmicAxialLengthSequence);

		ident_to_elem.insert("OphthalmicAxialLengthQualityMetricSequence", &tags::OphthalmicAxialLengthQualityMetricSequence);
		tag_to_elem.insert(0x00221262, &tags::OphthalmicAxialLengthQualityMetricSequence);

		ident_to_elem.insert("OphthalmicAxialLengthQualityMetricTypeCodeSequence", &tags::OphthalmicAxialLengthQualityMetricTypeCodeSequence);
		tag_to_elem.insert(0x00221265, &tags::OphthalmicAxialLengthQualityMetricTypeCodeSequence);

		ident_to_elem.insert("OphthalmicAxialLengthQualityMetricTypeDescription", &tags::OphthalmicAxialLengthQualityMetricTypeDescription);
		tag_to_elem.insert(0x00221273, &tags::OphthalmicAxialLengthQualityMetricTypeDescription);

		ident_to_elem.insert("IntraocularLensCalculationsRightEyeSequence", &tags::IntraocularLensCalculationsRightEyeSequence);
		tag_to_elem.insert(0x00221300, &tags::IntraocularLensCalculationsRightEyeSequence);

		ident_to_elem.insert("IntraocularLensCalculationsLeftEyeSequence", &tags::IntraocularLensCalculationsLeftEyeSequence);
		tag_to_elem.insert(0x00221310, &tags::IntraocularLensCalculationsLeftEyeSequence);

		ident_to_elem.insert("ReferencedOphthalmicAxialLengthMeasurementQCImageSequence", &tags::ReferencedOphthalmicAxialLengthMeasurementQCImageSequence);
		tag_to_elem.insert(0x00221330, &tags::ReferencedOphthalmicAxialLengthMeasurementQCImageSequence);

		ident_to_elem.insert("OphthalmicMappingDeviceType", &tags::OphthalmicMappingDeviceType);
		tag_to_elem.insert(0x00221415, &tags::OphthalmicMappingDeviceType);

		ident_to_elem.insert("AcquisitionMethodCodeSequence", &tags::AcquisitionMethodCodeSequence);
		tag_to_elem.insert(0x00221420, &tags::AcquisitionMethodCodeSequence);

		ident_to_elem.insert("AcquisitionMethodAlgorithmSequence", &tags::AcquisitionMethodAlgorithmSequence);
		tag_to_elem.insert(0x00221423, &tags::AcquisitionMethodAlgorithmSequence);

		ident_to_elem.insert("OphthalmicThicknessMapTypeCodeSequence", &tags::OphthalmicThicknessMapTypeCodeSequence);
		tag_to_elem.insert(0x00221436, &tags::OphthalmicThicknessMapTypeCodeSequence);

		ident_to_elem.insert("OphthalmicThicknessMappingNormalsSequence", &tags::OphthalmicThicknessMappingNormalsSequence);
		tag_to_elem.insert(0x00221443, &tags::OphthalmicThicknessMappingNormalsSequence);

		ident_to_elem.insert("RetinalThicknessDefinitionCodeSequence", &tags::RetinalThicknessDefinitionCodeSequence);
		tag_to_elem.insert(0x00221445, &tags::RetinalThicknessDefinitionCodeSequence);

		ident_to_elem.insert("PixelValueMappingToCodedConceptSequence", &tags::PixelValueMappingToCodedConceptSequence);
		tag_to_elem.insert(0x00221450, &tags::PixelValueMappingToCodedConceptSequence);

		ident_to_elem.insert("MappedPixelValue", &tags::MappedPixelValue);
		tag_to_elem.insert(0x00221452, &tags::MappedPixelValue);

		ident_to_elem.insert("PixelValueMappingExplanation", &tags::PixelValueMappingExplanation);
		tag_to_elem.insert(0x00221454, &tags::PixelValueMappingExplanation);

		ident_to_elem.insert("OphthalmicThicknessMapQualityThresholdSequence", &tags::OphthalmicThicknessMapQualityThresholdSequence);
		tag_to_elem.insert(0x00221458, &tags::OphthalmicThicknessMapQualityThresholdSequence);

		ident_to_elem.insert("OphthalmicThicknessMapThresholdQualityRating", &tags::OphthalmicThicknessMapThresholdQualityRating);
		tag_to_elem.insert(0x00221460, &tags::OphthalmicThicknessMapThresholdQualityRating);

		ident_to_elem.insert("AnatomicStructureReferencePoint", &tags::AnatomicStructureReferencePoint);
		tag_to_elem.insert(0x00221463, &tags::AnatomicStructureReferencePoint);

		ident_to_elem.insert("RegistrationToLocalizerSequence", &tags::RegistrationToLocalizerSequence);
		tag_to_elem.insert(0x00221465, &tags::RegistrationToLocalizerSequence);

		ident_to_elem.insert("RegisteredLocalizerUnits", &tags::RegisteredLocalizerUnits);
		tag_to_elem.insert(0x00221466, &tags::RegisteredLocalizerUnits);

		ident_to_elem.insert("RegisteredLocalizerTopLeftHandCorner", &tags::RegisteredLocalizerTopLeftHandCorner);
		tag_to_elem.insert(0x00221467, &tags::RegisteredLocalizerTopLeftHandCorner);

		ident_to_elem.insert("RegisteredLocalizerBottomRightHandCorner", &tags::RegisteredLocalizerBottomRightHandCorner);
		tag_to_elem.insert(0x00221468, &tags::RegisteredLocalizerBottomRightHandCorner);

		ident_to_elem.insert("OphthalmicThicknessMapQualityRatingSequence", &tags::OphthalmicThicknessMapQualityRatingSequence);
		tag_to_elem.insert(0x00221470, &tags::OphthalmicThicknessMapQualityRatingSequence);

		ident_to_elem.insert("RelevantOPTAttributesSequence", &tags::RelevantOPTAttributesSequence);
		tag_to_elem.insert(0x00221472, &tags::RelevantOPTAttributesSequence);

		ident_to_elem.insert("TransformationMethodCodeSequence", &tags::TransformationMethodCodeSequence);
		tag_to_elem.insert(0x00221512, &tags::TransformationMethodCodeSequence);

		ident_to_elem.insert("TransformationAlgorithmSequence", &tags::TransformationAlgorithmSequence);
		tag_to_elem.insert(0x00221513, &tags::TransformationAlgorithmSequence);

		ident_to_elem.insert("OphthalmicAxialLengthMethod", &tags::OphthalmicAxialLengthMethod);
		tag_to_elem.insert(0x00221515, &tags::OphthalmicAxialLengthMethod);

		ident_to_elem.insert("OphthalmicFOV", &tags::OphthalmicFOV);
		tag_to_elem.insert(0x00221517, &tags::OphthalmicFOV);

		ident_to_elem.insert("TwoDimensionalToThreeDimensionalMapSequence", &tags::TwoDimensionalToThreeDimensionalMapSequence);
		tag_to_elem.insert(0x00221518, &tags::TwoDimensionalToThreeDimensionalMapSequence);

		ident_to_elem.insert("WideFieldOphthalmicPhotographyQualityRatingSequence", &tags::WideFieldOphthalmicPhotographyQualityRatingSequence);
		tag_to_elem.insert(0x00221525, &tags::WideFieldOphthalmicPhotographyQualityRatingSequence);

		ident_to_elem.insert("WideFieldOphthalmicPhotographyQualityThresholdSequence", &tags::WideFieldOphthalmicPhotographyQualityThresholdSequence);
		tag_to_elem.insert(0x00221526, &tags::WideFieldOphthalmicPhotographyQualityThresholdSequence);

		ident_to_elem.insert("WideFieldOphthalmicPhotographyThresholdQualityRating", &tags::WideFieldOphthalmicPhotographyThresholdQualityRating);
		tag_to_elem.insert(0x00221527, &tags::WideFieldOphthalmicPhotographyThresholdQualityRating);

		ident_to_elem.insert("XCoordinatesCenterPixelViewAngle", &tags::XCoordinatesCenterPixelViewAngle);
		tag_to_elem.insert(0x00221528, &tags::XCoordinatesCenterPixelViewAngle);

		ident_to_elem.insert("YCoordinatesCenterPixelViewAngle", &tags::YCoordinatesCenterPixelViewAngle);
		tag_to_elem.insert(0x00221529, &tags::YCoordinatesCenterPixelViewAngle);

		ident_to_elem.insert("NumberOfMapPoints", &tags::NumberOfMapPoints);
		tag_to_elem.insert(0x00221530, &tags::NumberOfMapPoints);

		ident_to_elem.insert("TwoDimensionalToThreeDimensionalMapData", &tags::TwoDimensionalToThreeDimensionalMapData);
		tag_to_elem.insert(0x00221531, &tags::TwoDimensionalToThreeDimensionalMapData);

		ident_to_elem.insert("VisualFieldHorizontalExtent", &tags::VisualFieldHorizontalExtent);
		tag_to_elem.insert(0x00240010, &tags::VisualFieldHorizontalExtent);

		ident_to_elem.insert("VisualFieldVerticalExtent", &tags::VisualFieldVerticalExtent);
		tag_to_elem.insert(0x00240011, &tags::VisualFieldVerticalExtent);

		ident_to_elem.insert("VisualFieldShape", &tags::VisualFieldShape);
		tag_to_elem.insert(0x00240012, &tags::VisualFieldShape);

		ident_to_elem.insert("ScreeningTestModeCodeSequence", &tags::ScreeningTestModeCodeSequence);
		tag_to_elem.insert(0x00240016, &tags::ScreeningTestModeCodeSequence);

		ident_to_elem.insert("MaximumStimulusLuminance", &tags::MaximumStimulusLuminance);
		tag_to_elem.insert(0x00240018, &tags::MaximumStimulusLuminance);

		ident_to_elem.insert("BackgroundLuminance", &tags::BackgroundLuminance);
		tag_to_elem.insert(0x00240020, &tags::BackgroundLuminance);

		ident_to_elem.insert("StimulusColorCodeSequence", &tags::StimulusColorCodeSequence);
		tag_to_elem.insert(0x00240021, &tags::StimulusColorCodeSequence);

		ident_to_elem.insert("BackgroundIlluminationColorCodeSequence", &tags::BackgroundIlluminationColorCodeSequence);
		tag_to_elem.insert(0x00240024, &tags::BackgroundIlluminationColorCodeSequence);

		ident_to_elem.insert("StimulusArea", &tags::StimulusArea);
		tag_to_elem.insert(0x00240025, &tags::StimulusArea);

		ident_to_elem.insert("StimulusPresentationTime", &tags::StimulusPresentationTime);
		tag_to_elem.insert(0x00240028, &tags::StimulusPresentationTime);

		ident_to_elem.insert("FixationSequence", &tags::FixationSequence);
		tag_to_elem.insert(0x00240032, &tags::FixationSequence);

		ident_to_elem.insert("FixationMonitoringCodeSequence", &tags::FixationMonitoringCodeSequence);
		tag_to_elem.insert(0x00240033, &tags::FixationMonitoringCodeSequence);

		ident_to_elem.insert("VisualFieldCatchTrialSequence", &tags::VisualFieldCatchTrialSequence);
		tag_to_elem.insert(0x00240034, &tags::VisualFieldCatchTrialSequence);

		ident_to_elem.insert("FixationCheckedQuantity", &tags::FixationCheckedQuantity);
		tag_to_elem.insert(0x00240035, &tags::FixationCheckedQuantity);

		ident_to_elem.insert("PatientNotProperlyFixatedQuantity", &tags::PatientNotProperlyFixatedQuantity);
		tag_to_elem.insert(0x00240036, &tags::PatientNotProperlyFixatedQuantity);

		ident_to_elem.insert("PresentedVisualStimuliDataFlag", &tags::PresentedVisualStimuliDataFlag);
		tag_to_elem.insert(0x00240037, &tags::PresentedVisualStimuliDataFlag);

		ident_to_elem.insert("NumberOfVisualStimuli", &tags::NumberOfVisualStimuli);
		tag_to_elem.insert(0x00240038, &tags::NumberOfVisualStimuli);

		ident_to_elem.insert("ExcessiveFixationLossesDataFlag", &tags::ExcessiveFixationLossesDataFlag);
		tag_to_elem.insert(0x00240039, &tags::ExcessiveFixationLossesDataFlag);

		ident_to_elem.insert("ExcessiveFixationLosses", &tags::ExcessiveFixationLosses);
		tag_to_elem.insert(0x00240040, &tags::ExcessiveFixationLosses);

		ident_to_elem.insert("StimuliRetestingQuantity", &tags::StimuliRetestingQuantity);
		tag_to_elem.insert(0x00240042, &tags::StimuliRetestingQuantity);

		ident_to_elem.insert("CommentsOnPatientPerformanceOfVisualField", &tags::CommentsOnPatientPerformanceOfVisualField);
		tag_to_elem.insert(0x00240044, &tags::CommentsOnPatientPerformanceOfVisualField);

		ident_to_elem.insert("FalseNegativesEstimateFlag", &tags::FalseNegativesEstimateFlag);
		tag_to_elem.insert(0x00240045, &tags::FalseNegativesEstimateFlag);

		ident_to_elem.insert("FalseNegativesEstimate", &tags::FalseNegativesEstimate);
		tag_to_elem.insert(0x00240046, &tags::FalseNegativesEstimate);

		ident_to_elem.insert("NegativeCatchTrialsQuantity", &tags::NegativeCatchTrialsQuantity);
		tag_to_elem.insert(0x00240048, &tags::NegativeCatchTrialsQuantity);

		ident_to_elem.insert("FalseNegativesQuantity", &tags::FalseNegativesQuantity);
		tag_to_elem.insert(0x00240050, &tags::FalseNegativesQuantity);

		ident_to_elem.insert("ExcessiveFalseNegativesDataFlag", &tags::ExcessiveFalseNegativesDataFlag);
		tag_to_elem.insert(0x00240051, &tags::ExcessiveFalseNegativesDataFlag);

		ident_to_elem.insert("ExcessiveFalseNegatives", &tags::ExcessiveFalseNegatives);
		tag_to_elem.insert(0x00240052, &tags::ExcessiveFalseNegatives);

		ident_to_elem.insert("FalsePositivesEstimateFlag", &tags::FalsePositivesEstimateFlag);
		tag_to_elem.insert(0x00240053, &tags::FalsePositivesEstimateFlag);

		ident_to_elem.insert("FalsePositivesEstimate", &tags::FalsePositivesEstimate);
		tag_to_elem.insert(0x00240054, &tags::FalsePositivesEstimate);

		ident_to_elem.insert("CatchTrialsDataFlag", &tags::CatchTrialsDataFlag);
		tag_to_elem.insert(0x00240055, &tags::CatchTrialsDataFlag);

		ident_to_elem.insert("PositiveCatchTrialsQuantity", &tags::PositiveCatchTrialsQuantity);
		tag_to_elem.insert(0x00240056, &tags::PositiveCatchTrialsQuantity);

		ident_to_elem.insert("TestPointNormalsDataFlag", &tags::TestPointNormalsDataFlag);
		tag_to_elem.insert(0x00240057, &tags::TestPointNormalsDataFlag);

		ident_to_elem.insert("TestPointNormalsSequence", &tags::TestPointNormalsSequence);
		tag_to_elem.insert(0x00240058, &tags::TestPointNormalsSequence);

		ident_to_elem.insert("GlobalDeviationProbabilityNormalsFlag", &tags::GlobalDeviationProbabilityNormalsFlag);
		tag_to_elem.insert(0x00240059, &tags::GlobalDeviationProbabilityNormalsFlag);

		ident_to_elem.insert("FalsePositivesQuantity", &tags::FalsePositivesQuantity);
		tag_to_elem.insert(0x00240060, &tags::FalsePositivesQuantity);

		ident_to_elem.insert("ExcessiveFalsePositivesDataFlag", &tags::ExcessiveFalsePositivesDataFlag);
		tag_to_elem.insert(0x00240061, &tags::ExcessiveFalsePositivesDataFlag);

		ident_to_elem.insert("ExcessiveFalsePositives", &tags::ExcessiveFalsePositives);
		tag_to_elem.insert(0x00240062, &tags::ExcessiveFalsePositives);

		ident_to_elem.insert("VisualFieldTestNormalsFlag", &tags::VisualFieldTestNormalsFlag);
		tag_to_elem.insert(0x00240063, &tags::VisualFieldTestNormalsFlag);

		ident_to_elem.insert("ResultsNormalsSequence", &tags::ResultsNormalsSequence);
		tag_to_elem.insert(0x00240064, &tags::ResultsNormalsSequence);

		ident_to_elem.insert("AgeCorrectedSensitivityDeviationAlgorithmSequence", &tags::AgeCorrectedSensitivityDeviationAlgorithmSequence);
		tag_to_elem.insert(0x00240065, &tags::AgeCorrectedSensitivityDeviationAlgorithmSequence);

		ident_to_elem.insert("GlobalDeviationFromNormal", &tags::GlobalDeviationFromNormal);
		tag_to_elem.insert(0x00240066, &tags::GlobalDeviationFromNormal);

		ident_to_elem.insert("GeneralizedDefectSensitivityDeviationAlgorithmSequence", &tags::GeneralizedDefectSensitivityDeviationAlgorithmSequence);
		tag_to_elem.insert(0x00240067, &tags::GeneralizedDefectSensitivityDeviationAlgorithmSequence);

		ident_to_elem.insert("LocalizedDeviationFromNormal", &tags::LocalizedDeviationFromNormal);
		tag_to_elem.insert(0x00240068, &tags::LocalizedDeviationFromNormal);

		ident_to_elem.insert("PatientReliabilityIndicator", &tags::PatientReliabilityIndicator);
		tag_to_elem.insert(0x00240069, &tags::PatientReliabilityIndicator);

		ident_to_elem.insert("VisualFieldMeanSensitivity", &tags::VisualFieldMeanSensitivity);
		tag_to_elem.insert(0x00240070, &tags::VisualFieldMeanSensitivity);

		ident_to_elem.insert("GlobalDeviationProbability", &tags::GlobalDeviationProbability);
		tag_to_elem.insert(0x00240071, &tags::GlobalDeviationProbability);

		ident_to_elem.insert("LocalDeviationProbabilityNormalsFlag", &tags::LocalDeviationProbabilityNormalsFlag);
		tag_to_elem.insert(0x00240072, &tags::LocalDeviationProbabilityNormalsFlag);

		ident_to_elem.insert("LocalizedDeviationProbability", &tags::LocalizedDeviationProbability);
		tag_to_elem.insert(0x00240073, &tags::LocalizedDeviationProbability);

		ident_to_elem.insert("ShortTermFluctuationCalculated", &tags::ShortTermFluctuationCalculated);
		tag_to_elem.insert(0x00240074, &tags::ShortTermFluctuationCalculated);

		ident_to_elem.insert("ShortTermFluctuation", &tags::ShortTermFluctuation);
		tag_to_elem.insert(0x00240075, &tags::ShortTermFluctuation);

		ident_to_elem.insert("ShortTermFluctuationProbabilityCalculated", &tags::ShortTermFluctuationProbabilityCalculated);
		tag_to_elem.insert(0x00240076, &tags::ShortTermFluctuationProbabilityCalculated);

		ident_to_elem.insert("ShortTermFluctuationProbability", &tags::ShortTermFluctuationProbability);
		tag_to_elem.insert(0x00240077, &tags::ShortTermFluctuationProbability);

		ident_to_elem.insert("CorrectedLocalizedDeviationFromNormalCalculated", &tags::CorrectedLocalizedDeviationFromNormalCalculated);
		tag_to_elem.insert(0x00240078, &tags::CorrectedLocalizedDeviationFromNormalCalculated);

		ident_to_elem.insert("CorrectedLocalizedDeviationFromNormal", &tags::CorrectedLocalizedDeviationFromNormal);
		tag_to_elem.insert(0x00240079, &tags::CorrectedLocalizedDeviationFromNormal);

		ident_to_elem.insert("CorrectedLocalizedDeviationFromNormalProbabilityCalculated", &tags::CorrectedLocalizedDeviationFromNormalProbabilityCalculated);
		tag_to_elem.insert(0x00240080, &tags::CorrectedLocalizedDeviationFromNormalProbabilityCalculated);

		ident_to_elem.insert("CorrectedLocalizedDeviationFromNormalProbability", &tags::CorrectedLocalizedDeviationFromNormalProbability);
		tag_to_elem.insert(0x00240081, &tags::CorrectedLocalizedDeviationFromNormalProbability);

		ident_to_elem.insert("GlobalDeviationProbabilitySequence", &tags::GlobalDeviationProbabilitySequence);
		tag_to_elem.insert(0x00240083, &tags::GlobalDeviationProbabilitySequence);

		ident_to_elem.insert("LocalizedDeviationProbabilitySequence", &tags::LocalizedDeviationProbabilitySequence);
		tag_to_elem.insert(0x00240085, &tags::LocalizedDeviationProbabilitySequence);

		ident_to_elem.insert("FovealSensitivityMeasured", &tags::FovealSensitivityMeasured);
		tag_to_elem.insert(0x00240086, &tags::FovealSensitivityMeasured);

		ident_to_elem.insert("FovealSensitivity", &tags::FovealSensitivity);
		tag_to_elem.insert(0x00240087, &tags::FovealSensitivity);

		ident_to_elem.insert("VisualFieldTestDuration", &tags::VisualFieldTestDuration);
		tag_to_elem.insert(0x00240088, &tags::VisualFieldTestDuration);

		ident_to_elem.insert("VisualFieldTestPointSequence", &tags::VisualFieldTestPointSequence);
		tag_to_elem.insert(0x00240089, &tags::VisualFieldTestPointSequence);

		ident_to_elem.insert("VisualFieldTestPointXCoordinate", &tags::VisualFieldTestPointXCoordinate);
		tag_to_elem.insert(0x00240090, &tags::VisualFieldTestPointXCoordinate);

		ident_to_elem.insert("VisualFieldTestPointYCoordinate", &tags::VisualFieldTestPointYCoordinate);
		tag_to_elem.insert(0x00240091, &tags::VisualFieldTestPointYCoordinate);

		ident_to_elem.insert("AgeCorrectedSensitivityDeviationValue", &tags::AgeCorrectedSensitivityDeviationValue);
		tag_to_elem.insert(0x00240092, &tags::AgeCorrectedSensitivityDeviationValue);

		ident_to_elem.insert("StimulusResults", &tags::StimulusResults);
		tag_to_elem.insert(0x00240093, &tags::StimulusResults);

		ident_to_elem.insert("SensitivityValue", &tags::SensitivityValue);
		tag_to_elem.insert(0x00240094, &tags::SensitivityValue);

		ident_to_elem.insert("RetestStimulusSeen", &tags::RetestStimulusSeen);
		tag_to_elem.insert(0x00240095, &tags::RetestStimulusSeen);

		ident_to_elem.insert("RetestSensitivityValue", &tags::RetestSensitivityValue);
		tag_to_elem.insert(0x00240096, &tags::RetestSensitivityValue);

		ident_to_elem.insert("VisualFieldTestPointNormalsSequence", &tags::VisualFieldTestPointNormalsSequence);
		tag_to_elem.insert(0x00240097, &tags::VisualFieldTestPointNormalsSequence);

		ident_to_elem.insert("QuantifiedDefect", &tags::QuantifiedDefect);
		tag_to_elem.insert(0x00240098, &tags::QuantifiedDefect);

		ident_to_elem.insert("AgeCorrectedSensitivityDeviationProbabilityValue", &tags::AgeCorrectedSensitivityDeviationProbabilityValue);
		tag_to_elem.insert(0x00240100, &tags::AgeCorrectedSensitivityDeviationProbabilityValue);

		ident_to_elem.insert("GeneralizedDefectCorrectedSensitivityDeviationFlag", &tags::GeneralizedDefectCorrectedSensitivityDeviationFlag);
		tag_to_elem.insert(0x00240102, &tags::GeneralizedDefectCorrectedSensitivityDeviationFlag);

		ident_to_elem.insert("GeneralizedDefectCorrectedSensitivityDeviationValue", &tags::GeneralizedDefectCorrectedSensitivityDeviationValue);
		tag_to_elem.insert(0x00240103, &tags::GeneralizedDefectCorrectedSensitivityDeviationValue);

		ident_to_elem.insert("GeneralizedDefectCorrectedSensitivityDeviationProbabilityValue", &tags::GeneralizedDefectCorrectedSensitivityDeviationProbabilityValue);
		tag_to_elem.insert(0x00240104, &tags::GeneralizedDefectCorrectedSensitivityDeviationProbabilityValue);

		ident_to_elem.insert("MinimumSensitivityValue", &tags::MinimumSensitivityValue);
		tag_to_elem.insert(0x00240105, &tags::MinimumSensitivityValue);

		ident_to_elem.insert("BlindSpotLocalized", &tags::BlindSpotLocalized);
		tag_to_elem.insert(0x00240106, &tags::BlindSpotLocalized);

		ident_to_elem.insert("BlindSpotXCoordinate", &tags::BlindSpotXCoordinate);
		tag_to_elem.insert(0x00240107, &tags::BlindSpotXCoordinate);

		ident_to_elem.insert("BlindSpotYCoordinate", &tags::BlindSpotYCoordinate);
		tag_to_elem.insert(0x00240108, &tags::BlindSpotYCoordinate);

		ident_to_elem.insert("VisualAcuityMeasurementSequence", &tags::VisualAcuityMeasurementSequence);
		tag_to_elem.insert(0x00240110, &tags::VisualAcuityMeasurementSequence);

		ident_to_elem.insert("RefractiveParametersUsedOnPatientSequence", &tags::RefractiveParametersUsedOnPatientSequence);
		tag_to_elem.insert(0x00240112, &tags::RefractiveParametersUsedOnPatientSequence);

		ident_to_elem.insert("MeasurementLaterality", &tags::MeasurementLaterality);
		tag_to_elem.insert(0x00240113, &tags::MeasurementLaterality);

		ident_to_elem.insert("OphthalmicPatientClinicalInformationLeftEyeSequence", &tags::OphthalmicPatientClinicalInformationLeftEyeSequence);
		tag_to_elem.insert(0x00240114, &tags::OphthalmicPatientClinicalInformationLeftEyeSequence);

		ident_to_elem.insert("OphthalmicPatientClinicalInformationRightEyeSequence", &tags::OphthalmicPatientClinicalInformationRightEyeSequence);
		tag_to_elem.insert(0x00240115, &tags::OphthalmicPatientClinicalInformationRightEyeSequence);

		ident_to_elem.insert("FovealPointNormativeDataFlag", &tags::FovealPointNormativeDataFlag);
		tag_to_elem.insert(0x00240117, &tags::FovealPointNormativeDataFlag);

		ident_to_elem.insert("FovealPointProbabilityValue", &tags::FovealPointProbabilityValue);
		tag_to_elem.insert(0x00240118, &tags::FovealPointProbabilityValue);

		ident_to_elem.insert("ScreeningBaselineMeasured", &tags::ScreeningBaselineMeasured);
		tag_to_elem.insert(0x00240120, &tags::ScreeningBaselineMeasured);

		ident_to_elem.insert("ScreeningBaselineMeasuredSequence", &tags::ScreeningBaselineMeasuredSequence);
		tag_to_elem.insert(0x00240122, &tags::ScreeningBaselineMeasuredSequence);

		ident_to_elem.insert("ScreeningBaselineType", &tags::ScreeningBaselineType);
		tag_to_elem.insert(0x00240124, &tags::ScreeningBaselineType);

		ident_to_elem.insert("ScreeningBaselineValue", &tags::ScreeningBaselineValue);
		tag_to_elem.insert(0x00240126, &tags::ScreeningBaselineValue);

		ident_to_elem.insert("AlgorithmSource", &tags::AlgorithmSource);
		tag_to_elem.insert(0x00240202, &tags::AlgorithmSource);

		ident_to_elem.insert("DataSetName", &tags::DataSetName);
		tag_to_elem.insert(0x00240306, &tags::DataSetName);

		ident_to_elem.insert("DataSetVersion", &tags::DataSetVersion);
		tag_to_elem.insert(0x00240307, &tags::DataSetVersion);

		ident_to_elem.insert("DataSetSource", &tags::DataSetSource);
		tag_to_elem.insert(0x00240308, &tags::DataSetSource);

		ident_to_elem.insert("DataSetDescription", &tags::DataSetDescription);
		tag_to_elem.insert(0x00240309, &tags::DataSetDescription);

		ident_to_elem.insert("VisualFieldTestReliabilityGlobalIndexSequence", &tags::VisualFieldTestReliabilityGlobalIndexSequence);
		tag_to_elem.insert(0x00240317, &tags::VisualFieldTestReliabilityGlobalIndexSequence);

		ident_to_elem.insert("VisualFieldGlobalResultsIndexSequence", &tags::VisualFieldGlobalResultsIndexSequence);
		tag_to_elem.insert(0x00240320, &tags::VisualFieldGlobalResultsIndexSequence);

		ident_to_elem.insert("DataObservationSequence", &tags::DataObservationSequence);
		tag_to_elem.insert(0x00240325, &tags::DataObservationSequence);

		ident_to_elem.insert("IndexNormalsFlag", &tags::IndexNormalsFlag);
		tag_to_elem.insert(0x00240338, &tags::IndexNormalsFlag);

		ident_to_elem.insert("IndexProbability", &tags::IndexProbability);
		tag_to_elem.insert(0x00240341, &tags::IndexProbability);

		ident_to_elem.insert("IndexProbabilitySequence", &tags::IndexProbabilitySequence);
		tag_to_elem.insert(0x00240344, &tags::IndexProbabilitySequence);

		ident_to_elem.insert("SamplesPerPixel", &tags::SamplesPerPixel);
		tag_to_elem.insert(0x00280002, &tags::SamplesPerPixel);

		ident_to_elem.insert("SamplesPerPixelUsed", &tags::SamplesPerPixelUsed);
		tag_to_elem.insert(0x00280003, &tags::SamplesPerPixelUsed);

		ident_to_elem.insert("PhotometricInterpretation", &tags::PhotometricInterpretation);
		tag_to_elem.insert(0x00280004, &tags::PhotometricInterpretation);

		ident_to_elem.insert("ImageDimensions", &tags::ImageDimensions);
		tag_to_elem.insert(0x00280005, &tags::ImageDimensions);

		ident_to_elem.insert("PlanarConfiguration", &tags::PlanarConfiguration);
		tag_to_elem.insert(0x00280006, &tags::PlanarConfiguration);

		ident_to_elem.insert("NumberOfFrames", &tags::NumberOfFrames);
		tag_to_elem.insert(0x00280008, &tags::NumberOfFrames);

		ident_to_elem.insert("FrameIncrementPointer", &tags::FrameIncrementPointer);
		tag_to_elem.insert(0x00280009, &tags::FrameIncrementPointer);

		ident_to_elem.insert("FrameDimensionPointer", &tags::FrameDimensionPointer);
		tag_to_elem.insert(0x0028000A, &tags::FrameDimensionPointer);

		ident_to_elem.insert("Rows", &tags::Rows);
		tag_to_elem.insert(0x00280010, &tags::Rows);

		ident_to_elem.insert("Columns", &tags::Columns);
		tag_to_elem.insert(0x00280011, &tags::Columns);

		ident_to_elem.insert("Planes", &tags::Planes);
		tag_to_elem.insert(0x00280012, &tags::Planes);

		ident_to_elem.insert("UltrasoundColorDataPresent", &tags::UltrasoundColorDataPresent);
		tag_to_elem.insert(0x00280014, &tags::UltrasoundColorDataPresent);

		ident_to_elem.insert("PixelSpacing", &tags::PixelSpacing);
		tag_to_elem.insert(0x00280030, &tags::PixelSpacing);

		ident_to_elem.insert("ZoomFactor", &tags::ZoomFactor);
		tag_to_elem.insert(0x00280031, &tags::ZoomFactor);

		ident_to_elem.insert("ZoomCenter", &tags::ZoomCenter);
		tag_to_elem.insert(0x00280032, &tags::ZoomCenter);

		ident_to_elem.insert("PixelAspectRatio", &tags::PixelAspectRatio);
		tag_to_elem.insert(0x00280034, &tags::PixelAspectRatio);

		ident_to_elem.insert("ImageFormat", &tags::ImageFormat);
		tag_to_elem.insert(0x00280040, &tags::ImageFormat);

		ident_to_elem.insert("ManipulatedImage", &tags::ManipulatedImage);
		tag_to_elem.insert(0x00280050, &tags::ManipulatedImage);

		ident_to_elem.insert("CorrectedImage", &tags::CorrectedImage);
		tag_to_elem.insert(0x00280051, &tags::CorrectedImage);

		ident_to_elem.insert("CompressionRecognitionCode", &tags::CompressionRecognitionCode);
		tag_to_elem.insert(0x0028005F, &tags::CompressionRecognitionCode);

		ident_to_elem.insert("CompressionCode", &tags::CompressionCode);
		tag_to_elem.insert(0x00280060, &tags::CompressionCode);

		ident_to_elem.insert("CompressionOriginator", &tags::CompressionOriginator);
		tag_to_elem.insert(0x00280061, &tags::CompressionOriginator);

		ident_to_elem.insert("CompressionLabel", &tags::CompressionLabel);
		tag_to_elem.insert(0x00280062, &tags::CompressionLabel);

		ident_to_elem.insert("CompressionDescription", &tags::CompressionDescription);
		tag_to_elem.insert(0x00280063, &tags::CompressionDescription);

		ident_to_elem.insert("CompressionSequence", &tags::CompressionSequence);
		tag_to_elem.insert(0x00280065, &tags::CompressionSequence);

		ident_to_elem.insert("CompressionStepPointers", &tags::CompressionStepPointers);
		tag_to_elem.insert(0x00280066, &tags::CompressionStepPointers);

		ident_to_elem.insert("RepeatInterval", &tags::RepeatInterval);
		tag_to_elem.insert(0x00280068, &tags::RepeatInterval);

		ident_to_elem.insert("BitsGrouped", &tags::BitsGrouped);
		tag_to_elem.insert(0x00280069, &tags::BitsGrouped);

		ident_to_elem.insert("PerimeterTable", &tags::PerimeterTable);
		tag_to_elem.insert(0x00280070, &tags::PerimeterTable);

		ident_to_elem.insert("PerimeterValue", &tags::PerimeterValue);
		tag_to_elem.insert(0x00280071, &tags::PerimeterValue);

		ident_to_elem.insert("PredictorRows", &tags::PredictorRows);
		tag_to_elem.insert(0x00280080, &tags::PredictorRows);

		ident_to_elem.insert("PredictorColumns", &tags::PredictorColumns);
		tag_to_elem.insert(0x00280081, &tags::PredictorColumns);

		ident_to_elem.insert("PredictorConstants", &tags::PredictorConstants);
		tag_to_elem.insert(0x00280082, &tags::PredictorConstants);

		ident_to_elem.insert("BlockedPixels", &tags::BlockedPixels);
		tag_to_elem.insert(0x00280090, &tags::BlockedPixels);

		ident_to_elem.insert("BlockRows", &tags::BlockRows);
		tag_to_elem.insert(0x00280091, &tags::BlockRows);

		ident_to_elem.insert("BlockColumns", &tags::BlockColumns);
		tag_to_elem.insert(0x00280092, &tags::BlockColumns);

		ident_to_elem.insert("RowOverlap", &tags::RowOverlap);
		tag_to_elem.insert(0x00280093, &tags::RowOverlap);

		ident_to_elem.insert("ColumnOverlap", &tags::ColumnOverlap);
		tag_to_elem.insert(0x00280094, &tags::ColumnOverlap);

		ident_to_elem.insert("BitsAllocated", &tags::BitsAllocated);
		tag_to_elem.insert(0x00280100, &tags::BitsAllocated);

		ident_to_elem.insert("BitsStored", &tags::BitsStored);
		tag_to_elem.insert(0x00280101, &tags::BitsStored);

		ident_to_elem.insert("HighBit", &tags::HighBit);
		tag_to_elem.insert(0x00280102, &tags::HighBit);

		ident_to_elem.insert("PixelRepresentation", &tags::PixelRepresentation);
		tag_to_elem.insert(0x00280103, &tags::PixelRepresentation);

		ident_to_elem.insert("SmallestValidPixelValue", &tags::SmallestValidPixelValue);
		tag_to_elem.insert(0x00280104, &tags::SmallestValidPixelValue);

		ident_to_elem.insert("LargestValidPixelValue", &tags::LargestValidPixelValue);
		tag_to_elem.insert(0x00280105, &tags::LargestValidPixelValue);

		ident_to_elem.insert("SmallestImagePixelValue", &tags::SmallestImagePixelValue);
		tag_to_elem.insert(0x00280106, &tags::SmallestImagePixelValue);

		ident_to_elem.insert("LargestImagePixelValue", &tags::LargestImagePixelValue);
		tag_to_elem.insert(0x00280107, &tags::LargestImagePixelValue);

		ident_to_elem.insert("SmallestPixelValueInSeries", &tags::SmallestPixelValueInSeries);
		tag_to_elem.insert(0x00280108, &tags::SmallestPixelValueInSeries);

		ident_to_elem.insert("LargestPixelValueInSeries", &tags::LargestPixelValueInSeries);
		tag_to_elem.insert(0x00280109, &tags::LargestPixelValueInSeries);

		ident_to_elem.insert("SmallestImagePixelValueInPlane", &tags::SmallestImagePixelValueInPlane);
		tag_to_elem.insert(0x00280110, &tags::SmallestImagePixelValueInPlane);

		ident_to_elem.insert("LargestImagePixelValueInPlane", &tags::LargestImagePixelValueInPlane);
		tag_to_elem.insert(0x00280111, &tags::LargestImagePixelValueInPlane);

		ident_to_elem.insert("PixelPaddingValue", &tags::PixelPaddingValue);
		tag_to_elem.insert(0x00280120, &tags::PixelPaddingValue);

		ident_to_elem.insert("PixelPaddingRangeLimit", &tags::PixelPaddingRangeLimit);
		tag_to_elem.insert(0x00280121, &tags::PixelPaddingRangeLimit);

		ident_to_elem.insert("FloatPixelPaddingValue", &tags::FloatPixelPaddingValue);
		tag_to_elem.insert(0x00280122, &tags::FloatPixelPaddingValue);

		ident_to_elem.insert("DoubleFloatPixelPaddingValue", &tags::DoubleFloatPixelPaddingValue);
		tag_to_elem.insert(0x00280123, &tags::DoubleFloatPixelPaddingValue);

		ident_to_elem.insert("FloatPixelPaddingRangeLimit", &tags::FloatPixelPaddingRangeLimit);
		tag_to_elem.insert(0x00280124, &tags::FloatPixelPaddingRangeLimit);

		ident_to_elem.insert("DoubleFloatPixelPaddingRangeLimit", &tags::DoubleFloatPixelPaddingRangeLimit);
		tag_to_elem.insert(0x00280125, &tags::DoubleFloatPixelPaddingRangeLimit);

		ident_to_elem.insert("ImageLocation", &tags::ImageLocation);
		tag_to_elem.insert(0x00280200, &tags::ImageLocation);

		ident_to_elem.insert("QualityControlImage", &tags::QualityControlImage);
		tag_to_elem.insert(0x00280300, &tags::QualityControlImage);

		ident_to_elem.insert("BurnedInAnnotation", &tags::BurnedInAnnotation);
		tag_to_elem.insert(0x00280301, &tags::BurnedInAnnotation);

		ident_to_elem.insert("RecognizableVisualFeatures", &tags::RecognizableVisualFeatures);
		tag_to_elem.insert(0x00280302, &tags::RecognizableVisualFeatures);

		ident_to_elem.insert("LongitudinalTemporalInformationModified", &tags::LongitudinalTemporalInformationModified);
		tag_to_elem.insert(0x00280303, &tags::LongitudinalTemporalInformationModified);

		ident_to_elem.insert("ReferencedColorPaletteInstanceUID", &tags::ReferencedColorPaletteInstanceUID);
		tag_to_elem.insert(0x00280304, &tags::ReferencedColorPaletteInstanceUID);

		ident_to_elem.insert("TransformLabel", &tags::TransformLabel);
		tag_to_elem.insert(0x00280400, &tags::TransformLabel);

		ident_to_elem.insert("TransformVersionNumber", &tags::TransformVersionNumber);
		tag_to_elem.insert(0x00280401, &tags::TransformVersionNumber);

		ident_to_elem.insert("NumberOfTransformSteps", &tags::NumberOfTransformSteps);
		tag_to_elem.insert(0x00280402, &tags::NumberOfTransformSteps);

		ident_to_elem.insert("SequenceOfCompressedData", &tags::SequenceOfCompressedData);
		tag_to_elem.insert(0x00280403, &tags::SequenceOfCompressedData);

		ident_to_elem.insert("DetailsOfCoefficients", &tags::DetailsOfCoefficients);
		tag_to_elem.insert(0x00280404, &tags::DetailsOfCoefficients);

		ident_to_elem.insert("DCTLabel", &tags::DCTLabel);
		tag_to_elem.insert(0x00280700, &tags::DCTLabel);

		ident_to_elem.insert("DataBlockDescription", &tags::DataBlockDescription);
		tag_to_elem.insert(0x00280701, &tags::DataBlockDescription);

		ident_to_elem.insert("DataBlock", &tags::DataBlock);
		tag_to_elem.insert(0x00280702, &tags::DataBlock);

		ident_to_elem.insert("NormalizationFactorFormat", &tags::NormalizationFactorFormat);
		tag_to_elem.insert(0x00280710, &tags::NormalizationFactorFormat);

		ident_to_elem.insert("ZonalMapNumberFormat", &tags::ZonalMapNumberFormat);
		tag_to_elem.insert(0x00280720, &tags::ZonalMapNumberFormat);

		ident_to_elem.insert("ZonalMapLocation", &tags::ZonalMapLocation);
		tag_to_elem.insert(0x00280721, &tags::ZonalMapLocation);

		ident_to_elem.insert("ZonalMapFormat", &tags::ZonalMapFormat);
		tag_to_elem.insert(0x00280722, &tags::ZonalMapFormat);

		ident_to_elem.insert("AdaptiveMapFormat", &tags::AdaptiveMapFormat);
		tag_to_elem.insert(0x00280730, &tags::AdaptiveMapFormat);

		ident_to_elem.insert("CodeNumberFormat", &tags::CodeNumberFormat);
		tag_to_elem.insert(0x00280740, &tags::CodeNumberFormat);

		ident_to_elem.insert("PixelSpacingCalibrationType", &tags::PixelSpacingCalibrationType);
		tag_to_elem.insert(0x00280A02, &tags::PixelSpacingCalibrationType);

		ident_to_elem.insert("PixelSpacingCalibrationDescription", &tags::PixelSpacingCalibrationDescription);
		tag_to_elem.insert(0x00280A04, &tags::PixelSpacingCalibrationDescription);

		ident_to_elem.insert("PixelIntensityRelationship", &tags::PixelIntensityRelationship);
		tag_to_elem.insert(0x00281040, &tags::PixelIntensityRelationship);

		ident_to_elem.insert("PixelIntensityRelationshipSign", &tags::PixelIntensityRelationshipSign);
		tag_to_elem.insert(0x00281041, &tags::PixelIntensityRelationshipSign);

		ident_to_elem.insert("WindowCenter", &tags::WindowCenter);
		tag_to_elem.insert(0x00281050, &tags::WindowCenter);

		ident_to_elem.insert("WindowWidth", &tags::WindowWidth);
		tag_to_elem.insert(0x00281051, &tags::WindowWidth);

		ident_to_elem.insert("RescaleIntercept", &tags::RescaleIntercept);
		tag_to_elem.insert(0x00281052, &tags::RescaleIntercept);

		ident_to_elem.insert("RescaleSlope", &tags::RescaleSlope);
		tag_to_elem.insert(0x00281053, &tags::RescaleSlope);

		ident_to_elem.insert("RescaleType", &tags::RescaleType);
		tag_to_elem.insert(0x00281054, &tags::RescaleType);

		ident_to_elem.insert("WindowCenterWidthExplanation", &tags::WindowCenterWidthExplanation);
		tag_to_elem.insert(0x00281055, &tags::WindowCenterWidthExplanation);

		ident_to_elem.insert("VOILUTFunction", &tags::VOILUTFunction);
		tag_to_elem.insert(0x00281056, &tags::VOILUTFunction);

		ident_to_elem.insert("GrayScale", &tags::GrayScale);
		tag_to_elem.insert(0x00281080, &tags::GrayScale);

		ident_to_elem.insert("RecommendedViewingMode", &tags::RecommendedViewingMode);
		tag_to_elem.insert(0x00281090, &tags::RecommendedViewingMode);

		ident_to_elem.insert("GrayLookupTableDescriptor", &tags::GrayLookupTableDescriptor);
		tag_to_elem.insert(0x00281100, &tags::GrayLookupTableDescriptor);

		ident_to_elem.insert("RedPaletteColorLookupTableDescriptor", &tags::RedPaletteColorLookupTableDescriptor);
		tag_to_elem.insert(0x00281101, &tags::RedPaletteColorLookupTableDescriptor);

		ident_to_elem.insert("GreenPaletteColorLookupTableDescriptor", &tags::GreenPaletteColorLookupTableDescriptor);
		tag_to_elem.insert(0x00281102, &tags::GreenPaletteColorLookupTableDescriptor);

		ident_to_elem.insert("BluePaletteColorLookupTableDescriptor", &tags::BluePaletteColorLookupTableDescriptor);
		tag_to_elem.insert(0x00281103, &tags::BluePaletteColorLookupTableDescriptor);

		ident_to_elem.insert("AlphaPaletteColorLookupTableDescriptor", &tags::AlphaPaletteColorLookupTableDescriptor);
		tag_to_elem.insert(0x00281104, &tags::AlphaPaletteColorLookupTableDescriptor);

		ident_to_elem.insert("LargeRedPaletteColorLookupTableDescriptor", &tags::LargeRedPaletteColorLookupTableDescriptor);
		tag_to_elem.insert(0x00281111, &tags::LargeRedPaletteColorLookupTableDescriptor);

		ident_to_elem.insert("LargeGreenPaletteColorLookupTableDescriptor", &tags::LargeGreenPaletteColorLookupTableDescriptor);
		tag_to_elem.insert(0x00281112, &tags::LargeGreenPaletteColorLookupTableDescriptor);

		ident_to_elem.insert("LargeBluePaletteColorLookupTableDescriptor", &tags::LargeBluePaletteColorLookupTableDescriptor);
		tag_to_elem.insert(0x00281113, &tags::LargeBluePaletteColorLookupTableDescriptor);

		ident_to_elem.insert("PaletteColorLookupTableUID", &tags::PaletteColorLookupTableUID);
		tag_to_elem.insert(0x00281199, &tags::PaletteColorLookupTableUID);

		ident_to_elem.insert("GrayLookupTableData", &tags::GrayLookupTableData);
		tag_to_elem.insert(0x00281200, &tags::GrayLookupTableData);

		ident_to_elem.insert("RedPaletteColorLookupTableData", &tags::RedPaletteColorLookupTableData);
		tag_to_elem.insert(0x00281201, &tags::RedPaletteColorLookupTableData);

		ident_to_elem.insert("GreenPaletteColorLookupTableData", &tags::GreenPaletteColorLookupTableData);
		tag_to_elem.insert(0x00281202, &tags::GreenPaletteColorLookupTableData);

		ident_to_elem.insert("BluePaletteColorLookupTableData", &tags::BluePaletteColorLookupTableData);
		tag_to_elem.insert(0x00281203, &tags::BluePaletteColorLookupTableData);

		ident_to_elem.insert("AlphaPaletteColorLookupTableData", &tags::AlphaPaletteColorLookupTableData);
		tag_to_elem.insert(0x00281204, &tags::AlphaPaletteColorLookupTableData);

		ident_to_elem.insert("LargeRedPaletteColorLookupTableData", &tags::LargeRedPaletteColorLookupTableData);
		tag_to_elem.insert(0x00281211, &tags::LargeRedPaletteColorLookupTableData);

		ident_to_elem.insert("LargeGreenPaletteColorLookupTableData", &tags::LargeGreenPaletteColorLookupTableData);
		tag_to_elem.insert(0x00281212, &tags::LargeGreenPaletteColorLookupTableData);

		ident_to_elem.insert("LargeBluePaletteColorLookupTableData", &tags::LargeBluePaletteColorLookupTableData);
		tag_to_elem.insert(0x00281213, &tags::LargeBluePaletteColorLookupTableData);

		ident_to_elem.insert("LargePaletteColorLookupTableUID", &tags::LargePaletteColorLookupTableUID);
		tag_to_elem.insert(0x00281214, &tags::LargePaletteColorLookupTableUID);

		ident_to_elem.insert("SegmentedRedPaletteColorLookupTableData", &tags::SegmentedRedPaletteColorLookupTableData);
		tag_to_elem.insert(0x00281221, &tags::SegmentedRedPaletteColorLookupTableData);

		ident_to_elem.insert("SegmentedGreenPaletteColorLookupTableData", &tags::SegmentedGreenPaletteColorLookupTableData);
		tag_to_elem.insert(0x00281222, &tags::SegmentedGreenPaletteColorLookupTableData);

		ident_to_elem.insert("SegmentedBluePaletteColorLookupTableData", &tags::SegmentedBluePaletteColorLookupTableData);
		tag_to_elem.insert(0x00281223, &tags::SegmentedBluePaletteColorLookupTableData);

		ident_to_elem.insert("SegmentedAlphaPaletteColorLookupTableData", &tags::SegmentedAlphaPaletteColorLookupTableData);
		tag_to_elem.insert(0x00281224, &tags::SegmentedAlphaPaletteColorLookupTableData);

		ident_to_elem.insert("StoredValueColorRangeSequence", &tags::StoredValueColorRangeSequence);
		tag_to_elem.insert(0x00281230, &tags::StoredValueColorRangeSequence);

		ident_to_elem.insert("MinimumStoredValueMapped", &tags::MinimumStoredValueMapped);
		tag_to_elem.insert(0x00281231, &tags::MinimumStoredValueMapped);

		ident_to_elem.insert("MaximumStoredValueMapped", &tags::MaximumStoredValueMapped);
		tag_to_elem.insert(0x00281232, &tags::MaximumStoredValueMapped);

		ident_to_elem.insert("BreastImplantPresent", &tags::BreastImplantPresent);
		tag_to_elem.insert(0x00281300, &tags::BreastImplantPresent);

		ident_to_elem.insert("PartialView", &tags::PartialView);
		tag_to_elem.insert(0x00281350, &tags::PartialView);

		ident_to_elem.insert("PartialViewDescription", &tags::PartialViewDescription);
		tag_to_elem.insert(0x00281351, &tags::PartialViewDescription);

		ident_to_elem.insert("PartialViewCodeSequence", &tags::PartialViewCodeSequence);
		tag_to_elem.insert(0x00281352, &tags::PartialViewCodeSequence);

		ident_to_elem.insert("SpatialLocationsPreserved", &tags::SpatialLocationsPreserved);
		tag_to_elem.insert(0x0028135A, &tags::SpatialLocationsPreserved);

		ident_to_elem.insert("DataFrameAssignmentSequence", &tags::DataFrameAssignmentSequence);
		tag_to_elem.insert(0x00281401, &tags::DataFrameAssignmentSequence);

		ident_to_elem.insert("DataPathAssignment", &tags::DataPathAssignment);
		tag_to_elem.insert(0x00281402, &tags::DataPathAssignment);

		ident_to_elem.insert("BitsMappedToColorLookupTable", &tags::BitsMappedToColorLookupTable);
		tag_to_elem.insert(0x00281403, &tags::BitsMappedToColorLookupTable);

		ident_to_elem.insert("BlendingLUT1Sequence", &tags::BlendingLUT1Sequence);
		tag_to_elem.insert(0x00281404, &tags::BlendingLUT1Sequence);

		ident_to_elem.insert("BlendingLUT1TransferFunction", &tags::BlendingLUT1TransferFunction);
		tag_to_elem.insert(0x00281405, &tags::BlendingLUT1TransferFunction);

		ident_to_elem.insert("BlendingWeightConstant", &tags::BlendingWeightConstant);
		tag_to_elem.insert(0x00281406, &tags::BlendingWeightConstant);

		ident_to_elem.insert("BlendingLookupTableDescriptor", &tags::BlendingLookupTableDescriptor);
		tag_to_elem.insert(0x00281407, &tags::BlendingLookupTableDescriptor);

		ident_to_elem.insert("BlendingLookupTableData", &tags::BlendingLookupTableData);
		tag_to_elem.insert(0x00281408, &tags::BlendingLookupTableData);

		ident_to_elem.insert("EnhancedPaletteColorLookupTableSequence", &tags::EnhancedPaletteColorLookupTableSequence);
		tag_to_elem.insert(0x0028140B, &tags::EnhancedPaletteColorLookupTableSequence);

		ident_to_elem.insert("BlendingLUT2Sequence", &tags::BlendingLUT2Sequence);
		tag_to_elem.insert(0x0028140C, &tags::BlendingLUT2Sequence);

		ident_to_elem.insert("BlendingLUT2TransferFunction", &tags::BlendingLUT2TransferFunction);
		tag_to_elem.insert(0x0028140D, &tags::BlendingLUT2TransferFunction);

		ident_to_elem.insert("DataPathID", &tags::DataPathID);
		tag_to_elem.insert(0x0028140E, &tags::DataPathID);

		ident_to_elem.insert("RGBLUTTransferFunction", &tags::RGBLUTTransferFunction);
		tag_to_elem.insert(0x0028140F, &tags::RGBLUTTransferFunction);

		ident_to_elem.insert("AlphaLUTTransferFunction", &tags::AlphaLUTTransferFunction);
		tag_to_elem.insert(0x00281410, &tags::AlphaLUTTransferFunction);

		ident_to_elem.insert("ICCProfile", &tags::ICCProfile);
		tag_to_elem.insert(0x00282000, &tags::ICCProfile);

		ident_to_elem.insert("ColorSpace", &tags::ColorSpace);
		tag_to_elem.insert(0x00282002, &tags::ColorSpace);

		ident_to_elem.insert("LossyImageCompression", &tags::LossyImageCompression);
		tag_to_elem.insert(0x00282110, &tags::LossyImageCompression);

		ident_to_elem.insert("LossyImageCompressionRatio", &tags::LossyImageCompressionRatio);
		tag_to_elem.insert(0x00282112, &tags::LossyImageCompressionRatio);

		ident_to_elem.insert("LossyImageCompressionMethod", &tags::LossyImageCompressionMethod);
		tag_to_elem.insert(0x00282114, &tags::LossyImageCompressionMethod);

		ident_to_elem.insert("ModalityLUTSequence", &tags::ModalityLUTSequence);
		tag_to_elem.insert(0x00283000, &tags::ModalityLUTSequence);

		ident_to_elem.insert("LUTDescriptor", &tags::LUTDescriptor);
		tag_to_elem.insert(0x00283002, &tags::LUTDescriptor);

		ident_to_elem.insert("LUTExplanation", &tags::LUTExplanation);
		tag_to_elem.insert(0x00283003, &tags::LUTExplanation);

		ident_to_elem.insert("ModalityLUTType", &tags::ModalityLUTType);
		tag_to_elem.insert(0x00283004, &tags::ModalityLUTType);

		ident_to_elem.insert("LUTData", &tags::LUTData);
		tag_to_elem.insert(0x00283006, &tags::LUTData);

		ident_to_elem.insert("VOILUTSequence", &tags::VOILUTSequence);
		tag_to_elem.insert(0x00283010, &tags::VOILUTSequence);

		ident_to_elem.insert("SoftcopyVOILUTSequence", &tags::SoftcopyVOILUTSequence);
		tag_to_elem.insert(0x00283110, &tags::SoftcopyVOILUTSequence);

		ident_to_elem.insert("ImagePresentationComments", &tags::ImagePresentationComments);
		tag_to_elem.insert(0x00284000, &tags::ImagePresentationComments);

		ident_to_elem.insert("BiPlaneAcquisitionSequence", &tags::BiPlaneAcquisitionSequence);
		tag_to_elem.insert(0x00285000, &tags::BiPlaneAcquisitionSequence);

		ident_to_elem.insert("RepresentativeFrameNumber", &tags::RepresentativeFrameNumber);
		tag_to_elem.insert(0x00286010, &tags::RepresentativeFrameNumber);

		ident_to_elem.insert("FrameNumbersOfInterest", &tags::FrameNumbersOfInterest);
		tag_to_elem.insert(0x00286020, &tags::FrameNumbersOfInterest);

		ident_to_elem.insert("FrameOfInterestDescription", &tags::FrameOfInterestDescription);
		tag_to_elem.insert(0x00286022, &tags::FrameOfInterestDescription);

		ident_to_elem.insert("FrameOfInterestType", &tags::FrameOfInterestType);
		tag_to_elem.insert(0x00286023, &tags::FrameOfInterestType);

		ident_to_elem.insert("MaskPointers", &tags::MaskPointers);
		tag_to_elem.insert(0x00286030, &tags::MaskPointers);

		ident_to_elem.insert("RWavePointer", &tags::RWavePointer);
		tag_to_elem.insert(0x00286040, &tags::RWavePointer);

		ident_to_elem.insert("MaskSubtractionSequence", &tags::MaskSubtractionSequence);
		tag_to_elem.insert(0x00286100, &tags::MaskSubtractionSequence);

		ident_to_elem.insert("MaskOperation", &tags::MaskOperation);
		tag_to_elem.insert(0x00286101, &tags::MaskOperation);

		ident_to_elem.insert("ApplicableFrameRange", &tags::ApplicableFrameRange);
		tag_to_elem.insert(0x00286102, &tags::ApplicableFrameRange);

		ident_to_elem.insert("MaskFrameNumbers", &tags::MaskFrameNumbers);
		tag_to_elem.insert(0x00286110, &tags::MaskFrameNumbers);

		ident_to_elem.insert("ContrastFrameAveraging", &tags::ContrastFrameAveraging);
		tag_to_elem.insert(0x00286112, &tags::ContrastFrameAveraging);

		ident_to_elem.insert("MaskSubPixelShift", &tags::MaskSubPixelShift);
		tag_to_elem.insert(0x00286114, &tags::MaskSubPixelShift);

		ident_to_elem.insert("TIDOffset", &tags::TIDOffset);
		tag_to_elem.insert(0x00286120, &tags::TIDOffset);

		ident_to_elem.insert("MaskOperationExplanation", &tags::MaskOperationExplanation);
		tag_to_elem.insert(0x00286190, &tags::MaskOperationExplanation);

		ident_to_elem.insert("EquipmentAdministratorSequence", &tags::EquipmentAdministratorSequence);
		tag_to_elem.insert(0x00287000, &tags::EquipmentAdministratorSequence);

		ident_to_elem.insert("NumberOfDisplaySubsystems", &tags::NumberOfDisplaySubsystems);
		tag_to_elem.insert(0x00287001, &tags::NumberOfDisplaySubsystems);

		ident_to_elem.insert("CurrentConfigurationID", &tags::CurrentConfigurationID);
		tag_to_elem.insert(0x00287002, &tags::CurrentConfigurationID);

		ident_to_elem.insert("DisplaySubsystemID", &tags::DisplaySubsystemID);
		tag_to_elem.insert(0x00287003, &tags::DisplaySubsystemID);

		ident_to_elem.insert("DisplaySubsystemName", &tags::DisplaySubsystemName);
		tag_to_elem.insert(0x00287004, &tags::DisplaySubsystemName);

		ident_to_elem.insert("DisplaySubsystemDescription", &tags::DisplaySubsystemDescription);
		tag_to_elem.insert(0x00287005, &tags::DisplaySubsystemDescription);

		ident_to_elem.insert("SystemStatus", &tags::SystemStatus);
		tag_to_elem.insert(0x00287006, &tags::SystemStatus);

		ident_to_elem.insert("SystemStatusComment", &tags::SystemStatusComment);
		tag_to_elem.insert(0x00287007, &tags::SystemStatusComment);

		ident_to_elem.insert("TargetLuminanceCharacteristicsSequence", &tags::TargetLuminanceCharacteristicsSequence);
		tag_to_elem.insert(0x00287008, &tags::TargetLuminanceCharacteristicsSequence);

		ident_to_elem.insert("LuminanceCharacteristicsID", &tags::LuminanceCharacteristicsID);
		tag_to_elem.insert(0x00287009, &tags::LuminanceCharacteristicsID);

		ident_to_elem.insert("DisplaySubsystemConfigurationSequence", &tags::DisplaySubsystemConfigurationSequence);
		tag_to_elem.insert(0x0028700A, &tags::DisplaySubsystemConfigurationSequence);

		ident_to_elem.insert("ConfigurationID", &tags::ConfigurationID);
		tag_to_elem.insert(0x0028700B, &tags::ConfigurationID);

		ident_to_elem.insert("ConfigurationName", &tags::ConfigurationName);
		tag_to_elem.insert(0x0028700C, &tags::ConfigurationName);

		ident_to_elem.insert("ConfigurationDescription", &tags::ConfigurationDescription);
		tag_to_elem.insert(0x0028700D, &tags::ConfigurationDescription);

		ident_to_elem.insert("ReferencedTargetLuminanceCharacteristicsID", &tags::ReferencedTargetLuminanceCharacteristicsID);
		tag_to_elem.insert(0x0028700E, &tags::ReferencedTargetLuminanceCharacteristicsID);

		ident_to_elem.insert("QAResultsSequence", &tags::QAResultsSequence);
		tag_to_elem.insert(0x0028700F, &tags::QAResultsSequence);

		ident_to_elem.insert("DisplaySubsystemQAResultsSequence", &tags::DisplaySubsystemQAResultsSequence);
		tag_to_elem.insert(0x00287010, &tags::DisplaySubsystemQAResultsSequence);

		ident_to_elem.insert("ConfigurationQAResultsSequence", &tags::ConfigurationQAResultsSequence);
		tag_to_elem.insert(0x00287011, &tags::ConfigurationQAResultsSequence);

		ident_to_elem.insert("MeasurementEquipmentSequence", &tags::MeasurementEquipmentSequence);
		tag_to_elem.insert(0x00287012, &tags::MeasurementEquipmentSequence);

		ident_to_elem.insert("MeasurementFunctions", &tags::MeasurementFunctions);
		tag_to_elem.insert(0x00287013, &tags::MeasurementFunctions);

		ident_to_elem.insert("MeasurementEquipmentType", &tags::MeasurementEquipmentType);
		tag_to_elem.insert(0x00287014, &tags::MeasurementEquipmentType);

		ident_to_elem.insert("VisualEvaluationResultSequence", &tags::VisualEvaluationResultSequence);
		tag_to_elem.insert(0x00287015, &tags::VisualEvaluationResultSequence);

		ident_to_elem.insert("DisplayCalibrationResultSequence", &tags::DisplayCalibrationResultSequence);
		tag_to_elem.insert(0x00287016, &tags::DisplayCalibrationResultSequence);

		ident_to_elem.insert("DDLValue", &tags::DDLValue);
		tag_to_elem.insert(0x00287017, &tags::DDLValue);

		ident_to_elem.insert("CIExyWhitePoint", &tags::CIExyWhitePoint);
		tag_to_elem.insert(0x00287018, &tags::CIExyWhitePoint);

		ident_to_elem.insert("DisplayFunctionType", &tags::DisplayFunctionType);
		tag_to_elem.insert(0x00287019, &tags::DisplayFunctionType);

		ident_to_elem.insert("GammaValue", &tags::GammaValue);
		tag_to_elem.insert(0x0028701A, &tags::GammaValue);

		ident_to_elem.insert("NumberOfLuminancePoints", &tags::NumberOfLuminancePoints);
		tag_to_elem.insert(0x0028701B, &tags::NumberOfLuminancePoints);

		ident_to_elem.insert("LuminanceResponseSequence", &tags::LuminanceResponseSequence);
		tag_to_elem.insert(0x0028701C, &tags::LuminanceResponseSequence);

		ident_to_elem.insert("TargetMinimumLuminance", &tags::TargetMinimumLuminance);
		tag_to_elem.insert(0x0028701D, &tags::TargetMinimumLuminance);

		ident_to_elem.insert("TargetMaximumLuminance", &tags::TargetMaximumLuminance);
		tag_to_elem.insert(0x0028701E, &tags::TargetMaximumLuminance);

		ident_to_elem.insert("LuminanceValue", &tags::LuminanceValue);
		tag_to_elem.insert(0x0028701F, &tags::LuminanceValue);

		ident_to_elem.insert("LuminanceResponseDescription", &tags::LuminanceResponseDescription);
		tag_to_elem.insert(0x00287020, &tags::LuminanceResponseDescription);

		ident_to_elem.insert("WhitePointFlag", &tags::WhitePointFlag);
		tag_to_elem.insert(0x00287021, &tags::WhitePointFlag);

		ident_to_elem.insert("DisplayDeviceTypeCodeSequence", &tags::DisplayDeviceTypeCodeSequence);
		tag_to_elem.insert(0x00287022, &tags::DisplayDeviceTypeCodeSequence);

		ident_to_elem.insert("DisplaySubsystemSequence", &tags::DisplaySubsystemSequence);
		tag_to_elem.insert(0x00287023, &tags::DisplaySubsystemSequence);

		ident_to_elem.insert("LuminanceResultSequence", &tags::LuminanceResultSequence);
		tag_to_elem.insert(0x00287024, &tags::LuminanceResultSequence);

		ident_to_elem.insert("AmbientLightValueSource", &tags::AmbientLightValueSource);
		tag_to_elem.insert(0x00287025, &tags::AmbientLightValueSource);

		ident_to_elem.insert("MeasuredCharacteristics", &tags::MeasuredCharacteristics);
		tag_to_elem.insert(0x00287026, &tags::MeasuredCharacteristics);

		ident_to_elem.insert("LuminanceUniformityResultSequence", &tags::LuminanceUniformityResultSequence);
		tag_to_elem.insert(0x00287027, &tags::LuminanceUniformityResultSequence);

		ident_to_elem.insert("VisualEvaluationTestSequence", &tags::VisualEvaluationTestSequence);
		tag_to_elem.insert(0x00287028, &tags::VisualEvaluationTestSequence);

		ident_to_elem.insert("TestResult", &tags::TestResult);
		tag_to_elem.insert(0x00287029, &tags::TestResult);

		ident_to_elem.insert("TestResultComment", &tags::TestResultComment);
		tag_to_elem.insert(0x0028702A, &tags::TestResultComment);

		ident_to_elem.insert("TestImageValidation", &tags::TestImageValidation);
		tag_to_elem.insert(0x0028702B, &tags::TestImageValidation);

		ident_to_elem.insert("TestPatternCodeSequence", &tags::TestPatternCodeSequence);
		tag_to_elem.insert(0x0028702C, &tags::TestPatternCodeSequence);

		ident_to_elem.insert("MeasurementPatternCodeSequence", &tags::MeasurementPatternCodeSequence);
		tag_to_elem.insert(0x0028702D, &tags::MeasurementPatternCodeSequence);

		ident_to_elem.insert("VisualEvaluationMethodCodeSequence", &tags::VisualEvaluationMethodCodeSequence);
		tag_to_elem.insert(0x0028702E, &tags::VisualEvaluationMethodCodeSequence);

		ident_to_elem.insert("PixelDataProviderURL", &tags::PixelDataProviderURL);
		tag_to_elem.insert(0x00287FE0, &tags::PixelDataProviderURL);

		ident_to_elem.insert("DataPointRows", &tags::DataPointRows);
		tag_to_elem.insert(0x00289001, &tags::DataPointRows);

		ident_to_elem.insert("DataPointColumns", &tags::DataPointColumns);
		tag_to_elem.insert(0x00289002, &tags::DataPointColumns);

		ident_to_elem.insert("SignalDomainColumns", &tags::SignalDomainColumns);
		tag_to_elem.insert(0x00289003, &tags::SignalDomainColumns);

		ident_to_elem.insert("LargestMonochromePixelValue", &tags::LargestMonochromePixelValue);
		tag_to_elem.insert(0x00289099, &tags::LargestMonochromePixelValue);

		ident_to_elem.insert("DataRepresentation", &tags::DataRepresentation);
		tag_to_elem.insert(0x00289108, &tags::DataRepresentation);

		ident_to_elem.insert("PixelMeasuresSequence", &tags::PixelMeasuresSequence);
		tag_to_elem.insert(0x00289110, &tags::PixelMeasuresSequence);

		ident_to_elem.insert("FrameVOILUTSequence", &tags::FrameVOILUTSequence);
		tag_to_elem.insert(0x00289132, &tags::FrameVOILUTSequence);

		ident_to_elem.insert("PixelValueTransformationSequence", &tags::PixelValueTransformationSequence);
		tag_to_elem.insert(0x00289145, &tags::PixelValueTransformationSequence);

		ident_to_elem.insert("SignalDomainRows", &tags::SignalDomainRows);
		tag_to_elem.insert(0x00289235, &tags::SignalDomainRows);

		ident_to_elem.insert("DisplayFilterPercentage", &tags::DisplayFilterPercentage);
		tag_to_elem.insert(0x00289411, &tags::DisplayFilterPercentage);

		ident_to_elem.insert("FramePixelShiftSequence", &tags::FramePixelShiftSequence);
		tag_to_elem.insert(0x00289415, &tags::FramePixelShiftSequence);

		ident_to_elem.insert("SubtractionItemID", &tags::SubtractionItemID);
		tag_to_elem.insert(0x00289416, &tags::SubtractionItemID);

		ident_to_elem.insert("PixelIntensityRelationshipLUTSequence", &tags::PixelIntensityRelationshipLUTSequence);
		tag_to_elem.insert(0x00289422, &tags::PixelIntensityRelationshipLUTSequence);

		ident_to_elem.insert("FramePixelDataPropertiesSequence", &tags::FramePixelDataPropertiesSequence);
		tag_to_elem.insert(0x00289443, &tags::FramePixelDataPropertiesSequence);

		ident_to_elem.insert("GeometricalProperties", &tags::GeometricalProperties);
		tag_to_elem.insert(0x00289444, &tags::GeometricalProperties);

		ident_to_elem.insert("GeometricMaximumDistortion", &tags::GeometricMaximumDistortion);
		tag_to_elem.insert(0x00289445, &tags::GeometricMaximumDistortion);

		ident_to_elem.insert("ImageProcessingApplied", &tags::ImageProcessingApplied);
		tag_to_elem.insert(0x00289446, &tags::ImageProcessingApplied);

		ident_to_elem.insert("MaskSelectionMode", &tags::MaskSelectionMode);
		tag_to_elem.insert(0x00289454, &tags::MaskSelectionMode);

		ident_to_elem.insert("LUTFunction", &tags::LUTFunction);
		tag_to_elem.insert(0x00289474, &tags::LUTFunction);

		ident_to_elem.insert("MaskVisibilityPercentage", &tags::MaskVisibilityPercentage);
		tag_to_elem.insert(0x00289478, &tags::MaskVisibilityPercentage);

		ident_to_elem.insert("PixelShiftSequence", &tags::PixelShiftSequence);
		tag_to_elem.insert(0x00289501, &tags::PixelShiftSequence);

		ident_to_elem.insert("RegionPixelShiftSequence", &tags::RegionPixelShiftSequence);
		tag_to_elem.insert(0x00289502, &tags::RegionPixelShiftSequence);

		ident_to_elem.insert("VerticesOfTheRegion", &tags::VerticesOfTheRegion);
		tag_to_elem.insert(0x00289503, &tags::VerticesOfTheRegion);

		ident_to_elem.insert("MultiFramePresentationSequence", &tags::MultiFramePresentationSequence);
		tag_to_elem.insert(0x00289505, &tags::MultiFramePresentationSequence);

		ident_to_elem.insert("PixelShiftFrameRange", &tags::PixelShiftFrameRange);
		tag_to_elem.insert(0x00289506, &tags::PixelShiftFrameRange);

		ident_to_elem.insert("LUTFrameRange", &tags::LUTFrameRange);
		tag_to_elem.insert(0x00289507, &tags::LUTFrameRange);

		ident_to_elem.insert("ImageToEquipmentMappingMatrix", &tags::ImageToEquipmentMappingMatrix);
		tag_to_elem.insert(0x00289520, &tags::ImageToEquipmentMappingMatrix);

		ident_to_elem.insert("EquipmentCoordinateSystemIdentification", &tags::EquipmentCoordinateSystemIdentification);
		tag_to_elem.insert(0x00289537, &tags::EquipmentCoordinateSystemIdentification);

		ident_to_elem.insert("StudyStatusID", &tags::StudyStatusID);
		tag_to_elem.insert(0x0032000A, &tags::StudyStatusID);

		ident_to_elem.insert("StudyPriorityID", &tags::StudyPriorityID);
		tag_to_elem.insert(0x0032000C, &tags::StudyPriorityID);

		ident_to_elem.insert("StudyIDIssuer", &tags::StudyIDIssuer);
		tag_to_elem.insert(0x00320012, &tags::StudyIDIssuer);

		ident_to_elem.insert("StudyVerifiedDate", &tags::StudyVerifiedDate);
		tag_to_elem.insert(0x00320032, &tags::StudyVerifiedDate);

		ident_to_elem.insert("StudyVerifiedTime", &tags::StudyVerifiedTime);
		tag_to_elem.insert(0x00320033, &tags::StudyVerifiedTime);

		ident_to_elem.insert("StudyReadDate", &tags::StudyReadDate);
		tag_to_elem.insert(0x00320034, &tags::StudyReadDate);

		ident_to_elem.insert("StudyReadTime", &tags::StudyReadTime);
		tag_to_elem.insert(0x00320035, &tags::StudyReadTime);

		ident_to_elem.insert("ScheduledStudyStartDate", &tags::ScheduledStudyStartDate);
		tag_to_elem.insert(0x00321000, &tags::ScheduledStudyStartDate);

		ident_to_elem.insert("ScheduledStudyStartTime", &tags::ScheduledStudyStartTime);
		tag_to_elem.insert(0x00321001, &tags::ScheduledStudyStartTime);

		ident_to_elem.insert("ScheduledStudyStopDate", &tags::ScheduledStudyStopDate);
		tag_to_elem.insert(0x00321010, &tags::ScheduledStudyStopDate);

		ident_to_elem.insert("ScheduledStudyStopTime", &tags::ScheduledStudyStopTime);
		tag_to_elem.insert(0x00321011, &tags::ScheduledStudyStopTime);

		ident_to_elem.insert("ScheduledStudyLocation", &tags::ScheduledStudyLocation);
		tag_to_elem.insert(0x00321020, &tags::ScheduledStudyLocation);

		ident_to_elem.insert("ScheduledStudyLocationAETitle", &tags::ScheduledStudyLocationAETitle);
		tag_to_elem.insert(0x00321021, &tags::ScheduledStudyLocationAETitle);

		ident_to_elem.insert("ReasonForStudy", &tags::ReasonForStudy);
		tag_to_elem.insert(0x00321030, &tags::ReasonForStudy);

		ident_to_elem.insert("RequestingPhysicianIdentificationSequence", &tags::RequestingPhysicianIdentificationSequence);
		tag_to_elem.insert(0x00321031, &tags::RequestingPhysicianIdentificationSequence);

		ident_to_elem.insert("RequestingPhysician", &tags::RequestingPhysician);
		tag_to_elem.insert(0x00321032, &tags::RequestingPhysician);

		ident_to_elem.insert("RequestingService", &tags::RequestingService);
		tag_to_elem.insert(0x00321033, &tags::RequestingService);

		ident_to_elem.insert("RequestingServiceCodeSequence", &tags::RequestingServiceCodeSequence);
		tag_to_elem.insert(0x00321034, &tags::RequestingServiceCodeSequence);

		ident_to_elem.insert("StudyArrivalDate", &tags::StudyArrivalDate);
		tag_to_elem.insert(0x00321040, &tags::StudyArrivalDate);

		ident_to_elem.insert("StudyArrivalTime", &tags::StudyArrivalTime);
		tag_to_elem.insert(0x00321041, &tags::StudyArrivalTime);

		ident_to_elem.insert("StudyCompletionDate", &tags::StudyCompletionDate);
		tag_to_elem.insert(0x00321050, &tags::StudyCompletionDate);

		ident_to_elem.insert("StudyCompletionTime", &tags::StudyCompletionTime);
		tag_to_elem.insert(0x00321051, &tags::StudyCompletionTime);

		ident_to_elem.insert("StudyComponentStatusID", &tags::StudyComponentStatusID);
		tag_to_elem.insert(0x00321055, &tags::StudyComponentStatusID);

		ident_to_elem.insert("RequestedProcedureDescription", &tags::RequestedProcedureDescription);
		tag_to_elem.insert(0x00321060, &tags::RequestedProcedureDescription);

		ident_to_elem.insert("RequestedProcedureCodeSequence", &tags::RequestedProcedureCodeSequence);
		tag_to_elem.insert(0x00321064, &tags::RequestedProcedureCodeSequence);

		ident_to_elem.insert("RequestedContrastAgent", &tags::RequestedContrastAgent);
		tag_to_elem.insert(0x00321070, &tags::RequestedContrastAgent);

		ident_to_elem.insert("StudyComments", &tags::StudyComments);
		tag_to_elem.insert(0x00324000, &tags::StudyComments);

		ident_to_elem.insert("ReferencedPatientAliasSequence", &tags::ReferencedPatientAliasSequence);
		tag_to_elem.insert(0x00380004, &tags::ReferencedPatientAliasSequence);

		ident_to_elem.insert("VisitStatusID", &tags::VisitStatusID);
		tag_to_elem.insert(0x00380008, &tags::VisitStatusID);

		ident_to_elem.insert("AdmissionID", &tags::AdmissionID);
		tag_to_elem.insert(0x00380010, &tags::AdmissionID);

		ident_to_elem.insert("IssuerOfAdmissionID", &tags::IssuerOfAdmissionID);
		tag_to_elem.insert(0x00380011, &tags::IssuerOfAdmissionID);

		ident_to_elem.insert("IssuerOfAdmissionIDSequence", &tags::IssuerOfAdmissionIDSequence);
		tag_to_elem.insert(0x00380014, &tags::IssuerOfAdmissionIDSequence);

		ident_to_elem.insert("RouteOfAdmissions", &tags::RouteOfAdmissions);
		tag_to_elem.insert(0x00380016, &tags::RouteOfAdmissions);

		ident_to_elem.insert("ScheduledAdmissionDate", &tags::ScheduledAdmissionDate);
		tag_to_elem.insert(0x0038001A, &tags::ScheduledAdmissionDate);

		ident_to_elem.insert("ScheduledAdmissionTime", &tags::ScheduledAdmissionTime);
		tag_to_elem.insert(0x0038001B, &tags::ScheduledAdmissionTime);

		ident_to_elem.insert("ScheduledDischargeDate", &tags::ScheduledDischargeDate);
		tag_to_elem.insert(0x0038001C, &tags::ScheduledDischargeDate);

		ident_to_elem.insert("ScheduledDischargeTime", &tags::ScheduledDischargeTime);
		tag_to_elem.insert(0x0038001D, &tags::ScheduledDischargeTime);

		ident_to_elem.insert("ScheduledPatientInstitutionResidence", &tags::ScheduledPatientInstitutionResidence);
		tag_to_elem.insert(0x0038001E, &tags::ScheduledPatientInstitutionResidence);

		ident_to_elem.insert("AdmittingDate", &tags::AdmittingDate);
		tag_to_elem.insert(0x00380020, &tags::AdmittingDate);

		ident_to_elem.insert("AdmittingTime", &tags::AdmittingTime);
		tag_to_elem.insert(0x00380021, &tags::AdmittingTime);

		ident_to_elem.insert("DischargeDate", &tags::DischargeDate);
		tag_to_elem.insert(0x00380030, &tags::DischargeDate);

		ident_to_elem.insert("DischargeTime", &tags::DischargeTime);
		tag_to_elem.insert(0x00380032, &tags::DischargeTime);

		ident_to_elem.insert("DischargeDiagnosisDescription", &tags::DischargeDiagnosisDescription);
		tag_to_elem.insert(0x00380040, &tags::DischargeDiagnosisDescription);

		ident_to_elem.insert("DischargeDiagnosisCodeSequence", &tags::DischargeDiagnosisCodeSequence);
		tag_to_elem.insert(0x00380044, &tags::DischargeDiagnosisCodeSequence);

		ident_to_elem.insert("SpecialNeeds", &tags::SpecialNeeds);
		tag_to_elem.insert(0x00380050, &tags::SpecialNeeds);

		ident_to_elem.insert("ServiceEpisodeID", &tags::ServiceEpisodeID);
		tag_to_elem.insert(0x00380060, &tags::ServiceEpisodeID);

		ident_to_elem.insert("IssuerOfServiceEpisodeID", &tags::IssuerOfServiceEpisodeID);
		tag_to_elem.insert(0x00380061, &tags::IssuerOfServiceEpisodeID);

		ident_to_elem.insert("ServiceEpisodeDescription", &tags::ServiceEpisodeDescription);
		tag_to_elem.insert(0x00380062, &tags::ServiceEpisodeDescription);

		ident_to_elem.insert("IssuerOfServiceEpisodeIDSequence", &tags::IssuerOfServiceEpisodeIDSequence);
		tag_to_elem.insert(0x00380064, &tags::IssuerOfServiceEpisodeIDSequence);

		ident_to_elem.insert("PertinentDocumentsSequence", &tags::PertinentDocumentsSequence);
		tag_to_elem.insert(0x00380100, &tags::PertinentDocumentsSequence);

		ident_to_elem.insert("PertinentResourcesSequence", &tags::PertinentResourcesSequence);
		tag_to_elem.insert(0x00380101, &tags::PertinentResourcesSequence);

		ident_to_elem.insert("ResourceDescription", &tags::ResourceDescription);
		tag_to_elem.insert(0x00380102, &tags::ResourceDescription);

		ident_to_elem.insert("CurrentPatientLocation", &tags::CurrentPatientLocation);
		tag_to_elem.insert(0x00380300, &tags::CurrentPatientLocation);

		ident_to_elem.insert("PatientInstitutionResidence", &tags::PatientInstitutionResidence);
		tag_to_elem.insert(0x00380400, &tags::PatientInstitutionResidence);

		ident_to_elem.insert("PatientState", &tags::PatientState);
		tag_to_elem.insert(0x00380500, &tags::PatientState);

		ident_to_elem.insert("PatientClinicalTrialParticipationSequence", &tags::PatientClinicalTrialParticipationSequence);
		tag_to_elem.insert(0x00380502, &tags::PatientClinicalTrialParticipationSequence);

		ident_to_elem.insert("VisitComments", &tags::VisitComments);
		tag_to_elem.insert(0x00384000, &tags::VisitComments);

		ident_to_elem.insert("WaveformOriginality", &tags::WaveformOriginality);
		tag_to_elem.insert(0x003A0004, &tags::WaveformOriginality);

		ident_to_elem.insert("NumberOfWaveformChannels", &tags::NumberOfWaveformChannels);
		tag_to_elem.insert(0x003A0005, &tags::NumberOfWaveformChannels);

		ident_to_elem.insert("NumberOfWaveformSamples", &tags::NumberOfWaveformSamples);
		tag_to_elem.insert(0x003A0010, &tags::NumberOfWaveformSamples);

		ident_to_elem.insert("SamplingFrequency", &tags::SamplingFrequency);
		tag_to_elem.insert(0x003A001A, &tags::SamplingFrequency);

		ident_to_elem.insert("MultiplexGroupLabel", &tags::MultiplexGroupLabel);
		tag_to_elem.insert(0x003A0020, &tags::MultiplexGroupLabel);

		ident_to_elem.insert("ChannelDefinitionSequence", &tags::ChannelDefinitionSequence);
		tag_to_elem.insert(0x003A0200, &tags::ChannelDefinitionSequence);

		ident_to_elem.insert("WaveformChannelNumber", &tags::WaveformChannelNumber);
		tag_to_elem.insert(0x003A0202, &tags::WaveformChannelNumber);

		ident_to_elem.insert("ChannelLabel", &tags::ChannelLabel);
		tag_to_elem.insert(0x003A0203, &tags::ChannelLabel);

		ident_to_elem.insert("ChannelStatus", &tags::ChannelStatus);
		tag_to_elem.insert(0x003A0205, &tags::ChannelStatus);

		ident_to_elem.insert("ChannelSourceSequence", &tags::ChannelSourceSequence);
		tag_to_elem.insert(0x003A0208, &tags::ChannelSourceSequence);

		ident_to_elem.insert("ChannelSourceModifiersSequence", &tags::ChannelSourceModifiersSequence);
		tag_to_elem.insert(0x003A0209, &tags::ChannelSourceModifiersSequence);

		ident_to_elem.insert("SourceWaveformSequence", &tags::SourceWaveformSequence);
		tag_to_elem.insert(0x003A020A, &tags::SourceWaveformSequence);

		ident_to_elem.insert("ChannelDerivationDescription", &tags::ChannelDerivationDescription);
		tag_to_elem.insert(0x003A020C, &tags::ChannelDerivationDescription);

		ident_to_elem.insert("ChannelSensitivity", &tags::ChannelSensitivity);
		tag_to_elem.insert(0x003A0210, &tags::ChannelSensitivity);

		ident_to_elem.insert("ChannelSensitivityUnitsSequence", &tags::ChannelSensitivityUnitsSequence);
		tag_to_elem.insert(0x003A0211, &tags::ChannelSensitivityUnitsSequence);

		ident_to_elem.insert("ChannelSensitivityCorrectionFactor", &tags::ChannelSensitivityCorrectionFactor);
		tag_to_elem.insert(0x003A0212, &tags::ChannelSensitivityCorrectionFactor);

		ident_to_elem.insert("ChannelBaseline", &tags::ChannelBaseline);
		tag_to_elem.insert(0x003A0213, &tags::ChannelBaseline);

		ident_to_elem.insert("ChannelTimeSkew", &tags::ChannelTimeSkew);
		tag_to_elem.insert(0x003A0214, &tags::ChannelTimeSkew);

		ident_to_elem.insert("ChannelSampleSkew", &tags::ChannelSampleSkew);
		tag_to_elem.insert(0x003A0215, &tags::ChannelSampleSkew);

		ident_to_elem.insert("ChannelOffset", &tags::ChannelOffset);
		tag_to_elem.insert(0x003A0218, &tags::ChannelOffset);

		ident_to_elem.insert("WaveformBitsStored", &tags::WaveformBitsStored);
		tag_to_elem.insert(0x003A021A, &tags::WaveformBitsStored);

		ident_to_elem.insert("FilterLowFrequency", &tags::FilterLowFrequency);
		tag_to_elem.insert(0x003A0220, &tags::FilterLowFrequency);

		ident_to_elem.insert("FilterHighFrequency", &tags::FilterHighFrequency);
		tag_to_elem.insert(0x003A0221, &tags::FilterHighFrequency);

		ident_to_elem.insert("NotchFilterFrequency", &tags::NotchFilterFrequency);
		tag_to_elem.insert(0x003A0222, &tags::NotchFilterFrequency);

		ident_to_elem.insert("NotchFilterBandwidth", &tags::NotchFilterBandwidth);
		tag_to_elem.insert(0x003A0223, &tags::NotchFilterBandwidth);

		ident_to_elem.insert("WaveformDataDisplayScale", &tags::WaveformDataDisplayScale);
		tag_to_elem.insert(0x003A0230, &tags::WaveformDataDisplayScale);

		ident_to_elem.insert("WaveformDisplayBackgroundCIELabValue", &tags::WaveformDisplayBackgroundCIELabValue);
		tag_to_elem.insert(0x003A0231, &tags::WaveformDisplayBackgroundCIELabValue);

		ident_to_elem.insert("WaveformPresentationGroupSequence", &tags::WaveformPresentationGroupSequence);
		tag_to_elem.insert(0x003A0240, &tags::WaveformPresentationGroupSequence);

		ident_to_elem.insert("PresentationGroupNumber", &tags::PresentationGroupNumber);
		tag_to_elem.insert(0x003A0241, &tags::PresentationGroupNumber);

		ident_to_elem.insert("ChannelDisplaySequence", &tags::ChannelDisplaySequence);
		tag_to_elem.insert(0x003A0242, &tags::ChannelDisplaySequence);

		ident_to_elem.insert("ChannelRecommendedDisplayCIELabValue", &tags::ChannelRecommendedDisplayCIELabValue);
		tag_to_elem.insert(0x003A0244, &tags::ChannelRecommendedDisplayCIELabValue);

		ident_to_elem.insert("ChannelPosition", &tags::ChannelPosition);
		tag_to_elem.insert(0x003A0245, &tags::ChannelPosition);

		ident_to_elem.insert("DisplayShadingFlag", &tags::DisplayShadingFlag);
		tag_to_elem.insert(0x003A0246, &tags::DisplayShadingFlag);

		ident_to_elem.insert("FractionalChannelDisplayScale", &tags::FractionalChannelDisplayScale);
		tag_to_elem.insert(0x003A0247, &tags::FractionalChannelDisplayScale);

		ident_to_elem.insert("AbsoluteChannelDisplayScale", &tags::AbsoluteChannelDisplayScale);
		tag_to_elem.insert(0x003A0248, &tags::AbsoluteChannelDisplayScale);

		ident_to_elem.insert("MultiplexedAudioChannelsDescriptionCodeSequence", &tags::MultiplexedAudioChannelsDescriptionCodeSequence);
		tag_to_elem.insert(0x003A0300, &tags::MultiplexedAudioChannelsDescriptionCodeSequence);

		ident_to_elem.insert("ChannelIdentificationCode", &tags::ChannelIdentificationCode);
		tag_to_elem.insert(0x003A0301, &tags::ChannelIdentificationCode);

		ident_to_elem.insert("ChannelMode", &tags::ChannelMode);
		tag_to_elem.insert(0x003A0302, &tags::ChannelMode);

		ident_to_elem.insert("ScheduledStationAETitle", &tags::ScheduledStationAETitle);
		tag_to_elem.insert(0x00400001, &tags::ScheduledStationAETitle);

		ident_to_elem.insert("ScheduledProcedureStepStartDate", &tags::ScheduledProcedureStepStartDate);
		tag_to_elem.insert(0x00400002, &tags::ScheduledProcedureStepStartDate);

		ident_to_elem.insert("ScheduledProcedureStepStartTime", &tags::ScheduledProcedureStepStartTime);
		tag_to_elem.insert(0x00400003, &tags::ScheduledProcedureStepStartTime);

		ident_to_elem.insert("ScheduledProcedureStepEndDate", &tags::ScheduledProcedureStepEndDate);
		tag_to_elem.insert(0x00400004, &tags::ScheduledProcedureStepEndDate);

		ident_to_elem.insert("ScheduledProcedureStepEndTime", &tags::ScheduledProcedureStepEndTime);
		tag_to_elem.insert(0x00400005, &tags::ScheduledProcedureStepEndTime);

		ident_to_elem.insert("ScheduledPerformingPhysicianName", &tags::ScheduledPerformingPhysicianName);
		tag_to_elem.insert(0x00400006, &tags::ScheduledPerformingPhysicianName);

		ident_to_elem.insert("ScheduledProcedureStepDescription", &tags::ScheduledProcedureStepDescription);
		tag_to_elem.insert(0x00400007, &tags::ScheduledProcedureStepDescription);

		ident_to_elem.insert("ScheduledProtocolCodeSequence", &tags::ScheduledProtocolCodeSequence);
		tag_to_elem.insert(0x00400008, &tags::ScheduledProtocolCodeSequence);

		ident_to_elem.insert("ScheduledProcedureStepID", &tags::ScheduledProcedureStepID);
		tag_to_elem.insert(0x00400009, &tags::ScheduledProcedureStepID);

		ident_to_elem.insert("StageCodeSequence", &tags::StageCodeSequence);
		tag_to_elem.insert(0x0040000A, &tags::StageCodeSequence);

		ident_to_elem.insert("ScheduledPerformingPhysicianIdentificationSequence", &tags::ScheduledPerformingPhysicianIdentificationSequence);
		tag_to_elem.insert(0x0040000B, &tags::ScheduledPerformingPhysicianIdentificationSequence);

		ident_to_elem.insert("ScheduledStationName", &tags::ScheduledStationName);
		tag_to_elem.insert(0x00400010, &tags::ScheduledStationName);

		ident_to_elem.insert("ScheduledProcedureStepLocation", &tags::ScheduledProcedureStepLocation);
		tag_to_elem.insert(0x00400011, &tags::ScheduledProcedureStepLocation);

		ident_to_elem.insert("PreMedication", &tags::PreMedication);
		tag_to_elem.insert(0x00400012, &tags::PreMedication);

		ident_to_elem.insert("ScheduledProcedureStepStatus", &tags::ScheduledProcedureStepStatus);
		tag_to_elem.insert(0x00400020, &tags::ScheduledProcedureStepStatus);

		ident_to_elem.insert("OrderPlacerIdentifierSequence", &tags::OrderPlacerIdentifierSequence);
		tag_to_elem.insert(0x00400026, &tags::OrderPlacerIdentifierSequence);

		ident_to_elem.insert("OrderFillerIdentifierSequence", &tags::OrderFillerIdentifierSequence);
		tag_to_elem.insert(0x00400027, &tags::OrderFillerIdentifierSequence);

		ident_to_elem.insert("LocalNamespaceEntityID", &tags::LocalNamespaceEntityID);
		tag_to_elem.insert(0x00400031, &tags::LocalNamespaceEntityID);

		ident_to_elem.insert("UniversalEntityID", &tags::UniversalEntityID);
		tag_to_elem.insert(0x00400032, &tags::UniversalEntityID);

		ident_to_elem.insert("UniversalEntityIDType", &tags::UniversalEntityIDType);
		tag_to_elem.insert(0x00400033, &tags::UniversalEntityIDType);

		ident_to_elem.insert("IdentifierTypeCode", &tags::IdentifierTypeCode);
		tag_to_elem.insert(0x00400035, &tags::IdentifierTypeCode);

		ident_to_elem.insert("AssigningFacilitySequence", &tags::AssigningFacilitySequence);
		tag_to_elem.insert(0x00400036, &tags::AssigningFacilitySequence);

		ident_to_elem.insert("AssigningJurisdictionCodeSequence", &tags::AssigningJurisdictionCodeSequence);
		tag_to_elem.insert(0x00400039, &tags::AssigningJurisdictionCodeSequence);

		ident_to_elem.insert("AssigningAgencyOrDepartmentCodeSequence", &tags::AssigningAgencyOrDepartmentCodeSequence);
		tag_to_elem.insert(0x0040003A, &tags::AssigningAgencyOrDepartmentCodeSequence);

		ident_to_elem.insert("ScheduledProcedureStepSequence", &tags::ScheduledProcedureStepSequence);
		tag_to_elem.insert(0x00400100, &tags::ScheduledProcedureStepSequence);

		ident_to_elem.insert("ReferencedNonImageCompositeSOPInstanceSequence", &tags::ReferencedNonImageCompositeSOPInstanceSequence);
		tag_to_elem.insert(0x00400220, &tags::ReferencedNonImageCompositeSOPInstanceSequence);

		ident_to_elem.insert("PerformedStationAETitle", &tags::PerformedStationAETitle);
		tag_to_elem.insert(0x00400241, &tags::PerformedStationAETitle);

		ident_to_elem.insert("PerformedStationName", &tags::PerformedStationName);
		tag_to_elem.insert(0x00400242, &tags::PerformedStationName);

		ident_to_elem.insert("PerformedLocation", &tags::PerformedLocation);
		tag_to_elem.insert(0x00400243, &tags::PerformedLocation);

		ident_to_elem.insert("PerformedProcedureStepStartDate", &tags::PerformedProcedureStepStartDate);
		tag_to_elem.insert(0x00400244, &tags::PerformedProcedureStepStartDate);

		ident_to_elem.insert("PerformedProcedureStepStartTime", &tags::PerformedProcedureStepStartTime);
		tag_to_elem.insert(0x00400245, &tags::PerformedProcedureStepStartTime);

		ident_to_elem.insert("PerformedProcedureStepEndDate", &tags::PerformedProcedureStepEndDate);
		tag_to_elem.insert(0x00400250, &tags::PerformedProcedureStepEndDate);

		ident_to_elem.insert("PerformedProcedureStepEndTime", &tags::PerformedProcedureStepEndTime);
		tag_to_elem.insert(0x00400251, &tags::PerformedProcedureStepEndTime);

		ident_to_elem.insert("PerformedProcedureStepStatus", &tags::PerformedProcedureStepStatus);
		tag_to_elem.insert(0x00400252, &tags::PerformedProcedureStepStatus);

		ident_to_elem.insert("PerformedProcedureStepID", &tags::PerformedProcedureStepID);
		tag_to_elem.insert(0x00400253, &tags::PerformedProcedureStepID);

		ident_to_elem.insert("PerformedProcedureStepDescription", &tags::PerformedProcedureStepDescription);
		tag_to_elem.insert(0x00400254, &tags::PerformedProcedureStepDescription);

		ident_to_elem.insert("PerformedProcedureTypeDescription", &tags::PerformedProcedureTypeDescription);
		tag_to_elem.insert(0x00400255, &tags::PerformedProcedureTypeDescription);

		ident_to_elem.insert("PerformedProtocolCodeSequence", &tags::PerformedProtocolCodeSequence);
		tag_to_elem.insert(0x00400260, &tags::PerformedProtocolCodeSequence);

		ident_to_elem.insert("PerformedProtocolType", &tags::PerformedProtocolType);
		tag_to_elem.insert(0x00400261, &tags::PerformedProtocolType);

		ident_to_elem.insert("ScheduledStepAttributesSequence", &tags::ScheduledStepAttributesSequence);
		tag_to_elem.insert(0x00400270, &tags::ScheduledStepAttributesSequence);

		ident_to_elem.insert("RequestAttributesSequence", &tags::RequestAttributesSequence);
		tag_to_elem.insert(0x00400275, &tags::RequestAttributesSequence);

		ident_to_elem.insert("CommentsOnThePerformedProcedureStep", &tags::CommentsOnThePerformedProcedureStep);
		tag_to_elem.insert(0x00400280, &tags::CommentsOnThePerformedProcedureStep);

		ident_to_elem.insert("PerformedProcedureStepDiscontinuationReasonCodeSequence", &tags::PerformedProcedureStepDiscontinuationReasonCodeSequence);
		tag_to_elem.insert(0x00400281, &tags::PerformedProcedureStepDiscontinuationReasonCodeSequence);

		ident_to_elem.insert("QuantitySequence", &tags::QuantitySequence);
		tag_to_elem.insert(0x00400293, &tags::QuantitySequence);

		ident_to_elem.insert("Quantity", &tags::Quantity);
		tag_to_elem.insert(0x00400294, &tags::Quantity);

		ident_to_elem.insert("MeasuringUnitsSequence", &tags::MeasuringUnitsSequence);
		tag_to_elem.insert(0x00400295, &tags::MeasuringUnitsSequence);

		ident_to_elem.insert("BillingItemSequence", &tags::BillingItemSequence);
		tag_to_elem.insert(0x00400296, &tags::BillingItemSequence);

		ident_to_elem.insert("TotalTimeOfFluoroscopy", &tags::TotalTimeOfFluoroscopy);
		tag_to_elem.insert(0x00400300, &tags::TotalTimeOfFluoroscopy);

		ident_to_elem.insert("TotalNumberOfExposures", &tags::TotalNumberOfExposures);
		tag_to_elem.insert(0x00400301, &tags::TotalNumberOfExposures);

		ident_to_elem.insert("EntranceDose", &tags::EntranceDose);
		tag_to_elem.insert(0x00400302, &tags::EntranceDose);

		ident_to_elem.insert("ExposedArea", &tags::ExposedArea);
		tag_to_elem.insert(0x00400303, &tags::ExposedArea);

		ident_to_elem.insert("DistanceSourceToEntrance", &tags::DistanceSourceToEntrance);
		tag_to_elem.insert(0x00400306, &tags::DistanceSourceToEntrance);

		ident_to_elem.insert("DistanceSourceToSupport", &tags::DistanceSourceToSupport);
		tag_to_elem.insert(0x00400307, &tags::DistanceSourceToSupport);

		ident_to_elem.insert("ExposureDoseSequence", &tags::ExposureDoseSequence);
		tag_to_elem.insert(0x0040030E, &tags::ExposureDoseSequence);

		ident_to_elem.insert("CommentsOnRadiationDose", &tags::CommentsOnRadiationDose);
		tag_to_elem.insert(0x00400310, &tags::CommentsOnRadiationDose);

		ident_to_elem.insert("XRayOutput", &tags::XRayOutput);
		tag_to_elem.insert(0x00400312, &tags::XRayOutput);

		ident_to_elem.insert("HalfValueLayer", &tags::HalfValueLayer);
		tag_to_elem.insert(0x00400314, &tags::HalfValueLayer);

		ident_to_elem.insert("OrganDose", &tags::OrganDose);
		tag_to_elem.insert(0x00400316, &tags::OrganDose);

		ident_to_elem.insert("OrganExposed", &tags::OrganExposed);
		tag_to_elem.insert(0x00400318, &tags::OrganExposed);

		ident_to_elem.insert("BillingProcedureStepSequence", &tags::BillingProcedureStepSequence);
		tag_to_elem.insert(0x00400320, &tags::BillingProcedureStepSequence);

		ident_to_elem.insert("FilmConsumptionSequence", &tags::FilmConsumptionSequence);
		tag_to_elem.insert(0x00400321, &tags::FilmConsumptionSequence);

		ident_to_elem.insert("BillingSuppliesAndDevicesSequence", &tags::BillingSuppliesAndDevicesSequence);
		tag_to_elem.insert(0x00400324, &tags::BillingSuppliesAndDevicesSequence);

		ident_to_elem.insert("ReferencedProcedureStepSequence", &tags::ReferencedProcedureStepSequence);
		tag_to_elem.insert(0x00400330, &tags::ReferencedProcedureStepSequence);

		ident_to_elem.insert("PerformedSeriesSequence", &tags::PerformedSeriesSequence);
		tag_to_elem.insert(0x00400340, &tags::PerformedSeriesSequence);

		ident_to_elem.insert("CommentsOnTheScheduledProcedureStep", &tags::CommentsOnTheScheduledProcedureStep);
		tag_to_elem.insert(0x00400400, &tags::CommentsOnTheScheduledProcedureStep);

		ident_to_elem.insert("ProtocolContextSequence", &tags::ProtocolContextSequence);
		tag_to_elem.insert(0x00400440, &tags::ProtocolContextSequence);

		ident_to_elem.insert("ContentItemModifierSequence", &tags::ContentItemModifierSequence);
		tag_to_elem.insert(0x00400441, &tags::ContentItemModifierSequence);

		ident_to_elem.insert("ScheduledSpecimenSequence", &tags::ScheduledSpecimenSequence);
		tag_to_elem.insert(0x00400500, &tags::ScheduledSpecimenSequence);

		ident_to_elem.insert("SpecimenAccessionNumber", &tags::SpecimenAccessionNumber);
		tag_to_elem.insert(0x0040050A, &tags::SpecimenAccessionNumber);

		ident_to_elem.insert("ContainerIdentifier", &tags::ContainerIdentifier);
		tag_to_elem.insert(0x00400512, &tags::ContainerIdentifier);

		ident_to_elem.insert("IssuerOfTheContainerIdentifierSequence", &tags::IssuerOfTheContainerIdentifierSequence);
		tag_to_elem.insert(0x00400513, &tags::IssuerOfTheContainerIdentifierSequence);

		ident_to_elem.insert("AlternateContainerIdentifierSequence", &tags::AlternateContainerIdentifierSequence);
		tag_to_elem.insert(0x00400515, &tags::AlternateContainerIdentifierSequence);

		ident_to_elem.insert("ContainerTypeCodeSequence", &tags::ContainerTypeCodeSequence);
		tag_to_elem.insert(0x00400518, &tags::ContainerTypeCodeSequence);

		ident_to_elem.insert("ContainerDescription", &tags::ContainerDescription);
		tag_to_elem.insert(0x0040051A, &tags::ContainerDescription);

		ident_to_elem.insert("ContainerComponentSequence", &tags::ContainerComponentSequence);
		tag_to_elem.insert(0x00400520, &tags::ContainerComponentSequence);

		ident_to_elem.insert("SpecimenSequence", &tags::SpecimenSequence);
		tag_to_elem.insert(0x00400550, &tags::SpecimenSequence);

		ident_to_elem.insert("SpecimenIdentifier", &tags::SpecimenIdentifier);
		tag_to_elem.insert(0x00400551, &tags::SpecimenIdentifier);

		ident_to_elem.insert("SpecimenDescriptionSequenceTrial", &tags::SpecimenDescriptionSequenceTrial);
		tag_to_elem.insert(0x00400552, &tags::SpecimenDescriptionSequenceTrial);

		ident_to_elem.insert("SpecimenDescriptionTrial", &tags::SpecimenDescriptionTrial);
		tag_to_elem.insert(0x00400553, &tags::SpecimenDescriptionTrial);

		ident_to_elem.insert("SpecimenUID", &tags::SpecimenUID);
		tag_to_elem.insert(0x00400554, &tags::SpecimenUID);

		ident_to_elem.insert("AcquisitionContextSequence", &tags::AcquisitionContextSequence);
		tag_to_elem.insert(0x00400555, &tags::AcquisitionContextSequence);

		ident_to_elem.insert("AcquisitionContextDescription", &tags::AcquisitionContextDescription);
		tag_to_elem.insert(0x00400556, &tags::AcquisitionContextDescription);

		ident_to_elem.insert("SpecimenTypeCodeSequence", &tags::SpecimenTypeCodeSequence);
		tag_to_elem.insert(0x0040059A, &tags::SpecimenTypeCodeSequence);

		ident_to_elem.insert("SpecimenDescriptionSequence", &tags::SpecimenDescriptionSequence);
		tag_to_elem.insert(0x00400560, &tags::SpecimenDescriptionSequence);

		ident_to_elem.insert("IssuerOfTheSpecimenIdentifierSequence", &tags::IssuerOfTheSpecimenIdentifierSequence);
		tag_to_elem.insert(0x00400562, &tags::IssuerOfTheSpecimenIdentifierSequence);

		ident_to_elem.insert("SpecimenShortDescription", &tags::SpecimenShortDescription);
		tag_to_elem.insert(0x00400600, &tags::SpecimenShortDescription);

		ident_to_elem.insert("SpecimenDetailedDescription", &tags::SpecimenDetailedDescription);
		tag_to_elem.insert(0x00400602, &tags::SpecimenDetailedDescription);

		ident_to_elem.insert("SpecimenPreparationSequence", &tags::SpecimenPreparationSequence);
		tag_to_elem.insert(0x00400610, &tags::SpecimenPreparationSequence);

		ident_to_elem.insert("SpecimenPreparationStepContentItemSequence", &tags::SpecimenPreparationStepContentItemSequence);
		tag_to_elem.insert(0x00400612, &tags::SpecimenPreparationStepContentItemSequence);

		ident_to_elem.insert("SpecimenLocalizationContentItemSequence", &tags::SpecimenLocalizationContentItemSequence);
		tag_to_elem.insert(0x00400620, &tags::SpecimenLocalizationContentItemSequence);

		ident_to_elem.insert("SlideIdentifier", &tags::SlideIdentifier);
		tag_to_elem.insert(0x004006FA, &tags::SlideIdentifier);

		ident_to_elem.insert("ImageCenterPointCoordinatesSequence", &tags::ImageCenterPointCoordinatesSequence);
		tag_to_elem.insert(0x0040071A, &tags::ImageCenterPointCoordinatesSequence);

		ident_to_elem.insert("XOffsetInSlideCoordinateSystem", &tags::XOffsetInSlideCoordinateSystem);
		tag_to_elem.insert(0x0040072A, &tags::XOffsetInSlideCoordinateSystem);

		ident_to_elem.insert("YOffsetInSlideCoordinateSystem", &tags::YOffsetInSlideCoordinateSystem);
		tag_to_elem.insert(0x0040073A, &tags::YOffsetInSlideCoordinateSystem);

		ident_to_elem.insert("ZOffsetInSlideCoordinateSystem", &tags::ZOffsetInSlideCoordinateSystem);
		tag_to_elem.insert(0x0040074A, &tags::ZOffsetInSlideCoordinateSystem);

		ident_to_elem.insert("PixelSpacingSequence", &tags::PixelSpacingSequence);
		tag_to_elem.insert(0x004008D8, &tags::PixelSpacingSequence);

		ident_to_elem.insert("CoordinateSystemAxisCodeSequence", &tags::CoordinateSystemAxisCodeSequence);
		tag_to_elem.insert(0x004008DA, &tags::CoordinateSystemAxisCodeSequence);

		ident_to_elem.insert("MeasurementUnitsCodeSequence", &tags::MeasurementUnitsCodeSequence);
		tag_to_elem.insert(0x004008EA, &tags::MeasurementUnitsCodeSequence);

		ident_to_elem.insert("VitalStainCodeSequenceTrial", &tags::VitalStainCodeSequenceTrial);
		tag_to_elem.insert(0x004009F8, &tags::VitalStainCodeSequenceTrial);

		ident_to_elem.insert("RequestedProcedureID", &tags::RequestedProcedureID);
		tag_to_elem.insert(0x00401001, &tags::RequestedProcedureID);

		ident_to_elem.insert("ReasonForTheRequestedProcedure", &tags::ReasonForTheRequestedProcedure);
		tag_to_elem.insert(0x00401002, &tags::ReasonForTheRequestedProcedure);

		ident_to_elem.insert("RequestedProcedurePriority", &tags::RequestedProcedurePriority);
		tag_to_elem.insert(0x00401003, &tags::RequestedProcedurePriority);

		ident_to_elem.insert("PatientTransportArrangements", &tags::PatientTransportArrangements);
		tag_to_elem.insert(0x00401004, &tags::PatientTransportArrangements);

		ident_to_elem.insert("RequestedProcedureLocation", &tags::RequestedProcedureLocation);
		tag_to_elem.insert(0x00401005, &tags::RequestedProcedureLocation);

		ident_to_elem.insert("PlacerOrderNumberProcedure", &tags::PlacerOrderNumberProcedure);
		tag_to_elem.insert(0x00401006, &tags::PlacerOrderNumberProcedure);

		ident_to_elem.insert("FillerOrderNumberProcedure", &tags::FillerOrderNumberProcedure);
		tag_to_elem.insert(0x00401007, &tags::FillerOrderNumberProcedure);

		ident_to_elem.insert("ConfidentialityCode", &tags::ConfidentialityCode);
		tag_to_elem.insert(0x00401008, &tags::ConfidentialityCode);

		ident_to_elem.insert("ReportingPriority", &tags::ReportingPriority);
		tag_to_elem.insert(0x00401009, &tags::ReportingPriority);

		ident_to_elem.insert("ReasonForRequestedProcedureCodeSequence", &tags::ReasonForRequestedProcedureCodeSequence);
		tag_to_elem.insert(0x0040100A, &tags::ReasonForRequestedProcedureCodeSequence);

		ident_to_elem.insert("NamesOfIntendedRecipientsOfResults", &tags::NamesOfIntendedRecipientsOfResults);
		tag_to_elem.insert(0x00401010, &tags::NamesOfIntendedRecipientsOfResults);

		ident_to_elem.insert("IntendedRecipientsOfResultsIdentificationSequence", &tags::IntendedRecipientsOfResultsIdentificationSequence);
		tag_to_elem.insert(0x00401011, &tags::IntendedRecipientsOfResultsIdentificationSequence);

		ident_to_elem.insert("ReasonForPerformedProcedureCodeSequence", &tags::ReasonForPerformedProcedureCodeSequence);
		tag_to_elem.insert(0x00401012, &tags::ReasonForPerformedProcedureCodeSequence);

		ident_to_elem.insert("RequestedProcedureDescriptionTrial", &tags::RequestedProcedureDescriptionTrial);
		tag_to_elem.insert(0x00401060, &tags::RequestedProcedureDescriptionTrial);

		ident_to_elem.insert("PersonIdentificationCodeSequence", &tags::PersonIdentificationCodeSequence);
		tag_to_elem.insert(0x00401101, &tags::PersonIdentificationCodeSequence);

		ident_to_elem.insert("PersonAddress", &tags::PersonAddress);
		tag_to_elem.insert(0x00401102, &tags::PersonAddress);

		ident_to_elem.insert("PersonTelephoneNumbers", &tags::PersonTelephoneNumbers);
		tag_to_elem.insert(0x00401103, &tags::PersonTelephoneNumbers);

		ident_to_elem.insert("PersonTelecomInformation", &tags::PersonTelecomInformation);
		tag_to_elem.insert(0x00401104, &tags::PersonTelecomInformation);

		ident_to_elem.insert("RequestedProcedureComments", &tags::RequestedProcedureComments);
		tag_to_elem.insert(0x00401400, &tags::RequestedProcedureComments);

		ident_to_elem.insert("ReasonForTheImagingServiceRequest", &tags::ReasonForTheImagingServiceRequest);
		tag_to_elem.insert(0x00402001, &tags::ReasonForTheImagingServiceRequest);

		ident_to_elem.insert("IssueDateOfImagingServiceRequest", &tags::IssueDateOfImagingServiceRequest);
		tag_to_elem.insert(0x00402004, &tags::IssueDateOfImagingServiceRequest);

		ident_to_elem.insert("IssueTimeOfImagingServiceRequest", &tags::IssueTimeOfImagingServiceRequest);
		tag_to_elem.insert(0x00402005, &tags::IssueTimeOfImagingServiceRequest);

		ident_to_elem.insert("PlacerOrderNumberImagingServiceRequestRetired", &tags::PlacerOrderNumberImagingServiceRequestRetired);
		tag_to_elem.insert(0x00402006, &tags::PlacerOrderNumberImagingServiceRequestRetired);

		ident_to_elem.insert("FillerOrderNumberImagingServiceRequestRetired", &tags::FillerOrderNumberImagingServiceRequestRetired);
		tag_to_elem.insert(0x00402007, &tags::FillerOrderNumberImagingServiceRequestRetired);

		ident_to_elem.insert("OrderEnteredBy", &tags::OrderEnteredBy);
		tag_to_elem.insert(0x00402008, &tags::OrderEnteredBy);

		ident_to_elem.insert("OrderEntererLocation", &tags::OrderEntererLocation);
		tag_to_elem.insert(0x00402009, &tags::OrderEntererLocation);

		ident_to_elem.insert("OrderCallbackPhoneNumber", &tags::OrderCallbackPhoneNumber);
		tag_to_elem.insert(0x00402010, &tags::OrderCallbackPhoneNumber);

		ident_to_elem.insert("OrderCallbackTelecomInformation", &tags::OrderCallbackTelecomInformation);
		tag_to_elem.insert(0x00402011, &tags::OrderCallbackTelecomInformation);

		ident_to_elem.insert("PlacerOrderNumberImagingServiceRequest", &tags::PlacerOrderNumberImagingServiceRequest);
		tag_to_elem.insert(0x00402016, &tags::PlacerOrderNumberImagingServiceRequest);

		ident_to_elem.insert("FillerOrderNumberImagingServiceRequest", &tags::FillerOrderNumberImagingServiceRequest);
		tag_to_elem.insert(0x00402017, &tags::FillerOrderNumberImagingServiceRequest);

		ident_to_elem.insert("ImagingServiceRequestComments", &tags::ImagingServiceRequestComments);
		tag_to_elem.insert(0x00402400, &tags::ImagingServiceRequestComments);

		ident_to_elem.insert("ConfidentialityConstraintOnPatientDataDescription", &tags::ConfidentialityConstraintOnPatientDataDescription);
		tag_to_elem.insert(0x00403001, &tags::ConfidentialityConstraintOnPatientDataDescription);

		ident_to_elem.insert("GeneralPurposeScheduledProcedureStepStatus", &tags::GeneralPurposeScheduledProcedureStepStatus);
		tag_to_elem.insert(0x00404001, &tags::GeneralPurposeScheduledProcedureStepStatus);

		ident_to_elem.insert("GeneralPurposePerformedProcedureStepStatus", &tags::GeneralPurposePerformedProcedureStepStatus);
		tag_to_elem.insert(0x00404002, &tags::GeneralPurposePerformedProcedureStepStatus);

		ident_to_elem.insert("GeneralPurposeScheduledProcedureStepPriority", &tags::GeneralPurposeScheduledProcedureStepPriority);
		tag_to_elem.insert(0x00404003, &tags::GeneralPurposeScheduledProcedureStepPriority);

		ident_to_elem.insert("ScheduledProcessingApplicationsCodeSequence", &tags::ScheduledProcessingApplicationsCodeSequence);
		tag_to_elem.insert(0x00404004, &tags::ScheduledProcessingApplicationsCodeSequence);

		ident_to_elem.insert("ScheduledProcedureStepStartDateTime", &tags::ScheduledProcedureStepStartDateTime);
		tag_to_elem.insert(0x00404005, &tags::ScheduledProcedureStepStartDateTime);

		ident_to_elem.insert("MultipleCopiesFlag", &tags::MultipleCopiesFlag);
		tag_to_elem.insert(0x00404006, &tags::MultipleCopiesFlag);

		ident_to_elem.insert("PerformedProcessingApplicationsCodeSequence", &tags::PerformedProcessingApplicationsCodeSequence);
		tag_to_elem.insert(0x00404007, &tags::PerformedProcessingApplicationsCodeSequence);

		ident_to_elem.insert("HumanPerformerCodeSequence", &tags::HumanPerformerCodeSequence);
		tag_to_elem.insert(0x00404009, &tags::HumanPerformerCodeSequence);

		ident_to_elem.insert("ScheduledProcedureStepModificationDateTime", &tags::ScheduledProcedureStepModificationDateTime);
		tag_to_elem.insert(0x00404010, &tags::ScheduledProcedureStepModificationDateTime);

		ident_to_elem.insert("ExpectedCompletionDateTime", &tags::ExpectedCompletionDateTime);
		tag_to_elem.insert(0x00404011, &tags::ExpectedCompletionDateTime);

		ident_to_elem.insert("ResultingGeneralPurposePerformedProcedureStepsSequence", &tags::ResultingGeneralPurposePerformedProcedureStepsSequence);
		tag_to_elem.insert(0x00404015, &tags::ResultingGeneralPurposePerformedProcedureStepsSequence);

		ident_to_elem.insert("ReferencedGeneralPurposeScheduledProcedureStepSequence", &tags::ReferencedGeneralPurposeScheduledProcedureStepSequence);
		tag_to_elem.insert(0x00404016, &tags::ReferencedGeneralPurposeScheduledProcedureStepSequence);

		ident_to_elem.insert("ScheduledWorkitemCodeSequence", &tags::ScheduledWorkitemCodeSequence);
		tag_to_elem.insert(0x00404018, &tags::ScheduledWorkitemCodeSequence);

		ident_to_elem.insert("PerformedWorkitemCodeSequence", &tags::PerformedWorkitemCodeSequence);
		tag_to_elem.insert(0x00404019, &tags::PerformedWorkitemCodeSequence);

		ident_to_elem.insert("InputAvailabilityFlag", &tags::InputAvailabilityFlag);
		tag_to_elem.insert(0x00404020, &tags::InputAvailabilityFlag);

		ident_to_elem.insert("InputInformationSequence", &tags::InputInformationSequence);
		tag_to_elem.insert(0x00404021, &tags::InputInformationSequence);

		ident_to_elem.insert("RelevantInformationSequence", &tags::RelevantInformationSequence);
		tag_to_elem.insert(0x00404022, &tags::RelevantInformationSequence);

		ident_to_elem.insert("ReferencedGeneralPurposeScheduledProcedureStepTransactionUID", &tags::ReferencedGeneralPurposeScheduledProcedureStepTransactionUID);
		tag_to_elem.insert(0x00404023, &tags::ReferencedGeneralPurposeScheduledProcedureStepTransactionUID);

		ident_to_elem.insert("ScheduledStationNameCodeSequence", &tags::ScheduledStationNameCodeSequence);
		tag_to_elem.insert(0x00404025, &tags::ScheduledStationNameCodeSequence);

		ident_to_elem.insert("ScheduledStationClassCodeSequence", &tags::ScheduledStationClassCodeSequence);
		tag_to_elem.insert(0x00404026, &tags::ScheduledStationClassCodeSequence);

		ident_to_elem.insert("ScheduledStationGeographicLocationCodeSequence", &tags::ScheduledStationGeographicLocationCodeSequence);
		tag_to_elem.insert(0x00404027, &tags::ScheduledStationGeographicLocationCodeSequence);

		ident_to_elem.insert("PerformedStationNameCodeSequence", &tags::PerformedStationNameCodeSequence);
		tag_to_elem.insert(0x00404028, &tags::PerformedStationNameCodeSequence);

		ident_to_elem.insert("PerformedStationClassCodeSequence", &tags::PerformedStationClassCodeSequence);
		tag_to_elem.insert(0x00404029, &tags::PerformedStationClassCodeSequence);

		ident_to_elem.insert("PerformedStationGeographicLocationCodeSequence", &tags::PerformedStationGeographicLocationCodeSequence);
		tag_to_elem.insert(0x00404030, &tags::PerformedStationGeographicLocationCodeSequence);

		ident_to_elem.insert("RequestedSubsequentWorkitemCodeSequence", &tags::RequestedSubsequentWorkitemCodeSequence);
		tag_to_elem.insert(0x00404031, &tags::RequestedSubsequentWorkitemCodeSequence);

		ident_to_elem.insert("NonDICOMOutputCodeSequence", &tags::NonDICOMOutputCodeSequence);
		tag_to_elem.insert(0x00404032, &tags::NonDICOMOutputCodeSequence);

		ident_to_elem.insert("OutputInformationSequence", &tags::OutputInformationSequence);
		tag_to_elem.insert(0x00404033, &tags::OutputInformationSequence);

		ident_to_elem.insert("ScheduledHumanPerformersSequence", &tags::ScheduledHumanPerformersSequence);
		tag_to_elem.insert(0x00404034, &tags::ScheduledHumanPerformersSequence);

		ident_to_elem.insert("ActualHumanPerformersSequence", &tags::ActualHumanPerformersSequence);
		tag_to_elem.insert(0x00404035, &tags::ActualHumanPerformersSequence);

		ident_to_elem.insert("HumanPerformerOrganization", &tags::HumanPerformerOrganization);
		tag_to_elem.insert(0x00404036, &tags::HumanPerformerOrganization);

		ident_to_elem.insert("HumanPerformerName", &tags::HumanPerformerName);
		tag_to_elem.insert(0x00404037, &tags::HumanPerformerName);

		ident_to_elem.insert("RawDataHandling", &tags::RawDataHandling);
		tag_to_elem.insert(0x00404040, &tags::RawDataHandling);

		ident_to_elem.insert("InputReadinessState", &tags::InputReadinessState);
		tag_to_elem.insert(0x00404041, &tags::InputReadinessState);

		ident_to_elem.insert("PerformedProcedureStepStartDateTime", &tags::PerformedProcedureStepStartDateTime);
		tag_to_elem.insert(0x00404050, &tags::PerformedProcedureStepStartDateTime);

		ident_to_elem.insert("PerformedProcedureStepEndDateTime", &tags::PerformedProcedureStepEndDateTime);
		tag_to_elem.insert(0x00404051, &tags::PerformedProcedureStepEndDateTime);

		ident_to_elem.insert("ProcedureStepCancellationDateTime", &tags::ProcedureStepCancellationDateTime);
		tag_to_elem.insert(0x00404052, &tags::ProcedureStepCancellationDateTime);

		ident_to_elem.insert("OutputDestinationSequence", &tags::OutputDestinationSequence);
		tag_to_elem.insert(0x00404070, &tags::OutputDestinationSequence);

		ident_to_elem.insert("DICOMStorageSequence", &tags::DICOMStorageSequence);
		tag_to_elem.insert(0x00404071, &tags::DICOMStorageSequence);

		ident_to_elem.insert("STOWRSStorageSequence", &tags::STOWRSStorageSequence);
		tag_to_elem.insert(0x00404072, &tags::STOWRSStorageSequence);

		ident_to_elem.insert("StorageURL", &tags::StorageURL);
		tag_to_elem.insert(0x00404073, &tags::StorageURL);

		ident_to_elem.insert("XDSStorageSequence", &tags::XDSStorageSequence);
		tag_to_elem.insert(0x00404074, &tags::XDSStorageSequence);

		ident_to_elem.insert("EntranceDoseInmGy", &tags::EntranceDoseInmGy);
		tag_to_elem.insert(0x00408302, &tags::EntranceDoseInmGy);

		ident_to_elem.insert("ParametricMapFrameTypeSequence", &tags::ParametricMapFrameTypeSequence);
		tag_to_elem.insert(0x00409092, &tags::ParametricMapFrameTypeSequence);

		ident_to_elem.insert("ReferencedImageRealWorldValueMappingSequence", &tags::ReferencedImageRealWorldValueMappingSequence);
		tag_to_elem.insert(0x00409094, &tags::ReferencedImageRealWorldValueMappingSequence);

		ident_to_elem.insert("RealWorldValueMappingSequence", &tags::RealWorldValueMappingSequence);
		tag_to_elem.insert(0x00409096, &tags::RealWorldValueMappingSequence);

		ident_to_elem.insert("PixelValueMappingCodeSequence", &tags::PixelValueMappingCodeSequence);
		tag_to_elem.insert(0x00409098, &tags::PixelValueMappingCodeSequence);

		ident_to_elem.insert("LUTLabel", &tags::LUTLabel);
		tag_to_elem.insert(0x00409210, &tags::LUTLabel);

		ident_to_elem.insert("RealWorldValueLastValueMapped", &tags::RealWorldValueLastValueMapped);
		tag_to_elem.insert(0x00409211, &tags::RealWorldValueLastValueMapped);

		ident_to_elem.insert("RealWorldValueLUTData", &tags::RealWorldValueLUTData);
		tag_to_elem.insert(0x00409212, &tags::RealWorldValueLUTData);

		ident_to_elem.insert("DoubleFloatRealWorldValueLastValueMapped", &tags::DoubleFloatRealWorldValueLastValueMapped);
		tag_to_elem.insert(0x00409213, &tags::DoubleFloatRealWorldValueLastValueMapped);

		ident_to_elem.insert("DoubleFloatRealWorldValueFirstValueMapped", &tags::DoubleFloatRealWorldValueFirstValueMapped);
		tag_to_elem.insert(0x00409214, &tags::DoubleFloatRealWorldValueFirstValueMapped);

		ident_to_elem.insert("RealWorldValueFirstValueMapped", &tags::RealWorldValueFirstValueMapped);
		tag_to_elem.insert(0x00409216, &tags::RealWorldValueFirstValueMapped);

		ident_to_elem.insert("QuantityDefinitionSequence", &tags::QuantityDefinitionSequence);
		tag_to_elem.insert(0x00409220, &tags::QuantityDefinitionSequence);

		ident_to_elem.insert("RealWorldValueIntercept", &tags::RealWorldValueIntercept);
		tag_to_elem.insert(0x00409224, &tags::RealWorldValueIntercept);

		ident_to_elem.insert("RealWorldValueSlope", &tags::RealWorldValueSlope);
		tag_to_elem.insert(0x00409225, &tags::RealWorldValueSlope);

		ident_to_elem.insert("FindingsFlagTrial", &tags::FindingsFlagTrial);
		tag_to_elem.insert(0x0040A007, &tags::FindingsFlagTrial);

		ident_to_elem.insert("RelationshipType", &tags::RelationshipType);
		tag_to_elem.insert(0x0040A010, &tags::RelationshipType);

		ident_to_elem.insert("FindingsSequenceTrial", &tags::FindingsSequenceTrial);
		tag_to_elem.insert(0x0040A020, &tags::FindingsSequenceTrial);

		ident_to_elem.insert("FindingsGroupUIDTrial", &tags::FindingsGroupUIDTrial);
		tag_to_elem.insert(0x0040A021, &tags::FindingsGroupUIDTrial);

		ident_to_elem.insert("ReferencedFindingsGroupUIDTrial", &tags::ReferencedFindingsGroupUIDTrial);
		tag_to_elem.insert(0x0040A022, &tags::ReferencedFindingsGroupUIDTrial);

		ident_to_elem.insert("FindingsGroupRecordingDateTrial", &tags::FindingsGroupRecordingDateTrial);
		tag_to_elem.insert(0x0040A023, &tags::FindingsGroupRecordingDateTrial);

		ident_to_elem.insert("FindingsGroupRecordingTimeTrial", &tags::FindingsGroupRecordingTimeTrial);
		tag_to_elem.insert(0x0040A024, &tags::FindingsGroupRecordingTimeTrial);

		ident_to_elem.insert("FindingsSourceCategoryCodeSequenceTrial", &tags::FindingsSourceCategoryCodeSequenceTrial);
		tag_to_elem.insert(0x0040A026, &tags::FindingsSourceCategoryCodeSequenceTrial);

		ident_to_elem.insert("VerifyingOrganization", &tags::VerifyingOrganization);
		tag_to_elem.insert(0x0040A027, &tags::VerifyingOrganization);

		ident_to_elem.insert("DocumentingOrganizationIdentifierCodeSequenceTrial", &tags::DocumentingOrganizationIdentifierCodeSequenceTrial);
		tag_to_elem.insert(0x0040A028, &tags::DocumentingOrganizationIdentifierCodeSequenceTrial);

		ident_to_elem.insert("VerificationDateTime", &tags::VerificationDateTime);
		tag_to_elem.insert(0x0040A030, &tags::VerificationDateTime);

		ident_to_elem.insert("ObservationDateTime", &tags::ObservationDateTime);
		tag_to_elem.insert(0x0040A032, &tags::ObservationDateTime);

		ident_to_elem.insert("ValueType", &tags::ValueType);
		tag_to_elem.insert(0x0040A040, &tags::ValueType);

		ident_to_elem.insert("ConceptNameCodeSequence", &tags::ConceptNameCodeSequence);
		tag_to_elem.insert(0x0040A043, &tags::ConceptNameCodeSequence);

		ident_to_elem.insert("MeasurementPrecisionDescriptionTrial", &tags::MeasurementPrecisionDescriptionTrial);
		tag_to_elem.insert(0x0040A047, &tags::MeasurementPrecisionDescriptionTrial);

		ident_to_elem.insert("ContinuityOfContent", &tags::ContinuityOfContent);
		tag_to_elem.insert(0x0040A050, &tags::ContinuityOfContent);

		ident_to_elem.insert("UrgencyOrPriorityAlertsTrial", &tags::UrgencyOrPriorityAlertsTrial);
		tag_to_elem.insert(0x0040A057, &tags::UrgencyOrPriorityAlertsTrial);

		ident_to_elem.insert("SequencingIndicatorTrial", &tags::SequencingIndicatorTrial);
		tag_to_elem.insert(0x0040A060, &tags::SequencingIndicatorTrial);

		ident_to_elem.insert("DocumentIdentifierCodeSequenceTrial", &tags::DocumentIdentifierCodeSequenceTrial);
		tag_to_elem.insert(0x0040A066, &tags::DocumentIdentifierCodeSequenceTrial);

		ident_to_elem.insert("DocumentAuthorTrial", &tags::DocumentAuthorTrial);
		tag_to_elem.insert(0x0040A067, &tags::DocumentAuthorTrial);

		ident_to_elem.insert("DocumentAuthorIdentifierCodeSequenceTrial", &tags::DocumentAuthorIdentifierCodeSequenceTrial);
		tag_to_elem.insert(0x0040A068, &tags::DocumentAuthorIdentifierCodeSequenceTrial);

		ident_to_elem.insert("IdentifierCodeSequenceTrial", &tags::IdentifierCodeSequenceTrial);
		tag_to_elem.insert(0x0040A070, &tags::IdentifierCodeSequenceTrial);

		ident_to_elem.insert("VerifyingObserverSequence", &tags::VerifyingObserverSequence);
		tag_to_elem.insert(0x0040A073, &tags::VerifyingObserverSequence);

		ident_to_elem.insert("ObjectBinaryIdentifierTrial", &tags::ObjectBinaryIdentifierTrial);
		tag_to_elem.insert(0x0040A074, &tags::ObjectBinaryIdentifierTrial);

		ident_to_elem.insert("VerifyingObserverName", &tags::VerifyingObserverName);
		tag_to_elem.insert(0x0040A075, &tags::VerifyingObserverName);

		ident_to_elem.insert("DocumentingObserverIdentifierCodeSequenceTrial", &tags::DocumentingObserverIdentifierCodeSequenceTrial);
		tag_to_elem.insert(0x0040A076, &tags::DocumentingObserverIdentifierCodeSequenceTrial);

		ident_to_elem.insert("AuthorObserverSequence", &tags::AuthorObserverSequence);
		tag_to_elem.insert(0x0040A078, &tags::AuthorObserverSequence);

		ident_to_elem.insert("ParticipantSequence", &tags::ParticipantSequence);
		tag_to_elem.insert(0x0040A07A, &tags::ParticipantSequence);

		ident_to_elem.insert("CustodialOrganizationSequence", &tags::CustodialOrganizationSequence);
		tag_to_elem.insert(0x0040A07C, &tags::CustodialOrganizationSequence);

		ident_to_elem.insert("ParticipationType", &tags::ParticipationType);
		tag_to_elem.insert(0x0040A080, &tags::ParticipationType);

		ident_to_elem.insert("ParticipationDateTime", &tags::ParticipationDateTime);
		tag_to_elem.insert(0x0040A082, &tags::ParticipationDateTime);

		ident_to_elem.insert("ObserverType", &tags::ObserverType);
		tag_to_elem.insert(0x0040A084, &tags::ObserverType);

		ident_to_elem.insert("ProcedureIdentifierCodeSequenceTrial", &tags::ProcedureIdentifierCodeSequenceTrial);
		tag_to_elem.insert(0x0040A085, &tags::ProcedureIdentifierCodeSequenceTrial);

		ident_to_elem.insert("VerifyingObserverIdentificationCodeSequence", &tags::VerifyingObserverIdentificationCodeSequence);
		tag_to_elem.insert(0x0040A088, &tags::VerifyingObserverIdentificationCodeSequence);

		ident_to_elem.insert("ObjectDirectoryBinaryIdentifierTrial", &tags::ObjectDirectoryBinaryIdentifierTrial);
		tag_to_elem.insert(0x0040A089, &tags::ObjectDirectoryBinaryIdentifierTrial);

		ident_to_elem.insert("EquivalentCDADocumentSequence", &tags::EquivalentCDADocumentSequence);
		tag_to_elem.insert(0x0040A090, &tags::EquivalentCDADocumentSequence);

		ident_to_elem.insert("ReferencedWaveformChannels", &tags::ReferencedWaveformChannels);
		tag_to_elem.insert(0x0040A0B0, &tags::ReferencedWaveformChannels);

		ident_to_elem.insert("DateOfDocumentOrVerbalTransactionTrial", &tags::DateOfDocumentOrVerbalTransactionTrial);
		tag_to_elem.insert(0x0040A110, &tags::DateOfDocumentOrVerbalTransactionTrial);

		ident_to_elem.insert("TimeOfDocumentCreationOrVerbalTransactionTrial", &tags::TimeOfDocumentCreationOrVerbalTransactionTrial);
		tag_to_elem.insert(0x0040A112, &tags::TimeOfDocumentCreationOrVerbalTransactionTrial);

		ident_to_elem.insert("DateTime", &tags::DateTime);
		tag_to_elem.insert(0x0040A120, &tags::DateTime);

		ident_to_elem.insert("Date", &tags::Date);
		tag_to_elem.insert(0x0040A121, &tags::Date);

		ident_to_elem.insert("Time", &tags::Time);
		tag_to_elem.insert(0x0040A122, &tags::Time);

		ident_to_elem.insert("PersonName", &tags::PersonName);
		tag_to_elem.insert(0x0040A123, &tags::PersonName);

		ident_to_elem.insert("UID", &tags::UID);
		tag_to_elem.insert(0x0040A124, &tags::UID);

		ident_to_elem.insert("ReportStatusIDTrial", &tags::ReportStatusIDTrial);
		tag_to_elem.insert(0x0040A125, &tags::ReportStatusIDTrial);

		ident_to_elem.insert("TemporalRangeType", &tags::TemporalRangeType);
		tag_to_elem.insert(0x0040A130, &tags::TemporalRangeType);

		ident_to_elem.insert("ReferencedSamplePositions", &tags::ReferencedSamplePositions);
		tag_to_elem.insert(0x0040A132, &tags::ReferencedSamplePositions);

		ident_to_elem.insert("ReferencedFrameNumbers", &tags::ReferencedFrameNumbers);
		tag_to_elem.insert(0x0040A136, &tags::ReferencedFrameNumbers);

		ident_to_elem.insert("ReferencedTimeOffsets", &tags::ReferencedTimeOffsets);
		tag_to_elem.insert(0x0040A138, &tags::ReferencedTimeOffsets);

		ident_to_elem.insert("ReferencedDateTime", &tags::ReferencedDateTime);
		tag_to_elem.insert(0x0040A13A, &tags::ReferencedDateTime);

		ident_to_elem.insert("TextValue", &tags::TextValue);
		tag_to_elem.insert(0x0040A160, &tags::TextValue);

		ident_to_elem.insert("FloatingPointValue", &tags::FloatingPointValue);
		tag_to_elem.insert(0x0040A161, &tags::FloatingPointValue);

		ident_to_elem.insert("RationalNumeratorValue", &tags::RationalNumeratorValue);
		tag_to_elem.insert(0x0040A162, &tags::RationalNumeratorValue);

		ident_to_elem.insert("RationalDenominatorValue", &tags::RationalDenominatorValue);
		tag_to_elem.insert(0x0040A163, &tags::RationalDenominatorValue);

		ident_to_elem.insert("ObservationCategoryCodeSequenceTrial", &tags::ObservationCategoryCodeSequenceTrial);
		tag_to_elem.insert(0x0040A167, &tags::ObservationCategoryCodeSequenceTrial);

		ident_to_elem.insert("ConceptCodeSequence", &tags::ConceptCodeSequence);
		tag_to_elem.insert(0x0040A168, &tags::ConceptCodeSequence);

		ident_to_elem.insert("BibliographicCitationTrial", &tags::BibliographicCitationTrial);
		tag_to_elem.insert(0x0040A16A, &tags::BibliographicCitationTrial);

		ident_to_elem.insert("PurposeOfReferenceCodeSequence", &tags::PurposeOfReferenceCodeSequence);
		tag_to_elem.insert(0x0040A170, &tags::PurposeOfReferenceCodeSequence);

		ident_to_elem.insert("ObservationUID", &tags::ObservationUID);
		tag_to_elem.insert(0x0040A171, &tags::ObservationUID);

		ident_to_elem.insert("ReferencedObservationUIDTrial", &tags::ReferencedObservationUIDTrial);
		tag_to_elem.insert(0x0040A172, &tags::ReferencedObservationUIDTrial);

		ident_to_elem.insert("ReferencedObservationClassTrial", &tags::ReferencedObservationClassTrial);
		tag_to_elem.insert(0x0040A173, &tags::ReferencedObservationClassTrial);

		ident_to_elem.insert("ReferencedObjectObservationClassTrial", &tags::ReferencedObjectObservationClassTrial);
		tag_to_elem.insert(0x0040A174, &tags::ReferencedObjectObservationClassTrial);

		ident_to_elem.insert("AnnotationGroupNumber", &tags::AnnotationGroupNumber);
		tag_to_elem.insert(0x0040A180, &tags::AnnotationGroupNumber);

		ident_to_elem.insert("ObservationDateTrial", &tags::ObservationDateTrial);
		tag_to_elem.insert(0x0040A192, &tags::ObservationDateTrial);

		ident_to_elem.insert("ObservationTimeTrial", &tags::ObservationTimeTrial);
		tag_to_elem.insert(0x0040A193, &tags::ObservationTimeTrial);

		ident_to_elem.insert("MeasurementAutomationTrial", &tags::MeasurementAutomationTrial);
		tag_to_elem.insert(0x0040A194, &tags::MeasurementAutomationTrial);

		ident_to_elem.insert("ModifierCodeSequence", &tags::ModifierCodeSequence);
		tag_to_elem.insert(0x0040A195, &tags::ModifierCodeSequence);

		ident_to_elem.insert("IdentificationDescriptionTrial", &tags::IdentificationDescriptionTrial);
		tag_to_elem.insert(0x0040A224, &tags::IdentificationDescriptionTrial);

		ident_to_elem.insert("CoordinatesSetGeometricTypeTrial", &tags::CoordinatesSetGeometricTypeTrial);
		tag_to_elem.insert(0x0040A290, &tags::CoordinatesSetGeometricTypeTrial);

		ident_to_elem.insert("AlgorithmCodeSequenceTrial", &tags::AlgorithmCodeSequenceTrial);
		tag_to_elem.insert(0x0040A296, &tags::AlgorithmCodeSequenceTrial);

		ident_to_elem.insert("AlgorithmDescriptionTrial", &tags::AlgorithmDescriptionTrial);
		tag_to_elem.insert(0x0040A297, &tags::AlgorithmDescriptionTrial);

		ident_to_elem.insert("PixelCoordinatesSetTrial", &tags::PixelCoordinatesSetTrial);
		tag_to_elem.insert(0x0040A29A, &tags::PixelCoordinatesSetTrial);

		ident_to_elem.insert("MeasuredValueSequence", &tags::MeasuredValueSequence);
		tag_to_elem.insert(0x0040A300, &tags::MeasuredValueSequence);

		ident_to_elem.insert("NumericValueQualifierCodeSequence", &tags::NumericValueQualifierCodeSequence);
		tag_to_elem.insert(0x0040A301, &tags::NumericValueQualifierCodeSequence);

		ident_to_elem.insert("CurrentObserverTrial", &tags::CurrentObserverTrial);
		tag_to_elem.insert(0x0040A307, &tags::CurrentObserverTrial);

		ident_to_elem.insert("NumericValue", &tags::NumericValue);
		tag_to_elem.insert(0x0040A30A, &tags::NumericValue);

		ident_to_elem.insert("ReferencedAccessionSequenceTrial", &tags::ReferencedAccessionSequenceTrial);
		tag_to_elem.insert(0x0040A313, &tags::ReferencedAccessionSequenceTrial);

		ident_to_elem.insert("ReportStatusCommentTrial", &tags::ReportStatusCommentTrial);
		tag_to_elem.insert(0x0040A33A, &tags::ReportStatusCommentTrial);

		ident_to_elem.insert("ProcedureContextSequenceTrial", &tags::ProcedureContextSequenceTrial);
		tag_to_elem.insert(0x0040A340, &tags::ProcedureContextSequenceTrial);

		ident_to_elem.insert("VerbalSourceTrial", &tags::VerbalSourceTrial);
		tag_to_elem.insert(0x0040A352, &tags::VerbalSourceTrial);

		ident_to_elem.insert("AddressTrial", &tags::AddressTrial);
		tag_to_elem.insert(0x0040A353, &tags::AddressTrial);

		ident_to_elem.insert("TelephoneNumberTrial", &tags::TelephoneNumberTrial);
		tag_to_elem.insert(0x0040A354, &tags::TelephoneNumberTrial);

		ident_to_elem.insert("VerbalSourceIdentifierCodeSequenceTrial", &tags::VerbalSourceIdentifierCodeSequenceTrial);
		tag_to_elem.insert(0x0040A358, &tags::VerbalSourceIdentifierCodeSequenceTrial);

		ident_to_elem.insert("PredecessorDocumentsSequence", &tags::PredecessorDocumentsSequence);
		tag_to_elem.insert(0x0040A360, &tags::PredecessorDocumentsSequence);

		ident_to_elem.insert("ReferencedRequestSequence", &tags::ReferencedRequestSequence);
		tag_to_elem.insert(0x0040A370, &tags::ReferencedRequestSequence);

		ident_to_elem.insert("PerformedProcedureCodeSequence", &tags::PerformedProcedureCodeSequence);
		tag_to_elem.insert(0x0040A372, &tags::PerformedProcedureCodeSequence);

		ident_to_elem.insert("CurrentRequestedProcedureEvidenceSequence", &tags::CurrentRequestedProcedureEvidenceSequence);
		tag_to_elem.insert(0x0040A375, &tags::CurrentRequestedProcedureEvidenceSequence);

		ident_to_elem.insert("ReportDetailSequenceTrial", &tags::ReportDetailSequenceTrial);
		tag_to_elem.insert(0x0040A380, &tags::ReportDetailSequenceTrial);

		ident_to_elem.insert("PertinentOtherEvidenceSequence", &tags::PertinentOtherEvidenceSequence);
		tag_to_elem.insert(0x0040A385, &tags::PertinentOtherEvidenceSequence);

		ident_to_elem.insert("HL7StructuredDocumentReferenceSequence", &tags::HL7StructuredDocumentReferenceSequence);
		tag_to_elem.insert(0x0040A390, &tags::HL7StructuredDocumentReferenceSequence);

		ident_to_elem.insert("ObservationSubjectUIDTrial", &tags::ObservationSubjectUIDTrial);
		tag_to_elem.insert(0x0040A402, &tags::ObservationSubjectUIDTrial);

		ident_to_elem.insert("ObservationSubjectClassTrial", &tags::ObservationSubjectClassTrial);
		tag_to_elem.insert(0x0040A403, &tags::ObservationSubjectClassTrial);

		ident_to_elem.insert("ObservationSubjectTypeCodeSequenceTrial", &tags::ObservationSubjectTypeCodeSequenceTrial);
		tag_to_elem.insert(0x0040A404, &tags::ObservationSubjectTypeCodeSequenceTrial);

		ident_to_elem.insert("CompletionFlag", &tags::CompletionFlag);
		tag_to_elem.insert(0x0040A491, &tags::CompletionFlag);

		ident_to_elem.insert("CompletionFlagDescription", &tags::CompletionFlagDescription);
		tag_to_elem.insert(0x0040A492, &tags::CompletionFlagDescription);

		ident_to_elem.insert("VerificationFlag", &tags::VerificationFlag);
		tag_to_elem.insert(0x0040A493, &tags::VerificationFlag);

		ident_to_elem.insert("ArchiveRequested", &tags::ArchiveRequested);
		tag_to_elem.insert(0x0040A494, &tags::ArchiveRequested);

		ident_to_elem.insert("PreliminaryFlag", &tags::PreliminaryFlag);
		tag_to_elem.insert(0x0040A496, &tags::PreliminaryFlag);

		ident_to_elem.insert("ContentTemplateSequence", &tags::ContentTemplateSequence);
		tag_to_elem.insert(0x0040A504, &tags::ContentTemplateSequence);

		ident_to_elem.insert("IdenticalDocumentsSequence", &tags::IdenticalDocumentsSequence);
		tag_to_elem.insert(0x0040A525, &tags::IdenticalDocumentsSequence);

		ident_to_elem.insert("ObservationSubjectContextFlagTrial", &tags::ObservationSubjectContextFlagTrial);
		tag_to_elem.insert(0x0040A600, &tags::ObservationSubjectContextFlagTrial);

		ident_to_elem.insert("ObserverContextFlagTrial", &tags::ObserverContextFlagTrial);
		tag_to_elem.insert(0x0040A601, &tags::ObserverContextFlagTrial);

		ident_to_elem.insert("ProcedureContextFlagTrial", &tags::ProcedureContextFlagTrial);
		tag_to_elem.insert(0x0040A603, &tags::ProcedureContextFlagTrial);

		ident_to_elem.insert("ContentSequence", &tags::ContentSequence);
		tag_to_elem.insert(0x0040A730, &tags::ContentSequence);

		ident_to_elem.insert("RelationshipSequenceTrial", &tags::RelationshipSequenceTrial);
		tag_to_elem.insert(0x0040A731, &tags::RelationshipSequenceTrial);

		ident_to_elem.insert("RelationshipTypeCodeSequenceTrial", &tags::RelationshipTypeCodeSequenceTrial);
		tag_to_elem.insert(0x0040A732, &tags::RelationshipTypeCodeSequenceTrial);

		ident_to_elem.insert("LanguageCodeSequenceTrial", &tags::LanguageCodeSequenceTrial);
		tag_to_elem.insert(0x0040A744, &tags::LanguageCodeSequenceTrial);

		ident_to_elem.insert("UniformResourceLocatorTrial", &tags::UniformResourceLocatorTrial);
		tag_to_elem.insert(0x0040A992, &tags::UniformResourceLocatorTrial);

		ident_to_elem.insert("WaveformAnnotationSequence", &tags::WaveformAnnotationSequence);
		tag_to_elem.insert(0x0040B020, &tags::WaveformAnnotationSequence);

		ident_to_elem.insert("TemplateIdentifier", &tags::TemplateIdentifier);
		tag_to_elem.insert(0x0040DB00, &tags::TemplateIdentifier);

		ident_to_elem.insert("TemplateVersion", &tags::TemplateVersion);
		tag_to_elem.insert(0x0040DB06, &tags::TemplateVersion);

		ident_to_elem.insert("TemplateLocalVersion", &tags::TemplateLocalVersion);
		tag_to_elem.insert(0x0040DB07, &tags::TemplateLocalVersion);

		ident_to_elem.insert("TemplateExtensionFlag", &tags::TemplateExtensionFlag);
		tag_to_elem.insert(0x0040DB0B, &tags::TemplateExtensionFlag);

		ident_to_elem.insert("TemplateExtensionOrganizationUID", &tags::TemplateExtensionOrganizationUID);
		tag_to_elem.insert(0x0040DB0C, &tags::TemplateExtensionOrganizationUID);

		ident_to_elem.insert("TemplateExtensionCreatorUID", &tags::TemplateExtensionCreatorUID);
		tag_to_elem.insert(0x0040DB0D, &tags::TemplateExtensionCreatorUID);

		ident_to_elem.insert("ReferencedContentItemIdentifier", &tags::ReferencedContentItemIdentifier);
		tag_to_elem.insert(0x0040DB73, &tags::ReferencedContentItemIdentifier);

		ident_to_elem.insert("HL7InstanceIdentifier", &tags::HL7InstanceIdentifier);
		tag_to_elem.insert(0x0040E001, &tags::HL7InstanceIdentifier);

		ident_to_elem.insert("HL7DocumentEffectiveTime", &tags::HL7DocumentEffectiveTime);
		tag_to_elem.insert(0x0040E004, &tags::HL7DocumentEffectiveTime);

		ident_to_elem.insert("HL7DocumentTypeCodeSequence", &tags::HL7DocumentTypeCodeSequence);
		tag_to_elem.insert(0x0040E006, &tags::HL7DocumentTypeCodeSequence);

		ident_to_elem.insert("DocumentClassCodeSequence", &tags::DocumentClassCodeSequence);
		tag_to_elem.insert(0x0040E008, &tags::DocumentClassCodeSequence);

		ident_to_elem.insert("RetrieveURI", &tags::RetrieveURI);
		tag_to_elem.insert(0x0040E010, &tags::RetrieveURI);

		ident_to_elem.insert("RetrieveLocationUID", &tags::RetrieveLocationUID);
		tag_to_elem.insert(0x0040E011, &tags::RetrieveLocationUID);

		ident_to_elem.insert("TypeOfInstances", &tags::TypeOfInstances);
		tag_to_elem.insert(0x0040E020, &tags::TypeOfInstances);

		ident_to_elem.insert("DICOMRetrievalSequence", &tags::DICOMRetrievalSequence);
		tag_to_elem.insert(0x0040E021, &tags::DICOMRetrievalSequence);

		ident_to_elem.insert("DICOMMediaRetrievalSequence", &tags::DICOMMediaRetrievalSequence);
		tag_to_elem.insert(0x0040E022, &tags::DICOMMediaRetrievalSequence);

		ident_to_elem.insert("WADORetrievalSequence", &tags::WADORetrievalSequence);
		tag_to_elem.insert(0x0040E023, &tags::WADORetrievalSequence);

		ident_to_elem.insert("XDSRetrievalSequence", &tags::XDSRetrievalSequence);
		tag_to_elem.insert(0x0040E024, &tags::XDSRetrievalSequence);

		ident_to_elem.insert("WADORSRetrievalSequence", &tags::WADORSRetrievalSequence);
		tag_to_elem.insert(0x0040E025, &tags::WADORSRetrievalSequence);

		ident_to_elem.insert("RepositoryUniqueID", &tags::RepositoryUniqueID);
		tag_to_elem.insert(0x0040E030, &tags::RepositoryUniqueID);

		ident_to_elem.insert("HomeCommunityID", &tags::HomeCommunityID);
		tag_to_elem.insert(0x0040E031, &tags::HomeCommunityID);

		ident_to_elem.insert("DocumentTitle", &tags::DocumentTitle);
		tag_to_elem.insert(0x00420010, &tags::DocumentTitle);

		ident_to_elem.insert("EncapsulatedDocument", &tags::EncapsulatedDocument);
		tag_to_elem.insert(0x00420011, &tags::EncapsulatedDocument);

		ident_to_elem.insert("MIMETypeOfEncapsulatedDocument", &tags::MIMETypeOfEncapsulatedDocument);
		tag_to_elem.insert(0x00420012, &tags::MIMETypeOfEncapsulatedDocument);

		ident_to_elem.insert("SourceInstanceSequence", &tags::SourceInstanceSequence);
		tag_to_elem.insert(0x00420013, &tags::SourceInstanceSequence);

		ident_to_elem.insert("ListOfMIMETypes", &tags::ListOfMIMETypes);
		tag_to_elem.insert(0x00420014, &tags::ListOfMIMETypes);

		ident_to_elem.insert("ProductPackageIdentifier", &tags::ProductPackageIdentifier);
		tag_to_elem.insert(0x00440001, &tags::ProductPackageIdentifier);

		ident_to_elem.insert("SubstanceAdministrationApproval", &tags::SubstanceAdministrationApproval);
		tag_to_elem.insert(0x00440002, &tags::SubstanceAdministrationApproval);

		ident_to_elem.insert("ApprovalStatusFurtherDescription", &tags::ApprovalStatusFurtherDescription);
		tag_to_elem.insert(0x00440003, &tags::ApprovalStatusFurtherDescription);

		ident_to_elem.insert("ApprovalStatusDateTime", &tags::ApprovalStatusDateTime);
		tag_to_elem.insert(0x00440004, &tags::ApprovalStatusDateTime);

		ident_to_elem.insert("ProductTypeCodeSequence", &tags::ProductTypeCodeSequence);
		tag_to_elem.insert(0x00440007, &tags::ProductTypeCodeSequence);

		ident_to_elem.insert("ProductName", &tags::ProductName);
		tag_to_elem.insert(0x00440008, &tags::ProductName);

		ident_to_elem.insert("ProductDescription", &tags::ProductDescription);
		tag_to_elem.insert(0x00440009, &tags::ProductDescription);

		ident_to_elem.insert("ProductLotIdentifier", &tags::ProductLotIdentifier);
		tag_to_elem.insert(0x0044000A, &tags::ProductLotIdentifier);

		ident_to_elem.insert("ProductExpirationDateTime", &tags::ProductExpirationDateTime);
		tag_to_elem.insert(0x0044000B, &tags::ProductExpirationDateTime);

		ident_to_elem.insert("SubstanceAdministrationDateTime", &tags::SubstanceAdministrationDateTime);
		tag_to_elem.insert(0x00440010, &tags::SubstanceAdministrationDateTime);

		ident_to_elem.insert("SubstanceAdministrationNotes", &tags::SubstanceAdministrationNotes);
		tag_to_elem.insert(0x00440011, &tags::SubstanceAdministrationNotes);

		ident_to_elem.insert("SubstanceAdministrationDeviceID", &tags::SubstanceAdministrationDeviceID);
		tag_to_elem.insert(0x00440012, &tags::SubstanceAdministrationDeviceID);

		ident_to_elem.insert("ProductParameterSequence", &tags::ProductParameterSequence);
		tag_to_elem.insert(0x00440013, &tags::ProductParameterSequence);

		ident_to_elem.insert("SubstanceAdministrationParameterSequence", &tags::SubstanceAdministrationParameterSequence);
		tag_to_elem.insert(0x00440019, &tags::SubstanceAdministrationParameterSequence);

		ident_to_elem.insert("LensDescription", &tags::LensDescription);
		tag_to_elem.insert(0x00460012, &tags::LensDescription);

		ident_to_elem.insert("RightLensSequence", &tags::RightLensSequence);
		tag_to_elem.insert(0x00460014, &tags::RightLensSequence);

		ident_to_elem.insert("LeftLensSequence", &tags::LeftLensSequence);
		tag_to_elem.insert(0x00460015, &tags::LeftLensSequence);

		ident_to_elem.insert("UnspecifiedLateralityLensSequence", &tags::UnspecifiedLateralityLensSequence);
		tag_to_elem.insert(0x00460016, &tags::UnspecifiedLateralityLensSequence);

		ident_to_elem.insert("CylinderSequence", &tags::CylinderSequence);
		tag_to_elem.insert(0x00460018, &tags::CylinderSequence);

		ident_to_elem.insert("PrismSequence", &tags::PrismSequence);
		tag_to_elem.insert(0x00460028, &tags::PrismSequence);

		ident_to_elem.insert("HorizontalPrismPower", &tags::HorizontalPrismPower);
		tag_to_elem.insert(0x00460030, &tags::HorizontalPrismPower);

		ident_to_elem.insert("HorizontalPrismBase", &tags::HorizontalPrismBase);
		tag_to_elem.insert(0x00460032, &tags::HorizontalPrismBase);

		ident_to_elem.insert("VerticalPrismPower", &tags::VerticalPrismPower);
		tag_to_elem.insert(0x00460034, &tags::VerticalPrismPower);

		ident_to_elem.insert("VerticalPrismBase", &tags::VerticalPrismBase);
		tag_to_elem.insert(0x00460036, &tags::VerticalPrismBase);

		ident_to_elem.insert("LensSegmentType", &tags::LensSegmentType);
		tag_to_elem.insert(0x00460038, &tags::LensSegmentType);

		ident_to_elem.insert("OpticalTransmittance", &tags::OpticalTransmittance);
		tag_to_elem.insert(0x00460040, &tags::OpticalTransmittance);

		ident_to_elem.insert("ChannelWidth", &tags::ChannelWidth);
		tag_to_elem.insert(0x00460042, &tags::ChannelWidth);

		ident_to_elem.insert("PupilSize", &tags::PupilSize);
		tag_to_elem.insert(0x00460044, &tags::PupilSize);

		ident_to_elem.insert("CornealSize", &tags::CornealSize);
		tag_to_elem.insert(0x00460046, &tags::CornealSize);

		ident_to_elem.insert("AutorefractionRightEyeSequence", &tags::AutorefractionRightEyeSequence);
		tag_to_elem.insert(0x00460050, &tags::AutorefractionRightEyeSequence);

		ident_to_elem.insert("AutorefractionLeftEyeSequence", &tags::AutorefractionLeftEyeSequence);
		tag_to_elem.insert(0x00460052, &tags::AutorefractionLeftEyeSequence);

		ident_to_elem.insert("DistancePupillaryDistance", &tags::DistancePupillaryDistance);
		tag_to_elem.insert(0x00460060, &tags::DistancePupillaryDistance);

		ident_to_elem.insert("NearPupillaryDistance", &tags::NearPupillaryDistance);
		tag_to_elem.insert(0x00460062, &tags::NearPupillaryDistance);

		ident_to_elem.insert("IntermediatePupillaryDistance", &tags::IntermediatePupillaryDistance);
		tag_to_elem.insert(0x00460063, &tags::IntermediatePupillaryDistance);

		ident_to_elem.insert("OtherPupillaryDistance", &tags::OtherPupillaryDistance);
		tag_to_elem.insert(0x00460064, &tags::OtherPupillaryDistance);

		ident_to_elem.insert("KeratometryRightEyeSequence", &tags::KeratometryRightEyeSequence);
		tag_to_elem.insert(0x00460070, &tags::KeratometryRightEyeSequence);

		ident_to_elem.insert("KeratometryLeftEyeSequence", &tags::KeratometryLeftEyeSequence);
		tag_to_elem.insert(0x00460071, &tags::KeratometryLeftEyeSequence);

		ident_to_elem.insert("SteepKeratometricAxisSequence", &tags::SteepKeratometricAxisSequence);
		tag_to_elem.insert(0x00460074, &tags::SteepKeratometricAxisSequence);

		ident_to_elem.insert("RadiusOfCurvature", &tags::RadiusOfCurvature);
		tag_to_elem.insert(0x00460075, &tags::RadiusOfCurvature);

		ident_to_elem.insert("KeratometricPower", &tags::KeratometricPower);
		tag_to_elem.insert(0x00460076, &tags::KeratometricPower);

		ident_to_elem.insert("KeratometricAxis", &tags::KeratometricAxis);
		tag_to_elem.insert(0x00460077, &tags::KeratometricAxis);

		ident_to_elem.insert("FlatKeratometricAxisSequence", &tags::FlatKeratometricAxisSequence);
		tag_to_elem.insert(0x00460080, &tags::FlatKeratometricAxisSequence);

		ident_to_elem.insert("BackgroundColor", &tags::BackgroundColor);
		tag_to_elem.insert(0x00460092, &tags::BackgroundColor);

		ident_to_elem.insert("Optotype", &tags::Optotype);
		tag_to_elem.insert(0x00460094, &tags::Optotype);

		ident_to_elem.insert("OptotypePresentation", &tags::OptotypePresentation);
		tag_to_elem.insert(0x00460095, &tags::OptotypePresentation);

		ident_to_elem.insert("SubjectiveRefractionRightEyeSequence", &tags::SubjectiveRefractionRightEyeSequence);
		tag_to_elem.insert(0x00460097, &tags::SubjectiveRefractionRightEyeSequence);

		ident_to_elem.insert("SubjectiveRefractionLeftEyeSequence", &tags::SubjectiveRefractionLeftEyeSequence);
		tag_to_elem.insert(0x00460098, &tags::SubjectiveRefractionLeftEyeSequence);

		ident_to_elem.insert("AddNearSequence", &tags::AddNearSequence);
		tag_to_elem.insert(0x00460100, &tags::AddNearSequence);

		ident_to_elem.insert("AddIntermediateSequence", &tags::AddIntermediateSequence);
		tag_to_elem.insert(0x00460101, &tags::AddIntermediateSequence);

		ident_to_elem.insert("AddOtherSequence", &tags::AddOtherSequence);
		tag_to_elem.insert(0x00460102, &tags::AddOtherSequence);

		ident_to_elem.insert("AddPower", &tags::AddPower);
		tag_to_elem.insert(0x00460104, &tags::AddPower);

		ident_to_elem.insert("ViewingDistance", &tags::ViewingDistance);
		tag_to_elem.insert(0x00460106, &tags::ViewingDistance);

		ident_to_elem.insert("VisualAcuityTypeCodeSequence", &tags::VisualAcuityTypeCodeSequence);
		tag_to_elem.insert(0x00460121, &tags::VisualAcuityTypeCodeSequence);

		ident_to_elem.insert("VisualAcuityRightEyeSequence", &tags::VisualAcuityRightEyeSequence);
		tag_to_elem.insert(0x00460122, &tags::VisualAcuityRightEyeSequence);

		ident_to_elem.insert("VisualAcuityLeftEyeSequence", &tags::VisualAcuityLeftEyeSequence);
		tag_to_elem.insert(0x00460123, &tags::VisualAcuityLeftEyeSequence);

		ident_to_elem.insert("VisualAcuityBothEyesOpenSequence", &tags::VisualAcuityBothEyesOpenSequence);
		tag_to_elem.insert(0x00460124, &tags::VisualAcuityBothEyesOpenSequence);

		ident_to_elem.insert("ViewingDistanceType", &tags::ViewingDistanceType);
		tag_to_elem.insert(0x00460125, &tags::ViewingDistanceType);

		ident_to_elem.insert("VisualAcuityModifiers", &tags::VisualAcuityModifiers);
		tag_to_elem.insert(0x00460135, &tags::VisualAcuityModifiers);

		ident_to_elem.insert("DecimalVisualAcuity", &tags::DecimalVisualAcuity);
		tag_to_elem.insert(0x00460137, &tags::DecimalVisualAcuity);

		ident_to_elem.insert("OptotypeDetailedDefinition", &tags::OptotypeDetailedDefinition);
		tag_to_elem.insert(0x00460139, &tags::OptotypeDetailedDefinition);

		ident_to_elem.insert("ReferencedRefractiveMeasurementsSequence", &tags::ReferencedRefractiveMeasurementsSequence);
		tag_to_elem.insert(0x00460145, &tags::ReferencedRefractiveMeasurementsSequence);

		ident_to_elem.insert("SpherePower", &tags::SpherePower);
		tag_to_elem.insert(0x00460146, &tags::SpherePower);

		ident_to_elem.insert("CylinderPower", &tags::CylinderPower);
		tag_to_elem.insert(0x00460147, &tags::CylinderPower);

		ident_to_elem.insert("CornealTopographySurface", &tags::CornealTopographySurface);
		tag_to_elem.insert(0x00460201, &tags::CornealTopographySurface);

		ident_to_elem.insert("CornealVertexLocation", &tags::CornealVertexLocation);
		tag_to_elem.insert(0x00460202, &tags::CornealVertexLocation);

		ident_to_elem.insert("PupilCentroidXCoordinate", &tags::PupilCentroidXCoordinate);
		tag_to_elem.insert(0x00460203, &tags::PupilCentroidXCoordinate);

		ident_to_elem.insert("PupilCentroidYCoordinate", &tags::PupilCentroidYCoordinate);
		tag_to_elem.insert(0x00460204, &tags::PupilCentroidYCoordinate);

		ident_to_elem.insert("EquivalentPupilRadius", &tags::EquivalentPupilRadius);
		tag_to_elem.insert(0x00460205, &tags::EquivalentPupilRadius);

		ident_to_elem.insert("CornealTopographyMapTypeCodeSequence", &tags::CornealTopographyMapTypeCodeSequence);
		tag_to_elem.insert(0x00460207, &tags::CornealTopographyMapTypeCodeSequence);

		ident_to_elem.insert("VerticesOfTheOutlineOfPupil", &tags::VerticesOfTheOutlineOfPupil);
		tag_to_elem.insert(0x00460208, &tags::VerticesOfTheOutlineOfPupil);

		ident_to_elem.insert("CornealTopographyMappingNormalsSequence", &tags::CornealTopographyMappingNormalsSequence);
		tag_to_elem.insert(0x00460210, &tags::CornealTopographyMappingNormalsSequence);

		ident_to_elem.insert("MaximumCornealCurvatureSequence", &tags::MaximumCornealCurvatureSequence);
		tag_to_elem.insert(0x00460211, &tags::MaximumCornealCurvatureSequence);

		ident_to_elem.insert("MaximumCornealCurvature", &tags::MaximumCornealCurvature);
		tag_to_elem.insert(0x00460212, &tags::MaximumCornealCurvature);

		ident_to_elem.insert("MaximumCornealCurvatureLocation", &tags::MaximumCornealCurvatureLocation);
		tag_to_elem.insert(0x00460213, &tags::MaximumCornealCurvatureLocation);

		ident_to_elem.insert("MinimumKeratometricSequence", &tags::MinimumKeratometricSequence);
		tag_to_elem.insert(0x00460215, &tags::MinimumKeratometricSequence);

		ident_to_elem.insert("SimulatedKeratometricCylinderSequence", &tags::SimulatedKeratometricCylinderSequence);
		tag_to_elem.insert(0x00460218, &tags::SimulatedKeratometricCylinderSequence);

		ident_to_elem.insert("AverageCornealPower", &tags::AverageCornealPower);
		tag_to_elem.insert(0x00460220, &tags::AverageCornealPower);

		ident_to_elem.insert("CornealISValue", &tags::CornealISValue);
		tag_to_elem.insert(0x00460224, &tags::CornealISValue);

		ident_to_elem.insert("AnalyzedArea", &tags::AnalyzedArea);
		tag_to_elem.insert(0x00460227, &tags::AnalyzedArea);

		ident_to_elem.insert("SurfaceRegularityIndex", &tags::SurfaceRegularityIndex);
		tag_to_elem.insert(0x00460230, &tags::SurfaceRegularityIndex);

		ident_to_elem.insert("SurfaceAsymmetryIndex", &tags::SurfaceAsymmetryIndex);
		tag_to_elem.insert(0x00460232, &tags::SurfaceAsymmetryIndex);

		ident_to_elem.insert("CornealEccentricityIndex", &tags::CornealEccentricityIndex);
		tag_to_elem.insert(0x00460234, &tags::CornealEccentricityIndex);

		ident_to_elem.insert("KeratoconusPredictionIndex", &tags::KeratoconusPredictionIndex);
		tag_to_elem.insert(0x00460236, &tags::KeratoconusPredictionIndex);

		ident_to_elem.insert("DecimalPotentialVisualAcuity", &tags::DecimalPotentialVisualAcuity);
		tag_to_elem.insert(0x00460238, &tags::DecimalPotentialVisualAcuity);

		ident_to_elem.insert("CornealTopographyMapQualityEvaluation", &tags::CornealTopographyMapQualityEvaluation);
		tag_to_elem.insert(0x00460242, &tags::CornealTopographyMapQualityEvaluation);

		ident_to_elem.insert("SourceImageCornealProcessedDataSequence", &tags::SourceImageCornealProcessedDataSequence);
		tag_to_elem.insert(0x00460244, &tags::SourceImageCornealProcessedDataSequence);

		ident_to_elem.insert("CornealPointLocation", &tags::CornealPointLocation);
		tag_to_elem.insert(0x00460247, &tags::CornealPointLocation);

		ident_to_elem.insert("CornealPointEstimated", &tags::CornealPointEstimated);
		tag_to_elem.insert(0x00460248, &tags::CornealPointEstimated);

		ident_to_elem.insert("AxialPower", &tags::AxialPower);
		tag_to_elem.insert(0x00460249, &tags::AxialPower);

		ident_to_elem.insert("TangentialPower", &tags::TangentialPower);
		tag_to_elem.insert(0x00460250, &tags::TangentialPower);

		ident_to_elem.insert("RefractivePower", &tags::RefractivePower);
		tag_to_elem.insert(0x00460251, &tags::RefractivePower);

		ident_to_elem.insert("RelativeElevation", &tags::RelativeElevation);
		tag_to_elem.insert(0x00460252, &tags::RelativeElevation);

		ident_to_elem.insert("CornealWavefront", &tags::CornealWavefront);
		tag_to_elem.insert(0x00460253, &tags::CornealWavefront);

		ident_to_elem.insert("ImagedVolumeWidth", &tags::ImagedVolumeWidth);
		tag_to_elem.insert(0x00480001, &tags::ImagedVolumeWidth);

		ident_to_elem.insert("ImagedVolumeHeight", &tags::ImagedVolumeHeight);
		tag_to_elem.insert(0x00480002, &tags::ImagedVolumeHeight);

		ident_to_elem.insert("ImagedVolumeDepth", &tags::ImagedVolumeDepth);
		tag_to_elem.insert(0x00480003, &tags::ImagedVolumeDepth);

		ident_to_elem.insert("TotalPixelMatrixColumns", &tags::TotalPixelMatrixColumns);
		tag_to_elem.insert(0x00480006, &tags::TotalPixelMatrixColumns);

		ident_to_elem.insert("TotalPixelMatrixRows", &tags::TotalPixelMatrixRows);
		tag_to_elem.insert(0x00480007, &tags::TotalPixelMatrixRows);

		ident_to_elem.insert("TotalPixelMatrixOriginSequence", &tags::TotalPixelMatrixOriginSequence);
		tag_to_elem.insert(0x00480008, &tags::TotalPixelMatrixOriginSequence);

		ident_to_elem.insert("SpecimenLabelInImage", &tags::SpecimenLabelInImage);
		tag_to_elem.insert(0x00480010, &tags::SpecimenLabelInImage);

		ident_to_elem.insert("FocusMethod", &tags::FocusMethod);
		tag_to_elem.insert(0x00480011, &tags::FocusMethod);

		ident_to_elem.insert("ExtendedDepthOfField", &tags::ExtendedDepthOfField);
		tag_to_elem.insert(0x00480012, &tags::ExtendedDepthOfField);

		ident_to_elem.insert("NumberOfFocalPlanes", &tags::NumberOfFocalPlanes);
		tag_to_elem.insert(0x00480013, &tags::NumberOfFocalPlanes);

		ident_to_elem.insert("DistanceBetweenFocalPlanes", &tags::DistanceBetweenFocalPlanes);
		tag_to_elem.insert(0x00480014, &tags::DistanceBetweenFocalPlanes);

		ident_to_elem.insert("RecommendedAbsentPixelCIELabValue", &tags::RecommendedAbsentPixelCIELabValue);
		tag_to_elem.insert(0x00480015, &tags::RecommendedAbsentPixelCIELabValue);

		ident_to_elem.insert("IlluminatorTypeCodeSequence", &tags::IlluminatorTypeCodeSequence);
		tag_to_elem.insert(0x00480100, &tags::IlluminatorTypeCodeSequence);

		ident_to_elem.insert("ImageOrientationSlide", &tags::ImageOrientationSlide);
		tag_to_elem.insert(0x00480102, &tags::ImageOrientationSlide);

		ident_to_elem.insert("OpticalPathSequence", &tags::OpticalPathSequence);
		tag_to_elem.insert(0x00480105, &tags::OpticalPathSequence);

		ident_to_elem.insert("OpticalPathIdentifier", &tags::OpticalPathIdentifier);
		tag_to_elem.insert(0x00480106, &tags::OpticalPathIdentifier);

		ident_to_elem.insert("OpticalPathDescription", &tags::OpticalPathDescription);
		tag_to_elem.insert(0x00480107, &tags::OpticalPathDescription);

		ident_to_elem.insert("IlluminationColorCodeSequence", &tags::IlluminationColorCodeSequence);
		tag_to_elem.insert(0x00480108, &tags::IlluminationColorCodeSequence);

		ident_to_elem.insert("SpecimenReferenceSequence", &tags::SpecimenReferenceSequence);
		tag_to_elem.insert(0x00480110, &tags::SpecimenReferenceSequence);

		ident_to_elem.insert("CondenserLensPower", &tags::CondenserLensPower);
		tag_to_elem.insert(0x00480111, &tags::CondenserLensPower);

		ident_to_elem.insert("ObjectiveLensPower", &tags::ObjectiveLensPower);
		tag_to_elem.insert(0x00480112, &tags::ObjectiveLensPower);

		ident_to_elem.insert("ObjectiveLensNumericalAperture", &tags::ObjectiveLensNumericalAperture);
		tag_to_elem.insert(0x00480113, &tags::ObjectiveLensNumericalAperture);

		ident_to_elem.insert("PaletteColorLookupTableSequence", &tags::PaletteColorLookupTableSequence);
		tag_to_elem.insert(0x00480120, &tags::PaletteColorLookupTableSequence);

		ident_to_elem.insert("ReferencedImageNavigationSequence", &tags::ReferencedImageNavigationSequence);
		tag_to_elem.insert(0x00480200, &tags::ReferencedImageNavigationSequence);

		ident_to_elem.insert("TopLeftHandCornerOfLocalizerArea", &tags::TopLeftHandCornerOfLocalizerArea);
		tag_to_elem.insert(0x00480201, &tags::TopLeftHandCornerOfLocalizerArea);

		ident_to_elem.insert("BottomRightHandCornerOfLocalizerArea", &tags::BottomRightHandCornerOfLocalizerArea);
		tag_to_elem.insert(0x00480202, &tags::BottomRightHandCornerOfLocalizerArea);

		ident_to_elem.insert("OpticalPathIdentificationSequence", &tags::OpticalPathIdentificationSequence);
		tag_to_elem.insert(0x00480207, &tags::OpticalPathIdentificationSequence);

		ident_to_elem.insert("PlanePositionSlideSequence", &tags::PlanePositionSlideSequence);
		tag_to_elem.insert(0x0048021A, &tags::PlanePositionSlideSequence);

		ident_to_elem.insert("ColumnPositionInTotalImagePixelMatrix", &tags::ColumnPositionInTotalImagePixelMatrix);
		tag_to_elem.insert(0x0048021E, &tags::ColumnPositionInTotalImagePixelMatrix);

		ident_to_elem.insert("RowPositionInTotalImagePixelMatrix", &tags::RowPositionInTotalImagePixelMatrix);
		tag_to_elem.insert(0x0048021F, &tags::RowPositionInTotalImagePixelMatrix);

		ident_to_elem.insert("PixelOriginInterpretation", &tags::PixelOriginInterpretation);
		tag_to_elem.insert(0x00480301, &tags::PixelOriginInterpretation);

		ident_to_elem.insert("CalibrationImage", &tags::CalibrationImage);
		tag_to_elem.insert(0x00500004, &tags::CalibrationImage);

		ident_to_elem.insert("DeviceSequence", &tags::DeviceSequence);
		tag_to_elem.insert(0x00500010, &tags::DeviceSequence);

		ident_to_elem.insert("ContainerComponentTypeCodeSequence", &tags::ContainerComponentTypeCodeSequence);
		tag_to_elem.insert(0x00500012, &tags::ContainerComponentTypeCodeSequence);

		ident_to_elem.insert("ContainerComponentThickness", &tags::ContainerComponentThickness);
		tag_to_elem.insert(0x00500013, &tags::ContainerComponentThickness);

		ident_to_elem.insert("DeviceLength", &tags::DeviceLength);
		tag_to_elem.insert(0x00500014, &tags::DeviceLength);

		ident_to_elem.insert("ContainerComponentWidth", &tags::ContainerComponentWidth);
		tag_to_elem.insert(0x00500015, &tags::ContainerComponentWidth);

		ident_to_elem.insert("DeviceDiameter", &tags::DeviceDiameter);
		tag_to_elem.insert(0x00500016, &tags::DeviceDiameter);

		ident_to_elem.insert("DeviceDiameterUnits", &tags::DeviceDiameterUnits);
		tag_to_elem.insert(0x00500017, &tags::DeviceDiameterUnits);

		ident_to_elem.insert("DeviceVolume", &tags::DeviceVolume);
		tag_to_elem.insert(0x00500018, &tags::DeviceVolume);

		ident_to_elem.insert("InterMarkerDistance", &tags::InterMarkerDistance);
		tag_to_elem.insert(0x00500019, &tags::InterMarkerDistance);

		ident_to_elem.insert("ContainerComponentMaterial", &tags::ContainerComponentMaterial);
		tag_to_elem.insert(0x0050001A, &tags::ContainerComponentMaterial);

		ident_to_elem.insert("ContainerComponentID", &tags::ContainerComponentID);
		tag_to_elem.insert(0x0050001B, &tags::ContainerComponentID);

		ident_to_elem.insert("ContainerComponentLength", &tags::ContainerComponentLength);
		tag_to_elem.insert(0x0050001C, &tags::ContainerComponentLength);

		ident_to_elem.insert("ContainerComponentDiameter", &tags::ContainerComponentDiameter);
		tag_to_elem.insert(0x0050001D, &tags::ContainerComponentDiameter);

		ident_to_elem.insert("ContainerComponentDescription", &tags::ContainerComponentDescription);
		tag_to_elem.insert(0x0050001E, &tags::ContainerComponentDescription);

		ident_to_elem.insert("DeviceDescription", &tags::DeviceDescription);
		tag_to_elem.insert(0x00500020, &tags::DeviceDescription);

		ident_to_elem.insert("ContrastBolusIngredientPercentByVolume", &tags::ContrastBolusIngredientPercentByVolume);
		tag_to_elem.insert(0x00520001, &tags::ContrastBolusIngredientPercentByVolume);

		ident_to_elem.insert("OCTFocalDistance", &tags::OCTFocalDistance);
		tag_to_elem.insert(0x00520002, &tags::OCTFocalDistance);

		ident_to_elem.insert("BeamSpotSize", &tags::BeamSpotSize);
		tag_to_elem.insert(0x00520003, &tags::BeamSpotSize);

		ident_to_elem.insert("EffectiveRefractiveIndex", &tags::EffectiveRefractiveIndex);
		tag_to_elem.insert(0x00520004, &tags::EffectiveRefractiveIndex);

		ident_to_elem.insert("OCTAcquisitionDomain", &tags::OCTAcquisitionDomain);
		tag_to_elem.insert(0x00520006, &tags::OCTAcquisitionDomain);

		ident_to_elem.insert("OCTOpticalCenterWavelength", &tags::OCTOpticalCenterWavelength);
		tag_to_elem.insert(0x00520007, &tags::OCTOpticalCenterWavelength);

		ident_to_elem.insert("AxialResolution", &tags::AxialResolution);
		tag_to_elem.insert(0x00520008, &tags::AxialResolution);

		ident_to_elem.insert("RangingDepth", &tags::RangingDepth);
		tag_to_elem.insert(0x00520009, &tags::RangingDepth);

		ident_to_elem.insert("ALineRate", &tags::ALineRate);
		tag_to_elem.insert(0x00520011, &tags::ALineRate);

		ident_to_elem.insert("ALinesPerFrame", &tags::ALinesPerFrame);
		tag_to_elem.insert(0x00520012, &tags::ALinesPerFrame);

		ident_to_elem.insert("CatheterRotationalRate", &tags::CatheterRotationalRate);
		tag_to_elem.insert(0x00520013, &tags::CatheterRotationalRate);

		ident_to_elem.insert("ALinePixelSpacing", &tags::ALinePixelSpacing);
		tag_to_elem.insert(0x00520014, &tags::ALinePixelSpacing);

		ident_to_elem.insert("ModeOfPercutaneousAccessSequence", &tags::ModeOfPercutaneousAccessSequence);
		tag_to_elem.insert(0x00520016, &tags::ModeOfPercutaneousAccessSequence);

		ident_to_elem.insert("IntravascularOCTFrameTypeSequence", &tags::IntravascularOCTFrameTypeSequence);
		tag_to_elem.insert(0x00520025, &tags::IntravascularOCTFrameTypeSequence);

		ident_to_elem.insert("OCTZOffsetApplied", &tags::OCTZOffsetApplied);
		tag_to_elem.insert(0x00520026, &tags::OCTZOffsetApplied);

		ident_to_elem.insert("IntravascularFrameContentSequence", &tags::IntravascularFrameContentSequence);
		tag_to_elem.insert(0x00520027, &tags::IntravascularFrameContentSequence);

		ident_to_elem.insert("IntravascularLongitudinalDistance", &tags::IntravascularLongitudinalDistance);
		tag_to_elem.insert(0x00520028, &tags::IntravascularLongitudinalDistance);

		ident_to_elem.insert("IntravascularOCTFrameContentSequence", &tags::IntravascularOCTFrameContentSequence);
		tag_to_elem.insert(0x00520029, &tags::IntravascularOCTFrameContentSequence);

		ident_to_elem.insert("OCTZOffsetCorrection", &tags::OCTZOffsetCorrection);
		tag_to_elem.insert(0x00520030, &tags::OCTZOffsetCorrection);

		ident_to_elem.insert("CatheterDirectionOfRotation", &tags::CatheterDirectionOfRotation);
		tag_to_elem.insert(0x00520031, &tags::CatheterDirectionOfRotation);

		ident_to_elem.insert("SeamLineLocation", &tags::SeamLineLocation);
		tag_to_elem.insert(0x00520033, &tags::SeamLineLocation);

		ident_to_elem.insert("FirstALineLocation", &tags::FirstALineLocation);
		tag_to_elem.insert(0x00520034, &tags::FirstALineLocation);

		ident_to_elem.insert("SeamLineIndex", &tags::SeamLineIndex);
		tag_to_elem.insert(0x00520036, &tags::SeamLineIndex);

		ident_to_elem.insert("NumberOfPaddedALines", &tags::NumberOfPaddedALines);
		tag_to_elem.insert(0x00520038, &tags::NumberOfPaddedALines);

		ident_to_elem.insert("InterpolationType", &tags::InterpolationType);
		tag_to_elem.insert(0x00520039, &tags::InterpolationType);

		ident_to_elem.insert("RefractiveIndexApplied", &tags::RefractiveIndexApplied);
		tag_to_elem.insert(0x0052003A, &tags::RefractiveIndexApplied);

		ident_to_elem.insert("EnergyWindowVector", &tags::EnergyWindowVector);
		tag_to_elem.insert(0x00540010, &tags::EnergyWindowVector);

		ident_to_elem.insert("NumberOfEnergyWindows", &tags::NumberOfEnergyWindows);
		tag_to_elem.insert(0x00540011, &tags::NumberOfEnergyWindows);

		ident_to_elem.insert("EnergyWindowInformationSequence", &tags::EnergyWindowInformationSequence);
		tag_to_elem.insert(0x00540012, &tags::EnergyWindowInformationSequence);

		ident_to_elem.insert("EnergyWindowRangeSequence", &tags::EnergyWindowRangeSequence);
		tag_to_elem.insert(0x00540013, &tags::EnergyWindowRangeSequence);

		ident_to_elem.insert("EnergyWindowLowerLimit", &tags::EnergyWindowLowerLimit);
		tag_to_elem.insert(0x00540014, &tags::EnergyWindowLowerLimit);

		ident_to_elem.insert("EnergyWindowUpperLimit", &tags::EnergyWindowUpperLimit);
		tag_to_elem.insert(0x00540015, &tags::EnergyWindowUpperLimit);

		ident_to_elem.insert("RadiopharmaceuticalInformationSequence", &tags::RadiopharmaceuticalInformationSequence);
		tag_to_elem.insert(0x00540016, &tags::RadiopharmaceuticalInformationSequence);

		ident_to_elem.insert("ResidualSyringeCounts", &tags::ResidualSyringeCounts);
		tag_to_elem.insert(0x00540017, &tags::ResidualSyringeCounts);

		ident_to_elem.insert("EnergyWindowName", &tags::EnergyWindowName);
		tag_to_elem.insert(0x00540018, &tags::EnergyWindowName);

		ident_to_elem.insert("DetectorVector", &tags::DetectorVector);
		tag_to_elem.insert(0x00540020, &tags::DetectorVector);

		ident_to_elem.insert("NumberOfDetectors", &tags::NumberOfDetectors);
		tag_to_elem.insert(0x00540021, &tags::NumberOfDetectors);

		ident_to_elem.insert("DetectorInformationSequence", &tags::DetectorInformationSequence);
		tag_to_elem.insert(0x00540022, &tags::DetectorInformationSequence);

		ident_to_elem.insert("PhaseVector", &tags::PhaseVector);
		tag_to_elem.insert(0x00540030, &tags::PhaseVector);

		ident_to_elem.insert("NumberOfPhases", &tags::NumberOfPhases);
		tag_to_elem.insert(0x00540031, &tags::NumberOfPhases);

		ident_to_elem.insert("PhaseInformationSequence", &tags::PhaseInformationSequence);
		tag_to_elem.insert(0x00540032, &tags::PhaseInformationSequence);

		ident_to_elem.insert("NumberOfFramesInPhase", &tags::NumberOfFramesInPhase);
		tag_to_elem.insert(0x00540033, &tags::NumberOfFramesInPhase);

		ident_to_elem.insert("PhaseDelay", &tags::PhaseDelay);
		tag_to_elem.insert(0x00540036, &tags::PhaseDelay);

		ident_to_elem.insert("PauseBetweenFrames", &tags::PauseBetweenFrames);
		tag_to_elem.insert(0x00540038, &tags::PauseBetweenFrames);

		ident_to_elem.insert("PhaseDescription", &tags::PhaseDescription);
		tag_to_elem.insert(0x00540039, &tags::PhaseDescription);

		ident_to_elem.insert("RotationVector", &tags::RotationVector);
		tag_to_elem.insert(0x00540050, &tags::RotationVector);

		ident_to_elem.insert("NumberOfRotations", &tags::NumberOfRotations);
		tag_to_elem.insert(0x00540051, &tags::NumberOfRotations);

		ident_to_elem.insert("RotationInformationSequence", &tags::RotationInformationSequence);
		tag_to_elem.insert(0x00540052, &tags::RotationInformationSequence);

		ident_to_elem.insert("NumberOfFramesInRotation", &tags::NumberOfFramesInRotation);
		tag_to_elem.insert(0x00540053, &tags::NumberOfFramesInRotation);

		ident_to_elem.insert("RRIntervalVector", &tags::RRIntervalVector);
		tag_to_elem.insert(0x00540060, &tags::RRIntervalVector);

		ident_to_elem.insert("NumberOfRRIntervals", &tags::NumberOfRRIntervals);
		tag_to_elem.insert(0x00540061, &tags::NumberOfRRIntervals);

		ident_to_elem.insert("GatedInformationSequence", &tags::GatedInformationSequence);
		tag_to_elem.insert(0x00540062, &tags::GatedInformationSequence);

		ident_to_elem.insert("DataInformationSequence", &tags::DataInformationSequence);
		tag_to_elem.insert(0x00540063, &tags::DataInformationSequence);

		ident_to_elem.insert("TimeSlotVector", &tags::TimeSlotVector);
		tag_to_elem.insert(0x00540070, &tags::TimeSlotVector);

		ident_to_elem.insert("NumberOfTimeSlots", &tags::NumberOfTimeSlots);
		tag_to_elem.insert(0x00540071, &tags::NumberOfTimeSlots);

		ident_to_elem.insert("TimeSlotInformationSequence", &tags::TimeSlotInformationSequence);
		tag_to_elem.insert(0x00540072, &tags::TimeSlotInformationSequence);

		ident_to_elem.insert("TimeSlotTime", &tags::TimeSlotTime);
		tag_to_elem.insert(0x00540073, &tags::TimeSlotTime);

		ident_to_elem.insert("SliceVector", &tags::SliceVector);
		tag_to_elem.insert(0x00540080, &tags::SliceVector);

		ident_to_elem.insert("NumberOfSlices", &tags::NumberOfSlices);
		tag_to_elem.insert(0x00540081, &tags::NumberOfSlices);

		ident_to_elem.insert("AngularViewVector", &tags::AngularViewVector);
		tag_to_elem.insert(0x00540090, &tags::AngularViewVector);

		ident_to_elem.insert("TimeSliceVector", &tags::TimeSliceVector);
		tag_to_elem.insert(0x00540100, &tags::TimeSliceVector);

		ident_to_elem.insert("NumberOfTimeSlices", &tags::NumberOfTimeSlices);
		tag_to_elem.insert(0x00540101, &tags::NumberOfTimeSlices);

		ident_to_elem.insert("StartAngle", &tags::StartAngle);
		tag_to_elem.insert(0x00540200, &tags::StartAngle);

		ident_to_elem.insert("TypeOfDetectorMotion", &tags::TypeOfDetectorMotion);
		tag_to_elem.insert(0x00540202, &tags::TypeOfDetectorMotion);

		ident_to_elem.insert("TriggerVector", &tags::TriggerVector);
		tag_to_elem.insert(0x00540210, &tags::TriggerVector);

		ident_to_elem.insert("NumberOfTriggersInPhase", &tags::NumberOfTriggersInPhase);
		tag_to_elem.insert(0x00540211, &tags::NumberOfTriggersInPhase);

		ident_to_elem.insert("ViewCodeSequence", &tags::ViewCodeSequence);
		tag_to_elem.insert(0x00540220, &tags::ViewCodeSequence);

		ident_to_elem.insert("ViewModifierCodeSequence", &tags::ViewModifierCodeSequence);
		tag_to_elem.insert(0x00540222, &tags::ViewModifierCodeSequence);

		ident_to_elem.insert("RadionuclideCodeSequence", &tags::RadionuclideCodeSequence);
		tag_to_elem.insert(0x00540300, &tags::RadionuclideCodeSequence);

		ident_to_elem.insert("AdministrationRouteCodeSequence", &tags::AdministrationRouteCodeSequence);
		tag_to_elem.insert(0x00540302, &tags::AdministrationRouteCodeSequence);

		ident_to_elem.insert("RadiopharmaceuticalCodeSequence", &tags::RadiopharmaceuticalCodeSequence);
		tag_to_elem.insert(0x00540304, &tags::RadiopharmaceuticalCodeSequence);

		ident_to_elem.insert("CalibrationDataSequence", &tags::CalibrationDataSequence);
		tag_to_elem.insert(0x00540306, &tags::CalibrationDataSequence);

		ident_to_elem.insert("EnergyWindowNumber", &tags::EnergyWindowNumber);
		tag_to_elem.insert(0x00540308, &tags::EnergyWindowNumber);

		ident_to_elem.insert("ImageID", &tags::ImageID);
		tag_to_elem.insert(0x00540400, &tags::ImageID);

		ident_to_elem.insert("PatientOrientationCodeSequence", &tags::PatientOrientationCodeSequence);
		tag_to_elem.insert(0x00540410, &tags::PatientOrientationCodeSequence);

		ident_to_elem.insert("PatientOrientationModifierCodeSequence", &tags::PatientOrientationModifierCodeSequence);
		tag_to_elem.insert(0x00540412, &tags::PatientOrientationModifierCodeSequence);

		ident_to_elem.insert("PatientGantryRelationshipCodeSequence", &tags::PatientGantryRelationshipCodeSequence);
		tag_to_elem.insert(0x00540414, &tags::PatientGantryRelationshipCodeSequence);

		ident_to_elem.insert("SliceProgressionDirection", &tags::SliceProgressionDirection);
		tag_to_elem.insert(0x00540500, &tags::SliceProgressionDirection);

		ident_to_elem.insert("ScanProgressionDirection", &tags::ScanProgressionDirection);
		tag_to_elem.insert(0x00540501, &tags::ScanProgressionDirection);

		ident_to_elem.insert("SeriesType", &tags::SeriesType);
		tag_to_elem.insert(0x00541000, &tags::SeriesType);

		ident_to_elem.insert("Units", &tags::Units);
		tag_to_elem.insert(0x00541001, &tags::Units);

		ident_to_elem.insert("CountsSource", &tags::CountsSource);
		tag_to_elem.insert(0x00541002, &tags::CountsSource);

		ident_to_elem.insert("ReprojectionMethod", &tags::ReprojectionMethod);
		tag_to_elem.insert(0x00541004, &tags::ReprojectionMethod);

		ident_to_elem.insert("SUVType", &tags::SUVType);
		tag_to_elem.insert(0x00541006, &tags::SUVType);

		ident_to_elem.insert("RandomsCorrectionMethod", &tags::RandomsCorrectionMethod);
		tag_to_elem.insert(0x00541100, &tags::RandomsCorrectionMethod);

		ident_to_elem.insert("AttenuationCorrectionMethod", &tags::AttenuationCorrectionMethod);
		tag_to_elem.insert(0x00541101, &tags::AttenuationCorrectionMethod);

		ident_to_elem.insert("DecayCorrection", &tags::DecayCorrection);
		tag_to_elem.insert(0x00541102, &tags::DecayCorrection);

		ident_to_elem.insert("ReconstructionMethod", &tags::ReconstructionMethod);
		tag_to_elem.insert(0x00541103, &tags::ReconstructionMethod);

		ident_to_elem.insert("DetectorLinesOfResponseUsed", &tags::DetectorLinesOfResponseUsed);
		tag_to_elem.insert(0x00541104, &tags::DetectorLinesOfResponseUsed);

		ident_to_elem.insert("ScatterCorrectionMethod", &tags::ScatterCorrectionMethod);
		tag_to_elem.insert(0x00541105, &tags::ScatterCorrectionMethod);

		ident_to_elem.insert("AxialAcceptance", &tags::AxialAcceptance);
		tag_to_elem.insert(0x00541200, &tags::AxialAcceptance);

		ident_to_elem.insert("AxialMash", &tags::AxialMash);
		tag_to_elem.insert(0x00541201, &tags::AxialMash);

		ident_to_elem.insert("TransverseMash", &tags::TransverseMash);
		tag_to_elem.insert(0x00541202, &tags::TransverseMash);

		ident_to_elem.insert("DetectorElementSize", &tags::DetectorElementSize);
		tag_to_elem.insert(0x00541203, &tags::DetectorElementSize);

		ident_to_elem.insert("CoincidenceWindowWidth", &tags::CoincidenceWindowWidth);
		tag_to_elem.insert(0x00541210, &tags::CoincidenceWindowWidth);

		ident_to_elem.insert("SecondaryCountsType", &tags::SecondaryCountsType);
		tag_to_elem.insert(0x00541220, &tags::SecondaryCountsType);

		ident_to_elem.insert("FrameReferenceTime", &tags::FrameReferenceTime);
		tag_to_elem.insert(0x00541300, &tags::FrameReferenceTime);

		ident_to_elem.insert("PrimaryPromptsCountsAccumulated", &tags::PrimaryPromptsCountsAccumulated);
		tag_to_elem.insert(0x00541310, &tags::PrimaryPromptsCountsAccumulated);

		ident_to_elem.insert("SecondaryCountsAccumulated", &tags::SecondaryCountsAccumulated);
		tag_to_elem.insert(0x00541311, &tags::SecondaryCountsAccumulated);

		ident_to_elem.insert("SliceSensitivityFactor", &tags::SliceSensitivityFactor);
		tag_to_elem.insert(0x00541320, &tags::SliceSensitivityFactor);

		ident_to_elem.insert("DecayFactor", &tags::DecayFactor);
		tag_to_elem.insert(0x00541321, &tags::DecayFactor);

		ident_to_elem.insert("DoseCalibrationFactor", &tags::DoseCalibrationFactor);
		tag_to_elem.insert(0x00541322, &tags::DoseCalibrationFactor);

		ident_to_elem.insert("ScatterFractionFactor", &tags::ScatterFractionFactor);
		tag_to_elem.insert(0x00541323, &tags::ScatterFractionFactor);

		ident_to_elem.insert("DeadTimeFactor", &tags::DeadTimeFactor);
		tag_to_elem.insert(0x00541324, &tags::DeadTimeFactor);

		ident_to_elem.insert("ImageIndex", &tags::ImageIndex);
		tag_to_elem.insert(0x00541330, &tags::ImageIndex);

		ident_to_elem.insert("CountsIncluded", &tags::CountsIncluded);
		tag_to_elem.insert(0x00541400, &tags::CountsIncluded);

		ident_to_elem.insert("DeadTimeCorrectionFlag", &tags::DeadTimeCorrectionFlag);
		tag_to_elem.insert(0x00541401, &tags::DeadTimeCorrectionFlag);

		ident_to_elem.insert("HistogramSequence", &tags::HistogramSequence);
		tag_to_elem.insert(0x00603000, &tags::HistogramSequence);

		ident_to_elem.insert("HistogramNumberOfBins", &tags::HistogramNumberOfBins);
		tag_to_elem.insert(0x00603002, &tags::HistogramNumberOfBins);

		ident_to_elem.insert("HistogramFirstBinValue", &tags::HistogramFirstBinValue);
		tag_to_elem.insert(0x00603004, &tags::HistogramFirstBinValue);

		ident_to_elem.insert("HistogramLastBinValue", &tags::HistogramLastBinValue);
		tag_to_elem.insert(0x00603006, &tags::HistogramLastBinValue);

		ident_to_elem.insert("HistogramBinWidth", &tags::HistogramBinWidth);
		tag_to_elem.insert(0x00603008, &tags::HistogramBinWidth);

		ident_to_elem.insert("HistogramExplanation", &tags::HistogramExplanation);
		tag_to_elem.insert(0x00603010, &tags::HistogramExplanation);

		ident_to_elem.insert("HistogramData", &tags::HistogramData);
		tag_to_elem.insert(0x00603020, &tags::HistogramData);

		ident_to_elem.insert("SegmentationType", &tags::SegmentationType);
		tag_to_elem.insert(0x00620001, &tags::SegmentationType);

		ident_to_elem.insert("SegmentSequence", &tags::SegmentSequence);
		tag_to_elem.insert(0x00620002, &tags::SegmentSequence);

		ident_to_elem.insert("SegmentedPropertyCategoryCodeSequence", &tags::SegmentedPropertyCategoryCodeSequence);
		tag_to_elem.insert(0x00620003, &tags::SegmentedPropertyCategoryCodeSequence);

		ident_to_elem.insert("SegmentNumber", &tags::SegmentNumber);
		tag_to_elem.insert(0x00620004, &tags::SegmentNumber);

		ident_to_elem.insert("SegmentLabel", &tags::SegmentLabel);
		tag_to_elem.insert(0x00620005, &tags::SegmentLabel);

		ident_to_elem.insert("SegmentDescription", &tags::SegmentDescription);
		tag_to_elem.insert(0x00620006, &tags::SegmentDescription);

		ident_to_elem.insert("SegmentationAlgorithmIdentificationSequence", &tags::SegmentationAlgorithmIdentificationSequence);
		tag_to_elem.insert(0x00620007, &tags::SegmentationAlgorithmIdentificationSequence);

		ident_to_elem.insert("SegmentAlgorithmType", &tags::SegmentAlgorithmType);
		tag_to_elem.insert(0x00620008, &tags::SegmentAlgorithmType);

		ident_to_elem.insert("SegmentAlgorithmName", &tags::SegmentAlgorithmName);
		tag_to_elem.insert(0x00620009, &tags::SegmentAlgorithmName);

		ident_to_elem.insert("SegmentIdentificationSequence", &tags::SegmentIdentificationSequence);
		tag_to_elem.insert(0x0062000A, &tags::SegmentIdentificationSequence);

		ident_to_elem.insert("ReferencedSegmentNumber", &tags::ReferencedSegmentNumber);
		tag_to_elem.insert(0x0062000B, &tags::ReferencedSegmentNumber);

		ident_to_elem.insert("RecommendedDisplayGrayscaleValue", &tags::RecommendedDisplayGrayscaleValue);
		tag_to_elem.insert(0x0062000C, &tags::RecommendedDisplayGrayscaleValue);

		ident_to_elem.insert("RecommendedDisplayCIELabValue", &tags::RecommendedDisplayCIELabValue);
		tag_to_elem.insert(0x0062000D, &tags::RecommendedDisplayCIELabValue);

		ident_to_elem.insert("MaximumFractionalValue", &tags::MaximumFractionalValue);
		tag_to_elem.insert(0x0062000E, &tags::MaximumFractionalValue);

		ident_to_elem.insert("SegmentedPropertyTypeCodeSequence", &tags::SegmentedPropertyTypeCodeSequence);
		tag_to_elem.insert(0x0062000F, &tags::SegmentedPropertyTypeCodeSequence);

		ident_to_elem.insert("SegmentationFractionalType", &tags::SegmentationFractionalType);
		tag_to_elem.insert(0x00620010, &tags::SegmentationFractionalType);

		ident_to_elem.insert("SegmentedPropertyTypeModifierCodeSequence", &tags::SegmentedPropertyTypeModifierCodeSequence);
		tag_to_elem.insert(0x00620011, &tags::SegmentedPropertyTypeModifierCodeSequence);

		ident_to_elem.insert("UsedSegmentsSequence", &tags::UsedSegmentsSequence);
		tag_to_elem.insert(0x00620012, &tags::UsedSegmentsSequence);

		ident_to_elem.insert("TrackingID", &tags::TrackingID);
		tag_to_elem.insert(0x00620020, &tags::TrackingID);

		ident_to_elem.insert("TrackingUID", &tags::TrackingUID);
		tag_to_elem.insert(0x00620021, &tags::TrackingUID);

		ident_to_elem.insert("DeformableRegistrationSequence", &tags::DeformableRegistrationSequence);
		tag_to_elem.insert(0x00640002, &tags::DeformableRegistrationSequence);

		ident_to_elem.insert("SourceFrameOfReferenceUID", &tags::SourceFrameOfReferenceUID);
		tag_to_elem.insert(0x00640003, &tags::SourceFrameOfReferenceUID);

		ident_to_elem.insert("DeformableRegistrationGridSequence", &tags::DeformableRegistrationGridSequence);
		tag_to_elem.insert(0x00640005, &tags::DeformableRegistrationGridSequence);

		ident_to_elem.insert("GridDimensions", &tags::GridDimensions);
		tag_to_elem.insert(0x00640007, &tags::GridDimensions);

		ident_to_elem.insert("GridResolution", &tags::GridResolution);
		tag_to_elem.insert(0x00640008, &tags::GridResolution);

		ident_to_elem.insert("VectorGridData", &tags::VectorGridData);
		tag_to_elem.insert(0x00640009, &tags::VectorGridData);

		ident_to_elem.insert("PreDeformationMatrixRegistrationSequence", &tags::PreDeformationMatrixRegistrationSequence);
		tag_to_elem.insert(0x0064000F, &tags::PreDeformationMatrixRegistrationSequence);

		ident_to_elem.insert("PostDeformationMatrixRegistrationSequence", &tags::PostDeformationMatrixRegistrationSequence);
		tag_to_elem.insert(0x00640010, &tags::PostDeformationMatrixRegistrationSequence);

		ident_to_elem.insert("NumberOfSurfaces", &tags::NumberOfSurfaces);
		tag_to_elem.insert(0x00660001, &tags::NumberOfSurfaces);

		ident_to_elem.insert("SurfaceSequence", &tags::SurfaceSequence);
		tag_to_elem.insert(0x00660002, &tags::SurfaceSequence);

		ident_to_elem.insert("SurfaceNumber", &tags::SurfaceNumber);
		tag_to_elem.insert(0x00660003, &tags::SurfaceNumber);

		ident_to_elem.insert("SurfaceComments", &tags::SurfaceComments);
		tag_to_elem.insert(0x00660004, &tags::SurfaceComments);

		ident_to_elem.insert("SurfaceProcessing", &tags::SurfaceProcessing);
		tag_to_elem.insert(0x00660009, &tags::SurfaceProcessing);

		ident_to_elem.insert("SurfaceProcessingRatio", &tags::SurfaceProcessingRatio);
		tag_to_elem.insert(0x0066000A, &tags::SurfaceProcessingRatio);

		ident_to_elem.insert("SurfaceProcessingDescription", &tags::SurfaceProcessingDescription);
		tag_to_elem.insert(0x0066000B, &tags::SurfaceProcessingDescription);

		ident_to_elem.insert("RecommendedPresentationOpacity", &tags::RecommendedPresentationOpacity);
		tag_to_elem.insert(0x0066000C, &tags::RecommendedPresentationOpacity);

		ident_to_elem.insert("RecommendedPresentationType", &tags::RecommendedPresentationType);
		tag_to_elem.insert(0x0066000D, &tags::RecommendedPresentationType);

		ident_to_elem.insert("FiniteVolume", &tags::FiniteVolume);
		tag_to_elem.insert(0x0066000E, &tags::FiniteVolume);

		ident_to_elem.insert("Manifold", &tags::Manifold);
		tag_to_elem.insert(0x00660010, &tags::Manifold);

		ident_to_elem.insert("SurfacePointsSequence", &tags::SurfacePointsSequence);
		tag_to_elem.insert(0x00660011, &tags::SurfacePointsSequence);

		ident_to_elem.insert("SurfacePointsNormalsSequence", &tags::SurfacePointsNormalsSequence);
		tag_to_elem.insert(0x00660012, &tags::SurfacePointsNormalsSequence);

		ident_to_elem.insert("SurfaceMeshPrimitivesSequence", &tags::SurfaceMeshPrimitivesSequence);
		tag_to_elem.insert(0x00660013, &tags::SurfaceMeshPrimitivesSequence);

		ident_to_elem.insert("NumberOfSurfacePoints", &tags::NumberOfSurfacePoints);
		tag_to_elem.insert(0x00660015, &tags::NumberOfSurfacePoints);

		ident_to_elem.insert("PointCoordinatesData", &tags::PointCoordinatesData);
		tag_to_elem.insert(0x00660016, &tags::PointCoordinatesData);

		ident_to_elem.insert("PointPositionAccuracy", &tags::PointPositionAccuracy);
		tag_to_elem.insert(0x00660017, &tags::PointPositionAccuracy);

		ident_to_elem.insert("MeanPointDistance", &tags::MeanPointDistance);
		tag_to_elem.insert(0x00660018, &tags::MeanPointDistance);

		ident_to_elem.insert("MaximumPointDistance", &tags::MaximumPointDistance);
		tag_to_elem.insert(0x00660019, &tags::MaximumPointDistance);

		ident_to_elem.insert("PointsBoundingBoxCoordinates", &tags::PointsBoundingBoxCoordinates);
		tag_to_elem.insert(0x0066001A, &tags::PointsBoundingBoxCoordinates);

		ident_to_elem.insert("AxisOfRotation", &tags::AxisOfRotation);
		tag_to_elem.insert(0x0066001B, &tags::AxisOfRotation);

		ident_to_elem.insert("CenterOfRotation", &tags::CenterOfRotation);
		tag_to_elem.insert(0x0066001C, &tags::CenterOfRotation);

		ident_to_elem.insert("NumberOfVectors", &tags::NumberOfVectors);
		tag_to_elem.insert(0x0066001E, &tags::NumberOfVectors);

		ident_to_elem.insert("VectorDimensionality", &tags::VectorDimensionality);
		tag_to_elem.insert(0x0066001F, &tags::VectorDimensionality);

		ident_to_elem.insert("VectorAccuracy", &tags::VectorAccuracy);
		tag_to_elem.insert(0x00660020, &tags::VectorAccuracy);

		ident_to_elem.insert("VectorCoordinateData", &tags::VectorCoordinateData);
		tag_to_elem.insert(0x00660021, &tags::VectorCoordinateData);

		ident_to_elem.insert("TrianglePointIndexList", &tags::TrianglePointIndexList);
		tag_to_elem.insert(0x00660023, &tags::TrianglePointIndexList);

		ident_to_elem.insert("EdgePointIndexList", &tags::EdgePointIndexList);
		tag_to_elem.insert(0x00660024, &tags::EdgePointIndexList);

		ident_to_elem.insert("VertexPointIndexList", &tags::VertexPointIndexList);
		tag_to_elem.insert(0x00660025, &tags::VertexPointIndexList);

		ident_to_elem.insert("TriangleStripSequence", &tags::TriangleStripSequence);
		tag_to_elem.insert(0x00660026, &tags::TriangleStripSequence);

		ident_to_elem.insert("TriangleFanSequence", &tags::TriangleFanSequence);
		tag_to_elem.insert(0x00660027, &tags::TriangleFanSequence);

		ident_to_elem.insert("LineSequence", &tags::LineSequence);
		tag_to_elem.insert(0x00660028, &tags::LineSequence);

		ident_to_elem.insert("PrimitivePointIndexList", &tags::PrimitivePointIndexList);
		tag_to_elem.insert(0x00660029, &tags::PrimitivePointIndexList);

		ident_to_elem.insert("SurfaceCount", &tags::SurfaceCount);
		tag_to_elem.insert(0x0066002A, &tags::SurfaceCount);

		ident_to_elem.insert("ReferencedSurfaceSequence", &tags::ReferencedSurfaceSequence);
		tag_to_elem.insert(0x0066002B, &tags::ReferencedSurfaceSequence);

		ident_to_elem.insert("ReferencedSurfaceNumber", &tags::ReferencedSurfaceNumber);
		tag_to_elem.insert(0x0066002C, &tags::ReferencedSurfaceNumber);

		ident_to_elem.insert("SegmentSurfaceGenerationAlgorithmIdentificationSequence", &tags::SegmentSurfaceGenerationAlgorithmIdentificationSequence);
		tag_to_elem.insert(0x0066002D, &tags::SegmentSurfaceGenerationAlgorithmIdentificationSequence);

		ident_to_elem.insert("SegmentSurfaceSourceInstanceSequence", &tags::SegmentSurfaceSourceInstanceSequence);
		tag_to_elem.insert(0x0066002E, &tags::SegmentSurfaceSourceInstanceSequence);

		ident_to_elem.insert("AlgorithmFamilyCodeSequence", &tags::AlgorithmFamilyCodeSequence);
		tag_to_elem.insert(0x0066002F, &tags::AlgorithmFamilyCodeSequence);

		ident_to_elem.insert("AlgorithmNameCodeSequence", &tags::AlgorithmNameCodeSequence);
		tag_to_elem.insert(0x00660030, &tags::AlgorithmNameCodeSequence);

		ident_to_elem.insert("AlgorithmVersion", &tags::AlgorithmVersion);
		tag_to_elem.insert(0x00660031, &tags::AlgorithmVersion);

		ident_to_elem.insert("AlgorithmParameters", &tags::AlgorithmParameters);
		tag_to_elem.insert(0x00660032, &tags::AlgorithmParameters);

		ident_to_elem.insert("FacetSequence", &tags::FacetSequence);
		tag_to_elem.insert(0x00660034, &tags::FacetSequence);

		ident_to_elem.insert("SurfaceProcessingAlgorithmIdentificationSequence", &tags::SurfaceProcessingAlgorithmIdentificationSequence);
		tag_to_elem.insert(0x00660035, &tags::SurfaceProcessingAlgorithmIdentificationSequence);

		ident_to_elem.insert("AlgorithmName", &tags::AlgorithmName);
		tag_to_elem.insert(0x00660036, &tags::AlgorithmName);

		ident_to_elem.insert("RecommendedPointRadius", &tags::RecommendedPointRadius);
		tag_to_elem.insert(0x00660037, &tags::RecommendedPointRadius);

		ident_to_elem.insert("RecommendedLineThickness", &tags::RecommendedLineThickness);
		tag_to_elem.insert(0x00660038, &tags::RecommendedLineThickness);

		ident_to_elem.insert("LongPrimitivePointIndexList", &tags::LongPrimitivePointIndexList);
		tag_to_elem.insert(0x00660040, &tags::LongPrimitivePointIndexList);

		ident_to_elem.insert("LongTrianglePointIndexList", &tags::LongTrianglePointIndexList);
		tag_to_elem.insert(0x00660041, &tags::LongTrianglePointIndexList);

		ident_to_elem.insert("LongEdgePointIndexList", &tags::LongEdgePointIndexList);
		tag_to_elem.insert(0x00660042, &tags::LongEdgePointIndexList);

		ident_to_elem.insert("LongVertexPointIndexList", &tags::LongVertexPointIndexList);
		tag_to_elem.insert(0x00660043, &tags::LongVertexPointIndexList);

		ident_to_elem.insert("TrackSetSequence", &tags::TrackSetSequence);
		tag_to_elem.insert(0x00660101, &tags::TrackSetSequence);

		ident_to_elem.insert("TrackSequence", &tags::TrackSequence);
		tag_to_elem.insert(0x00660102, &tags::TrackSequence);

		ident_to_elem.insert("RecommendedDisplayCIELabValueList", &tags::RecommendedDisplayCIELabValueList);
		tag_to_elem.insert(0x00660103, &tags::RecommendedDisplayCIELabValueList);

		ident_to_elem.insert("TrackingAlgorithmIdentificationSequence", &tags::TrackingAlgorithmIdentificationSequence);
		tag_to_elem.insert(0x00660104, &tags::TrackingAlgorithmIdentificationSequence);

		ident_to_elem.insert("TrackSetNumber", &tags::TrackSetNumber);
		tag_to_elem.insert(0x00660105, &tags::TrackSetNumber);

		ident_to_elem.insert("TrackSetLabel", &tags::TrackSetLabel);
		tag_to_elem.insert(0x00660106, &tags::TrackSetLabel);

		ident_to_elem.insert("TrackSetDescription", &tags::TrackSetDescription);
		tag_to_elem.insert(0x00660107, &tags::TrackSetDescription);

		ident_to_elem.insert("TrackSetAnatomicalTypeCodeSequence", &tags::TrackSetAnatomicalTypeCodeSequence);
		tag_to_elem.insert(0x00660108, &tags::TrackSetAnatomicalTypeCodeSequence);

		ident_to_elem.insert("MeasurementsSequence", &tags::MeasurementsSequence);
		tag_to_elem.insert(0x00660121, &tags::MeasurementsSequence);

		ident_to_elem.insert("TrackSetStatisticsSequence", &tags::TrackSetStatisticsSequence);
		tag_to_elem.insert(0x00660124, &tags::TrackSetStatisticsSequence);

		ident_to_elem.insert("FloatingPointValues", &tags::FloatingPointValues);
		tag_to_elem.insert(0x00660125, &tags::FloatingPointValues);

		ident_to_elem.insert("TrackPointIndexList", &tags::TrackPointIndexList);
		tag_to_elem.insert(0x00660129, &tags::TrackPointIndexList);

		ident_to_elem.insert("TrackStatisticsSequence", &tags::TrackStatisticsSequence);
		tag_to_elem.insert(0x00660130, &tags::TrackStatisticsSequence);

		ident_to_elem.insert("MeasurementValuesSequence", &tags::MeasurementValuesSequence);
		tag_to_elem.insert(0x00660132, &tags::MeasurementValuesSequence);

		ident_to_elem.insert("DiffusionAcquisitionCodeSequence", &tags::DiffusionAcquisitionCodeSequence);
		tag_to_elem.insert(0x00660133, &tags::DiffusionAcquisitionCodeSequence);

		ident_to_elem.insert("DiffusionModelCodeSequence", &tags::DiffusionModelCodeSequence);
		tag_to_elem.insert(0x00660134, &tags::DiffusionModelCodeSequence);

		ident_to_elem.insert("ImplantSize", &tags::ImplantSize);
		tag_to_elem.insert(0x00686210, &tags::ImplantSize);

		ident_to_elem.insert("ImplantTemplateVersion", &tags::ImplantTemplateVersion);
		tag_to_elem.insert(0x00686221, &tags::ImplantTemplateVersion);

		ident_to_elem.insert("ReplacedImplantTemplateSequence", &tags::ReplacedImplantTemplateSequence);
		tag_to_elem.insert(0x00686222, &tags::ReplacedImplantTemplateSequence);

		ident_to_elem.insert("ImplantType", &tags::ImplantType);
		tag_to_elem.insert(0x00686223, &tags::ImplantType);

		ident_to_elem.insert("DerivationImplantTemplateSequence", &tags::DerivationImplantTemplateSequence);
		tag_to_elem.insert(0x00686224, &tags::DerivationImplantTemplateSequence);

		ident_to_elem.insert("OriginalImplantTemplateSequence", &tags::OriginalImplantTemplateSequence);
		tag_to_elem.insert(0x00686225, &tags::OriginalImplantTemplateSequence);

		ident_to_elem.insert("EffectiveDateTime", &tags::EffectiveDateTime);
		tag_to_elem.insert(0x00686226, &tags::EffectiveDateTime);

		ident_to_elem.insert("ImplantTargetAnatomySequence", &tags::ImplantTargetAnatomySequence);
		tag_to_elem.insert(0x00686230, &tags::ImplantTargetAnatomySequence);

		ident_to_elem.insert("InformationFromManufacturerSequence", &tags::InformationFromManufacturerSequence);
		tag_to_elem.insert(0x00686260, &tags::InformationFromManufacturerSequence);

		ident_to_elem.insert("NotificationFromManufacturerSequence", &tags::NotificationFromManufacturerSequence);
		tag_to_elem.insert(0x00686265, &tags::NotificationFromManufacturerSequence);

		ident_to_elem.insert("InformationIssueDateTime", &tags::InformationIssueDateTime);
		tag_to_elem.insert(0x00686270, &tags::InformationIssueDateTime);

		ident_to_elem.insert("InformationSummary", &tags::InformationSummary);
		tag_to_elem.insert(0x00686280, &tags::InformationSummary);

		ident_to_elem.insert("ImplantRegulatoryDisapprovalCodeSequence", &tags::ImplantRegulatoryDisapprovalCodeSequence);
		tag_to_elem.insert(0x006862A0, &tags::ImplantRegulatoryDisapprovalCodeSequence);

		ident_to_elem.insert("OverallTemplateSpatialTolerance", &tags::OverallTemplateSpatialTolerance);
		tag_to_elem.insert(0x006862A5, &tags::OverallTemplateSpatialTolerance);

		ident_to_elem.insert("HPGLDocumentSequence", &tags::HPGLDocumentSequence);
		tag_to_elem.insert(0x006862C0, &tags::HPGLDocumentSequence);

		ident_to_elem.insert("HPGLDocumentID", &tags::HPGLDocumentID);
		tag_to_elem.insert(0x006862D0, &tags::HPGLDocumentID);

		ident_to_elem.insert("HPGLDocumentLabel", &tags::HPGLDocumentLabel);
		tag_to_elem.insert(0x006862D5, &tags::HPGLDocumentLabel);

		ident_to_elem.insert("ViewOrientationCodeSequence", &tags::ViewOrientationCodeSequence);
		tag_to_elem.insert(0x006862E0, &tags::ViewOrientationCodeSequence);

		ident_to_elem.insert("ViewOrientationModifier", &tags::ViewOrientationModifier);
		tag_to_elem.insert(0x006862F0, &tags::ViewOrientationModifier);

		ident_to_elem.insert("HPGLDocumentScaling", &tags::HPGLDocumentScaling);
		tag_to_elem.insert(0x006862F2, &tags::HPGLDocumentScaling);

		ident_to_elem.insert("HPGLDocument", &tags::HPGLDocument);
		tag_to_elem.insert(0x00686300, &tags::HPGLDocument);

		ident_to_elem.insert("HPGLContourPenNumber", &tags::HPGLContourPenNumber);
		tag_to_elem.insert(0x00686310, &tags::HPGLContourPenNumber);

		ident_to_elem.insert("HPGLPenSequence", &tags::HPGLPenSequence);
		tag_to_elem.insert(0x00686320, &tags::HPGLPenSequence);

		ident_to_elem.insert("HPGLPenNumber", &tags::HPGLPenNumber);
		tag_to_elem.insert(0x00686330, &tags::HPGLPenNumber);

		ident_to_elem.insert("HPGLPenLabel", &tags::HPGLPenLabel);
		tag_to_elem.insert(0x00686340, &tags::HPGLPenLabel);

		ident_to_elem.insert("HPGLPenDescription", &tags::HPGLPenDescription);
		tag_to_elem.insert(0x00686345, &tags::HPGLPenDescription);

		ident_to_elem.insert("RecommendedRotationPoint", &tags::RecommendedRotationPoint);
		tag_to_elem.insert(0x00686346, &tags::RecommendedRotationPoint);

		ident_to_elem.insert("BoundingRectangle", &tags::BoundingRectangle);
		tag_to_elem.insert(0x00686347, &tags::BoundingRectangle);

		ident_to_elem.insert("ImplantTemplate3DModelSurfaceNumber", &tags::ImplantTemplate3DModelSurfaceNumber);
		tag_to_elem.insert(0x00686350, &tags::ImplantTemplate3DModelSurfaceNumber);

		ident_to_elem.insert("SurfaceModelDescriptionSequence", &tags::SurfaceModelDescriptionSequence);
		tag_to_elem.insert(0x00686360, &tags::SurfaceModelDescriptionSequence);

		ident_to_elem.insert("SurfaceModelLabel", &tags::SurfaceModelLabel);
		tag_to_elem.insert(0x00686380, &tags::SurfaceModelLabel);

		ident_to_elem.insert("SurfaceModelScalingFactor", &tags::SurfaceModelScalingFactor);
		tag_to_elem.insert(0x00686390, &tags::SurfaceModelScalingFactor);

		ident_to_elem.insert("MaterialsCodeSequence", &tags::MaterialsCodeSequence);
		tag_to_elem.insert(0x006863A0, &tags::MaterialsCodeSequence);

		ident_to_elem.insert("CoatingMaterialsCodeSequence", &tags::CoatingMaterialsCodeSequence);
		tag_to_elem.insert(0x006863A4, &tags::CoatingMaterialsCodeSequence);

		ident_to_elem.insert("ImplantTypeCodeSequence", &tags::ImplantTypeCodeSequence);
		tag_to_elem.insert(0x006863A8, &tags::ImplantTypeCodeSequence);

		ident_to_elem.insert("FixationMethodCodeSequence", &tags::FixationMethodCodeSequence);
		tag_to_elem.insert(0x006863AC, &tags::FixationMethodCodeSequence);

		ident_to_elem.insert("MatingFeatureSetsSequence", &tags::MatingFeatureSetsSequence);
		tag_to_elem.insert(0x006863B0, &tags::MatingFeatureSetsSequence);

		ident_to_elem.insert("MatingFeatureSetID", &tags::MatingFeatureSetID);
		tag_to_elem.insert(0x006863C0, &tags::MatingFeatureSetID);

		ident_to_elem.insert("MatingFeatureSetLabel", &tags::MatingFeatureSetLabel);
		tag_to_elem.insert(0x006863D0, &tags::MatingFeatureSetLabel);

		ident_to_elem.insert("MatingFeatureSequence", &tags::MatingFeatureSequence);
		tag_to_elem.insert(0x006863E0, &tags::MatingFeatureSequence);

		ident_to_elem.insert("MatingFeatureID", &tags::MatingFeatureID);
		tag_to_elem.insert(0x006863F0, &tags::MatingFeatureID);

		ident_to_elem.insert("MatingFeatureDegreeOfFreedomSequence", &tags::MatingFeatureDegreeOfFreedomSequence);
		tag_to_elem.insert(0x00686400, &tags::MatingFeatureDegreeOfFreedomSequence);

		ident_to_elem.insert("DegreeOfFreedomID", &tags::DegreeOfFreedomID);
		tag_to_elem.insert(0x00686410, &tags::DegreeOfFreedomID);

		ident_to_elem.insert("DegreeOfFreedomType", &tags::DegreeOfFreedomType);
		tag_to_elem.insert(0x00686420, &tags::DegreeOfFreedomType);

		ident_to_elem.insert("TwoDMatingFeatureCoordinatesSequence", &tags::TwoDMatingFeatureCoordinatesSequence);
		tag_to_elem.insert(0x00686430, &tags::TwoDMatingFeatureCoordinatesSequence);

		ident_to_elem.insert("ReferencedHPGLDocumentID", &tags::ReferencedHPGLDocumentID);
		tag_to_elem.insert(0x00686440, &tags::ReferencedHPGLDocumentID);

		ident_to_elem.insert("TwoDMatingPoint", &tags::TwoDMatingPoint);
		tag_to_elem.insert(0x00686450, &tags::TwoDMatingPoint);

		ident_to_elem.insert("TwoDMatingAxes", &tags::TwoDMatingAxes);
		tag_to_elem.insert(0x00686460, &tags::TwoDMatingAxes);

		ident_to_elem.insert("TwoDDegreeOfFreedomSequence", &tags::TwoDDegreeOfFreedomSequence);
		tag_to_elem.insert(0x00686470, &tags::TwoDDegreeOfFreedomSequence);

		ident_to_elem.insert("ThreeDDegreeOfFreedomAxis", &tags::ThreeDDegreeOfFreedomAxis);
		tag_to_elem.insert(0x00686490, &tags::ThreeDDegreeOfFreedomAxis);

		ident_to_elem.insert("RangeOfFreedom", &tags::RangeOfFreedom);
		tag_to_elem.insert(0x006864A0, &tags::RangeOfFreedom);

		ident_to_elem.insert("ThreeDMatingPoint", &tags::ThreeDMatingPoint);
		tag_to_elem.insert(0x006864C0, &tags::ThreeDMatingPoint);

		ident_to_elem.insert("ThreeDMatingAxes", &tags::ThreeDMatingAxes);
		tag_to_elem.insert(0x006864D0, &tags::ThreeDMatingAxes);

		ident_to_elem.insert("TwoDDegreeOfFreedomAxis", &tags::TwoDDegreeOfFreedomAxis);
		tag_to_elem.insert(0x006864F0, &tags::TwoDDegreeOfFreedomAxis);

		ident_to_elem.insert("PlanningLandmarkPointSequence", &tags::PlanningLandmarkPointSequence);
		tag_to_elem.insert(0x00686500, &tags::PlanningLandmarkPointSequence);

		ident_to_elem.insert("PlanningLandmarkLineSequence", &tags::PlanningLandmarkLineSequence);
		tag_to_elem.insert(0x00686510, &tags::PlanningLandmarkLineSequence);

		ident_to_elem.insert("PlanningLandmarkPlaneSequence", &tags::PlanningLandmarkPlaneSequence);
		tag_to_elem.insert(0x00686520, &tags::PlanningLandmarkPlaneSequence);

		ident_to_elem.insert("PlanningLandmarkID", &tags::PlanningLandmarkID);
		tag_to_elem.insert(0x00686530, &tags::PlanningLandmarkID);

		ident_to_elem.insert("PlanningLandmarkDescription", &tags::PlanningLandmarkDescription);
		tag_to_elem.insert(0x00686540, &tags::PlanningLandmarkDescription);

		ident_to_elem.insert("PlanningLandmarkIdentificationCodeSequence", &tags::PlanningLandmarkIdentificationCodeSequence);
		tag_to_elem.insert(0x00686545, &tags::PlanningLandmarkIdentificationCodeSequence);

		ident_to_elem.insert("TwoDPointCoordinatesSequence", &tags::TwoDPointCoordinatesSequence);
		tag_to_elem.insert(0x00686550, &tags::TwoDPointCoordinatesSequence);

		ident_to_elem.insert("TwoDPointCoordinates", &tags::TwoDPointCoordinates);
		tag_to_elem.insert(0x00686560, &tags::TwoDPointCoordinates);

		ident_to_elem.insert("ThreeDPointCoordinates", &tags::ThreeDPointCoordinates);
		tag_to_elem.insert(0x00686590, &tags::ThreeDPointCoordinates);

		ident_to_elem.insert("TwoDLineCoordinatesSequence", &tags::TwoDLineCoordinatesSequence);
		tag_to_elem.insert(0x006865A0, &tags::TwoDLineCoordinatesSequence);

		ident_to_elem.insert("TwoDLineCoordinates", &tags::TwoDLineCoordinates);
		tag_to_elem.insert(0x006865B0, &tags::TwoDLineCoordinates);

		ident_to_elem.insert("ThreeDLineCoordinates", &tags::ThreeDLineCoordinates);
		tag_to_elem.insert(0x006865D0, &tags::ThreeDLineCoordinates);

		ident_to_elem.insert("TwoDPlaneCoordinatesSequence", &tags::TwoDPlaneCoordinatesSequence);
		tag_to_elem.insert(0x006865E0, &tags::TwoDPlaneCoordinatesSequence);

		ident_to_elem.insert("TwoDPlaneIntersection", &tags::TwoDPlaneIntersection);
		tag_to_elem.insert(0x006865F0, &tags::TwoDPlaneIntersection);

		ident_to_elem.insert("ThreeDPlaneOrigin", &tags::ThreeDPlaneOrigin);
		tag_to_elem.insert(0x00686610, &tags::ThreeDPlaneOrigin);

		ident_to_elem.insert("ThreeDPlaneNormal", &tags::ThreeDPlaneNormal);
		tag_to_elem.insert(0x00686620, &tags::ThreeDPlaneNormal);

		ident_to_elem.insert("GraphicAnnotationSequence", &tags::GraphicAnnotationSequence);
		tag_to_elem.insert(0x00700001, &tags::GraphicAnnotationSequence);

		ident_to_elem.insert("GraphicLayer", &tags::GraphicLayer);
		tag_to_elem.insert(0x00700002, &tags::GraphicLayer);

		ident_to_elem.insert("BoundingBoxAnnotationUnits", &tags::BoundingBoxAnnotationUnits);
		tag_to_elem.insert(0x00700003, &tags::BoundingBoxAnnotationUnits);

		ident_to_elem.insert("AnchorPointAnnotationUnits", &tags::AnchorPointAnnotationUnits);
		tag_to_elem.insert(0x00700004, &tags::AnchorPointAnnotationUnits);

		ident_to_elem.insert("GraphicAnnotationUnits", &tags::GraphicAnnotationUnits);
		tag_to_elem.insert(0x00700005, &tags::GraphicAnnotationUnits);

		ident_to_elem.insert("UnformattedTextValue", &tags::UnformattedTextValue);
		tag_to_elem.insert(0x00700006, &tags::UnformattedTextValue);

		ident_to_elem.insert("TextObjectSequence", &tags::TextObjectSequence);
		tag_to_elem.insert(0x00700008, &tags::TextObjectSequence);

		ident_to_elem.insert("GraphicObjectSequence", &tags::GraphicObjectSequence);
		tag_to_elem.insert(0x00700009, &tags::GraphicObjectSequence);

		ident_to_elem.insert("BoundingBoxTopLeftHandCorner", &tags::BoundingBoxTopLeftHandCorner);
		tag_to_elem.insert(0x00700010, &tags::BoundingBoxTopLeftHandCorner);

		ident_to_elem.insert("BoundingBoxBottomRightHandCorner", &tags::BoundingBoxBottomRightHandCorner);
		tag_to_elem.insert(0x00700011, &tags::BoundingBoxBottomRightHandCorner);

		ident_to_elem.insert("BoundingBoxTextHorizontalJustification", &tags::BoundingBoxTextHorizontalJustification);
		tag_to_elem.insert(0x00700012, &tags::BoundingBoxTextHorizontalJustification);

		ident_to_elem.insert("AnchorPoint", &tags::AnchorPoint);
		tag_to_elem.insert(0x00700014, &tags::AnchorPoint);

		ident_to_elem.insert("AnchorPointVisibility", &tags::AnchorPointVisibility);
		tag_to_elem.insert(0x00700015, &tags::AnchorPointVisibility);

		ident_to_elem.insert("GraphicDimensions", &tags::GraphicDimensions);
		tag_to_elem.insert(0x00700020, &tags::GraphicDimensions);

		ident_to_elem.insert("NumberOfGraphicPoints", &tags::NumberOfGraphicPoints);
		tag_to_elem.insert(0x00700021, &tags::NumberOfGraphicPoints);

		ident_to_elem.insert("GraphicData", &tags::GraphicData);
		tag_to_elem.insert(0x00700022, &tags::GraphicData);

		ident_to_elem.insert("GraphicType", &tags::GraphicType);
		tag_to_elem.insert(0x00700023, &tags::GraphicType);

		ident_to_elem.insert("GraphicFilled", &tags::GraphicFilled);
		tag_to_elem.insert(0x00700024, &tags::GraphicFilled);

		ident_to_elem.insert("ImageRotationRetired", &tags::ImageRotationRetired);
		tag_to_elem.insert(0x00700040, &tags::ImageRotationRetired);

		ident_to_elem.insert("ImageHorizontalFlip", &tags::ImageHorizontalFlip);
		tag_to_elem.insert(0x00700041, &tags::ImageHorizontalFlip);

		ident_to_elem.insert("ImageRotation", &tags::ImageRotation);
		tag_to_elem.insert(0x00700042, &tags::ImageRotation);

		ident_to_elem.insert("DisplayedAreaTopLeftHandCornerTrial", &tags::DisplayedAreaTopLeftHandCornerTrial);
		tag_to_elem.insert(0x00700050, &tags::DisplayedAreaTopLeftHandCornerTrial);

		ident_to_elem.insert("DisplayedAreaBottomRightHandCornerTrial", &tags::DisplayedAreaBottomRightHandCornerTrial);
		tag_to_elem.insert(0x00700051, &tags::DisplayedAreaBottomRightHandCornerTrial);

		ident_to_elem.insert("DisplayedAreaTopLeftHandCorner", &tags::DisplayedAreaTopLeftHandCorner);
		tag_to_elem.insert(0x00700052, &tags::DisplayedAreaTopLeftHandCorner);

		ident_to_elem.insert("DisplayedAreaBottomRightHandCorner", &tags::DisplayedAreaBottomRightHandCorner);
		tag_to_elem.insert(0x00700053, &tags::DisplayedAreaBottomRightHandCorner);

		ident_to_elem.insert("DisplayedAreaSelectionSequence", &tags::DisplayedAreaSelectionSequence);
		tag_to_elem.insert(0x0070005A, &tags::DisplayedAreaSelectionSequence);

		ident_to_elem.insert("GraphicLayerSequence", &tags::GraphicLayerSequence);
		tag_to_elem.insert(0x00700060, &tags::GraphicLayerSequence);

		ident_to_elem.insert("GraphicLayerOrder", &tags::GraphicLayerOrder);
		tag_to_elem.insert(0x00700062, &tags::GraphicLayerOrder);

		ident_to_elem.insert("GraphicLayerRecommendedDisplayGrayscaleValue", &tags::GraphicLayerRecommendedDisplayGrayscaleValue);
		tag_to_elem.insert(0x00700066, &tags::GraphicLayerRecommendedDisplayGrayscaleValue);

		ident_to_elem.insert("GraphicLayerRecommendedDisplayRGBValue", &tags::GraphicLayerRecommendedDisplayRGBValue);
		tag_to_elem.insert(0x00700067, &tags::GraphicLayerRecommendedDisplayRGBValue);

		ident_to_elem.insert("GraphicLayerDescription", &tags::GraphicLayerDescription);
		tag_to_elem.insert(0x00700068, &tags::GraphicLayerDescription);

		ident_to_elem.insert("ContentLabel", &tags::ContentLabel);
		tag_to_elem.insert(0x00700080, &tags::ContentLabel);

		ident_to_elem.insert("ContentDescription", &tags::ContentDescription);
		tag_to_elem.insert(0x00700081, &tags::ContentDescription);

		ident_to_elem.insert("PresentationCreationDate", &tags::PresentationCreationDate);
		tag_to_elem.insert(0x00700082, &tags::PresentationCreationDate);

		ident_to_elem.insert("PresentationCreationTime", &tags::PresentationCreationTime);
		tag_to_elem.insert(0x00700083, &tags::PresentationCreationTime);

		ident_to_elem.insert("ContentCreatorName", &tags::ContentCreatorName);
		tag_to_elem.insert(0x00700084, &tags::ContentCreatorName);

		ident_to_elem.insert("ContentCreatorIdentificationCodeSequence", &tags::ContentCreatorIdentificationCodeSequence);
		tag_to_elem.insert(0x00700086, &tags::ContentCreatorIdentificationCodeSequence);

		ident_to_elem.insert("AlternateContentDescriptionSequence", &tags::AlternateContentDescriptionSequence);
		tag_to_elem.insert(0x00700087, &tags::AlternateContentDescriptionSequence);

		ident_to_elem.insert("PresentationSizeMode", &tags::PresentationSizeMode);
		tag_to_elem.insert(0x00700100, &tags::PresentationSizeMode);

		ident_to_elem.insert("PresentationPixelSpacing", &tags::PresentationPixelSpacing);
		tag_to_elem.insert(0x00700101, &tags::PresentationPixelSpacing);

		ident_to_elem.insert("PresentationPixelAspectRatio", &tags::PresentationPixelAspectRatio);
		tag_to_elem.insert(0x00700102, &tags::PresentationPixelAspectRatio);

		ident_to_elem.insert("PresentationPixelMagnificationRatio", &tags::PresentationPixelMagnificationRatio);
		tag_to_elem.insert(0x00700103, &tags::PresentationPixelMagnificationRatio);

		ident_to_elem.insert("GraphicGroupLabel", &tags::GraphicGroupLabel);
		tag_to_elem.insert(0x00700207, &tags::GraphicGroupLabel);

		ident_to_elem.insert("GraphicGroupDescription", &tags::GraphicGroupDescription);
		tag_to_elem.insert(0x00700208, &tags::GraphicGroupDescription);

		ident_to_elem.insert("CompoundGraphicSequence", &tags::CompoundGraphicSequence);
		tag_to_elem.insert(0x00700209, &tags::CompoundGraphicSequence);

		ident_to_elem.insert("CompoundGraphicInstanceID", &tags::CompoundGraphicInstanceID);
		tag_to_elem.insert(0x00700226, &tags::CompoundGraphicInstanceID);

		ident_to_elem.insert("FontName", &tags::FontName);
		tag_to_elem.insert(0x00700227, &tags::FontName);

		ident_to_elem.insert("FontNameType", &tags::FontNameType);
		tag_to_elem.insert(0x00700228, &tags::FontNameType);

		ident_to_elem.insert("CSSFontName", &tags::CSSFontName);
		tag_to_elem.insert(0x00700229, &tags::CSSFontName);

		ident_to_elem.insert("RotationAngle", &tags::RotationAngle);
		tag_to_elem.insert(0x00700230, &tags::RotationAngle);

		ident_to_elem.insert("TextStyleSequence", &tags::TextStyleSequence);
		tag_to_elem.insert(0x00700231, &tags::TextStyleSequence);

		ident_to_elem.insert("LineStyleSequence", &tags::LineStyleSequence);
		tag_to_elem.insert(0x00700232, &tags::LineStyleSequence);

		ident_to_elem.insert("FillStyleSequence", &tags::FillStyleSequence);
		tag_to_elem.insert(0x00700233, &tags::FillStyleSequence);

		ident_to_elem.insert("GraphicGroupSequence", &tags::GraphicGroupSequence);
		tag_to_elem.insert(0x00700234, &tags::GraphicGroupSequence);

		ident_to_elem.insert("TextColorCIELabValue", &tags::TextColorCIELabValue);
		tag_to_elem.insert(0x00700241, &tags::TextColorCIELabValue);

		ident_to_elem.insert("HorizontalAlignment", &tags::HorizontalAlignment);
		tag_to_elem.insert(0x00700242, &tags::HorizontalAlignment);

		ident_to_elem.insert("VerticalAlignment", &tags::VerticalAlignment);
		tag_to_elem.insert(0x00700243, &tags::VerticalAlignment);

		ident_to_elem.insert("ShadowStyle", &tags::ShadowStyle);
		tag_to_elem.insert(0x00700244, &tags::ShadowStyle);

		ident_to_elem.insert("ShadowOffsetX", &tags::ShadowOffsetX);
		tag_to_elem.insert(0x00700245, &tags::ShadowOffsetX);

		ident_to_elem.insert("ShadowOffsetY", &tags::ShadowOffsetY);
		tag_to_elem.insert(0x00700246, &tags::ShadowOffsetY);

		ident_to_elem.insert("ShadowColorCIELabValue", &tags::ShadowColorCIELabValue);
		tag_to_elem.insert(0x00700247, &tags::ShadowColorCIELabValue);

		ident_to_elem.insert("Underlined", &tags::Underlined);
		tag_to_elem.insert(0x00700248, &tags::Underlined);

		ident_to_elem.insert("Bold", &tags::Bold);
		tag_to_elem.insert(0x00700249, &tags::Bold);

		ident_to_elem.insert("Italic", &tags::Italic);
		tag_to_elem.insert(0x00700250, &tags::Italic);

		ident_to_elem.insert("PatternOnColorCIELabValue", &tags::PatternOnColorCIELabValue);
		tag_to_elem.insert(0x00700251, &tags::PatternOnColorCIELabValue);

		ident_to_elem.insert("PatternOffColorCIELabValue", &tags::PatternOffColorCIELabValue);
		tag_to_elem.insert(0x00700252, &tags::PatternOffColorCIELabValue);

		ident_to_elem.insert("LineThickness", &tags::LineThickness);
		tag_to_elem.insert(0x00700253, &tags::LineThickness);

		ident_to_elem.insert("LineDashingStyle", &tags::LineDashingStyle);
		tag_to_elem.insert(0x00700254, &tags::LineDashingStyle);

		ident_to_elem.insert("LinePattern", &tags::LinePattern);
		tag_to_elem.insert(0x00700255, &tags::LinePattern);

		ident_to_elem.insert("FillPattern", &tags::FillPattern);
		tag_to_elem.insert(0x00700256, &tags::FillPattern);

		ident_to_elem.insert("FillMode", &tags::FillMode);
		tag_to_elem.insert(0x00700257, &tags::FillMode);

		ident_to_elem.insert("ShadowOpacity", &tags::ShadowOpacity);
		tag_to_elem.insert(0x00700258, &tags::ShadowOpacity);

		ident_to_elem.insert("GapLength", &tags::GapLength);
		tag_to_elem.insert(0x00700261, &tags::GapLength);

		ident_to_elem.insert("DiameterOfVisibility", &tags::DiameterOfVisibility);
		tag_to_elem.insert(0x00700262, &tags::DiameterOfVisibility);

		ident_to_elem.insert("RotationPoint", &tags::RotationPoint);
		tag_to_elem.insert(0x00700273, &tags::RotationPoint);

		ident_to_elem.insert("TickAlignment", &tags::TickAlignment);
		tag_to_elem.insert(0x00700274, &tags::TickAlignment);

		ident_to_elem.insert("ShowTickLabel", &tags::ShowTickLabel);
		tag_to_elem.insert(0x00700278, &tags::ShowTickLabel);

		ident_to_elem.insert("TickLabelAlignment", &tags::TickLabelAlignment);
		tag_to_elem.insert(0x00700279, &tags::TickLabelAlignment);

		ident_to_elem.insert("CompoundGraphicUnits", &tags::CompoundGraphicUnits);
		tag_to_elem.insert(0x00700282, &tags::CompoundGraphicUnits);

		ident_to_elem.insert("PatternOnOpacity", &tags::PatternOnOpacity);
		tag_to_elem.insert(0x00700284, &tags::PatternOnOpacity);

		ident_to_elem.insert("PatternOffOpacity", &tags::PatternOffOpacity);
		tag_to_elem.insert(0x00700285, &tags::PatternOffOpacity);

		ident_to_elem.insert("MajorTicksSequence", &tags::MajorTicksSequence);
		tag_to_elem.insert(0x00700287, &tags::MajorTicksSequence);

		ident_to_elem.insert("TickPosition", &tags::TickPosition);
		tag_to_elem.insert(0x00700288, &tags::TickPosition);

		ident_to_elem.insert("TickLabel", &tags::TickLabel);
		tag_to_elem.insert(0x00700289, &tags::TickLabel);

		ident_to_elem.insert("CompoundGraphicType", &tags::CompoundGraphicType);
		tag_to_elem.insert(0x00700294, &tags::CompoundGraphicType);

		ident_to_elem.insert("GraphicGroupID", &tags::GraphicGroupID);
		tag_to_elem.insert(0x00700295, &tags::GraphicGroupID);

		ident_to_elem.insert("ShapeType", &tags::ShapeType);
		tag_to_elem.insert(0x00700306, &tags::ShapeType);

		ident_to_elem.insert("RegistrationSequence", &tags::RegistrationSequence);
		tag_to_elem.insert(0x00700308, &tags::RegistrationSequence);

		ident_to_elem.insert("MatrixRegistrationSequence", &tags::MatrixRegistrationSequence);
		tag_to_elem.insert(0x00700309, &tags::MatrixRegistrationSequence);

		ident_to_elem.insert("MatrixSequence", &tags::MatrixSequence);
		tag_to_elem.insert(0x0070030A, &tags::MatrixSequence);

		ident_to_elem.insert("FrameOfReferenceToDisplayedCoordinateSystemTransformationMatrix", &tags::FrameOfReferenceToDisplayedCoordinateSystemTransformationMatrix);
		tag_to_elem.insert(0x0070030B, &tags::FrameOfReferenceToDisplayedCoordinateSystemTransformationMatrix);

		ident_to_elem.insert("FrameOfReferenceTransformationMatrixType", &tags::FrameOfReferenceTransformationMatrixType);
		tag_to_elem.insert(0x0070030C, &tags::FrameOfReferenceTransformationMatrixType);

		ident_to_elem.insert("RegistrationTypeCodeSequence", &tags::RegistrationTypeCodeSequence);
		tag_to_elem.insert(0x0070030D, &tags::RegistrationTypeCodeSequence);

		ident_to_elem.insert("FiducialDescription", &tags::FiducialDescription);
		tag_to_elem.insert(0x0070030F, &tags::FiducialDescription);

		ident_to_elem.insert("FiducialIdentifier", &tags::FiducialIdentifier);
		tag_to_elem.insert(0x00700310, &tags::FiducialIdentifier);

		ident_to_elem.insert("FiducialIdentifierCodeSequence", &tags::FiducialIdentifierCodeSequence);
		tag_to_elem.insert(0x00700311, &tags::FiducialIdentifierCodeSequence);

		ident_to_elem.insert("ContourUncertaintyRadius", &tags::ContourUncertaintyRadius);
		tag_to_elem.insert(0x00700312, &tags::ContourUncertaintyRadius);

		ident_to_elem.insert("UsedFiducialsSequence", &tags::UsedFiducialsSequence);
		tag_to_elem.insert(0x00700314, &tags::UsedFiducialsSequence);

		ident_to_elem.insert("GraphicCoordinatesDataSequence", &tags::GraphicCoordinatesDataSequence);
		tag_to_elem.insert(0x00700318, &tags::GraphicCoordinatesDataSequence);

		ident_to_elem.insert("FiducialUID", &tags::FiducialUID);
		tag_to_elem.insert(0x0070031A, &tags::FiducialUID);

		ident_to_elem.insert("FiducialSetSequence", &tags::FiducialSetSequence);
		tag_to_elem.insert(0x0070031C, &tags::FiducialSetSequence);

		ident_to_elem.insert("FiducialSequence", &tags::FiducialSequence);
		tag_to_elem.insert(0x0070031E, &tags::FiducialSequence);

		ident_to_elem.insert("FiducialsPropertyCategoryCodeSequence", &tags::FiducialsPropertyCategoryCodeSequence);
		tag_to_elem.insert(0x0070031F, &tags::FiducialsPropertyCategoryCodeSequence);

		ident_to_elem.insert("GraphicLayerRecommendedDisplayCIELabValue", &tags::GraphicLayerRecommendedDisplayCIELabValue);
		tag_to_elem.insert(0x00700401, &tags::GraphicLayerRecommendedDisplayCIELabValue);

		ident_to_elem.insert("BlendingSequence", &tags::BlendingSequence);
		tag_to_elem.insert(0x00700402, &tags::BlendingSequence);

		ident_to_elem.insert("RelativeOpacity", &tags::RelativeOpacity);
		tag_to_elem.insert(0x00700403, &tags::RelativeOpacity);

		ident_to_elem.insert("ReferencedSpatialRegistrationSequence", &tags::ReferencedSpatialRegistrationSequence);
		tag_to_elem.insert(0x00700404, &tags::ReferencedSpatialRegistrationSequence);

		ident_to_elem.insert("BlendingPosition", &tags::BlendingPosition);
		tag_to_elem.insert(0x00700405, &tags::BlendingPosition);

		ident_to_elem.insert("PresentationDisplayCollectionUID", &tags::PresentationDisplayCollectionUID);
		tag_to_elem.insert(0x00701101, &tags::PresentationDisplayCollectionUID);

		ident_to_elem.insert("PresentationSequenceCollectionUID", &tags::PresentationSequenceCollectionUID);
		tag_to_elem.insert(0x00701102, &tags::PresentationSequenceCollectionUID);

		ident_to_elem.insert("PresentationSequencePositionIndex", &tags::PresentationSequencePositionIndex);
		tag_to_elem.insert(0x00701103, &tags::PresentationSequencePositionIndex);

		ident_to_elem.insert("RenderedImageReferenceSequence", &tags::RenderedImageReferenceSequence);
		tag_to_elem.insert(0x00701104, &tags::RenderedImageReferenceSequence);

		ident_to_elem.insert("VolumetricPresentationStateInputSequence", &tags::VolumetricPresentationStateInputSequence);
		tag_to_elem.insert(0x00701201, &tags::VolumetricPresentationStateInputSequence);

		ident_to_elem.insert("PresentationInputType", &tags::PresentationInputType);
		tag_to_elem.insert(0x00701202, &tags::PresentationInputType);

		ident_to_elem.insert("InputSequencePositionIndex", &tags::InputSequencePositionIndex);
		tag_to_elem.insert(0x00701203, &tags::InputSequencePositionIndex);

		ident_to_elem.insert("Crop", &tags::Crop);
		tag_to_elem.insert(0x00701204, &tags::Crop);

		ident_to_elem.insert("CroppingSpecificationIndex", &tags::CroppingSpecificationIndex);
		tag_to_elem.insert(0x00701205, &tags::CroppingSpecificationIndex);

		ident_to_elem.insert("CompositingMethod", &tags::CompositingMethod);
		tag_to_elem.insert(0x00701206, &tags::CompositingMethod);

		ident_to_elem.insert("VolumetricPresentationInputNumber", &tags::VolumetricPresentationInputNumber);
		tag_to_elem.insert(0x00701207, &tags::VolumetricPresentationInputNumber);

		ident_to_elem.insert("ImageVolumeGeometry", &tags::ImageVolumeGeometry);
		tag_to_elem.insert(0x00701208, &tags::ImageVolumeGeometry);

		ident_to_elem.insert("VolumeCroppingSequence", &tags::VolumeCroppingSequence);
		tag_to_elem.insert(0x00701301, &tags::VolumeCroppingSequence);

		ident_to_elem.insert("VolumeCroppingMethod", &tags::VolumeCroppingMethod);
		tag_to_elem.insert(0x00701302, &tags::VolumeCroppingMethod);

		ident_to_elem.insert("BoundingBoxCrop", &tags::BoundingBoxCrop);
		tag_to_elem.insert(0x00701303, &tags::BoundingBoxCrop);

		ident_to_elem.insert("ObliqueCroppingPlaneSequence", &tags::ObliqueCroppingPlaneSequence);
		tag_to_elem.insert(0x00701304, &tags::ObliqueCroppingPlaneSequence);

		ident_to_elem.insert("Plane", &tags::Plane);
		tag_to_elem.insert(0x00701305, &tags::Plane);

		ident_to_elem.insert("PlaneNormal", &tags::PlaneNormal);
		tag_to_elem.insert(0x00701306, &tags::PlaneNormal);

		ident_to_elem.insert("CroppingSpecificationNumber", &tags::CroppingSpecificationNumber);
		tag_to_elem.insert(0x00701309, &tags::CroppingSpecificationNumber);

		ident_to_elem.insert("MultiPlanarReconstructionStyle", &tags::MultiPlanarReconstructionStyle);
		tag_to_elem.insert(0x00701501, &tags::MultiPlanarReconstructionStyle);

		ident_to_elem.insert("MPRThicknessType", &tags::MPRThicknessType);
		tag_to_elem.insert(0x00701502, &tags::MPRThicknessType);

		ident_to_elem.insert("MPRSlabThickness", &tags::MPRSlabThickness);
		tag_to_elem.insert(0x00701503, &tags::MPRSlabThickness);

		ident_to_elem.insert("MPRTopLeftHandCorner", &tags::MPRTopLeftHandCorner);
		tag_to_elem.insert(0x00701505, &tags::MPRTopLeftHandCorner);

		ident_to_elem.insert("MPRViewWidthDirection", &tags::MPRViewWidthDirection);
		tag_to_elem.insert(0x00701507, &tags::MPRViewWidthDirection);

		ident_to_elem.insert("MPRViewWidth", &tags::MPRViewWidth);
		tag_to_elem.insert(0x00701508, &tags::MPRViewWidth);

		ident_to_elem.insert("NumberOfVolumetricCurvePoints", &tags::NumberOfVolumetricCurvePoints);
		tag_to_elem.insert(0x0070150C, &tags::NumberOfVolumetricCurvePoints);

		ident_to_elem.insert("VolumetricCurvePoints", &tags::VolumetricCurvePoints);
		tag_to_elem.insert(0x0070150D, &tags::VolumetricCurvePoints);

		ident_to_elem.insert("MPRViewHeightDirection", &tags::MPRViewHeightDirection);
		tag_to_elem.insert(0x00701511, &tags::MPRViewHeightDirection);

		ident_to_elem.insert("MPRViewHeight", &tags::MPRViewHeight);
		tag_to_elem.insert(0x00701512, &tags::MPRViewHeight);

		ident_to_elem.insert("PresentationStateClassificationComponentSequence", &tags::PresentationStateClassificationComponentSequence);
		tag_to_elem.insert(0x00701801, &tags::PresentationStateClassificationComponentSequence);

		ident_to_elem.insert("ComponentType", &tags::ComponentType);
		tag_to_elem.insert(0x00701802, &tags::ComponentType);

		ident_to_elem.insert("ComponentInputSequence", &tags::ComponentInputSequence);
		tag_to_elem.insert(0x00701803, &tags::ComponentInputSequence);

		ident_to_elem.insert("VolumetricPresentationInputIndex", &tags::VolumetricPresentationInputIndex);
		tag_to_elem.insert(0x00701804, &tags::VolumetricPresentationInputIndex);

		ident_to_elem.insert("PresentationStateCompositorComponentSequence", &tags::PresentationStateCompositorComponentSequence);
		tag_to_elem.insert(0x00701805, &tags::PresentationStateCompositorComponentSequence);

		ident_to_elem.insert("WeightingTransferFunctionSequence", &tags::WeightingTransferFunctionSequence);
		tag_to_elem.insert(0x00701806, &tags::WeightingTransferFunctionSequence);

		ident_to_elem.insert("WeightingLookupTableDescriptor", &tags::WeightingLookupTableDescriptor);
		tag_to_elem.insert(0x00701807, &tags::WeightingLookupTableDescriptor);

		ident_to_elem.insert("WeightingLookupTableData", &tags::WeightingLookupTableData);
		tag_to_elem.insert(0x00701808, &tags::WeightingLookupTableData);

		ident_to_elem.insert("VolumetricAnnotationSequence", &tags::VolumetricAnnotationSequence);
		tag_to_elem.insert(0x00701901, &tags::VolumetricAnnotationSequence);

		ident_to_elem.insert("ReferencedStructuredContextSequence", &tags::ReferencedStructuredContextSequence);
		tag_to_elem.insert(0x00701903, &tags::ReferencedStructuredContextSequence);

		ident_to_elem.insert("ReferencedContentItem", &tags::ReferencedContentItem);
		tag_to_elem.insert(0x00701904, &tags::ReferencedContentItem);

		ident_to_elem.insert("VolumetricPresentationInputAnnotationSequence", &tags::VolumetricPresentationInputAnnotationSequence);
		tag_to_elem.insert(0x00701905, &tags::VolumetricPresentationInputAnnotationSequence);

		ident_to_elem.insert("AnnotationClipping", &tags::AnnotationClipping);
		tag_to_elem.insert(0x00701907, &tags::AnnotationClipping);

		ident_to_elem.insert("PresentationAnimationStyle", &tags::PresentationAnimationStyle);
		tag_to_elem.insert(0x00701A01, &tags::PresentationAnimationStyle);

		ident_to_elem.insert("RecommendedAnimationRate", &tags::RecommendedAnimationRate);
		tag_to_elem.insert(0x00701A03, &tags::RecommendedAnimationRate);

		ident_to_elem.insert("AnimationCurveSequence", &tags::AnimationCurveSequence);
		tag_to_elem.insert(0x00701A04, &tags::AnimationCurveSequence);

		ident_to_elem.insert("AnimationStepSize", &tags::AnimationStepSize);
		tag_to_elem.insert(0x00701A05, &tags::AnimationStepSize);

		ident_to_elem.insert("HangingProtocolName", &tags::HangingProtocolName);
		tag_to_elem.insert(0x00720002, &tags::HangingProtocolName);

		ident_to_elem.insert("HangingProtocolDescription", &tags::HangingProtocolDescription);
		tag_to_elem.insert(0x00720004, &tags::HangingProtocolDescription);

		ident_to_elem.insert("HangingProtocolLevel", &tags::HangingProtocolLevel);
		tag_to_elem.insert(0x00720006, &tags::HangingProtocolLevel);

		ident_to_elem.insert("HangingProtocolCreator", &tags::HangingProtocolCreator);
		tag_to_elem.insert(0x00720008, &tags::HangingProtocolCreator);

		ident_to_elem.insert("HangingProtocolCreationDateTime", &tags::HangingProtocolCreationDateTime);
		tag_to_elem.insert(0x0072000A, &tags::HangingProtocolCreationDateTime);

		ident_to_elem.insert("HangingProtocolDefinitionSequence", &tags::HangingProtocolDefinitionSequence);
		tag_to_elem.insert(0x0072000C, &tags::HangingProtocolDefinitionSequence);

		ident_to_elem.insert("HangingProtocolUserIdentificationCodeSequence", &tags::HangingProtocolUserIdentificationCodeSequence);
		tag_to_elem.insert(0x0072000E, &tags::HangingProtocolUserIdentificationCodeSequence);

		ident_to_elem.insert("HangingProtocolUserGroupName", &tags::HangingProtocolUserGroupName);
		tag_to_elem.insert(0x00720010, &tags::HangingProtocolUserGroupName);

		ident_to_elem.insert("SourceHangingProtocolSequence", &tags::SourceHangingProtocolSequence);
		tag_to_elem.insert(0x00720012, &tags::SourceHangingProtocolSequence);

		ident_to_elem.insert("NumberOfPriorsReferenced", &tags::NumberOfPriorsReferenced);
		tag_to_elem.insert(0x00720014, &tags::NumberOfPriorsReferenced);

		ident_to_elem.insert("ImageSetsSequence", &tags::ImageSetsSequence);
		tag_to_elem.insert(0x00720020, &tags::ImageSetsSequence);

		ident_to_elem.insert("ImageSetSelectorSequence", &tags::ImageSetSelectorSequence);
		tag_to_elem.insert(0x00720022, &tags::ImageSetSelectorSequence);

		ident_to_elem.insert("ImageSetSelectorUsageFlag", &tags::ImageSetSelectorUsageFlag);
		tag_to_elem.insert(0x00720024, &tags::ImageSetSelectorUsageFlag);

		ident_to_elem.insert("SelectorAttribute", &tags::SelectorAttribute);
		tag_to_elem.insert(0x00720026, &tags::SelectorAttribute);

		ident_to_elem.insert("SelectorValueNumber", &tags::SelectorValueNumber);
		tag_to_elem.insert(0x00720028, &tags::SelectorValueNumber);

		ident_to_elem.insert("TimeBasedImageSetsSequence", &tags::TimeBasedImageSetsSequence);
		tag_to_elem.insert(0x00720030, &tags::TimeBasedImageSetsSequence);

		ident_to_elem.insert("ImageSetNumber", &tags::ImageSetNumber);
		tag_to_elem.insert(0x00720032, &tags::ImageSetNumber);

		ident_to_elem.insert("ImageSetSelectorCategory", &tags::ImageSetSelectorCategory);
		tag_to_elem.insert(0x00720034, &tags::ImageSetSelectorCategory);

		ident_to_elem.insert("RelativeTime", &tags::RelativeTime);
		tag_to_elem.insert(0x00720038, &tags::RelativeTime);

		ident_to_elem.insert("RelativeTimeUnits", &tags::RelativeTimeUnits);
		tag_to_elem.insert(0x0072003A, &tags::RelativeTimeUnits);

		ident_to_elem.insert("AbstractPriorValue", &tags::AbstractPriorValue);
		tag_to_elem.insert(0x0072003C, &tags::AbstractPriorValue);

		ident_to_elem.insert("AbstractPriorCodeSequence", &tags::AbstractPriorCodeSequence);
		tag_to_elem.insert(0x0072003E, &tags::AbstractPriorCodeSequence);

		ident_to_elem.insert("ImageSetLabel", &tags::ImageSetLabel);
		tag_to_elem.insert(0x00720040, &tags::ImageSetLabel);

		ident_to_elem.insert("SelectorAttributeVR", &tags::SelectorAttributeVR);
		tag_to_elem.insert(0x00720050, &tags::SelectorAttributeVR);

		ident_to_elem.insert("SelectorSequencePointer", &tags::SelectorSequencePointer);
		tag_to_elem.insert(0x00720052, &tags::SelectorSequencePointer);

		ident_to_elem.insert("SelectorSequencePointerPrivateCreator", &tags::SelectorSequencePointerPrivateCreator);
		tag_to_elem.insert(0x00720054, &tags::SelectorSequencePointerPrivateCreator);

		ident_to_elem.insert("SelectorAttributePrivateCreator", &tags::SelectorAttributePrivateCreator);
		tag_to_elem.insert(0x00720056, &tags::SelectorAttributePrivateCreator);

		ident_to_elem.insert("SelectorAEValue", &tags::SelectorAEValue);
		tag_to_elem.insert(0x0072005E, &tags::SelectorAEValue);

		ident_to_elem.insert("SelectorASValue", &tags::SelectorASValue);
		tag_to_elem.insert(0x0072005F, &tags::SelectorASValue);

		ident_to_elem.insert("SelectorATValue", &tags::SelectorATValue);
		tag_to_elem.insert(0x00720060, &tags::SelectorATValue);

		ident_to_elem.insert("SelectorDAValue", &tags::SelectorDAValue);
		tag_to_elem.insert(0x00720061, &tags::SelectorDAValue);

		ident_to_elem.insert("SelectorCSValue", &tags::SelectorCSValue);
		tag_to_elem.insert(0x00720062, &tags::SelectorCSValue);

		ident_to_elem.insert("SelectorDTValue", &tags::SelectorDTValue);
		tag_to_elem.insert(0x00720063, &tags::SelectorDTValue);

		ident_to_elem.insert("SelectorISValue", &tags::SelectorISValue);
		tag_to_elem.insert(0x00720064, &tags::SelectorISValue);

		ident_to_elem.insert("SelectorOBValue", &tags::SelectorOBValue);
		tag_to_elem.insert(0x00720065, &tags::SelectorOBValue);

		ident_to_elem.insert("SelectorLOValue", &tags::SelectorLOValue);
		tag_to_elem.insert(0x00720066, &tags::SelectorLOValue);

		ident_to_elem.insert("SelectorOFValue", &tags::SelectorOFValue);
		tag_to_elem.insert(0x00720067, &tags::SelectorOFValue);

		ident_to_elem.insert("SelectorLTValue", &tags::SelectorLTValue);
		tag_to_elem.insert(0x00720068, &tags::SelectorLTValue);

		ident_to_elem.insert("SelectorOWValue", &tags::SelectorOWValue);
		tag_to_elem.insert(0x00720069, &tags::SelectorOWValue);

		ident_to_elem.insert("SelectorPNValue", &tags::SelectorPNValue);
		tag_to_elem.insert(0x0072006A, &tags::SelectorPNValue);

		ident_to_elem.insert("SelectorTMValue", &tags::SelectorTMValue);
		tag_to_elem.insert(0x0072006B, &tags::SelectorTMValue);

		ident_to_elem.insert("SelectorSHValue", &tags::SelectorSHValue);
		tag_to_elem.insert(0x0072006C, &tags::SelectorSHValue);

		ident_to_elem.insert("SelectorUNValue", &tags::SelectorUNValue);
		tag_to_elem.insert(0x0072006D, &tags::SelectorUNValue);

		ident_to_elem.insert("SelectorSTValue", &tags::SelectorSTValue);
		tag_to_elem.insert(0x0072006E, &tags::SelectorSTValue);

		ident_to_elem.insert("SelectorUCValue", &tags::SelectorUCValue);
		tag_to_elem.insert(0x0072006F, &tags::SelectorUCValue);

		ident_to_elem.insert("SelectorUTValue", &tags::SelectorUTValue);
		tag_to_elem.insert(0x00720070, &tags::SelectorUTValue);

		ident_to_elem.insert("SelectorURValue", &tags::SelectorURValue);
		tag_to_elem.insert(0x00720071, &tags::SelectorURValue);

		ident_to_elem.insert("SelectorDSValue", &tags::SelectorDSValue);
		tag_to_elem.insert(0x00720072, &tags::SelectorDSValue);

		ident_to_elem.insert("SelectorODValue", &tags::SelectorODValue);
		tag_to_elem.insert(0x00720073, &tags::SelectorODValue);

		ident_to_elem.insert("SelectorFDValue", &tags::SelectorFDValue);
		tag_to_elem.insert(0x00720074, &tags::SelectorFDValue);

		ident_to_elem.insert("SelectorOLValue", &tags::SelectorOLValue);
		tag_to_elem.insert(0x00720075, &tags::SelectorOLValue);

		ident_to_elem.insert("SelectorFLValue", &tags::SelectorFLValue);
		tag_to_elem.insert(0x00720076, &tags::SelectorFLValue);

		ident_to_elem.insert("SelectorULValue", &tags::SelectorULValue);
		tag_to_elem.insert(0x00720078, &tags::SelectorULValue);

		ident_to_elem.insert("SelectorUSValue", &tags::SelectorUSValue);
		tag_to_elem.insert(0x0072007A, &tags::SelectorUSValue);

		ident_to_elem.insert("SelectorSLValue", &tags::SelectorSLValue);
		tag_to_elem.insert(0x0072007C, &tags::SelectorSLValue);

		ident_to_elem.insert("SelectorSSValue", &tags::SelectorSSValue);
		tag_to_elem.insert(0x0072007E, &tags::SelectorSSValue);

		ident_to_elem.insert("SelectorUIValue", &tags::SelectorUIValue);
		tag_to_elem.insert(0x0072007F, &tags::SelectorUIValue);

		ident_to_elem.insert("SelectorCodeSequenceValue", &tags::SelectorCodeSequenceValue);
		tag_to_elem.insert(0x00720080, &tags::SelectorCodeSequenceValue);

		ident_to_elem.insert("NumberOfScreens", &tags::NumberOfScreens);
		tag_to_elem.insert(0x00720100, &tags::NumberOfScreens);

		ident_to_elem.insert("NominalScreenDefinitionSequence", &tags::NominalScreenDefinitionSequence);
		tag_to_elem.insert(0x00720102, &tags::NominalScreenDefinitionSequence);

		ident_to_elem.insert("NumberOfVerticalPixels", &tags::NumberOfVerticalPixels);
		tag_to_elem.insert(0x00720104, &tags::NumberOfVerticalPixels);

		ident_to_elem.insert("NumberOfHorizontalPixels", &tags::NumberOfHorizontalPixels);
		tag_to_elem.insert(0x00720106, &tags::NumberOfHorizontalPixels);

		ident_to_elem.insert("DisplayEnvironmentSpatialPosition", &tags::DisplayEnvironmentSpatialPosition);
		tag_to_elem.insert(0x00720108, &tags::DisplayEnvironmentSpatialPosition);

		ident_to_elem.insert("ScreenMinimumGrayscaleBitDepth", &tags::ScreenMinimumGrayscaleBitDepth);
		tag_to_elem.insert(0x0072010A, &tags::ScreenMinimumGrayscaleBitDepth);

		ident_to_elem.insert("ScreenMinimumColorBitDepth", &tags::ScreenMinimumColorBitDepth);
		tag_to_elem.insert(0x0072010C, &tags::ScreenMinimumColorBitDepth);

		ident_to_elem.insert("ApplicationMaximumRepaintTime", &tags::ApplicationMaximumRepaintTime);
		tag_to_elem.insert(0x0072010E, &tags::ApplicationMaximumRepaintTime);

		ident_to_elem.insert("DisplaySetsSequence", &tags::DisplaySetsSequence);
		tag_to_elem.insert(0x00720200, &tags::DisplaySetsSequence);

		ident_to_elem.insert("DisplaySetNumber", &tags::DisplaySetNumber);
		tag_to_elem.insert(0x00720202, &tags::DisplaySetNumber);

		ident_to_elem.insert("DisplaySetLabel", &tags::DisplaySetLabel);
		tag_to_elem.insert(0x00720203, &tags::DisplaySetLabel);

		ident_to_elem.insert("DisplaySetPresentationGroup", &tags::DisplaySetPresentationGroup);
		tag_to_elem.insert(0x00720204, &tags::DisplaySetPresentationGroup);

		ident_to_elem.insert("DisplaySetPresentationGroupDescription", &tags::DisplaySetPresentationGroupDescription);
		tag_to_elem.insert(0x00720206, &tags::DisplaySetPresentationGroupDescription);

		ident_to_elem.insert("PartialDataDisplayHandling", &tags::PartialDataDisplayHandling);
		tag_to_elem.insert(0x00720208, &tags::PartialDataDisplayHandling);

		ident_to_elem.insert("SynchronizedScrollingSequence", &tags::SynchronizedScrollingSequence);
		tag_to_elem.insert(0x00720210, &tags::SynchronizedScrollingSequence);

		ident_to_elem.insert("DisplaySetScrollingGroup", &tags::DisplaySetScrollingGroup);
		tag_to_elem.insert(0x00720212, &tags::DisplaySetScrollingGroup);

		ident_to_elem.insert("NavigationIndicatorSequence", &tags::NavigationIndicatorSequence);
		tag_to_elem.insert(0x00720214, &tags::NavigationIndicatorSequence);

		ident_to_elem.insert("NavigationDisplaySet", &tags::NavigationDisplaySet);
		tag_to_elem.insert(0x00720216, &tags::NavigationDisplaySet);

		ident_to_elem.insert("ReferenceDisplaySets", &tags::ReferenceDisplaySets);
		tag_to_elem.insert(0x00720218, &tags::ReferenceDisplaySets);

		ident_to_elem.insert("ImageBoxesSequence", &tags::ImageBoxesSequence);
		tag_to_elem.insert(0x00720300, &tags::ImageBoxesSequence);

		ident_to_elem.insert("ImageBoxNumber", &tags::ImageBoxNumber);
		tag_to_elem.insert(0x00720302, &tags::ImageBoxNumber);

		ident_to_elem.insert("ImageBoxLayoutType", &tags::ImageBoxLayoutType);
		tag_to_elem.insert(0x00720304, &tags::ImageBoxLayoutType);

		ident_to_elem.insert("ImageBoxTileHorizontalDimension", &tags::ImageBoxTileHorizontalDimension);
		tag_to_elem.insert(0x00720306, &tags::ImageBoxTileHorizontalDimension);

		ident_to_elem.insert("ImageBoxTileVerticalDimension", &tags::ImageBoxTileVerticalDimension);
		tag_to_elem.insert(0x00720308, &tags::ImageBoxTileVerticalDimension);

		ident_to_elem.insert("ImageBoxScrollDirection", &tags::ImageBoxScrollDirection);
		tag_to_elem.insert(0x00720310, &tags::ImageBoxScrollDirection);

		ident_to_elem.insert("ImageBoxSmallScrollType", &tags::ImageBoxSmallScrollType);
		tag_to_elem.insert(0x00720312, &tags::ImageBoxSmallScrollType);

		ident_to_elem.insert("ImageBoxSmallScrollAmount", &tags::ImageBoxSmallScrollAmount);
		tag_to_elem.insert(0x00720314, &tags::ImageBoxSmallScrollAmount);

		ident_to_elem.insert("ImageBoxLargeScrollType", &tags::ImageBoxLargeScrollType);
		tag_to_elem.insert(0x00720316, &tags::ImageBoxLargeScrollType);

		ident_to_elem.insert("ImageBoxLargeScrollAmount", &tags::ImageBoxLargeScrollAmount);
		tag_to_elem.insert(0x00720318, &tags::ImageBoxLargeScrollAmount);

		ident_to_elem.insert("ImageBoxOverlapPriority", &tags::ImageBoxOverlapPriority);
		tag_to_elem.insert(0x00720320, &tags::ImageBoxOverlapPriority);

		ident_to_elem.insert("CineRelativeToRealTime", &tags::CineRelativeToRealTime);
		tag_to_elem.insert(0x00720330, &tags::CineRelativeToRealTime);

		ident_to_elem.insert("FilterOperationsSequence", &tags::FilterOperationsSequence);
		tag_to_elem.insert(0x00720400, &tags::FilterOperationsSequence);

		ident_to_elem.insert("FilterByCategory", &tags::FilterByCategory);
		tag_to_elem.insert(0x00720402, &tags::FilterByCategory);

		ident_to_elem.insert("FilterByAttributePresence", &tags::FilterByAttributePresence);
		tag_to_elem.insert(0x00720404, &tags::FilterByAttributePresence);

		ident_to_elem.insert("FilterByOperator", &tags::FilterByOperator);
		tag_to_elem.insert(0x00720406, &tags::FilterByOperator);

		ident_to_elem.insert("StructuredDisplayBackgroundCIELabValue", &tags::StructuredDisplayBackgroundCIELabValue);
		tag_to_elem.insert(0x00720420, &tags::StructuredDisplayBackgroundCIELabValue);

		ident_to_elem.insert("EmptyImageBoxCIELabValue", &tags::EmptyImageBoxCIELabValue);
		tag_to_elem.insert(0x00720421, &tags::EmptyImageBoxCIELabValue);

		ident_to_elem.insert("StructuredDisplayImageBoxSequence", &tags::StructuredDisplayImageBoxSequence);
		tag_to_elem.insert(0x00720422, &tags::StructuredDisplayImageBoxSequence);

		ident_to_elem.insert("StructuredDisplayTextBoxSequence", &tags::StructuredDisplayTextBoxSequence);
		tag_to_elem.insert(0x00720424, &tags::StructuredDisplayTextBoxSequence);

		ident_to_elem.insert("ReferencedFirstFrameSequence", &tags::ReferencedFirstFrameSequence);
		tag_to_elem.insert(0x00720427, &tags::ReferencedFirstFrameSequence);

		ident_to_elem.insert("ImageBoxSynchronizationSequence", &tags::ImageBoxSynchronizationSequence);
		tag_to_elem.insert(0x00720430, &tags::ImageBoxSynchronizationSequence);

		ident_to_elem.insert("SynchronizedImageBoxList", &tags::SynchronizedImageBoxList);
		tag_to_elem.insert(0x00720432, &tags::SynchronizedImageBoxList);

		ident_to_elem.insert("TypeOfSynchronization", &tags::TypeOfSynchronization);
		tag_to_elem.insert(0x00720434, &tags::TypeOfSynchronization);

		ident_to_elem.insert("BlendingOperationType", &tags::BlendingOperationType);
		tag_to_elem.insert(0x00720500, &tags::BlendingOperationType);

		ident_to_elem.insert("ReformattingOperationType", &tags::ReformattingOperationType);
		tag_to_elem.insert(0x00720510, &tags::ReformattingOperationType);

		ident_to_elem.insert("ReformattingThickness", &tags::ReformattingThickness);
		tag_to_elem.insert(0x00720512, &tags::ReformattingThickness);

		ident_to_elem.insert("ReformattingInterval", &tags::ReformattingInterval);
		tag_to_elem.insert(0x00720514, &tags::ReformattingInterval);

		ident_to_elem.insert("ReformattingOperationInitialViewDirection", &tags::ReformattingOperationInitialViewDirection);
		tag_to_elem.insert(0x00720516, &tags::ReformattingOperationInitialViewDirection);

		ident_to_elem.insert("ThreeDRenderingType", &tags::ThreeDRenderingType);
		tag_to_elem.insert(0x00720520, &tags::ThreeDRenderingType);

		ident_to_elem.insert("SortingOperationsSequence", &tags::SortingOperationsSequence);
		tag_to_elem.insert(0x00720600, &tags::SortingOperationsSequence);

		ident_to_elem.insert("SortByCategory", &tags::SortByCategory);
		tag_to_elem.insert(0x00720602, &tags::SortByCategory);

		ident_to_elem.insert("SortingDirection", &tags::SortingDirection);
		tag_to_elem.insert(0x00720604, &tags::SortingDirection);

		ident_to_elem.insert("DisplaySetPatientOrientation", &tags::DisplaySetPatientOrientation);
		tag_to_elem.insert(0x00720700, &tags::DisplaySetPatientOrientation);

		ident_to_elem.insert("VOIType", &tags::VOIType);
		tag_to_elem.insert(0x00720702, &tags::VOIType);

		ident_to_elem.insert("PseudoColorType", &tags::PseudoColorType);
		tag_to_elem.insert(0x00720704, &tags::PseudoColorType);

		ident_to_elem.insert("PseudoColorPaletteInstanceReferenceSequence", &tags::PseudoColorPaletteInstanceReferenceSequence);
		tag_to_elem.insert(0x00720705, &tags::PseudoColorPaletteInstanceReferenceSequence);

		ident_to_elem.insert("ShowGrayscaleInverted", &tags::ShowGrayscaleInverted);
		tag_to_elem.insert(0x00720706, &tags::ShowGrayscaleInverted);

		ident_to_elem.insert("ShowImageTrueSizeFlag", &tags::ShowImageTrueSizeFlag);
		tag_to_elem.insert(0x00720710, &tags::ShowImageTrueSizeFlag);

		ident_to_elem.insert("ShowGraphicAnnotationFlag", &tags::ShowGraphicAnnotationFlag);
		tag_to_elem.insert(0x00720712, &tags::ShowGraphicAnnotationFlag);

		ident_to_elem.insert("ShowPatientDemographicsFlag", &tags::ShowPatientDemographicsFlag);
		tag_to_elem.insert(0x00720714, &tags::ShowPatientDemographicsFlag);

		ident_to_elem.insert("ShowAcquisitionTechniquesFlag", &tags::ShowAcquisitionTechniquesFlag);
		tag_to_elem.insert(0x00720716, &tags::ShowAcquisitionTechniquesFlag);

		ident_to_elem.insert("DisplaySetHorizontalJustification", &tags::DisplaySetHorizontalJustification);
		tag_to_elem.insert(0x00720717, &tags::DisplaySetHorizontalJustification);

		ident_to_elem.insert("DisplaySetVerticalJustification", &tags::DisplaySetVerticalJustification);
		tag_to_elem.insert(0x00720718, &tags::DisplaySetVerticalJustification);

		ident_to_elem.insert("ContinuationStartMeterset", &tags::ContinuationStartMeterset);
		tag_to_elem.insert(0x00740120, &tags::ContinuationStartMeterset);

		ident_to_elem.insert("ContinuationEndMeterset", &tags::ContinuationEndMeterset);
		tag_to_elem.insert(0x00740121, &tags::ContinuationEndMeterset);

		ident_to_elem.insert("ProcedureStepState", &tags::ProcedureStepState);
		tag_to_elem.insert(0x00741000, &tags::ProcedureStepState);

		ident_to_elem.insert("ProcedureStepProgressInformationSequence", &tags::ProcedureStepProgressInformationSequence);
		tag_to_elem.insert(0x00741002, &tags::ProcedureStepProgressInformationSequence);

		ident_to_elem.insert("ProcedureStepProgress", &tags::ProcedureStepProgress);
		tag_to_elem.insert(0x00741004, &tags::ProcedureStepProgress);

		ident_to_elem.insert("ProcedureStepProgressDescription", &tags::ProcedureStepProgressDescription);
		tag_to_elem.insert(0x00741006, &tags::ProcedureStepProgressDescription);

		ident_to_elem.insert("ProcedureStepCommunicationsURISequence", &tags::ProcedureStepCommunicationsURISequence);
		tag_to_elem.insert(0x00741008, &tags::ProcedureStepCommunicationsURISequence);

		ident_to_elem.insert("ContactURI", &tags::ContactURI);
		tag_to_elem.insert(0x0074100A, &tags::ContactURI);

		ident_to_elem.insert("ContactDisplayName", &tags::ContactDisplayName);
		tag_to_elem.insert(0x0074100C, &tags::ContactDisplayName);

		ident_to_elem.insert("ProcedureStepDiscontinuationReasonCodeSequence", &tags::ProcedureStepDiscontinuationReasonCodeSequence);
		tag_to_elem.insert(0x0074100E, &tags::ProcedureStepDiscontinuationReasonCodeSequence);

		ident_to_elem.insert("BeamTaskSequence", &tags::BeamTaskSequence);
		tag_to_elem.insert(0x00741020, &tags::BeamTaskSequence);

		ident_to_elem.insert("BeamTaskType", &tags::BeamTaskType);
		tag_to_elem.insert(0x00741022, &tags::BeamTaskType);

		ident_to_elem.insert("BeamOrderIndexTrial", &tags::BeamOrderIndexTrial);
		tag_to_elem.insert(0x00741024, &tags::BeamOrderIndexTrial);

		ident_to_elem.insert("AutosequenceFlag", &tags::AutosequenceFlag);
		tag_to_elem.insert(0x00741025, &tags::AutosequenceFlag);

		ident_to_elem.insert("TableTopVerticalAdjustedPosition", &tags::TableTopVerticalAdjustedPosition);
		tag_to_elem.insert(0x00741026, &tags::TableTopVerticalAdjustedPosition);

		ident_to_elem.insert("TableTopLongitudinalAdjustedPosition", &tags::TableTopLongitudinalAdjustedPosition);
		tag_to_elem.insert(0x00741027, &tags::TableTopLongitudinalAdjustedPosition);

		ident_to_elem.insert("TableTopLateralAdjustedPosition", &tags::TableTopLateralAdjustedPosition);
		tag_to_elem.insert(0x00741028, &tags::TableTopLateralAdjustedPosition);

		ident_to_elem.insert("PatientSupportAdjustedAngle", &tags::PatientSupportAdjustedAngle);
		tag_to_elem.insert(0x0074102A, &tags::PatientSupportAdjustedAngle);

		ident_to_elem.insert("TableTopEccentricAdjustedAngle", &tags::TableTopEccentricAdjustedAngle);
		tag_to_elem.insert(0x0074102B, &tags::TableTopEccentricAdjustedAngle);

		ident_to_elem.insert("TableTopPitchAdjustedAngle", &tags::TableTopPitchAdjustedAngle);
		tag_to_elem.insert(0x0074102C, &tags::TableTopPitchAdjustedAngle);

		ident_to_elem.insert("TableTopRollAdjustedAngle", &tags::TableTopRollAdjustedAngle);
		tag_to_elem.insert(0x0074102D, &tags::TableTopRollAdjustedAngle);

		ident_to_elem.insert("DeliveryVerificationImageSequence", &tags::DeliveryVerificationImageSequence);
		tag_to_elem.insert(0x00741030, &tags::DeliveryVerificationImageSequence);

		ident_to_elem.insert("VerificationImageTiming", &tags::VerificationImageTiming);
		tag_to_elem.insert(0x00741032, &tags::VerificationImageTiming);

		ident_to_elem.insert("DoubleExposureFlag", &tags::DoubleExposureFlag);
		tag_to_elem.insert(0x00741034, &tags::DoubleExposureFlag);

		ident_to_elem.insert("DoubleExposureOrdering", &tags::DoubleExposureOrdering);
		tag_to_elem.insert(0x00741036, &tags::DoubleExposureOrdering);

		ident_to_elem.insert("DoubleExposureMetersetTrial", &tags::DoubleExposureMetersetTrial);
		tag_to_elem.insert(0x00741038, &tags::DoubleExposureMetersetTrial);

		ident_to_elem.insert("DoubleExposureFieldDeltaTrial", &tags::DoubleExposureFieldDeltaTrial);
		tag_to_elem.insert(0x0074103A, &tags::DoubleExposureFieldDeltaTrial);

		ident_to_elem.insert("RelatedReferenceRTImageSequence", &tags::RelatedReferenceRTImageSequence);
		tag_to_elem.insert(0x00741040, &tags::RelatedReferenceRTImageSequence);

		ident_to_elem.insert("GeneralMachineVerificationSequence", &tags::GeneralMachineVerificationSequence);
		tag_to_elem.insert(0x00741042, &tags::GeneralMachineVerificationSequence);

		ident_to_elem.insert("ConventionalMachineVerificationSequence", &tags::ConventionalMachineVerificationSequence);
		tag_to_elem.insert(0x00741044, &tags::ConventionalMachineVerificationSequence);

		ident_to_elem.insert("IonMachineVerificationSequence", &tags::IonMachineVerificationSequence);
		tag_to_elem.insert(0x00741046, &tags::IonMachineVerificationSequence);

		ident_to_elem.insert("FailedAttributesSequence", &tags::FailedAttributesSequence);
		tag_to_elem.insert(0x00741048, &tags::FailedAttributesSequence);

		ident_to_elem.insert("OverriddenAttributesSequence", &tags::OverriddenAttributesSequence);
		tag_to_elem.insert(0x0074104A, &tags::OverriddenAttributesSequence);

		ident_to_elem.insert("ConventionalControlPointVerificationSequence", &tags::ConventionalControlPointVerificationSequence);
		tag_to_elem.insert(0x0074104C, &tags::ConventionalControlPointVerificationSequence);

		ident_to_elem.insert("IonControlPointVerificationSequence", &tags::IonControlPointVerificationSequence);
		tag_to_elem.insert(0x0074104E, &tags::IonControlPointVerificationSequence);

		ident_to_elem.insert("AttributeOccurrenceSequence", &tags::AttributeOccurrenceSequence);
		tag_to_elem.insert(0x00741050, &tags::AttributeOccurrenceSequence);

		ident_to_elem.insert("AttributeOccurrencePointer", &tags::AttributeOccurrencePointer);
		tag_to_elem.insert(0x00741052, &tags::AttributeOccurrencePointer);

		ident_to_elem.insert("AttributeItemSelector", &tags::AttributeItemSelector);
		tag_to_elem.insert(0x00741054, &tags::AttributeItemSelector);

		ident_to_elem.insert("AttributeOccurrencePrivateCreator", &tags::AttributeOccurrencePrivateCreator);
		tag_to_elem.insert(0x00741056, &tags::AttributeOccurrencePrivateCreator);

		ident_to_elem.insert("SelectorSequencePointerItems", &tags::SelectorSequencePointerItems);
		tag_to_elem.insert(0x00741057, &tags::SelectorSequencePointerItems);

		ident_to_elem.insert("ScheduledProcedureStepPriority", &tags::ScheduledProcedureStepPriority);
		tag_to_elem.insert(0x00741200, &tags::ScheduledProcedureStepPriority);

		ident_to_elem.insert("WorklistLabel", &tags::WorklistLabel);
		tag_to_elem.insert(0x00741202, &tags::WorklistLabel);

		ident_to_elem.insert("ProcedureStepLabel", &tags::ProcedureStepLabel);
		tag_to_elem.insert(0x00741204, &tags::ProcedureStepLabel);

		ident_to_elem.insert("ScheduledProcessingParametersSequence", &tags::ScheduledProcessingParametersSequence);
		tag_to_elem.insert(0x00741210, &tags::ScheduledProcessingParametersSequence);

		ident_to_elem.insert("PerformedProcessingParametersSequence", &tags::PerformedProcessingParametersSequence);
		tag_to_elem.insert(0x00741212, &tags::PerformedProcessingParametersSequence);

		ident_to_elem.insert("UnifiedProcedureStepPerformedProcedureSequence", &tags::UnifiedProcedureStepPerformedProcedureSequence);
		tag_to_elem.insert(0x00741216, &tags::UnifiedProcedureStepPerformedProcedureSequence);

		ident_to_elem.insert("RelatedProcedureStepSequence", &tags::RelatedProcedureStepSequence);
		tag_to_elem.insert(0x00741220, &tags::RelatedProcedureStepSequence);

		ident_to_elem.insert("ProcedureStepRelationshipType", &tags::ProcedureStepRelationshipType);
		tag_to_elem.insert(0x00741222, &tags::ProcedureStepRelationshipType);

		ident_to_elem.insert("ReplacedProcedureStepSequence", &tags::ReplacedProcedureStepSequence);
		tag_to_elem.insert(0x00741224, &tags::ReplacedProcedureStepSequence);

		ident_to_elem.insert("DeletionLock", &tags::DeletionLock);
		tag_to_elem.insert(0x00741230, &tags::DeletionLock);

		ident_to_elem.insert("ReceivingAE", &tags::ReceivingAE);
		tag_to_elem.insert(0x00741234, &tags::ReceivingAE);

		ident_to_elem.insert("RequestingAE", &tags::RequestingAE);
		tag_to_elem.insert(0x00741236, &tags::RequestingAE);

		ident_to_elem.insert("ReasonForCancellation", &tags::ReasonForCancellation);
		tag_to_elem.insert(0x00741238, &tags::ReasonForCancellation);

		ident_to_elem.insert("SCPStatus", &tags::SCPStatus);
		tag_to_elem.insert(0x00741242, &tags::SCPStatus);

		ident_to_elem.insert("SubscriptionListStatus", &tags::SubscriptionListStatus);
		tag_to_elem.insert(0x00741244, &tags::SubscriptionListStatus);

		ident_to_elem.insert("UnifiedProcedureStepListStatus", &tags::UnifiedProcedureStepListStatus);
		tag_to_elem.insert(0x00741246, &tags::UnifiedProcedureStepListStatus);

		ident_to_elem.insert("BeamOrderIndex", &tags::BeamOrderIndex);
		tag_to_elem.insert(0x00741324, &tags::BeamOrderIndex);

		ident_to_elem.insert("DoubleExposureMeterset", &tags::DoubleExposureMeterset);
		tag_to_elem.insert(0x00741338, &tags::DoubleExposureMeterset);

		ident_to_elem.insert("DoubleExposureFieldDelta", &tags::DoubleExposureFieldDelta);
		tag_to_elem.insert(0x0074133A, &tags::DoubleExposureFieldDelta);

		ident_to_elem.insert("BrachyTaskSequence", &tags::BrachyTaskSequence);
		tag_to_elem.insert(0x00741401, &tags::BrachyTaskSequence);

		ident_to_elem.insert("ContinuationStartTotalReferenceAirKerma", &tags::ContinuationStartTotalReferenceAirKerma);
		tag_to_elem.insert(0x00741402, &tags::ContinuationStartTotalReferenceAirKerma);

		ident_to_elem.insert("ContinuationEndTotalReferenceAirKerma", &tags::ContinuationEndTotalReferenceAirKerma);
		tag_to_elem.insert(0x00741403, &tags::ContinuationEndTotalReferenceAirKerma);

		ident_to_elem.insert("ContinuationPulseNumber", &tags::ContinuationPulseNumber);
		tag_to_elem.insert(0x00741404, &tags::ContinuationPulseNumber);

		ident_to_elem.insert("ChannelDeliveryOrderSequence", &tags::ChannelDeliveryOrderSequence);
		tag_to_elem.insert(0x00741405, &tags::ChannelDeliveryOrderSequence);

		ident_to_elem.insert("ReferencedChannelNumber", &tags::ReferencedChannelNumber);
		tag_to_elem.insert(0x00741406, &tags::ReferencedChannelNumber);

		ident_to_elem.insert("StartCumulativeTimeWeight", &tags::StartCumulativeTimeWeight);
		tag_to_elem.insert(0x00741407, &tags::StartCumulativeTimeWeight);

		ident_to_elem.insert("EndCumulativeTimeWeight", &tags::EndCumulativeTimeWeight);
		tag_to_elem.insert(0x00741408, &tags::EndCumulativeTimeWeight);

		ident_to_elem.insert("OmittedChannelSequence", &tags::OmittedChannelSequence);
		tag_to_elem.insert(0x00741409, &tags::OmittedChannelSequence);

		ident_to_elem.insert("ReasonForChannelOmission", &tags::ReasonForChannelOmission);
		tag_to_elem.insert(0x0074140A, &tags::ReasonForChannelOmission);

		ident_to_elem.insert("ReasonForChannelOmissionDescription", &tags::ReasonForChannelOmissionDescription);
		tag_to_elem.insert(0x0074140B, &tags::ReasonForChannelOmissionDescription);

		ident_to_elem.insert("ChannelDeliveryOrderIndex", &tags::ChannelDeliveryOrderIndex);
		tag_to_elem.insert(0x0074140C, &tags::ChannelDeliveryOrderIndex);

		ident_to_elem.insert("ChannelDeliveryContinuationSequence", &tags::ChannelDeliveryContinuationSequence);
		tag_to_elem.insert(0x0074140D, &tags::ChannelDeliveryContinuationSequence);

		ident_to_elem.insert("OmittedApplicationSetupSequence", &tags::OmittedApplicationSetupSequence);
		tag_to_elem.insert(0x0074140E, &tags::OmittedApplicationSetupSequence);

		ident_to_elem.insert("ImplantAssemblyTemplateName", &tags::ImplantAssemblyTemplateName);
		tag_to_elem.insert(0x00760001, &tags::ImplantAssemblyTemplateName);

		ident_to_elem.insert("ImplantAssemblyTemplateIssuer", &tags::ImplantAssemblyTemplateIssuer);
		tag_to_elem.insert(0x00760003, &tags::ImplantAssemblyTemplateIssuer);

		ident_to_elem.insert("ImplantAssemblyTemplateVersion", &tags::ImplantAssemblyTemplateVersion);
		tag_to_elem.insert(0x00760006, &tags::ImplantAssemblyTemplateVersion);

		ident_to_elem.insert("ReplacedImplantAssemblyTemplateSequence", &tags::ReplacedImplantAssemblyTemplateSequence);
		tag_to_elem.insert(0x00760008, &tags::ReplacedImplantAssemblyTemplateSequence);

		ident_to_elem.insert("ImplantAssemblyTemplateType", &tags::ImplantAssemblyTemplateType);
		tag_to_elem.insert(0x0076000A, &tags::ImplantAssemblyTemplateType);

		ident_to_elem.insert("OriginalImplantAssemblyTemplateSequence", &tags::OriginalImplantAssemblyTemplateSequence);
		tag_to_elem.insert(0x0076000C, &tags::OriginalImplantAssemblyTemplateSequence);

		ident_to_elem.insert("DerivationImplantAssemblyTemplateSequence", &tags::DerivationImplantAssemblyTemplateSequence);
		tag_to_elem.insert(0x0076000E, &tags::DerivationImplantAssemblyTemplateSequence);

		ident_to_elem.insert("ImplantAssemblyTemplateTargetAnatomySequence", &tags::ImplantAssemblyTemplateTargetAnatomySequence);
		tag_to_elem.insert(0x00760010, &tags::ImplantAssemblyTemplateTargetAnatomySequence);

		ident_to_elem.insert("ProcedureTypeCodeSequence", &tags::ProcedureTypeCodeSequence);
		tag_to_elem.insert(0x00760020, &tags::ProcedureTypeCodeSequence);

		ident_to_elem.insert("SurgicalTechnique", &tags::SurgicalTechnique);
		tag_to_elem.insert(0x00760030, &tags::SurgicalTechnique);

		ident_to_elem.insert("ComponentTypesSequence", &tags::ComponentTypesSequence);
		tag_to_elem.insert(0x00760032, &tags::ComponentTypesSequence);

		ident_to_elem.insert("ComponentTypeCodeSequence", &tags::ComponentTypeCodeSequence);
		tag_to_elem.insert(0x00760034, &tags::ComponentTypeCodeSequence);

		ident_to_elem.insert("ExclusiveComponentType", &tags::ExclusiveComponentType);
		tag_to_elem.insert(0x00760036, &tags::ExclusiveComponentType);

		ident_to_elem.insert("MandatoryComponentType", &tags::MandatoryComponentType);
		tag_to_elem.insert(0x00760038, &tags::MandatoryComponentType);

		ident_to_elem.insert("ComponentSequence", &tags::ComponentSequence);
		tag_to_elem.insert(0x00760040, &tags::ComponentSequence);

		ident_to_elem.insert("ComponentID", &tags::ComponentID);
		tag_to_elem.insert(0x00760055, &tags::ComponentID);

		ident_to_elem.insert("ComponentAssemblySequence", &tags::ComponentAssemblySequence);
		tag_to_elem.insert(0x00760060, &tags::ComponentAssemblySequence);

		ident_to_elem.insert("Component1ReferencedID", &tags::Component1ReferencedID);
		tag_to_elem.insert(0x00760070, &tags::Component1ReferencedID);

		ident_to_elem.insert("Component1ReferencedMatingFeatureSetID", &tags::Component1ReferencedMatingFeatureSetID);
		tag_to_elem.insert(0x00760080, &tags::Component1ReferencedMatingFeatureSetID);

		ident_to_elem.insert("Component1ReferencedMatingFeatureID", &tags::Component1ReferencedMatingFeatureID);
		tag_to_elem.insert(0x00760090, &tags::Component1ReferencedMatingFeatureID);

		ident_to_elem.insert("Component2ReferencedID", &tags::Component2ReferencedID);
		tag_to_elem.insert(0x007600A0, &tags::Component2ReferencedID);

		ident_to_elem.insert("Component2ReferencedMatingFeatureSetID", &tags::Component2ReferencedMatingFeatureSetID);
		tag_to_elem.insert(0x007600B0, &tags::Component2ReferencedMatingFeatureSetID);

		ident_to_elem.insert("Component2ReferencedMatingFeatureID", &tags::Component2ReferencedMatingFeatureID);
		tag_to_elem.insert(0x007600C0, &tags::Component2ReferencedMatingFeatureID);

		ident_to_elem.insert("ImplantTemplateGroupName", &tags::ImplantTemplateGroupName);
		tag_to_elem.insert(0x00780001, &tags::ImplantTemplateGroupName);

		ident_to_elem.insert("ImplantTemplateGroupDescription", &tags::ImplantTemplateGroupDescription);
		tag_to_elem.insert(0x00780010, &tags::ImplantTemplateGroupDescription);

		ident_to_elem.insert("ImplantTemplateGroupIssuer", &tags::ImplantTemplateGroupIssuer);
		tag_to_elem.insert(0x00780020, &tags::ImplantTemplateGroupIssuer);

		ident_to_elem.insert("ImplantTemplateGroupVersion", &tags::ImplantTemplateGroupVersion);
		tag_to_elem.insert(0x00780024, &tags::ImplantTemplateGroupVersion);

		ident_to_elem.insert("ReplacedImplantTemplateGroupSequence", &tags::ReplacedImplantTemplateGroupSequence);
		tag_to_elem.insert(0x00780026, &tags::ReplacedImplantTemplateGroupSequence);

		ident_to_elem.insert("ImplantTemplateGroupTargetAnatomySequence", &tags::ImplantTemplateGroupTargetAnatomySequence);
		tag_to_elem.insert(0x00780028, &tags::ImplantTemplateGroupTargetAnatomySequence);

		ident_to_elem.insert("ImplantTemplateGroupMembersSequence", &tags::ImplantTemplateGroupMembersSequence);
		tag_to_elem.insert(0x0078002A, &tags::ImplantTemplateGroupMembersSequence);

		ident_to_elem.insert("ImplantTemplateGroupMemberID", &tags::ImplantTemplateGroupMemberID);
		tag_to_elem.insert(0x0078002E, &tags::ImplantTemplateGroupMemberID);

		ident_to_elem.insert("ThreeDImplantTemplateGroupMemberMatchingPoint", &tags::ThreeDImplantTemplateGroupMemberMatchingPoint);
		tag_to_elem.insert(0x00780050, &tags::ThreeDImplantTemplateGroupMemberMatchingPoint);

		ident_to_elem.insert("ThreeDImplantTemplateGroupMemberMatchingAxes", &tags::ThreeDImplantTemplateGroupMemberMatchingAxes);
		tag_to_elem.insert(0x00780060, &tags::ThreeDImplantTemplateGroupMemberMatchingAxes);

		ident_to_elem.insert("ImplantTemplateGroupMemberMatching2DCoordinatesSequence", &tags::ImplantTemplateGroupMemberMatching2DCoordinatesSequence);
		tag_to_elem.insert(0x00780070, &tags::ImplantTemplateGroupMemberMatching2DCoordinatesSequence);

		ident_to_elem.insert("TwoDImplantTemplateGroupMemberMatchingPoint", &tags::TwoDImplantTemplateGroupMemberMatchingPoint);
		tag_to_elem.insert(0x00780090, &tags::TwoDImplantTemplateGroupMemberMatchingPoint);

		ident_to_elem.insert("TwoDImplantTemplateGroupMemberMatchingAxes", &tags::TwoDImplantTemplateGroupMemberMatchingAxes);
		tag_to_elem.insert(0x007800A0, &tags::TwoDImplantTemplateGroupMemberMatchingAxes);

		ident_to_elem.insert("ImplantTemplateGroupVariationDimensionSequence", &tags::ImplantTemplateGroupVariationDimensionSequence);
		tag_to_elem.insert(0x007800B0, &tags::ImplantTemplateGroupVariationDimensionSequence);

		ident_to_elem.insert("ImplantTemplateGroupVariationDimensionName", &tags::ImplantTemplateGroupVariationDimensionName);
		tag_to_elem.insert(0x007800B2, &tags::ImplantTemplateGroupVariationDimensionName);

		ident_to_elem.insert("ImplantTemplateGroupVariationDimensionRankSequence", &tags::ImplantTemplateGroupVariationDimensionRankSequence);
		tag_to_elem.insert(0x007800B4, &tags::ImplantTemplateGroupVariationDimensionRankSequence);

		ident_to_elem.insert("ReferencedImplantTemplateGroupMemberID", &tags::ReferencedImplantTemplateGroupMemberID);
		tag_to_elem.insert(0x007800B6, &tags::ReferencedImplantTemplateGroupMemberID);

		ident_to_elem.insert("ImplantTemplateGroupVariationDimensionRank", &tags::ImplantTemplateGroupVariationDimensionRank);
		tag_to_elem.insert(0x007800B8, &tags::ImplantTemplateGroupVariationDimensionRank);

		ident_to_elem.insert("SurfaceScanAcquisitionTypeCodeSequence", &tags::SurfaceScanAcquisitionTypeCodeSequence);
		tag_to_elem.insert(0x00800001, &tags::SurfaceScanAcquisitionTypeCodeSequence);

		ident_to_elem.insert("SurfaceScanModeCodeSequence", &tags::SurfaceScanModeCodeSequence);
		tag_to_elem.insert(0x00800002, &tags::SurfaceScanModeCodeSequence);

		ident_to_elem.insert("RegistrationMethodCodeSequence", &tags::RegistrationMethodCodeSequence);
		tag_to_elem.insert(0x00800003, &tags::RegistrationMethodCodeSequence);

		ident_to_elem.insert("ShotDurationTime", &tags::ShotDurationTime);
		tag_to_elem.insert(0x00800004, &tags::ShotDurationTime);

		ident_to_elem.insert("ShotOffsetTime", &tags::ShotOffsetTime);
		tag_to_elem.insert(0x00800005, &tags::ShotOffsetTime);

		ident_to_elem.insert("SurfacePointPresentationValueData", &tags::SurfacePointPresentationValueData);
		tag_to_elem.insert(0x00800006, &tags::SurfacePointPresentationValueData);

		ident_to_elem.insert("SurfacePointColorCIELabValueData", &tags::SurfacePointColorCIELabValueData);
		tag_to_elem.insert(0x00800007, &tags::SurfacePointColorCIELabValueData);

		ident_to_elem.insert("UVMappingSequence", &tags::UVMappingSequence);
		tag_to_elem.insert(0x00800008, &tags::UVMappingSequence);

		ident_to_elem.insert("TextureLabel", &tags::TextureLabel);
		tag_to_elem.insert(0x00800009, &tags::TextureLabel);

		ident_to_elem.insert("UValueData", &tags::UValueData);
		tag_to_elem.insert(0x00800010, &tags::UValueData);

		ident_to_elem.insert("VValueData", &tags::VValueData);
		tag_to_elem.insert(0x00800011, &tags::VValueData);

		ident_to_elem.insert("ReferencedTextureSequence", &tags::ReferencedTextureSequence);
		tag_to_elem.insert(0x00800012, &tags::ReferencedTextureSequence);

		ident_to_elem.insert("ReferencedSurfaceDataSequence", &tags::ReferencedSurfaceDataSequence);
		tag_to_elem.insert(0x00800013, &tags::ReferencedSurfaceDataSequence);

		ident_to_elem.insert("AssessmentSummary", &tags::AssessmentSummary);
		tag_to_elem.insert(0x00820001, &tags::AssessmentSummary);

		ident_to_elem.insert("AssessmentSummaryDescription", &tags::AssessmentSummaryDescription);
		tag_to_elem.insert(0x00820003, &tags::AssessmentSummaryDescription);

		ident_to_elem.insert("AssessedSOPInstanceSequence", &tags::AssessedSOPInstanceSequence);
		tag_to_elem.insert(0x00820004, &tags::AssessedSOPInstanceSequence);

		ident_to_elem.insert("ReferencedComparisonSOPInstanceSequence", &tags::ReferencedComparisonSOPInstanceSequence);
		tag_to_elem.insert(0x00820005, &tags::ReferencedComparisonSOPInstanceSequence);

		ident_to_elem.insert("NumberOfAssessmentObservations", &tags::NumberOfAssessmentObservations);
		tag_to_elem.insert(0x00820006, &tags::NumberOfAssessmentObservations);

		ident_to_elem.insert("AssessmentObservationsSequence", &tags::AssessmentObservationsSequence);
		tag_to_elem.insert(0x00820007, &tags::AssessmentObservationsSequence);

		ident_to_elem.insert("ObservationSignificance", &tags::ObservationSignificance);
		tag_to_elem.insert(0x00820008, &tags::ObservationSignificance);

		ident_to_elem.insert("ObservationDescription", &tags::ObservationDescription);
		tag_to_elem.insert(0x0082000A, &tags::ObservationDescription);

		ident_to_elem.insert("StructuredContraintObservationSequence", &tags::StructuredContraintObservationSequence);
		tag_to_elem.insert(0x0082000C, &tags::StructuredContraintObservationSequence);

		ident_to_elem.insert("AssessedAttributeValueSequence", &tags::AssessedAttributeValueSequence);
		tag_to_elem.insert(0x00820010, &tags::AssessedAttributeValueSequence);

		ident_to_elem.insert("AssessmentSetID", &tags::AssessmentSetID);
		tag_to_elem.insert(0x00820016, &tags::AssessmentSetID);

		ident_to_elem.insert("AssessmentRequesterSequence", &tags::AssessmentRequesterSequence);
		tag_to_elem.insert(0x00820017, &tags::AssessmentRequesterSequence);

		ident_to_elem.insert("SelectorAttributeName", &tags::SelectorAttributeName);
		tag_to_elem.insert(0x00820018, &tags::SelectorAttributeName);

		ident_to_elem.insert("SelectorAttributeKeyword", &tags::SelectorAttributeKeyword);
		tag_to_elem.insert(0x00820019, &tags::SelectorAttributeKeyword);

		ident_to_elem.insert("AssessmentTypeCodeSequence", &tags::AssessmentTypeCodeSequence);
		tag_to_elem.insert(0x00820021, &tags::AssessmentTypeCodeSequence);

		ident_to_elem.insert("ObservationBasisCodeSequence", &tags::ObservationBasisCodeSequence);
		tag_to_elem.insert(0x00820022, &tags::ObservationBasisCodeSequence);

		ident_to_elem.insert("AssessmentLabel", &tags::AssessmentLabel);
		tag_to_elem.insert(0x00820023, &tags::AssessmentLabel);

		ident_to_elem.insert("ConstraintType", &tags::ConstraintType);
		tag_to_elem.insert(0x00820032, &tags::ConstraintType);

		ident_to_elem.insert("SpecificationSelectionGuidance", &tags::SpecificationSelectionGuidance);
		tag_to_elem.insert(0x00820033, &tags::SpecificationSelectionGuidance);

		ident_to_elem.insert("ConstraintValueSequence", &tags::ConstraintValueSequence);
		tag_to_elem.insert(0x00820034, &tags::ConstraintValueSequence);

		ident_to_elem.insert("RecommendedDefaultValueSequence", &tags::RecommendedDefaultValueSequence);
		tag_to_elem.insert(0x00820035, &tags::RecommendedDefaultValueSequence);

		ident_to_elem.insert("ConstraintViolationSignificance", &tags::ConstraintViolationSignificance);
		tag_to_elem.insert(0x00820036, &tags::ConstraintViolationSignificance);

		ident_to_elem.insert("ConstraintViolationCondition", &tags::ConstraintViolationCondition);
		tag_to_elem.insert(0x00820037, &tags::ConstraintViolationCondition);

		ident_to_elem.insert("ModifiableConstraintFlag", &tags::ModifiableConstraintFlag);
		tag_to_elem.insert(0x00820038, &tags::ModifiableConstraintFlag);

		ident_to_elem.insert("StorageMediaFileSetID", &tags::StorageMediaFileSetID);
		tag_to_elem.insert(0x00880130, &tags::StorageMediaFileSetID);

		ident_to_elem.insert("StorageMediaFileSetUID", &tags::StorageMediaFileSetUID);
		tag_to_elem.insert(0x00880140, &tags::StorageMediaFileSetUID);

		ident_to_elem.insert("IconImageSequence", &tags::IconImageSequence);
		tag_to_elem.insert(0x00880200, &tags::IconImageSequence);

		ident_to_elem.insert("TopicTitle", &tags::TopicTitle);
		tag_to_elem.insert(0x00880904, &tags::TopicTitle);

		ident_to_elem.insert("TopicSubject", &tags::TopicSubject);
		tag_to_elem.insert(0x00880906, &tags::TopicSubject);

		ident_to_elem.insert("TopicAuthor", &tags::TopicAuthor);
		tag_to_elem.insert(0x00880910, &tags::TopicAuthor);

		ident_to_elem.insert("TopicKeywords", &tags::TopicKeywords);
		tag_to_elem.insert(0x00880912, &tags::TopicKeywords);

		ident_to_elem.insert("SOPInstanceStatus", &tags::SOPInstanceStatus);
		tag_to_elem.insert(0x01000410, &tags::SOPInstanceStatus);

		ident_to_elem.insert("SOPAuthorizationDateTime", &tags::SOPAuthorizationDateTime);
		tag_to_elem.insert(0x01000420, &tags::SOPAuthorizationDateTime);

		ident_to_elem.insert("SOPAuthorizationComment", &tags::SOPAuthorizationComment);
		tag_to_elem.insert(0x01000424, &tags::SOPAuthorizationComment);

		ident_to_elem.insert("AuthorizationEquipmentCertificationNumber", &tags::AuthorizationEquipmentCertificationNumber);
		tag_to_elem.insert(0x01000426, &tags::AuthorizationEquipmentCertificationNumber);

		ident_to_elem.insert("MACIDNumber", &tags::MACIDNumber);
		tag_to_elem.insert(0x04000005, &tags::MACIDNumber);

		ident_to_elem.insert("MACCalculationTransferSyntaxUID", &tags::MACCalculationTransferSyntaxUID);
		tag_to_elem.insert(0x04000010, &tags::MACCalculationTransferSyntaxUID);

		ident_to_elem.insert("MACAlgorithm", &tags::MACAlgorithm);
		tag_to_elem.insert(0x04000015, &tags::MACAlgorithm);

		ident_to_elem.insert("DataElementsSigned", &tags::DataElementsSigned);
		tag_to_elem.insert(0x04000020, &tags::DataElementsSigned);

		ident_to_elem.insert("DigitalSignatureUID", &tags::DigitalSignatureUID);
		tag_to_elem.insert(0x04000100, &tags::DigitalSignatureUID);

		ident_to_elem.insert("DigitalSignatureDateTime", &tags::DigitalSignatureDateTime);
		tag_to_elem.insert(0x04000105, &tags::DigitalSignatureDateTime);

		ident_to_elem.insert("CertificateType", &tags::CertificateType);
		tag_to_elem.insert(0x04000110, &tags::CertificateType);

		ident_to_elem.insert("CertificateOfSigner", &tags::CertificateOfSigner);
		tag_to_elem.insert(0x04000115, &tags::CertificateOfSigner);

		ident_to_elem.insert("Signature", &tags::Signature);
		tag_to_elem.insert(0x04000120, &tags::Signature);

		ident_to_elem.insert("CertifiedTimestampType", &tags::CertifiedTimestampType);
		tag_to_elem.insert(0x04000305, &tags::CertifiedTimestampType);

		ident_to_elem.insert("CertifiedTimestamp", &tags::CertifiedTimestamp);
		tag_to_elem.insert(0x04000310, &tags::CertifiedTimestamp);

		ident_to_elem.insert("DigitalSignaturePurposeCodeSequence", &tags::DigitalSignaturePurposeCodeSequence);
		tag_to_elem.insert(0x04000401, &tags::DigitalSignaturePurposeCodeSequence);

		ident_to_elem.insert("ReferencedDigitalSignatureSequence", &tags::ReferencedDigitalSignatureSequence);
		tag_to_elem.insert(0x04000402, &tags::ReferencedDigitalSignatureSequence);

		ident_to_elem.insert("ReferencedSOPInstanceMACSequence", &tags::ReferencedSOPInstanceMACSequence);
		tag_to_elem.insert(0x04000403, &tags::ReferencedSOPInstanceMACSequence);

		ident_to_elem.insert("MAC", &tags::MAC);
		tag_to_elem.insert(0x04000404, &tags::MAC);

		ident_to_elem.insert("EncryptedAttributesSequence", &tags::EncryptedAttributesSequence);
		tag_to_elem.insert(0x04000500, &tags::EncryptedAttributesSequence);

		ident_to_elem.insert("EncryptedContentTransferSyntaxUID", &tags::EncryptedContentTransferSyntaxUID);
		tag_to_elem.insert(0x04000510, &tags::EncryptedContentTransferSyntaxUID);

		ident_to_elem.insert("EncryptedContent", &tags::EncryptedContent);
		tag_to_elem.insert(0x04000520, &tags::EncryptedContent);

		ident_to_elem.insert("ModifiedAttributesSequence", &tags::ModifiedAttributesSequence);
		tag_to_elem.insert(0x04000550, &tags::ModifiedAttributesSequence);

		ident_to_elem.insert("OriginalAttributesSequence", &tags::OriginalAttributesSequence);
		tag_to_elem.insert(0x04000561, &tags::OriginalAttributesSequence);

		ident_to_elem.insert("AttributeModificationDateTime", &tags::AttributeModificationDateTime);
		tag_to_elem.insert(0x04000562, &tags::AttributeModificationDateTime);

		ident_to_elem.insert("ModifyingSystem", &tags::ModifyingSystem);
		tag_to_elem.insert(0x04000563, &tags::ModifyingSystem);

		ident_to_elem.insert("SourceOfPreviousValues", &tags::SourceOfPreviousValues);
		tag_to_elem.insert(0x04000564, &tags::SourceOfPreviousValues);

		ident_to_elem.insert("ReasonForTheAttributeModification", &tags::ReasonForTheAttributeModification);
		tag_to_elem.insert(0x04000565, &tags::ReasonForTheAttributeModification);

		ident_to_elem.insert("NumberOfCopies", &tags::NumberOfCopies);
		tag_to_elem.insert(0x20000010, &tags::NumberOfCopies);

		ident_to_elem.insert("PrinterConfigurationSequence", &tags::PrinterConfigurationSequence);
		tag_to_elem.insert(0x2000001E, &tags::PrinterConfigurationSequence);

		ident_to_elem.insert("PrintPriority", &tags::PrintPriority);
		tag_to_elem.insert(0x20000020, &tags::PrintPriority);

		ident_to_elem.insert("MediumType", &tags::MediumType);
		tag_to_elem.insert(0x20000030, &tags::MediumType);

		ident_to_elem.insert("FilmDestination", &tags::FilmDestination);
		tag_to_elem.insert(0x20000040, &tags::FilmDestination);

		ident_to_elem.insert("FilmSessionLabel", &tags::FilmSessionLabel);
		tag_to_elem.insert(0x20000050, &tags::FilmSessionLabel);

		ident_to_elem.insert("MemoryAllocation", &tags::MemoryAllocation);
		tag_to_elem.insert(0x20000060, &tags::MemoryAllocation);

		ident_to_elem.insert("MaximumMemoryAllocation", &tags::MaximumMemoryAllocation);
		tag_to_elem.insert(0x20000061, &tags::MaximumMemoryAllocation);

		ident_to_elem.insert("ColorImagePrintingFlag", &tags::ColorImagePrintingFlag);
		tag_to_elem.insert(0x20000062, &tags::ColorImagePrintingFlag);

		ident_to_elem.insert("CollationFlag", &tags::CollationFlag);
		tag_to_elem.insert(0x20000063, &tags::CollationFlag);

		ident_to_elem.insert("AnnotationFlag", &tags::AnnotationFlag);
		tag_to_elem.insert(0x20000065, &tags::AnnotationFlag);

		ident_to_elem.insert("ImageOverlayFlag", &tags::ImageOverlayFlag);
		tag_to_elem.insert(0x20000067, &tags::ImageOverlayFlag);

		ident_to_elem.insert("PresentationLUTFlag", &tags::PresentationLUTFlag);
		tag_to_elem.insert(0x20000069, &tags::PresentationLUTFlag);

		ident_to_elem.insert("ImageBoxPresentationLUTFlag", &tags::ImageBoxPresentationLUTFlag);
		tag_to_elem.insert(0x2000006A, &tags::ImageBoxPresentationLUTFlag);

		ident_to_elem.insert("MemoryBitDepth", &tags::MemoryBitDepth);
		tag_to_elem.insert(0x200000A0, &tags::MemoryBitDepth);

		ident_to_elem.insert("PrintingBitDepth", &tags::PrintingBitDepth);
		tag_to_elem.insert(0x200000A1, &tags::PrintingBitDepth);

		ident_to_elem.insert("MediaInstalledSequence", &tags::MediaInstalledSequence);
		tag_to_elem.insert(0x200000A2, &tags::MediaInstalledSequence);

		ident_to_elem.insert("OtherMediaAvailableSequence", &tags::OtherMediaAvailableSequence);
		tag_to_elem.insert(0x200000A4, &tags::OtherMediaAvailableSequence);

		ident_to_elem.insert("SupportedImageDisplayFormatsSequence", &tags::SupportedImageDisplayFormatsSequence);
		tag_to_elem.insert(0x200000A8, &tags::SupportedImageDisplayFormatsSequence);

		ident_to_elem.insert("ReferencedFilmBoxSequence", &tags::ReferencedFilmBoxSequence);
		tag_to_elem.insert(0x20000500, &tags::ReferencedFilmBoxSequence);

		ident_to_elem.insert("ReferencedStoredPrintSequence", &tags::ReferencedStoredPrintSequence);
		tag_to_elem.insert(0x20000510, &tags::ReferencedStoredPrintSequence);

		ident_to_elem.insert("ImageDisplayFormat", &tags::ImageDisplayFormat);
		tag_to_elem.insert(0x20100010, &tags::ImageDisplayFormat);

		ident_to_elem.insert("AnnotationDisplayFormatID", &tags::AnnotationDisplayFormatID);
		tag_to_elem.insert(0x20100030, &tags::AnnotationDisplayFormatID);

		ident_to_elem.insert("FilmOrientation", &tags::FilmOrientation);
		tag_to_elem.insert(0x20100040, &tags::FilmOrientation);

		ident_to_elem.insert("FilmSizeID", &tags::FilmSizeID);
		tag_to_elem.insert(0x20100050, &tags::FilmSizeID);

		ident_to_elem.insert("PrinterResolutionID", &tags::PrinterResolutionID);
		tag_to_elem.insert(0x20100052, &tags::PrinterResolutionID);

		ident_to_elem.insert("DefaultPrinterResolutionID", &tags::DefaultPrinterResolutionID);
		tag_to_elem.insert(0x20100054, &tags::DefaultPrinterResolutionID);

		ident_to_elem.insert("MagnificationType", &tags::MagnificationType);
		tag_to_elem.insert(0x20100060, &tags::MagnificationType);

		ident_to_elem.insert("SmoothingType", &tags::SmoothingType);
		tag_to_elem.insert(0x20100080, &tags::SmoothingType);

		ident_to_elem.insert("DefaultMagnificationType", &tags::DefaultMagnificationType);
		tag_to_elem.insert(0x201000A6, &tags::DefaultMagnificationType);

		ident_to_elem.insert("OtherMagnificationTypesAvailable", &tags::OtherMagnificationTypesAvailable);
		tag_to_elem.insert(0x201000A7, &tags::OtherMagnificationTypesAvailable);

		ident_to_elem.insert("DefaultSmoothingType", &tags::DefaultSmoothingType);
		tag_to_elem.insert(0x201000A8, &tags::DefaultSmoothingType);

		ident_to_elem.insert("OtherSmoothingTypesAvailable", &tags::OtherSmoothingTypesAvailable);
		tag_to_elem.insert(0x201000A9, &tags::OtherSmoothingTypesAvailable);

		ident_to_elem.insert("BorderDensity", &tags::BorderDensity);
		tag_to_elem.insert(0x20100100, &tags::BorderDensity);

		ident_to_elem.insert("EmptyImageDensity", &tags::EmptyImageDensity);
		tag_to_elem.insert(0x20100110, &tags::EmptyImageDensity);

		ident_to_elem.insert("MinDensity", &tags::MinDensity);
		tag_to_elem.insert(0x20100120, &tags::MinDensity);

		ident_to_elem.insert("MaxDensity", &tags::MaxDensity);
		tag_to_elem.insert(0x20100130, &tags::MaxDensity);

		ident_to_elem.insert("Trim", &tags::Trim);
		tag_to_elem.insert(0x20100140, &tags::Trim);

		ident_to_elem.insert("ConfigurationInformation", &tags::ConfigurationInformation);
		tag_to_elem.insert(0x20100150, &tags::ConfigurationInformation);

		ident_to_elem.insert("ConfigurationInformationDescription", &tags::ConfigurationInformationDescription);
		tag_to_elem.insert(0x20100152, &tags::ConfigurationInformationDescription);

		ident_to_elem.insert("MaximumCollatedFilms", &tags::MaximumCollatedFilms);
		tag_to_elem.insert(0x20100154, &tags::MaximumCollatedFilms);

		ident_to_elem.insert("Illumination", &tags::Illumination);
		tag_to_elem.insert(0x2010015E, &tags::Illumination);

		ident_to_elem.insert("ReflectedAmbientLight", &tags::ReflectedAmbientLight);
		tag_to_elem.insert(0x20100160, &tags::ReflectedAmbientLight);

		ident_to_elem.insert("PrinterPixelSpacing", &tags::PrinterPixelSpacing);
		tag_to_elem.insert(0x20100376, &tags::PrinterPixelSpacing);

		ident_to_elem.insert("ReferencedFilmSessionSequence", &tags::ReferencedFilmSessionSequence);
		tag_to_elem.insert(0x20100500, &tags::ReferencedFilmSessionSequence);

		ident_to_elem.insert("ReferencedImageBoxSequence", &tags::ReferencedImageBoxSequence);
		tag_to_elem.insert(0x20100510, &tags::ReferencedImageBoxSequence);

		ident_to_elem.insert("ReferencedBasicAnnotationBoxSequence", &tags::ReferencedBasicAnnotationBoxSequence);
		tag_to_elem.insert(0x20100520, &tags::ReferencedBasicAnnotationBoxSequence);

		ident_to_elem.insert("ImageBoxPosition", &tags::ImageBoxPosition);
		tag_to_elem.insert(0x20200010, &tags::ImageBoxPosition);

		ident_to_elem.insert("Polarity", &tags::Polarity);
		tag_to_elem.insert(0x20200020, &tags::Polarity);

		ident_to_elem.insert("RequestedImageSize", &tags::RequestedImageSize);
		tag_to_elem.insert(0x20200030, &tags::RequestedImageSize);

		ident_to_elem.insert("RequestedDecimateCropBehavior", &tags::RequestedDecimateCropBehavior);
		tag_to_elem.insert(0x20200040, &tags::RequestedDecimateCropBehavior);

		ident_to_elem.insert("RequestedResolutionID", &tags::RequestedResolutionID);
		tag_to_elem.insert(0x20200050, &tags::RequestedResolutionID);

		ident_to_elem.insert("RequestedImageSizeFlag", &tags::RequestedImageSizeFlag);
		tag_to_elem.insert(0x202000A0, &tags::RequestedImageSizeFlag);

		ident_to_elem.insert("DecimateCropResult", &tags::DecimateCropResult);
		tag_to_elem.insert(0x202000A2, &tags::DecimateCropResult);

		ident_to_elem.insert("BasicGrayscaleImageSequence", &tags::BasicGrayscaleImageSequence);
		tag_to_elem.insert(0x20200110, &tags::BasicGrayscaleImageSequence);

		ident_to_elem.insert("BasicColorImageSequence", &tags::BasicColorImageSequence);
		tag_to_elem.insert(0x20200111, &tags::BasicColorImageSequence);

		ident_to_elem.insert("ReferencedImageOverlayBoxSequence", &tags::ReferencedImageOverlayBoxSequence);
		tag_to_elem.insert(0x20200130, &tags::ReferencedImageOverlayBoxSequence);

		ident_to_elem.insert("ReferencedVOILUTBoxSequence", &tags::ReferencedVOILUTBoxSequence);
		tag_to_elem.insert(0x20200140, &tags::ReferencedVOILUTBoxSequence);

		ident_to_elem.insert("AnnotationPosition", &tags::AnnotationPosition);
		tag_to_elem.insert(0x20300010, &tags::AnnotationPosition);

		ident_to_elem.insert("TextString", &tags::TextString);
		tag_to_elem.insert(0x20300020, &tags::TextString);

		ident_to_elem.insert("ReferencedOverlayPlaneSequence", &tags::ReferencedOverlayPlaneSequence);
		tag_to_elem.insert(0x20400010, &tags::ReferencedOverlayPlaneSequence);

		ident_to_elem.insert("ReferencedOverlayPlaneGroups", &tags::ReferencedOverlayPlaneGroups);
		tag_to_elem.insert(0x20400011, &tags::ReferencedOverlayPlaneGroups);

		ident_to_elem.insert("OverlayPixelDataSequence", &tags::OverlayPixelDataSequence);
		tag_to_elem.insert(0x20400020, &tags::OverlayPixelDataSequence);

		ident_to_elem.insert("OverlayMagnificationType", &tags::OverlayMagnificationType);
		tag_to_elem.insert(0x20400060, &tags::OverlayMagnificationType);

		ident_to_elem.insert("OverlaySmoothingType", &tags::OverlaySmoothingType);
		tag_to_elem.insert(0x20400070, &tags::OverlaySmoothingType);

		ident_to_elem.insert("OverlayOrImageMagnification", &tags::OverlayOrImageMagnification);
		tag_to_elem.insert(0x20400072, &tags::OverlayOrImageMagnification);

		ident_to_elem.insert("MagnifyToNumberOfColumns", &tags::MagnifyToNumberOfColumns);
		tag_to_elem.insert(0x20400074, &tags::MagnifyToNumberOfColumns);

		ident_to_elem.insert("OverlayForegroundDensity", &tags::OverlayForegroundDensity);
		tag_to_elem.insert(0x20400080, &tags::OverlayForegroundDensity);

		ident_to_elem.insert("OverlayBackgroundDensity", &tags::OverlayBackgroundDensity);
		tag_to_elem.insert(0x20400082, &tags::OverlayBackgroundDensity);

		ident_to_elem.insert("OverlayMode", &tags::OverlayMode);
		tag_to_elem.insert(0x20400090, &tags::OverlayMode);

		ident_to_elem.insert("ThresholdDensity", &tags::ThresholdDensity);
		tag_to_elem.insert(0x20400100, &tags::ThresholdDensity);

		ident_to_elem.insert("ReferencedImageBoxSequenceRetired", &tags::ReferencedImageBoxSequenceRetired);
		tag_to_elem.insert(0x20400500, &tags::ReferencedImageBoxSequenceRetired);

		ident_to_elem.insert("PresentationLUTSequence", &tags::PresentationLUTSequence);
		tag_to_elem.insert(0x20500010, &tags::PresentationLUTSequence);

		ident_to_elem.insert("PresentationLUTShape", &tags::PresentationLUTShape);
		tag_to_elem.insert(0x20500020, &tags::PresentationLUTShape);

		ident_to_elem.insert("ReferencedPresentationLUTSequence", &tags::ReferencedPresentationLUTSequence);
		tag_to_elem.insert(0x20500500, &tags::ReferencedPresentationLUTSequence);

		ident_to_elem.insert("PrintJobID", &tags::PrintJobID);
		tag_to_elem.insert(0x21000010, &tags::PrintJobID);

		ident_to_elem.insert("ExecutionStatus", &tags::ExecutionStatus);
		tag_to_elem.insert(0x21000020, &tags::ExecutionStatus);

		ident_to_elem.insert("ExecutionStatusInfo", &tags::ExecutionStatusInfo);
		tag_to_elem.insert(0x21000030, &tags::ExecutionStatusInfo);

		ident_to_elem.insert("CreationDate", &tags::CreationDate);
		tag_to_elem.insert(0x21000040, &tags::CreationDate);

		ident_to_elem.insert("CreationTime", &tags::CreationTime);
		tag_to_elem.insert(0x21000050, &tags::CreationTime);

		ident_to_elem.insert("Originator", &tags::Originator);
		tag_to_elem.insert(0x21000070, &tags::Originator);

		ident_to_elem.insert("DestinationAE", &tags::DestinationAE);
		tag_to_elem.insert(0x21000140, &tags::DestinationAE);

		ident_to_elem.insert("OwnerID", &tags::OwnerID);
		tag_to_elem.insert(0x21000160, &tags::OwnerID);

		ident_to_elem.insert("NumberOfFilms", &tags::NumberOfFilms);
		tag_to_elem.insert(0x21000170, &tags::NumberOfFilms);

		ident_to_elem.insert("ReferencedPrintJobSequencePullStoredPrint", &tags::ReferencedPrintJobSequencePullStoredPrint);
		tag_to_elem.insert(0x21000500, &tags::ReferencedPrintJobSequencePullStoredPrint);

		ident_to_elem.insert("PrinterStatus", &tags::PrinterStatus);
		tag_to_elem.insert(0x21100010, &tags::PrinterStatus);

		ident_to_elem.insert("PrinterStatusInfo", &tags::PrinterStatusInfo);
		tag_to_elem.insert(0x21100020, &tags::PrinterStatusInfo);

		ident_to_elem.insert("PrinterName", &tags::PrinterName);
		tag_to_elem.insert(0x21100030, &tags::PrinterName);

		ident_to_elem.insert("PrintQueueID", &tags::PrintQueueID);
		tag_to_elem.insert(0x21100099, &tags::PrintQueueID);

		ident_to_elem.insert("QueueStatus", &tags::QueueStatus);
		tag_to_elem.insert(0x21200010, &tags::QueueStatus);

		ident_to_elem.insert("PrintJobDescriptionSequence", &tags::PrintJobDescriptionSequence);
		tag_to_elem.insert(0x21200050, &tags::PrintJobDescriptionSequence);

		ident_to_elem.insert("ReferencedPrintJobSequence", &tags::ReferencedPrintJobSequence);
		tag_to_elem.insert(0x21200070, &tags::ReferencedPrintJobSequence);

		ident_to_elem.insert("PrintManagementCapabilitiesSequence", &tags::PrintManagementCapabilitiesSequence);
		tag_to_elem.insert(0x21300010, &tags::PrintManagementCapabilitiesSequence);

		ident_to_elem.insert("PrinterCharacteristicsSequence", &tags::PrinterCharacteristicsSequence);
		tag_to_elem.insert(0x21300015, &tags::PrinterCharacteristicsSequence);

		ident_to_elem.insert("FilmBoxContentSequence", &tags::FilmBoxContentSequence);
		tag_to_elem.insert(0x21300030, &tags::FilmBoxContentSequence);

		ident_to_elem.insert("ImageBoxContentSequence", &tags::ImageBoxContentSequence);
		tag_to_elem.insert(0x21300040, &tags::ImageBoxContentSequence);

		ident_to_elem.insert("AnnotationContentSequence", &tags::AnnotationContentSequence);
		tag_to_elem.insert(0x21300050, &tags::AnnotationContentSequence);

		ident_to_elem.insert("ImageOverlayBoxContentSequence", &tags::ImageOverlayBoxContentSequence);
		tag_to_elem.insert(0x21300060, &tags::ImageOverlayBoxContentSequence);

		ident_to_elem.insert("PresentationLUTContentSequence", &tags::PresentationLUTContentSequence);
		tag_to_elem.insert(0x21300080, &tags::PresentationLUTContentSequence);

		ident_to_elem.insert("ProposedStudySequence", &tags::ProposedStudySequence);
		tag_to_elem.insert(0x213000A0, &tags::ProposedStudySequence);

		ident_to_elem.insert("OriginalImageSequence", &tags::OriginalImageSequence);
		tag_to_elem.insert(0x213000C0, &tags::OriginalImageSequence);

		ident_to_elem.insert("LabelUsingInformationExtractedFromInstances", &tags::LabelUsingInformationExtractedFromInstances);
		tag_to_elem.insert(0x22000001, &tags::LabelUsingInformationExtractedFromInstances);

		ident_to_elem.insert("LabelText", &tags::LabelText);
		tag_to_elem.insert(0x22000002, &tags::LabelText);

		ident_to_elem.insert("LabelStyleSelection", &tags::LabelStyleSelection);
		tag_to_elem.insert(0x22000003, &tags::LabelStyleSelection);

		ident_to_elem.insert("MediaDisposition", &tags::MediaDisposition);
		tag_to_elem.insert(0x22000004, &tags::MediaDisposition);

		ident_to_elem.insert("BarcodeValue", &tags::BarcodeValue);
		tag_to_elem.insert(0x22000005, &tags::BarcodeValue);

		ident_to_elem.insert("BarcodeSymbology", &tags::BarcodeSymbology);
		tag_to_elem.insert(0x22000006, &tags::BarcodeSymbology);

		ident_to_elem.insert("AllowMediaSplitting", &tags::AllowMediaSplitting);
		tag_to_elem.insert(0x22000007, &tags::AllowMediaSplitting);

		ident_to_elem.insert("IncludeNonDICOMObjects", &tags::IncludeNonDICOMObjects);
		tag_to_elem.insert(0x22000008, &tags::IncludeNonDICOMObjects);

		ident_to_elem.insert("IncludeDisplayApplication", &tags::IncludeDisplayApplication);
		tag_to_elem.insert(0x22000009, &tags::IncludeDisplayApplication);

		ident_to_elem.insert("PreserveCompositeInstancesAfterMediaCreation", &tags::PreserveCompositeInstancesAfterMediaCreation);
		tag_to_elem.insert(0x2200000A, &tags::PreserveCompositeInstancesAfterMediaCreation);

		ident_to_elem.insert("TotalNumberOfPiecesOfMediaCreated", &tags::TotalNumberOfPiecesOfMediaCreated);
		tag_to_elem.insert(0x2200000B, &tags::TotalNumberOfPiecesOfMediaCreated);

		ident_to_elem.insert("RequestedMediaApplicationProfile", &tags::RequestedMediaApplicationProfile);
		tag_to_elem.insert(0x2200000C, &tags::RequestedMediaApplicationProfile);

		ident_to_elem.insert("ReferencedStorageMediaSequence", &tags::ReferencedStorageMediaSequence);
		tag_to_elem.insert(0x2200000D, &tags::ReferencedStorageMediaSequence);

		ident_to_elem.insert("FailureAttributes", &tags::FailureAttributes);
		tag_to_elem.insert(0x2200000E, &tags::FailureAttributes);

		ident_to_elem.insert("AllowLossyCompression", &tags::AllowLossyCompression);
		tag_to_elem.insert(0x2200000F, &tags::AllowLossyCompression);

		ident_to_elem.insert("RequestPriority", &tags::RequestPriority);
		tag_to_elem.insert(0x22000020, &tags::RequestPriority);

		ident_to_elem.insert("RTImageLabel", &tags::RTImageLabel);
		tag_to_elem.insert(0x30020002, &tags::RTImageLabel);

		ident_to_elem.insert("RTImageName", &tags::RTImageName);
		tag_to_elem.insert(0x30020003, &tags::RTImageName);

		ident_to_elem.insert("RTImageDescription", &tags::RTImageDescription);
		tag_to_elem.insert(0x30020004, &tags::RTImageDescription);

		ident_to_elem.insert("ReportedValuesOrigin", &tags::ReportedValuesOrigin);
		tag_to_elem.insert(0x3002000A, &tags::ReportedValuesOrigin);

		ident_to_elem.insert("RTImagePlane", &tags::RTImagePlane);
		tag_to_elem.insert(0x3002000C, &tags::RTImagePlane);

		ident_to_elem.insert("XRayImageReceptorTranslation", &tags::XRayImageReceptorTranslation);
		tag_to_elem.insert(0x3002000D, &tags::XRayImageReceptorTranslation);

		ident_to_elem.insert("XRayImageReceptorAngle", &tags::XRayImageReceptorAngle);
		tag_to_elem.insert(0x3002000E, &tags::XRayImageReceptorAngle);

		ident_to_elem.insert("RTImageOrientation", &tags::RTImageOrientation);
		tag_to_elem.insert(0x30020010, &tags::RTImageOrientation);

		ident_to_elem.insert("ImagePlanePixelSpacing", &tags::ImagePlanePixelSpacing);
		tag_to_elem.insert(0x30020011, &tags::ImagePlanePixelSpacing);

		ident_to_elem.insert("RTImagePosition", &tags::RTImagePosition);
		tag_to_elem.insert(0x30020012, &tags::RTImagePosition);

		ident_to_elem.insert("RadiationMachineName", &tags::RadiationMachineName);
		tag_to_elem.insert(0x30020020, &tags::RadiationMachineName);

		ident_to_elem.insert("RadiationMachineSAD", &tags::RadiationMachineSAD);
		tag_to_elem.insert(0x30020022, &tags::RadiationMachineSAD);

		ident_to_elem.insert("RadiationMachineSSD", &tags::RadiationMachineSSD);
		tag_to_elem.insert(0x30020024, &tags::RadiationMachineSSD);

		ident_to_elem.insert("RTImageSID", &tags::RTImageSID);
		tag_to_elem.insert(0x30020026, &tags::RTImageSID);

		ident_to_elem.insert("SourceToReferenceObjectDistance", &tags::SourceToReferenceObjectDistance);
		tag_to_elem.insert(0x30020028, &tags::SourceToReferenceObjectDistance);

		ident_to_elem.insert("FractionNumber", &tags::FractionNumber);
		tag_to_elem.insert(0x30020029, &tags::FractionNumber);

		ident_to_elem.insert("ExposureSequence", &tags::ExposureSequence);
		tag_to_elem.insert(0x30020030, &tags::ExposureSequence);

		ident_to_elem.insert("MetersetExposure", &tags::MetersetExposure);
		tag_to_elem.insert(0x30020032, &tags::MetersetExposure);

		ident_to_elem.insert("DiaphragmPosition", &tags::DiaphragmPosition);
		tag_to_elem.insert(0x30020034, &tags::DiaphragmPosition);

		ident_to_elem.insert("FluenceMapSequence", &tags::FluenceMapSequence);
		tag_to_elem.insert(0x30020040, &tags::FluenceMapSequence);

		ident_to_elem.insert("FluenceDataSource", &tags::FluenceDataSource);
		tag_to_elem.insert(0x30020041, &tags::FluenceDataSource);

		ident_to_elem.insert("FluenceDataScale", &tags::FluenceDataScale);
		tag_to_elem.insert(0x30020042, &tags::FluenceDataScale);

		ident_to_elem.insert("PrimaryFluenceModeSequence", &tags::PrimaryFluenceModeSequence);
		tag_to_elem.insert(0x30020050, &tags::PrimaryFluenceModeSequence);

		ident_to_elem.insert("FluenceMode", &tags::FluenceMode);
		tag_to_elem.insert(0x30020051, &tags::FluenceMode);

		ident_to_elem.insert("FluenceModeID", &tags::FluenceModeID);
		tag_to_elem.insert(0x30020052, &tags::FluenceModeID);

		ident_to_elem.insert("DVHType", &tags::DVHType);
		tag_to_elem.insert(0x30040001, &tags::DVHType);

		ident_to_elem.insert("DoseUnits", &tags::DoseUnits);
		tag_to_elem.insert(0x30040002, &tags::DoseUnits);

		ident_to_elem.insert("DoseType", &tags::DoseType);
		tag_to_elem.insert(0x30040004, &tags::DoseType);

		ident_to_elem.insert("SpatialTransformOfDose", &tags::SpatialTransformOfDose);
		tag_to_elem.insert(0x30040005, &tags::SpatialTransformOfDose);

		ident_to_elem.insert("DoseComment", &tags::DoseComment);
		tag_to_elem.insert(0x30040006, &tags::DoseComment);

		ident_to_elem.insert("NormalizationPoint", &tags::NormalizationPoint);
		tag_to_elem.insert(0x30040008, &tags::NormalizationPoint);

		ident_to_elem.insert("DoseSummationType", &tags::DoseSummationType);
		tag_to_elem.insert(0x3004000A, &tags::DoseSummationType);

		ident_to_elem.insert("GridFrameOffsetVector", &tags::GridFrameOffsetVector);
		tag_to_elem.insert(0x3004000C, &tags::GridFrameOffsetVector);

		ident_to_elem.insert("DoseGridScaling", &tags::DoseGridScaling);
		tag_to_elem.insert(0x3004000E, &tags::DoseGridScaling);

		ident_to_elem.insert("RTDoseROISequence", &tags::RTDoseROISequence);
		tag_to_elem.insert(0x30040010, &tags::RTDoseROISequence);

		ident_to_elem.insert("DoseValue", &tags::DoseValue);
		tag_to_elem.insert(0x30040012, &tags::DoseValue);

		ident_to_elem.insert("TissueHeterogeneityCorrection", &tags::TissueHeterogeneityCorrection);
		tag_to_elem.insert(0x30040014, &tags::TissueHeterogeneityCorrection);

		ident_to_elem.insert("DVHNormalizationPoint", &tags::DVHNormalizationPoint);
		tag_to_elem.insert(0x30040040, &tags::DVHNormalizationPoint);

		ident_to_elem.insert("DVHNormalizationDoseValue", &tags::DVHNormalizationDoseValue);
		tag_to_elem.insert(0x30040042, &tags::DVHNormalizationDoseValue);

		ident_to_elem.insert("DVHSequence", &tags::DVHSequence);
		tag_to_elem.insert(0x30040050, &tags::DVHSequence);

		ident_to_elem.insert("DVHDoseScaling", &tags::DVHDoseScaling);
		tag_to_elem.insert(0x30040052, &tags::DVHDoseScaling);

		ident_to_elem.insert("DVHVolumeUnits", &tags::DVHVolumeUnits);
		tag_to_elem.insert(0x30040054, &tags::DVHVolumeUnits);

		ident_to_elem.insert("DVHNumberOfBins", &tags::DVHNumberOfBins);
		tag_to_elem.insert(0x30040056, &tags::DVHNumberOfBins);

		ident_to_elem.insert("DVHData", &tags::DVHData);
		tag_to_elem.insert(0x30040058, &tags::DVHData);

		ident_to_elem.insert("DVHReferencedROISequence", &tags::DVHReferencedROISequence);
		tag_to_elem.insert(0x30040060, &tags::DVHReferencedROISequence);

		ident_to_elem.insert("DVHROIContributionType", &tags::DVHROIContributionType);
		tag_to_elem.insert(0x30040062, &tags::DVHROIContributionType);

		ident_to_elem.insert("DVHMinimumDose", &tags::DVHMinimumDose);
		tag_to_elem.insert(0x30040070, &tags::DVHMinimumDose);

		ident_to_elem.insert("DVHMaximumDose", &tags::DVHMaximumDose);
		tag_to_elem.insert(0x30040072, &tags::DVHMaximumDose);

		ident_to_elem.insert("DVHMeanDose", &tags::DVHMeanDose);
		tag_to_elem.insert(0x30040074, &tags::DVHMeanDose);

		ident_to_elem.insert("StructureSetLabel", &tags::StructureSetLabel);
		tag_to_elem.insert(0x30060002, &tags::StructureSetLabel);

		ident_to_elem.insert("StructureSetName", &tags::StructureSetName);
		tag_to_elem.insert(0x30060004, &tags::StructureSetName);

		ident_to_elem.insert("StructureSetDescription", &tags::StructureSetDescription);
		tag_to_elem.insert(0x30060006, &tags::StructureSetDescription);

		ident_to_elem.insert("StructureSetDate", &tags::StructureSetDate);
		tag_to_elem.insert(0x30060008, &tags::StructureSetDate);

		ident_to_elem.insert("StructureSetTime", &tags::StructureSetTime);
		tag_to_elem.insert(0x30060009, &tags::StructureSetTime);

		ident_to_elem.insert("ReferencedFrameOfReferenceSequence", &tags::ReferencedFrameOfReferenceSequence);
		tag_to_elem.insert(0x30060010, &tags::ReferencedFrameOfReferenceSequence);

		ident_to_elem.insert("RTReferencedStudySequence", &tags::RTReferencedStudySequence);
		tag_to_elem.insert(0x30060012, &tags::RTReferencedStudySequence);

		ident_to_elem.insert("RTReferencedSeriesSequence", &tags::RTReferencedSeriesSequence);
		tag_to_elem.insert(0x30060014, &tags::RTReferencedSeriesSequence);

		ident_to_elem.insert("ContourImageSequence", &tags::ContourImageSequence);
		tag_to_elem.insert(0x30060016, &tags::ContourImageSequence);

		ident_to_elem.insert("PredecessorStructureSetSequence", &tags::PredecessorStructureSetSequence);
		tag_to_elem.insert(0x30060018, &tags::PredecessorStructureSetSequence);

		ident_to_elem.insert("StructureSetROISequence", &tags::StructureSetROISequence);
		tag_to_elem.insert(0x30060020, &tags::StructureSetROISequence);

		ident_to_elem.insert("ROINumber", &tags::ROINumber);
		tag_to_elem.insert(0x30060022, &tags::ROINumber);

		ident_to_elem.insert("ReferencedFrameOfReferenceUID", &tags::ReferencedFrameOfReferenceUID);
		tag_to_elem.insert(0x30060024, &tags::ReferencedFrameOfReferenceUID);

		ident_to_elem.insert("ROIName", &tags::ROIName);
		tag_to_elem.insert(0x30060026, &tags::ROIName);

		ident_to_elem.insert("ROIDescription", &tags::ROIDescription);
		tag_to_elem.insert(0x30060028, &tags::ROIDescription);

		ident_to_elem.insert("ROIDisplayColor", &tags::ROIDisplayColor);
		tag_to_elem.insert(0x3006002A, &tags::ROIDisplayColor);

		ident_to_elem.insert("ROIVolume", &tags::ROIVolume);
		tag_to_elem.insert(0x3006002C, &tags::ROIVolume);

		ident_to_elem.insert("RTRelatedROISequence", &tags::RTRelatedROISequence);
		tag_to_elem.insert(0x30060030, &tags::RTRelatedROISequence);

		ident_to_elem.insert("RTROIRelationship", &tags::RTROIRelationship);
		tag_to_elem.insert(0x30060033, &tags::RTROIRelationship);

		ident_to_elem.insert("ROIGenerationAlgorithm", &tags::ROIGenerationAlgorithm);
		tag_to_elem.insert(0x30060036, &tags::ROIGenerationAlgorithm);

		ident_to_elem.insert("ROIGenerationDescription", &tags::ROIGenerationDescription);
		tag_to_elem.insert(0x30060038, &tags::ROIGenerationDescription);

		ident_to_elem.insert("ROIContourSequence", &tags::ROIContourSequence);
		tag_to_elem.insert(0x30060039, &tags::ROIContourSequence);

		ident_to_elem.insert("ContourSequence", &tags::ContourSequence);
		tag_to_elem.insert(0x30060040, &tags::ContourSequence);

		ident_to_elem.insert("ContourGeometricType", &tags::ContourGeometricType);
		tag_to_elem.insert(0x30060042, &tags::ContourGeometricType);

		ident_to_elem.insert("ContourSlabThickness", &tags::ContourSlabThickness);
		tag_to_elem.insert(0x30060044, &tags::ContourSlabThickness);

		ident_to_elem.insert("ContourOffsetVector", &tags::ContourOffsetVector);
		tag_to_elem.insert(0x30060045, &tags::ContourOffsetVector);

		ident_to_elem.insert("NumberOfContourPoints", &tags::NumberOfContourPoints);
		tag_to_elem.insert(0x30060046, &tags::NumberOfContourPoints);

		ident_to_elem.insert("ContourNumber", &tags::ContourNumber);
		tag_to_elem.insert(0x30060048, &tags::ContourNumber);

		ident_to_elem.insert("AttachedContours", &tags::AttachedContours);
		tag_to_elem.insert(0x30060049, &tags::AttachedContours);

		ident_to_elem.insert("ContourData", &tags::ContourData);
		tag_to_elem.insert(0x30060050, &tags::ContourData);

		ident_to_elem.insert("RTROIObservationsSequence", &tags::RTROIObservationsSequence);
		tag_to_elem.insert(0x30060080, &tags::RTROIObservationsSequence);

		ident_to_elem.insert("ObservationNumber", &tags::ObservationNumber);
		tag_to_elem.insert(0x30060082, &tags::ObservationNumber);

		ident_to_elem.insert("ReferencedROINumber", &tags::ReferencedROINumber);
		tag_to_elem.insert(0x30060084, &tags::ReferencedROINumber);

		ident_to_elem.insert("ROIObservationLabel", &tags::ROIObservationLabel);
		tag_to_elem.insert(0x30060085, &tags::ROIObservationLabel);

		ident_to_elem.insert("RTROIIdentificationCodeSequence", &tags::RTROIIdentificationCodeSequence);
		tag_to_elem.insert(0x30060086, &tags::RTROIIdentificationCodeSequence);

		ident_to_elem.insert("ROIObservationDescription", &tags::ROIObservationDescription);
		tag_to_elem.insert(0x30060088, &tags::ROIObservationDescription);

		ident_to_elem.insert("RelatedRTROIObservationsSequence", &tags::RelatedRTROIObservationsSequence);
		tag_to_elem.insert(0x300600A0, &tags::RelatedRTROIObservationsSequence);

		ident_to_elem.insert("RTROIInterpretedType", &tags::RTROIInterpretedType);
		tag_to_elem.insert(0x300600A4, &tags::RTROIInterpretedType);

		ident_to_elem.insert("ROIInterpreter", &tags::ROIInterpreter);
		tag_to_elem.insert(0x300600A6, &tags::ROIInterpreter);

		ident_to_elem.insert("ROIPhysicalPropertiesSequence", &tags::ROIPhysicalPropertiesSequence);
		tag_to_elem.insert(0x300600B0, &tags::ROIPhysicalPropertiesSequence);

		ident_to_elem.insert("ROIPhysicalProperty", &tags::ROIPhysicalProperty);
		tag_to_elem.insert(0x300600B2, &tags::ROIPhysicalProperty);

		ident_to_elem.insert("ROIPhysicalPropertyValue", &tags::ROIPhysicalPropertyValue);
		tag_to_elem.insert(0x300600B4, &tags::ROIPhysicalPropertyValue);

		ident_to_elem.insert("ROIElementalCompositionSequence", &tags::ROIElementalCompositionSequence);
		tag_to_elem.insert(0x300600B6, &tags::ROIElementalCompositionSequence);

		ident_to_elem.insert("ROIElementalCompositionAtomicNumber", &tags::ROIElementalCompositionAtomicNumber);
		tag_to_elem.insert(0x300600B7, &tags::ROIElementalCompositionAtomicNumber);

		ident_to_elem.insert("ROIElementalCompositionAtomicMassFraction", &tags::ROIElementalCompositionAtomicMassFraction);
		tag_to_elem.insert(0x300600B8, &tags::ROIElementalCompositionAtomicMassFraction);

		ident_to_elem.insert("AdditionalRTROIIdentificationCodeSequence", &tags::AdditionalRTROIIdentificationCodeSequence);
		tag_to_elem.insert(0x300600B9, &tags::AdditionalRTROIIdentificationCodeSequence);

		ident_to_elem.insert("FrameOfReferenceRelationshipSequence", &tags::FrameOfReferenceRelationshipSequence);
		tag_to_elem.insert(0x300600C0, &tags::FrameOfReferenceRelationshipSequence);

		ident_to_elem.insert("RelatedFrameOfReferenceUID", &tags::RelatedFrameOfReferenceUID);
		tag_to_elem.insert(0x300600C2, &tags::RelatedFrameOfReferenceUID);

		ident_to_elem.insert("FrameOfReferenceTransformationType", &tags::FrameOfReferenceTransformationType);
		tag_to_elem.insert(0x300600C4, &tags::FrameOfReferenceTransformationType);

		ident_to_elem.insert("FrameOfReferenceTransformationMatrix", &tags::FrameOfReferenceTransformationMatrix);
		tag_to_elem.insert(0x300600C6, &tags::FrameOfReferenceTransformationMatrix);

		ident_to_elem.insert("FrameOfReferenceTransformationComment", &tags::FrameOfReferenceTransformationComment);
		tag_to_elem.insert(0x300600C8, &tags::FrameOfReferenceTransformationComment);

		ident_to_elem.insert("MeasuredDoseReferenceSequence", &tags::MeasuredDoseReferenceSequence);
		tag_to_elem.insert(0x30080010, &tags::MeasuredDoseReferenceSequence);

		ident_to_elem.insert("MeasuredDoseDescription", &tags::MeasuredDoseDescription);
		tag_to_elem.insert(0x30080012, &tags::MeasuredDoseDescription);

		ident_to_elem.insert("MeasuredDoseType", &tags::MeasuredDoseType);
		tag_to_elem.insert(0x30080014, &tags::MeasuredDoseType);

		ident_to_elem.insert("MeasuredDoseValue", &tags::MeasuredDoseValue);
		tag_to_elem.insert(0x30080016, &tags::MeasuredDoseValue);

		ident_to_elem.insert("TreatmentSessionBeamSequence", &tags::TreatmentSessionBeamSequence);
		tag_to_elem.insert(0x30080020, &tags::TreatmentSessionBeamSequence);

		ident_to_elem.insert("TreatmentSessionIonBeamSequence", &tags::TreatmentSessionIonBeamSequence);
		tag_to_elem.insert(0x30080021, &tags::TreatmentSessionIonBeamSequence);

		ident_to_elem.insert("CurrentFractionNumber", &tags::CurrentFractionNumber);
		tag_to_elem.insert(0x30080022, &tags::CurrentFractionNumber);

		ident_to_elem.insert("TreatmentControlPointDate", &tags::TreatmentControlPointDate);
		tag_to_elem.insert(0x30080024, &tags::TreatmentControlPointDate);

		ident_to_elem.insert("TreatmentControlPointTime", &tags::TreatmentControlPointTime);
		tag_to_elem.insert(0x30080025, &tags::TreatmentControlPointTime);

		ident_to_elem.insert("TreatmentTerminationStatus", &tags::TreatmentTerminationStatus);
		tag_to_elem.insert(0x3008002A, &tags::TreatmentTerminationStatus);

		ident_to_elem.insert("TreatmentTerminationCode", &tags::TreatmentTerminationCode);
		tag_to_elem.insert(0x3008002B, &tags::TreatmentTerminationCode);

		ident_to_elem.insert("TreatmentVerificationStatus", &tags::TreatmentVerificationStatus);
		tag_to_elem.insert(0x3008002C, &tags::TreatmentVerificationStatus);

		ident_to_elem.insert("ReferencedTreatmentRecordSequence", &tags::ReferencedTreatmentRecordSequence);
		tag_to_elem.insert(0x30080030, &tags::ReferencedTreatmentRecordSequence);

		ident_to_elem.insert("SpecifiedPrimaryMeterset", &tags::SpecifiedPrimaryMeterset);
		tag_to_elem.insert(0x30080032, &tags::SpecifiedPrimaryMeterset);

		ident_to_elem.insert("SpecifiedSecondaryMeterset", &tags::SpecifiedSecondaryMeterset);
		tag_to_elem.insert(0x30080033, &tags::SpecifiedSecondaryMeterset);

		ident_to_elem.insert("DeliveredPrimaryMeterset", &tags::DeliveredPrimaryMeterset);
		tag_to_elem.insert(0x30080036, &tags::DeliveredPrimaryMeterset);

		ident_to_elem.insert("DeliveredSecondaryMeterset", &tags::DeliveredSecondaryMeterset);
		tag_to_elem.insert(0x30080037, &tags::DeliveredSecondaryMeterset);

		ident_to_elem.insert("SpecifiedTreatmentTime", &tags::SpecifiedTreatmentTime);
		tag_to_elem.insert(0x3008003A, &tags::SpecifiedTreatmentTime);

		ident_to_elem.insert("DeliveredTreatmentTime", &tags::DeliveredTreatmentTime);
		tag_to_elem.insert(0x3008003B, &tags::DeliveredTreatmentTime);

		ident_to_elem.insert("ControlPointDeliverySequence", &tags::ControlPointDeliverySequence);
		tag_to_elem.insert(0x30080040, &tags::ControlPointDeliverySequence);

		ident_to_elem.insert("IonControlPointDeliverySequence", &tags::IonControlPointDeliverySequence);
		tag_to_elem.insert(0x30080041, &tags::IonControlPointDeliverySequence);

		ident_to_elem.insert("SpecifiedMeterset", &tags::SpecifiedMeterset);
		tag_to_elem.insert(0x30080042, &tags::SpecifiedMeterset);

		ident_to_elem.insert("DeliveredMeterset", &tags::DeliveredMeterset);
		tag_to_elem.insert(0x30080044, &tags::DeliveredMeterset);

		ident_to_elem.insert("MetersetRateSet", &tags::MetersetRateSet);
		tag_to_elem.insert(0x30080045, &tags::MetersetRateSet);

		ident_to_elem.insert("MetersetRateDelivered", &tags::MetersetRateDelivered);
		tag_to_elem.insert(0x30080046, &tags::MetersetRateDelivered);

		ident_to_elem.insert("ScanSpotMetersetsDelivered", &tags::ScanSpotMetersetsDelivered);
		tag_to_elem.insert(0x30080047, &tags::ScanSpotMetersetsDelivered);

		ident_to_elem.insert("DoseRateDelivered", &tags::DoseRateDelivered);
		tag_to_elem.insert(0x30080048, &tags::DoseRateDelivered);

		ident_to_elem.insert("TreatmentSummaryCalculatedDoseReferenceSequence", &tags::TreatmentSummaryCalculatedDoseReferenceSequence);
		tag_to_elem.insert(0x30080050, &tags::TreatmentSummaryCalculatedDoseReferenceSequence);

		ident_to_elem.insert("CumulativeDoseToDoseReference", &tags::CumulativeDoseToDoseReference);
		tag_to_elem.insert(0x30080052, &tags::CumulativeDoseToDoseReference);

		ident_to_elem.insert("FirstTreatmentDate", &tags::FirstTreatmentDate);
		tag_to_elem.insert(0x30080054, &tags::FirstTreatmentDate);

		ident_to_elem.insert("MostRecentTreatmentDate", &tags::MostRecentTreatmentDate);
		tag_to_elem.insert(0x30080056, &tags::MostRecentTreatmentDate);

		ident_to_elem.insert("NumberOfFractionsDelivered", &tags::NumberOfFractionsDelivered);
		tag_to_elem.insert(0x3008005A, &tags::NumberOfFractionsDelivered);

		ident_to_elem.insert("OverrideSequence", &tags::OverrideSequence);
		tag_to_elem.insert(0x30080060, &tags::OverrideSequence);

		ident_to_elem.insert("ParameterSequencePointer", &tags::ParameterSequencePointer);
		tag_to_elem.insert(0x30080061, &tags::ParameterSequencePointer);

		ident_to_elem.insert("OverrideParameterPointer", &tags::OverrideParameterPointer);
		tag_to_elem.insert(0x30080062, &tags::OverrideParameterPointer);

		ident_to_elem.insert("ParameterItemIndex", &tags::ParameterItemIndex);
		tag_to_elem.insert(0x30080063, &tags::ParameterItemIndex);

		ident_to_elem.insert("MeasuredDoseReferenceNumber", &tags::MeasuredDoseReferenceNumber);
		tag_to_elem.insert(0x30080064, &tags::MeasuredDoseReferenceNumber);

		ident_to_elem.insert("ParameterPointer", &tags::ParameterPointer);
		tag_to_elem.insert(0x30080065, &tags::ParameterPointer);

		ident_to_elem.insert("OverrideReason", &tags::OverrideReason);
		tag_to_elem.insert(0x30080066, &tags::OverrideReason);

		ident_to_elem.insert("ParameterValueNumber", &tags::ParameterValueNumber);
		tag_to_elem.insert(0x30080067, &tags::ParameterValueNumber);

		ident_to_elem.insert("CorrectedParameterSequence", &tags::CorrectedParameterSequence);
		tag_to_elem.insert(0x30080068, &tags::CorrectedParameterSequence);

		ident_to_elem.insert("CorrectionValue", &tags::CorrectionValue);
		tag_to_elem.insert(0x3008006A, &tags::CorrectionValue);

		ident_to_elem.insert("CalculatedDoseReferenceSequence", &tags::CalculatedDoseReferenceSequence);
		tag_to_elem.insert(0x30080070, &tags::CalculatedDoseReferenceSequence);

		ident_to_elem.insert("CalculatedDoseReferenceNumber", &tags::CalculatedDoseReferenceNumber);
		tag_to_elem.insert(0x30080072, &tags::CalculatedDoseReferenceNumber);

		ident_to_elem.insert("CalculatedDoseReferenceDescription", &tags::CalculatedDoseReferenceDescription);
		tag_to_elem.insert(0x30080074, &tags::CalculatedDoseReferenceDescription);

		ident_to_elem.insert("CalculatedDoseReferenceDoseValue", &tags::CalculatedDoseReferenceDoseValue);
		tag_to_elem.insert(0x30080076, &tags::CalculatedDoseReferenceDoseValue);

		ident_to_elem.insert("StartMeterset", &tags::StartMeterset);
		tag_to_elem.insert(0x30080078, &tags::StartMeterset);

		ident_to_elem.insert("EndMeterset", &tags::EndMeterset);
		tag_to_elem.insert(0x3008007A, &tags::EndMeterset);

		ident_to_elem.insert("ReferencedMeasuredDoseReferenceSequence", &tags::ReferencedMeasuredDoseReferenceSequence);
		tag_to_elem.insert(0x30080080, &tags::ReferencedMeasuredDoseReferenceSequence);

		ident_to_elem.insert("ReferencedMeasuredDoseReferenceNumber", &tags::ReferencedMeasuredDoseReferenceNumber);
		tag_to_elem.insert(0x30080082, &tags::ReferencedMeasuredDoseReferenceNumber);

		ident_to_elem.insert("ReferencedCalculatedDoseReferenceSequence", &tags::ReferencedCalculatedDoseReferenceSequence);
		tag_to_elem.insert(0x30080090, &tags::ReferencedCalculatedDoseReferenceSequence);

		ident_to_elem.insert("ReferencedCalculatedDoseReferenceNumber", &tags::ReferencedCalculatedDoseReferenceNumber);
		tag_to_elem.insert(0x30080092, &tags::ReferencedCalculatedDoseReferenceNumber);

		ident_to_elem.insert("BeamLimitingDeviceLeafPairsSequence", &tags::BeamLimitingDeviceLeafPairsSequence);
		tag_to_elem.insert(0x300800A0, &tags::BeamLimitingDeviceLeafPairsSequence);

		ident_to_elem.insert("RecordedWedgeSequence", &tags::RecordedWedgeSequence);
		tag_to_elem.insert(0x300800B0, &tags::RecordedWedgeSequence);

		ident_to_elem.insert("RecordedCompensatorSequence", &tags::RecordedCompensatorSequence);
		tag_to_elem.insert(0x300800C0, &tags::RecordedCompensatorSequence);

		ident_to_elem.insert("RecordedBlockSequence", &tags::RecordedBlockSequence);
		tag_to_elem.insert(0x300800D0, &tags::RecordedBlockSequence);

		ident_to_elem.insert("TreatmentSummaryMeasuredDoseReferenceSequence", &tags::TreatmentSummaryMeasuredDoseReferenceSequence);
		tag_to_elem.insert(0x300800E0, &tags::TreatmentSummaryMeasuredDoseReferenceSequence);

		ident_to_elem.insert("RecordedSnoutSequence", &tags::RecordedSnoutSequence);
		tag_to_elem.insert(0x300800F0, &tags::RecordedSnoutSequence);

		ident_to_elem.insert("RecordedRangeShifterSequence", &tags::RecordedRangeShifterSequence);
		tag_to_elem.insert(0x300800F2, &tags::RecordedRangeShifterSequence);

		ident_to_elem.insert("RecordedLateralSpreadingDeviceSequence", &tags::RecordedLateralSpreadingDeviceSequence);
		tag_to_elem.insert(0x300800F4, &tags::RecordedLateralSpreadingDeviceSequence);

		ident_to_elem.insert("RecordedRangeModulatorSequence", &tags::RecordedRangeModulatorSequence);
		tag_to_elem.insert(0x300800F6, &tags::RecordedRangeModulatorSequence);

		ident_to_elem.insert("RecordedSourceSequence", &tags::RecordedSourceSequence);
		tag_to_elem.insert(0x30080100, &tags::RecordedSourceSequence);

		ident_to_elem.insert("SourceSerialNumber", &tags::SourceSerialNumber);
		tag_to_elem.insert(0x30080105, &tags::SourceSerialNumber);

		ident_to_elem.insert("TreatmentSessionApplicationSetupSequence", &tags::TreatmentSessionApplicationSetupSequence);
		tag_to_elem.insert(0x30080110, &tags::TreatmentSessionApplicationSetupSequence);

		ident_to_elem.insert("ApplicationSetupCheck", &tags::ApplicationSetupCheck);
		tag_to_elem.insert(0x30080116, &tags::ApplicationSetupCheck);

		ident_to_elem.insert("RecordedBrachyAccessoryDeviceSequence", &tags::RecordedBrachyAccessoryDeviceSequence);
		tag_to_elem.insert(0x30080120, &tags::RecordedBrachyAccessoryDeviceSequence);

		ident_to_elem.insert("ReferencedBrachyAccessoryDeviceNumber", &tags::ReferencedBrachyAccessoryDeviceNumber);
		tag_to_elem.insert(0x30080122, &tags::ReferencedBrachyAccessoryDeviceNumber);

		ident_to_elem.insert("RecordedChannelSequence", &tags::RecordedChannelSequence);
		tag_to_elem.insert(0x30080130, &tags::RecordedChannelSequence);

		ident_to_elem.insert("SpecifiedChannelTotalTime", &tags::SpecifiedChannelTotalTime);
		tag_to_elem.insert(0x30080132, &tags::SpecifiedChannelTotalTime);

		ident_to_elem.insert("DeliveredChannelTotalTime", &tags::DeliveredChannelTotalTime);
		tag_to_elem.insert(0x30080134, &tags::DeliveredChannelTotalTime);

		ident_to_elem.insert("SpecifiedNumberOfPulses", &tags::SpecifiedNumberOfPulses);
		tag_to_elem.insert(0x30080136, &tags::SpecifiedNumberOfPulses);

		ident_to_elem.insert("DeliveredNumberOfPulses", &tags::DeliveredNumberOfPulses);
		tag_to_elem.insert(0x30080138, &tags::DeliveredNumberOfPulses);

		ident_to_elem.insert("SpecifiedPulseRepetitionInterval", &tags::SpecifiedPulseRepetitionInterval);
		tag_to_elem.insert(0x3008013A, &tags::SpecifiedPulseRepetitionInterval);

		ident_to_elem.insert("DeliveredPulseRepetitionInterval", &tags::DeliveredPulseRepetitionInterval);
		tag_to_elem.insert(0x3008013C, &tags::DeliveredPulseRepetitionInterval);

		ident_to_elem.insert("RecordedSourceApplicatorSequence", &tags::RecordedSourceApplicatorSequence);
		tag_to_elem.insert(0x30080140, &tags::RecordedSourceApplicatorSequence);

		ident_to_elem.insert("ReferencedSourceApplicatorNumber", &tags::ReferencedSourceApplicatorNumber);
		tag_to_elem.insert(0x30080142, &tags::ReferencedSourceApplicatorNumber);

		ident_to_elem.insert("RecordedChannelShieldSequence", &tags::RecordedChannelShieldSequence);
		tag_to_elem.insert(0x30080150, &tags::RecordedChannelShieldSequence);

		ident_to_elem.insert("ReferencedChannelShieldNumber", &tags::ReferencedChannelShieldNumber);
		tag_to_elem.insert(0x30080152, &tags::ReferencedChannelShieldNumber);

		ident_to_elem.insert("BrachyControlPointDeliveredSequence", &tags::BrachyControlPointDeliveredSequence);
		tag_to_elem.insert(0x30080160, &tags::BrachyControlPointDeliveredSequence);

		ident_to_elem.insert("SafePositionExitDate", &tags::SafePositionExitDate);
		tag_to_elem.insert(0x30080162, &tags::SafePositionExitDate);

		ident_to_elem.insert("SafePositionExitTime", &tags::SafePositionExitTime);
		tag_to_elem.insert(0x30080164, &tags::SafePositionExitTime);

		ident_to_elem.insert("SafePositionReturnDate", &tags::SafePositionReturnDate);
		tag_to_elem.insert(0x30080166, &tags::SafePositionReturnDate);

		ident_to_elem.insert("SafePositionReturnTime", &tags::SafePositionReturnTime);
		tag_to_elem.insert(0x30080168, &tags::SafePositionReturnTime);

		ident_to_elem.insert("PulseSpecificBrachyControlPointDeliveredSequence", &tags::PulseSpecificBrachyControlPointDeliveredSequence);
		tag_to_elem.insert(0x30080171, &tags::PulseSpecificBrachyControlPointDeliveredSequence);

		ident_to_elem.insert("PulseNumber", &tags::PulseNumber);
		tag_to_elem.insert(0x30080172, &tags::PulseNumber);

		ident_to_elem.insert("BrachyPulseControlPointDeliveredSequence", &tags::BrachyPulseControlPointDeliveredSequence);
		tag_to_elem.insert(0x30080173, &tags::BrachyPulseControlPointDeliveredSequence);

		ident_to_elem.insert("CurrentTreatmentStatus", &tags::CurrentTreatmentStatus);
		tag_to_elem.insert(0x30080200, &tags::CurrentTreatmentStatus);

		ident_to_elem.insert("TreatmentStatusComment", &tags::TreatmentStatusComment);
		tag_to_elem.insert(0x30080202, &tags::TreatmentStatusComment);

		ident_to_elem.insert("FractionGroupSummarySequence", &tags::FractionGroupSummarySequence);
		tag_to_elem.insert(0x30080220, &tags::FractionGroupSummarySequence);

		ident_to_elem.insert("ReferencedFractionNumber", &tags::ReferencedFractionNumber);
		tag_to_elem.insert(0x30080223, &tags::ReferencedFractionNumber);

		ident_to_elem.insert("FractionGroupType", &tags::FractionGroupType);
		tag_to_elem.insert(0x30080224, &tags::FractionGroupType);

		ident_to_elem.insert("BeamStopperPosition", &tags::BeamStopperPosition);
		tag_to_elem.insert(0x30080230, &tags::BeamStopperPosition);

		ident_to_elem.insert("FractionStatusSummarySequence", &tags::FractionStatusSummarySequence);
		tag_to_elem.insert(0x30080240, &tags::FractionStatusSummarySequence);

		ident_to_elem.insert("TreatmentDate", &tags::TreatmentDate);
		tag_to_elem.insert(0x30080250, &tags::TreatmentDate);

		ident_to_elem.insert("TreatmentTime", &tags::TreatmentTime);
		tag_to_elem.insert(0x30080251, &tags::TreatmentTime);

		ident_to_elem.insert("RTPlanLabel", &tags::RTPlanLabel);
		tag_to_elem.insert(0x300A0002, &tags::RTPlanLabel);

		ident_to_elem.insert("RTPlanName", &tags::RTPlanName);
		tag_to_elem.insert(0x300A0003, &tags::RTPlanName);

		ident_to_elem.insert("RTPlanDescription", &tags::RTPlanDescription);
		tag_to_elem.insert(0x300A0004, &tags::RTPlanDescription);

		ident_to_elem.insert("RTPlanDate", &tags::RTPlanDate);
		tag_to_elem.insert(0x300A0006, &tags::RTPlanDate);

		ident_to_elem.insert("RTPlanTime", &tags::RTPlanTime);
		tag_to_elem.insert(0x300A0007, &tags::RTPlanTime);

		ident_to_elem.insert("TreatmentProtocols", &tags::TreatmentProtocols);
		tag_to_elem.insert(0x300A0009, &tags::TreatmentProtocols);

		ident_to_elem.insert("PlanIntent", &tags::PlanIntent);
		tag_to_elem.insert(0x300A000A, &tags::PlanIntent);

		ident_to_elem.insert("TreatmentSites", &tags::TreatmentSites);
		tag_to_elem.insert(0x300A000B, &tags::TreatmentSites);

		ident_to_elem.insert("RTPlanGeometry", &tags::RTPlanGeometry);
		tag_to_elem.insert(0x300A000C, &tags::RTPlanGeometry);

		ident_to_elem.insert("PrescriptionDescription", &tags::PrescriptionDescription);
		tag_to_elem.insert(0x300A000E, &tags::PrescriptionDescription);

		ident_to_elem.insert("DoseReferenceSequence", &tags::DoseReferenceSequence);
		tag_to_elem.insert(0x300A0010, &tags::DoseReferenceSequence);

		ident_to_elem.insert("DoseReferenceNumber", &tags::DoseReferenceNumber);
		tag_to_elem.insert(0x300A0012, &tags::DoseReferenceNumber);

		ident_to_elem.insert("DoseReferenceUID", &tags::DoseReferenceUID);
		tag_to_elem.insert(0x300A0013, &tags::DoseReferenceUID);

		ident_to_elem.insert("DoseReferenceStructureType", &tags::DoseReferenceStructureType);
		tag_to_elem.insert(0x300A0014, &tags::DoseReferenceStructureType);

		ident_to_elem.insert("NominalBeamEnergyUnit", &tags::NominalBeamEnergyUnit);
		tag_to_elem.insert(0x300A0015, &tags::NominalBeamEnergyUnit);

		ident_to_elem.insert("DoseReferenceDescription", &tags::DoseReferenceDescription);
		tag_to_elem.insert(0x300A0016, &tags::DoseReferenceDescription);

		ident_to_elem.insert("DoseReferencePointCoordinates", &tags::DoseReferencePointCoordinates);
		tag_to_elem.insert(0x300A0018, &tags::DoseReferencePointCoordinates);

		ident_to_elem.insert("NominalPriorDose", &tags::NominalPriorDose);
		tag_to_elem.insert(0x300A001A, &tags::NominalPriorDose);

		ident_to_elem.insert("DoseReferenceType", &tags::DoseReferenceType);
		tag_to_elem.insert(0x300A0020, &tags::DoseReferenceType);

		ident_to_elem.insert("ConstraintWeight", &tags::ConstraintWeight);
		tag_to_elem.insert(0x300A0021, &tags::ConstraintWeight);

		ident_to_elem.insert("DeliveryWarningDose", &tags::DeliveryWarningDose);
		tag_to_elem.insert(0x300A0022, &tags::DeliveryWarningDose);

		ident_to_elem.insert("DeliveryMaximumDose", &tags::DeliveryMaximumDose);
		tag_to_elem.insert(0x300A0023, &tags::DeliveryMaximumDose);

		ident_to_elem.insert("TargetMinimumDose", &tags::TargetMinimumDose);
		tag_to_elem.insert(0x300A0025, &tags::TargetMinimumDose);

		ident_to_elem.insert("TargetPrescriptionDose", &tags::TargetPrescriptionDose);
		tag_to_elem.insert(0x300A0026, &tags::TargetPrescriptionDose);

		ident_to_elem.insert("TargetMaximumDose", &tags::TargetMaximumDose);
		tag_to_elem.insert(0x300A0027, &tags::TargetMaximumDose);

		ident_to_elem.insert("TargetUnderdoseVolumeFraction", &tags::TargetUnderdoseVolumeFraction);
		tag_to_elem.insert(0x300A0028, &tags::TargetUnderdoseVolumeFraction);

		ident_to_elem.insert("OrganAtRiskFullVolumeDose", &tags::OrganAtRiskFullVolumeDose);
		tag_to_elem.insert(0x300A002A, &tags::OrganAtRiskFullVolumeDose);

		ident_to_elem.insert("OrganAtRiskLimitDose", &tags::OrganAtRiskLimitDose);
		tag_to_elem.insert(0x300A002B, &tags::OrganAtRiskLimitDose);

		ident_to_elem.insert("OrganAtRiskMaximumDose", &tags::OrganAtRiskMaximumDose);
		tag_to_elem.insert(0x300A002C, &tags::OrganAtRiskMaximumDose);

		ident_to_elem.insert("OrganAtRiskOverdoseVolumeFraction", &tags::OrganAtRiskOverdoseVolumeFraction);
		tag_to_elem.insert(0x300A002D, &tags::OrganAtRiskOverdoseVolumeFraction);

		ident_to_elem.insert("ToleranceTableSequence", &tags::ToleranceTableSequence);
		tag_to_elem.insert(0x300A0040, &tags::ToleranceTableSequence);

		ident_to_elem.insert("ToleranceTableNumber", &tags::ToleranceTableNumber);
		tag_to_elem.insert(0x300A0042, &tags::ToleranceTableNumber);

		ident_to_elem.insert("ToleranceTableLabel", &tags::ToleranceTableLabel);
		tag_to_elem.insert(0x300A0043, &tags::ToleranceTableLabel);

		ident_to_elem.insert("GantryAngleTolerance", &tags::GantryAngleTolerance);
		tag_to_elem.insert(0x300A0044, &tags::GantryAngleTolerance);

		ident_to_elem.insert("BeamLimitingDeviceAngleTolerance", &tags::BeamLimitingDeviceAngleTolerance);
		tag_to_elem.insert(0x300A0046, &tags::BeamLimitingDeviceAngleTolerance);

		ident_to_elem.insert("BeamLimitingDeviceToleranceSequence", &tags::BeamLimitingDeviceToleranceSequence);
		tag_to_elem.insert(0x300A0048, &tags::BeamLimitingDeviceToleranceSequence);

		ident_to_elem.insert("BeamLimitingDevicePositionTolerance", &tags::BeamLimitingDevicePositionTolerance);
		tag_to_elem.insert(0x300A004A, &tags::BeamLimitingDevicePositionTolerance);

		ident_to_elem.insert("SnoutPositionTolerance", &tags::SnoutPositionTolerance);
		tag_to_elem.insert(0x300A004B, &tags::SnoutPositionTolerance);

		ident_to_elem.insert("PatientSupportAngleTolerance", &tags::PatientSupportAngleTolerance);
		tag_to_elem.insert(0x300A004C, &tags::PatientSupportAngleTolerance);

		ident_to_elem.insert("TableTopEccentricAngleTolerance", &tags::TableTopEccentricAngleTolerance);
		tag_to_elem.insert(0x300A004E, &tags::TableTopEccentricAngleTolerance);

		ident_to_elem.insert("TableTopPitchAngleTolerance", &tags::TableTopPitchAngleTolerance);
		tag_to_elem.insert(0x300A004F, &tags::TableTopPitchAngleTolerance);

		ident_to_elem.insert("TableTopRollAngleTolerance", &tags::TableTopRollAngleTolerance);
		tag_to_elem.insert(0x300A0050, &tags::TableTopRollAngleTolerance);

		ident_to_elem.insert("TableTopVerticalPositionTolerance", &tags::TableTopVerticalPositionTolerance);
		tag_to_elem.insert(0x300A0051, &tags::TableTopVerticalPositionTolerance);

		ident_to_elem.insert("TableTopLongitudinalPositionTolerance", &tags::TableTopLongitudinalPositionTolerance);
		tag_to_elem.insert(0x300A0052, &tags::TableTopLongitudinalPositionTolerance);

		ident_to_elem.insert("TableTopLateralPositionTolerance", &tags::TableTopLateralPositionTolerance);
		tag_to_elem.insert(0x300A0053, &tags::TableTopLateralPositionTolerance);

		ident_to_elem.insert("RTPlanRelationship", &tags::RTPlanRelationship);
		tag_to_elem.insert(0x300A0055, &tags::RTPlanRelationship);

		ident_to_elem.insert("FractionGroupSequence", &tags::FractionGroupSequence);
		tag_to_elem.insert(0x300A0070, &tags::FractionGroupSequence);

		ident_to_elem.insert("FractionGroupNumber", &tags::FractionGroupNumber);
		tag_to_elem.insert(0x300A0071, &tags::FractionGroupNumber);

		ident_to_elem.insert("FractionGroupDescription", &tags::FractionGroupDescription);
		tag_to_elem.insert(0x300A0072, &tags::FractionGroupDescription);

		ident_to_elem.insert("NumberOfFractionsPlanned", &tags::NumberOfFractionsPlanned);
		tag_to_elem.insert(0x300A0078, &tags::NumberOfFractionsPlanned);

		ident_to_elem.insert("NumberOfFractionPatternDigitsPerDay", &tags::NumberOfFractionPatternDigitsPerDay);
		tag_to_elem.insert(0x300A0079, &tags::NumberOfFractionPatternDigitsPerDay);

		ident_to_elem.insert("RepeatFractionCycleLength", &tags::RepeatFractionCycleLength);
		tag_to_elem.insert(0x300A007A, &tags::RepeatFractionCycleLength);

		ident_to_elem.insert("FractionPattern", &tags::FractionPattern);
		tag_to_elem.insert(0x300A007B, &tags::FractionPattern);

		ident_to_elem.insert("NumberOfBeams", &tags::NumberOfBeams);
		tag_to_elem.insert(0x300A0080, &tags::NumberOfBeams);

		ident_to_elem.insert("BeamDoseSpecificationPoint", &tags::BeamDoseSpecificationPoint);
		tag_to_elem.insert(0x300A0082, &tags::BeamDoseSpecificationPoint);

		ident_to_elem.insert("BeamDose", &tags::BeamDose);
		tag_to_elem.insert(0x300A0084, &tags::BeamDose);

		ident_to_elem.insert("BeamMeterset", &tags::BeamMeterset);
		tag_to_elem.insert(0x300A0086, &tags::BeamMeterset);

		ident_to_elem.insert("BeamDosePointDepth", &tags::BeamDosePointDepth);
		tag_to_elem.insert(0x300A0088, &tags::BeamDosePointDepth);

		ident_to_elem.insert("BeamDosePointEquivalentDepth", &tags::BeamDosePointEquivalentDepth);
		tag_to_elem.insert(0x300A0089, &tags::BeamDosePointEquivalentDepth);

		ident_to_elem.insert("BeamDosePointSSD", &tags::BeamDosePointSSD);
		tag_to_elem.insert(0x300A008A, &tags::BeamDosePointSSD);

		ident_to_elem.insert("BeamDoseMeaning", &tags::BeamDoseMeaning);
		tag_to_elem.insert(0x300A008B, &tags::BeamDoseMeaning);

		ident_to_elem.insert("BeamDoseVerificationControlPointSequence", &tags::BeamDoseVerificationControlPointSequence);
		tag_to_elem.insert(0x300A008C, &tags::BeamDoseVerificationControlPointSequence);

		ident_to_elem.insert("AverageBeamDosePointDepth", &tags::AverageBeamDosePointDepth);
		tag_to_elem.insert(0x300A008D, &tags::AverageBeamDosePointDepth);

		ident_to_elem.insert("AverageBeamDosePointEquivalentDepth", &tags::AverageBeamDosePointEquivalentDepth);
		tag_to_elem.insert(0x300A008E, &tags::AverageBeamDosePointEquivalentDepth);

		ident_to_elem.insert("AverageBeamDosePointSSD", &tags::AverageBeamDosePointSSD);
		tag_to_elem.insert(0x300A008F, &tags::AverageBeamDosePointSSD);

		ident_to_elem.insert("BeamDoseType", &tags::BeamDoseType);
		tag_to_elem.insert(0x300A0090, &tags::BeamDoseType);

		ident_to_elem.insert("AlternateBeamDose", &tags::AlternateBeamDose);
		tag_to_elem.insert(0x300A0091, &tags::AlternateBeamDose);

		ident_to_elem.insert("AlternateBeamDoseType", &tags::AlternateBeamDoseType);
		tag_to_elem.insert(0x300A0092, &tags::AlternateBeamDoseType);

		ident_to_elem.insert("NumberOfBrachyApplicationSetups", &tags::NumberOfBrachyApplicationSetups);
		tag_to_elem.insert(0x300A00A0, &tags::NumberOfBrachyApplicationSetups);

		ident_to_elem.insert("BrachyApplicationSetupDoseSpecificationPoint", &tags::BrachyApplicationSetupDoseSpecificationPoint);
		tag_to_elem.insert(0x300A00A2, &tags::BrachyApplicationSetupDoseSpecificationPoint);

		ident_to_elem.insert("BrachyApplicationSetupDose", &tags::BrachyApplicationSetupDose);
		tag_to_elem.insert(0x300A00A4, &tags::BrachyApplicationSetupDose);

		ident_to_elem.insert("BeamSequence", &tags::BeamSequence);
		tag_to_elem.insert(0x300A00B0, &tags::BeamSequence);

		ident_to_elem.insert("TreatmentMachineName", &tags::TreatmentMachineName);
		tag_to_elem.insert(0x300A00B2, &tags::TreatmentMachineName);

		ident_to_elem.insert("PrimaryDosimeterUnit", &tags::PrimaryDosimeterUnit);
		tag_to_elem.insert(0x300A00B3, &tags::PrimaryDosimeterUnit);

		ident_to_elem.insert("SourceAxisDistance", &tags::SourceAxisDistance);
		tag_to_elem.insert(0x300A00B4, &tags::SourceAxisDistance);

		ident_to_elem.insert("BeamLimitingDeviceSequence", &tags::BeamLimitingDeviceSequence);
		tag_to_elem.insert(0x300A00B6, &tags::BeamLimitingDeviceSequence);

		ident_to_elem.insert("RTBeamLimitingDeviceType", &tags::RTBeamLimitingDeviceType);
		tag_to_elem.insert(0x300A00B8, &tags::RTBeamLimitingDeviceType);

		ident_to_elem.insert("SourceToBeamLimitingDeviceDistance", &tags::SourceToBeamLimitingDeviceDistance);
		tag_to_elem.insert(0x300A00BA, &tags::SourceToBeamLimitingDeviceDistance);

		ident_to_elem.insert("IsocenterToBeamLimitingDeviceDistance", &tags::IsocenterToBeamLimitingDeviceDistance);
		tag_to_elem.insert(0x300A00BB, &tags::IsocenterToBeamLimitingDeviceDistance);

		ident_to_elem.insert("NumberOfLeafJawPairs", &tags::NumberOfLeafJawPairs);
		tag_to_elem.insert(0x300A00BC, &tags::NumberOfLeafJawPairs);

		ident_to_elem.insert("LeafPositionBoundaries", &tags::LeafPositionBoundaries);
		tag_to_elem.insert(0x300A00BE, &tags::LeafPositionBoundaries);

		ident_to_elem.insert("BeamNumber", &tags::BeamNumber);
		tag_to_elem.insert(0x300A00C0, &tags::BeamNumber);

		ident_to_elem.insert("BeamName", &tags::BeamName);
		tag_to_elem.insert(0x300A00C2, &tags::BeamName);

		ident_to_elem.insert("BeamDescription", &tags::BeamDescription);
		tag_to_elem.insert(0x300A00C3, &tags::BeamDescription);

		ident_to_elem.insert("BeamType", &tags::BeamType);
		tag_to_elem.insert(0x300A00C4, &tags::BeamType);

		ident_to_elem.insert("BeamDeliveryDurationLimit", &tags::BeamDeliveryDurationLimit);
		tag_to_elem.insert(0x300A00C5, &tags::BeamDeliveryDurationLimit);

		ident_to_elem.insert("RadiationType", &tags::RadiationType);
		tag_to_elem.insert(0x300A00C6, &tags::RadiationType);

		ident_to_elem.insert("HighDoseTechniqueType", &tags::HighDoseTechniqueType);
		tag_to_elem.insert(0x300A00C7, &tags::HighDoseTechniqueType);

		ident_to_elem.insert("ReferenceImageNumber", &tags::ReferenceImageNumber);
		tag_to_elem.insert(0x300A00C8, &tags::ReferenceImageNumber);

		ident_to_elem.insert("PlannedVerificationImageSequence", &tags::PlannedVerificationImageSequence);
		tag_to_elem.insert(0x300A00CA, &tags::PlannedVerificationImageSequence);

		ident_to_elem.insert("ImagingDeviceSpecificAcquisitionParameters", &tags::ImagingDeviceSpecificAcquisitionParameters);
		tag_to_elem.insert(0x300A00CC, &tags::ImagingDeviceSpecificAcquisitionParameters);

		ident_to_elem.insert("TreatmentDeliveryType", &tags::TreatmentDeliveryType);
		tag_to_elem.insert(0x300A00CE, &tags::TreatmentDeliveryType);

		ident_to_elem.insert("NumberOfWedges", &tags::NumberOfWedges);
		tag_to_elem.insert(0x300A00D0, &tags::NumberOfWedges);

		ident_to_elem.insert("WedgeSequence", &tags::WedgeSequence);
		tag_to_elem.insert(0x300A00D1, &tags::WedgeSequence);

		ident_to_elem.insert("WedgeNumber", &tags::WedgeNumber);
		tag_to_elem.insert(0x300A00D2, &tags::WedgeNumber);

		ident_to_elem.insert("WedgeType", &tags::WedgeType);
		tag_to_elem.insert(0x300A00D3, &tags::WedgeType);

		ident_to_elem.insert("WedgeID", &tags::WedgeID);
		tag_to_elem.insert(0x300A00D4, &tags::WedgeID);

		ident_to_elem.insert("WedgeAngle", &tags::WedgeAngle);
		tag_to_elem.insert(0x300A00D5, &tags::WedgeAngle);

		ident_to_elem.insert("WedgeFactor", &tags::WedgeFactor);
		tag_to_elem.insert(0x300A00D6, &tags::WedgeFactor);

		ident_to_elem.insert("TotalWedgeTrayWaterEquivalentThickness", &tags::TotalWedgeTrayWaterEquivalentThickness);
		tag_to_elem.insert(0x300A00D7, &tags::TotalWedgeTrayWaterEquivalentThickness);

		ident_to_elem.insert("WedgeOrientation", &tags::WedgeOrientation);
		tag_to_elem.insert(0x300A00D8, &tags::WedgeOrientation);

		ident_to_elem.insert("IsocenterToWedgeTrayDistance", &tags::IsocenterToWedgeTrayDistance);
		tag_to_elem.insert(0x300A00D9, &tags::IsocenterToWedgeTrayDistance);

		ident_to_elem.insert("SourceToWedgeTrayDistance", &tags::SourceToWedgeTrayDistance);
		tag_to_elem.insert(0x300A00DA, &tags::SourceToWedgeTrayDistance);

		ident_to_elem.insert("WedgeThinEdgePosition", &tags::WedgeThinEdgePosition);
		tag_to_elem.insert(0x300A00DB, &tags::WedgeThinEdgePosition);

		ident_to_elem.insert("BolusID", &tags::BolusID);
		tag_to_elem.insert(0x300A00DC, &tags::BolusID);

		ident_to_elem.insert("BolusDescription", &tags::BolusDescription);
		tag_to_elem.insert(0x300A00DD, &tags::BolusDescription);

		ident_to_elem.insert("EffectiveWedgeAngle", &tags::EffectiveWedgeAngle);
		tag_to_elem.insert(0x300A00DE, &tags::EffectiveWedgeAngle);

		ident_to_elem.insert("NumberOfCompensators", &tags::NumberOfCompensators);
		tag_to_elem.insert(0x300A00E0, &tags::NumberOfCompensators);

		ident_to_elem.insert("MaterialID", &tags::MaterialID);
		tag_to_elem.insert(0x300A00E1, &tags::MaterialID);

		ident_to_elem.insert("TotalCompensatorTrayFactor", &tags::TotalCompensatorTrayFactor);
		tag_to_elem.insert(0x300A00E2, &tags::TotalCompensatorTrayFactor);

		ident_to_elem.insert("CompensatorSequence", &tags::CompensatorSequence);
		tag_to_elem.insert(0x300A00E3, &tags::CompensatorSequence);

		ident_to_elem.insert("CompensatorNumber", &tags::CompensatorNumber);
		tag_to_elem.insert(0x300A00E4, &tags::CompensatorNumber);

		ident_to_elem.insert("CompensatorID", &tags::CompensatorID);
		tag_to_elem.insert(0x300A00E5, &tags::CompensatorID);

		ident_to_elem.insert("SourceToCompensatorTrayDistance", &tags::SourceToCompensatorTrayDistance);
		tag_to_elem.insert(0x300A00E6, &tags::SourceToCompensatorTrayDistance);

		ident_to_elem.insert("CompensatorRows", &tags::CompensatorRows);
		tag_to_elem.insert(0x300A00E7, &tags::CompensatorRows);

		ident_to_elem.insert("CompensatorColumns", &tags::CompensatorColumns);
		tag_to_elem.insert(0x300A00E8, &tags::CompensatorColumns);

		ident_to_elem.insert("CompensatorPixelSpacing", &tags::CompensatorPixelSpacing);
		tag_to_elem.insert(0x300A00E9, &tags::CompensatorPixelSpacing);

		ident_to_elem.insert("CompensatorPosition", &tags::CompensatorPosition);
		tag_to_elem.insert(0x300A00EA, &tags::CompensatorPosition);

		ident_to_elem.insert("CompensatorTransmissionData", &tags::CompensatorTransmissionData);
		tag_to_elem.insert(0x300A00EB, &tags::CompensatorTransmissionData);

		ident_to_elem.insert("CompensatorThicknessData", &tags::CompensatorThicknessData);
		tag_to_elem.insert(0x300A00EC, &tags::CompensatorThicknessData);

		ident_to_elem.insert("NumberOfBoli", &tags::NumberOfBoli);
		tag_to_elem.insert(0x300A00ED, &tags::NumberOfBoli);

		ident_to_elem.insert("CompensatorType", &tags::CompensatorType);
		tag_to_elem.insert(0x300A00EE, &tags::CompensatorType);

		ident_to_elem.insert("CompensatorTrayID", &tags::CompensatorTrayID);
		tag_to_elem.insert(0x300A00EF, &tags::CompensatorTrayID);

		ident_to_elem.insert("NumberOfBlocks", &tags::NumberOfBlocks);
		tag_to_elem.insert(0x300A00F0, &tags::NumberOfBlocks);

		ident_to_elem.insert("TotalBlockTrayFactor", &tags::TotalBlockTrayFactor);
		tag_to_elem.insert(0x300A00F2, &tags::TotalBlockTrayFactor);

		ident_to_elem.insert("TotalBlockTrayWaterEquivalentThickness", &tags::TotalBlockTrayWaterEquivalentThickness);
		tag_to_elem.insert(0x300A00F3, &tags::TotalBlockTrayWaterEquivalentThickness);

		ident_to_elem.insert("BlockSequence", &tags::BlockSequence);
		tag_to_elem.insert(0x300A00F4, &tags::BlockSequence);

		ident_to_elem.insert("BlockTrayID", &tags::BlockTrayID);
		tag_to_elem.insert(0x300A00F5, &tags::BlockTrayID);

		ident_to_elem.insert("SourceToBlockTrayDistance", &tags::SourceToBlockTrayDistance);
		tag_to_elem.insert(0x300A00F6, &tags::SourceToBlockTrayDistance);

		ident_to_elem.insert("IsocenterToBlockTrayDistance", &tags::IsocenterToBlockTrayDistance);
		tag_to_elem.insert(0x300A00F7, &tags::IsocenterToBlockTrayDistance);

		ident_to_elem.insert("BlockType", &tags::BlockType);
		tag_to_elem.insert(0x300A00F8, &tags::BlockType);

		ident_to_elem.insert("AccessoryCode", &tags::AccessoryCode);
		tag_to_elem.insert(0x300A00F9, &tags::AccessoryCode);

		ident_to_elem.insert("BlockDivergence", &tags::BlockDivergence);
		tag_to_elem.insert(0x300A00FA, &tags::BlockDivergence);

		ident_to_elem.insert("BlockMountingPosition", &tags::BlockMountingPosition);
		tag_to_elem.insert(0x300A00FB, &tags::BlockMountingPosition);

		ident_to_elem.insert("BlockNumber", &tags::BlockNumber);
		tag_to_elem.insert(0x300A00FC, &tags::BlockNumber);

		ident_to_elem.insert("BlockName", &tags::BlockName);
		tag_to_elem.insert(0x300A00FE, &tags::BlockName);

		ident_to_elem.insert("BlockThickness", &tags::BlockThickness);
		tag_to_elem.insert(0x300A0100, &tags::BlockThickness);

		ident_to_elem.insert("BlockTransmission", &tags::BlockTransmission);
		tag_to_elem.insert(0x300A0102, &tags::BlockTransmission);

		ident_to_elem.insert("BlockNumberOfPoints", &tags::BlockNumberOfPoints);
		tag_to_elem.insert(0x300A0104, &tags::BlockNumberOfPoints);

		ident_to_elem.insert("BlockData", &tags::BlockData);
		tag_to_elem.insert(0x300A0106, &tags::BlockData);

		ident_to_elem.insert("ApplicatorSequence", &tags::ApplicatorSequence);
		tag_to_elem.insert(0x300A0107, &tags::ApplicatorSequence);

		ident_to_elem.insert("ApplicatorID", &tags::ApplicatorID);
		tag_to_elem.insert(0x300A0108, &tags::ApplicatorID);

		ident_to_elem.insert("ApplicatorType", &tags::ApplicatorType);
		tag_to_elem.insert(0x300A0109, &tags::ApplicatorType);

		ident_to_elem.insert("ApplicatorDescription", &tags::ApplicatorDescription);
		tag_to_elem.insert(0x300A010A, &tags::ApplicatorDescription);

		ident_to_elem.insert("CumulativeDoseReferenceCoefficient", &tags::CumulativeDoseReferenceCoefficient);
		tag_to_elem.insert(0x300A010C, &tags::CumulativeDoseReferenceCoefficient);

		ident_to_elem.insert("FinalCumulativeMetersetWeight", &tags::FinalCumulativeMetersetWeight);
		tag_to_elem.insert(0x300A010E, &tags::FinalCumulativeMetersetWeight);

		ident_to_elem.insert("NumberOfControlPoints", &tags::NumberOfControlPoints);
		tag_to_elem.insert(0x300A0110, &tags::NumberOfControlPoints);

		ident_to_elem.insert("ControlPointSequence", &tags::ControlPointSequence);
		tag_to_elem.insert(0x300A0111, &tags::ControlPointSequence);

		ident_to_elem.insert("ControlPointIndex", &tags::ControlPointIndex);
		tag_to_elem.insert(0x300A0112, &tags::ControlPointIndex);

		ident_to_elem.insert("NominalBeamEnergy", &tags::NominalBeamEnergy);
		tag_to_elem.insert(0x300A0114, &tags::NominalBeamEnergy);

		ident_to_elem.insert("DoseRateSet", &tags::DoseRateSet);
		tag_to_elem.insert(0x300A0115, &tags::DoseRateSet);

		ident_to_elem.insert("WedgePositionSequence", &tags::WedgePositionSequence);
		tag_to_elem.insert(0x300A0116, &tags::WedgePositionSequence);

		ident_to_elem.insert("WedgePosition", &tags::WedgePosition);
		tag_to_elem.insert(0x300A0118, &tags::WedgePosition);

		ident_to_elem.insert("BeamLimitingDevicePositionSequence", &tags::BeamLimitingDevicePositionSequence);
		tag_to_elem.insert(0x300A011A, &tags::BeamLimitingDevicePositionSequence);

		ident_to_elem.insert("LeafJawPositions", &tags::LeafJawPositions);
		tag_to_elem.insert(0x300A011C, &tags::LeafJawPositions);

		ident_to_elem.insert("GantryAngle", &tags::GantryAngle);
		tag_to_elem.insert(0x300A011E, &tags::GantryAngle);

		ident_to_elem.insert("GantryRotationDirection", &tags::GantryRotationDirection);
		tag_to_elem.insert(0x300A011F, &tags::GantryRotationDirection);

		ident_to_elem.insert("BeamLimitingDeviceAngle", &tags::BeamLimitingDeviceAngle);
		tag_to_elem.insert(0x300A0120, &tags::BeamLimitingDeviceAngle);

		ident_to_elem.insert("BeamLimitingDeviceRotationDirection", &tags::BeamLimitingDeviceRotationDirection);
		tag_to_elem.insert(0x300A0121, &tags::BeamLimitingDeviceRotationDirection);

		ident_to_elem.insert("PatientSupportAngle", &tags::PatientSupportAngle);
		tag_to_elem.insert(0x300A0122, &tags::PatientSupportAngle);

		ident_to_elem.insert("PatientSupportRotationDirection", &tags::PatientSupportRotationDirection);
		tag_to_elem.insert(0x300A0123, &tags::PatientSupportRotationDirection);

		ident_to_elem.insert("TableTopEccentricAxisDistance", &tags::TableTopEccentricAxisDistance);
		tag_to_elem.insert(0x300A0124, &tags::TableTopEccentricAxisDistance);

		ident_to_elem.insert("TableTopEccentricAngle", &tags::TableTopEccentricAngle);
		tag_to_elem.insert(0x300A0125, &tags::TableTopEccentricAngle);

		ident_to_elem.insert("TableTopEccentricRotationDirection", &tags::TableTopEccentricRotationDirection);
		tag_to_elem.insert(0x300A0126, &tags::TableTopEccentricRotationDirection);

		ident_to_elem.insert("TableTopVerticalPosition", &tags::TableTopVerticalPosition);
		tag_to_elem.insert(0x300A0128, &tags::TableTopVerticalPosition);

		ident_to_elem.insert("TableTopLongitudinalPosition", &tags::TableTopLongitudinalPosition);
		tag_to_elem.insert(0x300A0129, &tags::TableTopLongitudinalPosition);

		ident_to_elem.insert("TableTopLateralPosition", &tags::TableTopLateralPosition);
		tag_to_elem.insert(0x300A012A, &tags::TableTopLateralPosition);

		ident_to_elem.insert("IsocenterPosition", &tags::IsocenterPosition);
		tag_to_elem.insert(0x300A012C, &tags::IsocenterPosition);

		ident_to_elem.insert("SurfaceEntryPoint", &tags::SurfaceEntryPoint);
		tag_to_elem.insert(0x300A012E, &tags::SurfaceEntryPoint);

		ident_to_elem.insert("SourceToSurfaceDistance", &tags::SourceToSurfaceDistance);
		tag_to_elem.insert(0x300A0130, &tags::SourceToSurfaceDistance);

		ident_to_elem.insert("AverageBeamDosePointSourceToExternalContourDistance", &tags::AverageBeamDosePointSourceToExternalContourDistance);
		tag_to_elem.insert(0x300A0131, &tags::AverageBeamDosePointSourceToExternalContourDistance);

		ident_to_elem.insert("SourceToExternalContourDistance", &tags::SourceToExternalContourDistance);
		tag_to_elem.insert(0x300A0132, &tags::SourceToExternalContourDistance);

		ident_to_elem.insert("ExternalContourEntryPoint", &tags::ExternalContourEntryPoint);
		tag_to_elem.insert(0x300A0133, &tags::ExternalContourEntryPoint);

		ident_to_elem.insert("CumulativeMetersetWeight", &tags::CumulativeMetersetWeight);
		tag_to_elem.insert(0x300A0134, &tags::CumulativeMetersetWeight);

		ident_to_elem.insert("TableTopPitchAngle", &tags::TableTopPitchAngle);
		tag_to_elem.insert(0x300A0140, &tags::TableTopPitchAngle);

		ident_to_elem.insert("TableTopPitchRotationDirection", &tags::TableTopPitchRotationDirection);
		tag_to_elem.insert(0x300A0142, &tags::TableTopPitchRotationDirection);

		ident_to_elem.insert("TableTopRollAngle", &tags::TableTopRollAngle);
		tag_to_elem.insert(0x300A0144, &tags::TableTopRollAngle);

		ident_to_elem.insert("TableTopRollRotationDirection", &tags::TableTopRollRotationDirection);
		tag_to_elem.insert(0x300A0146, &tags::TableTopRollRotationDirection);

		ident_to_elem.insert("HeadFixationAngle", &tags::HeadFixationAngle);
		tag_to_elem.insert(0x300A0148, &tags::HeadFixationAngle);

		ident_to_elem.insert("GantryPitchAngle", &tags::GantryPitchAngle);
		tag_to_elem.insert(0x300A014A, &tags::GantryPitchAngle);

		ident_to_elem.insert("GantryPitchRotationDirection", &tags::GantryPitchRotationDirection);
		tag_to_elem.insert(0x300A014C, &tags::GantryPitchRotationDirection);

		ident_to_elem.insert("GantryPitchAngleTolerance", &tags::GantryPitchAngleTolerance);
		tag_to_elem.insert(0x300A014E, &tags::GantryPitchAngleTolerance);

		ident_to_elem.insert("FixationEye", &tags::FixationEye);
		tag_to_elem.insert(0x300A0150, &tags::FixationEye);

		ident_to_elem.insert("ChairHeadFramePosition", &tags::ChairHeadFramePosition);
		tag_to_elem.insert(0x300A0151, &tags::ChairHeadFramePosition);

		ident_to_elem.insert("HeadFixationAngleTolerance", &tags::HeadFixationAngleTolerance);
		tag_to_elem.insert(0x300A0152, &tags::HeadFixationAngleTolerance);

		ident_to_elem.insert("ChairHeadFramePositionTolerance", &tags::ChairHeadFramePositionTolerance);
		tag_to_elem.insert(0x300A0153, &tags::ChairHeadFramePositionTolerance);

		ident_to_elem.insert("FixationLightAzimuthalAngleTolerance", &tags::FixationLightAzimuthalAngleTolerance);
		tag_to_elem.insert(0x300A0154, &tags::FixationLightAzimuthalAngleTolerance);

		ident_to_elem.insert("FixationLightPolarAngleTolerance", &tags::FixationLightPolarAngleTolerance);
		tag_to_elem.insert(0x300A0155, &tags::FixationLightPolarAngleTolerance);

		ident_to_elem.insert("PatientSetupSequence", &tags::PatientSetupSequence);
		tag_to_elem.insert(0x300A0180, &tags::PatientSetupSequence);

		ident_to_elem.insert("PatientSetupNumber", &tags::PatientSetupNumber);
		tag_to_elem.insert(0x300A0182, &tags::PatientSetupNumber);

		ident_to_elem.insert("PatientSetupLabel", &tags::PatientSetupLabel);
		tag_to_elem.insert(0x300A0183, &tags::PatientSetupLabel);

		ident_to_elem.insert("PatientAdditionalPosition", &tags::PatientAdditionalPosition);
		tag_to_elem.insert(0x300A0184, &tags::PatientAdditionalPosition);

		ident_to_elem.insert("FixationDeviceSequence", &tags::FixationDeviceSequence);
		tag_to_elem.insert(0x300A0190, &tags::FixationDeviceSequence);

		ident_to_elem.insert("FixationDeviceType", &tags::FixationDeviceType);
		tag_to_elem.insert(0x300A0192, &tags::FixationDeviceType);

		ident_to_elem.insert("FixationDeviceLabel", &tags::FixationDeviceLabel);
		tag_to_elem.insert(0x300A0194, &tags::FixationDeviceLabel);

		ident_to_elem.insert("FixationDeviceDescription", &tags::FixationDeviceDescription);
		tag_to_elem.insert(0x300A0196, &tags::FixationDeviceDescription);

		ident_to_elem.insert("FixationDevicePosition", &tags::FixationDevicePosition);
		tag_to_elem.insert(0x300A0198, &tags::FixationDevicePosition);

		ident_to_elem.insert("FixationDevicePitchAngle", &tags::FixationDevicePitchAngle);
		tag_to_elem.insert(0x300A0199, &tags::FixationDevicePitchAngle);

		ident_to_elem.insert("FixationDeviceRollAngle", &tags::FixationDeviceRollAngle);
		tag_to_elem.insert(0x300A019A, &tags::FixationDeviceRollAngle);

		ident_to_elem.insert("ShieldingDeviceSequence", &tags::ShieldingDeviceSequence);
		tag_to_elem.insert(0x300A01A0, &tags::ShieldingDeviceSequence);

		ident_to_elem.insert("ShieldingDeviceType", &tags::ShieldingDeviceType);
		tag_to_elem.insert(0x300A01A2, &tags::ShieldingDeviceType);

		ident_to_elem.insert("ShieldingDeviceLabel", &tags::ShieldingDeviceLabel);
		tag_to_elem.insert(0x300A01A4, &tags::ShieldingDeviceLabel);

		ident_to_elem.insert("ShieldingDeviceDescription", &tags::ShieldingDeviceDescription);
		tag_to_elem.insert(0x300A01A6, &tags::ShieldingDeviceDescription);

		ident_to_elem.insert("ShieldingDevicePosition", &tags::ShieldingDevicePosition);
		tag_to_elem.insert(0x300A01A8, &tags::ShieldingDevicePosition);

		ident_to_elem.insert("SetupTechnique", &tags::SetupTechnique);
		tag_to_elem.insert(0x300A01B0, &tags::SetupTechnique);

		ident_to_elem.insert("SetupTechniqueDescription", &tags::SetupTechniqueDescription);
		tag_to_elem.insert(0x300A01B2, &tags::SetupTechniqueDescription);

		ident_to_elem.insert("SetupDeviceSequence", &tags::SetupDeviceSequence);
		tag_to_elem.insert(0x300A01B4, &tags::SetupDeviceSequence);

		ident_to_elem.insert("SetupDeviceType", &tags::SetupDeviceType);
		tag_to_elem.insert(0x300A01B6, &tags::SetupDeviceType);

		ident_to_elem.insert("SetupDeviceLabel", &tags::SetupDeviceLabel);
		tag_to_elem.insert(0x300A01B8, &tags::SetupDeviceLabel);

		ident_to_elem.insert("SetupDeviceDescription", &tags::SetupDeviceDescription);
		tag_to_elem.insert(0x300A01BA, &tags::SetupDeviceDescription);

		ident_to_elem.insert("SetupDeviceParameter", &tags::SetupDeviceParameter);
		tag_to_elem.insert(0x300A01BC, &tags::SetupDeviceParameter);

		ident_to_elem.insert("SetupReferenceDescription", &tags::SetupReferenceDescription);
		tag_to_elem.insert(0x300A01D0, &tags::SetupReferenceDescription);

		ident_to_elem.insert("TableTopVerticalSetupDisplacement", &tags::TableTopVerticalSetupDisplacement);
		tag_to_elem.insert(0x300A01D2, &tags::TableTopVerticalSetupDisplacement);

		ident_to_elem.insert("TableTopLongitudinalSetupDisplacement", &tags::TableTopLongitudinalSetupDisplacement);
		tag_to_elem.insert(0x300A01D4, &tags::TableTopLongitudinalSetupDisplacement);

		ident_to_elem.insert("TableTopLateralSetupDisplacement", &tags::TableTopLateralSetupDisplacement);
		tag_to_elem.insert(0x300A01D6, &tags::TableTopLateralSetupDisplacement);

		ident_to_elem.insert("BrachyTreatmentTechnique", &tags::BrachyTreatmentTechnique);
		tag_to_elem.insert(0x300A0200, &tags::BrachyTreatmentTechnique);

		ident_to_elem.insert("BrachyTreatmentType", &tags::BrachyTreatmentType);
		tag_to_elem.insert(0x300A0202, &tags::BrachyTreatmentType);

		ident_to_elem.insert("TreatmentMachineSequence", &tags::TreatmentMachineSequence);
		tag_to_elem.insert(0x300A0206, &tags::TreatmentMachineSequence);

		ident_to_elem.insert("SourceSequence", &tags::SourceSequence);
		tag_to_elem.insert(0x300A0210, &tags::SourceSequence);

		ident_to_elem.insert("SourceNumber", &tags::SourceNumber);
		tag_to_elem.insert(0x300A0212, &tags::SourceNumber);

		ident_to_elem.insert("SourceType", &tags::SourceType);
		tag_to_elem.insert(0x300A0214, &tags::SourceType);

		ident_to_elem.insert("SourceManufacturer", &tags::SourceManufacturer);
		tag_to_elem.insert(0x300A0216, &tags::SourceManufacturer);

		ident_to_elem.insert("ActiveSourceDiameter", &tags::ActiveSourceDiameter);
		tag_to_elem.insert(0x300A0218, &tags::ActiveSourceDiameter);

		ident_to_elem.insert("ActiveSourceLength", &tags::ActiveSourceLength);
		tag_to_elem.insert(0x300A021A, &tags::ActiveSourceLength);

		ident_to_elem.insert("SourceModelID", &tags::SourceModelID);
		tag_to_elem.insert(0x300A021B, &tags::SourceModelID);

		ident_to_elem.insert("SourceDescription", &tags::SourceDescription);
		tag_to_elem.insert(0x300A021C, &tags::SourceDescription);

		ident_to_elem.insert("SourceEncapsulationNominalThickness", &tags::SourceEncapsulationNominalThickness);
		tag_to_elem.insert(0x300A0222, &tags::SourceEncapsulationNominalThickness);

		ident_to_elem.insert("SourceEncapsulationNominalTransmission", &tags::SourceEncapsulationNominalTransmission);
		tag_to_elem.insert(0x300A0224, &tags::SourceEncapsulationNominalTransmission);

		ident_to_elem.insert("SourceIsotopeName", &tags::SourceIsotopeName);
		tag_to_elem.insert(0x300A0226, &tags::SourceIsotopeName);

		ident_to_elem.insert("SourceIsotopeHalfLife", &tags::SourceIsotopeHalfLife);
		tag_to_elem.insert(0x300A0228, &tags::SourceIsotopeHalfLife);

		ident_to_elem.insert("SourceStrengthUnits", &tags::SourceStrengthUnits);
		tag_to_elem.insert(0x300A0229, &tags::SourceStrengthUnits);

		ident_to_elem.insert("ReferenceAirKermaRate", &tags::ReferenceAirKermaRate);
		tag_to_elem.insert(0x300A022A, &tags::ReferenceAirKermaRate);

		ident_to_elem.insert("SourceStrength", &tags::SourceStrength);
		tag_to_elem.insert(0x300A022B, &tags::SourceStrength);

		ident_to_elem.insert("SourceStrengthReferenceDate", &tags::SourceStrengthReferenceDate);
		tag_to_elem.insert(0x300A022C, &tags::SourceStrengthReferenceDate);

		ident_to_elem.insert("SourceStrengthReferenceTime", &tags::SourceStrengthReferenceTime);
		tag_to_elem.insert(0x300A022E, &tags::SourceStrengthReferenceTime);

		ident_to_elem.insert("ApplicationSetupSequence", &tags::ApplicationSetupSequence);
		tag_to_elem.insert(0x300A0230, &tags::ApplicationSetupSequence);

		ident_to_elem.insert("ApplicationSetupType", &tags::ApplicationSetupType);
		tag_to_elem.insert(0x300A0232, &tags::ApplicationSetupType);

		ident_to_elem.insert("ApplicationSetupNumber", &tags::ApplicationSetupNumber);
		tag_to_elem.insert(0x300A0234, &tags::ApplicationSetupNumber);

		ident_to_elem.insert("ApplicationSetupName", &tags::ApplicationSetupName);
		tag_to_elem.insert(0x300A0236, &tags::ApplicationSetupName);

		ident_to_elem.insert("ApplicationSetupManufacturer", &tags::ApplicationSetupManufacturer);
		tag_to_elem.insert(0x300A0238, &tags::ApplicationSetupManufacturer);

		ident_to_elem.insert("TemplateNumber", &tags::TemplateNumber);
		tag_to_elem.insert(0x300A0240, &tags::TemplateNumber);

		ident_to_elem.insert("TemplateType", &tags::TemplateType);
		tag_to_elem.insert(0x300A0242, &tags::TemplateType);

		ident_to_elem.insert("TemplateName", &tags::TemplateName);
		tag_to_elem.insert(0x300A0244, &tags::TemplateName);

		ident_to_elem.insert("TotalReferenceAirKerma", &tags::TotalReferenceAirKerma);
		tag_to_elem.insert(0x300A0250, &tags::TotalReferenceAirKerma);

		ident_to_elem.insert("BrachyAccessoryDeviceSequence", &tags::BrachyAccessoryDeviceSequence);
		tag_to_elem.insert(0x300A0260, &tags::BrachyAccessoryDeviceSequence);

		ident_to_elem.insert("BrachyAccessoryDeviceNumber", &tags::BrachyAccessoryDeviceNumber);
		tag_to_elem.insert(0x300A0262, &tags::BrachyAccessoryDeviceNumber);

		ident_to_elem.insert("BrachyAccessoryDeviceID", &tags::BrachyAccessoryDeviceID);
		tag_to_elem.insert(0x300A0263, &tags::BrachyAccessoryDeviceID);

		ident_to_elem.insert("BrachyAccessoryDeviceType", &tags::BrachyAccessoryDeviceType);
		tag_to_elem.insert(0x300A0264, &tags::BrachyAccessoryDeviceType);

		ident_to_elem.insert("BrachyAccessoryDeviceName", &tags::BrachyAccessoryDeviceName);
		tag_to_elem.insert(0x300A0266, &tags::BrachyAccessoryDeviceName);

		ident_to_elem.insert("BrachyAccessoryDeviceNominalThickness", &tags::BrachyAccessoryDeviceNominalThickness);
		tag_to_elem.insert(0x300A026A, &tags::BrachyAccessoryDeviceNominalThickness);

		ident_to_elem.insert("BrachyAccessoryDeviceNominalTransmission", &tags::BrachyAccessoryDeviceNominalTransmission);
		tag_to_elem.insert(0x300A026C, &tags::BrachyAccessoryDeviceNominalTransmission);

		ident_to_elem.insert("ChannelSequence", &tags::ChannelSequence);
		tag_to_elem.insert(0x300A0280, &tags::ChannelSequence);

		ident_to_elem.insert("ChannelNumber", &tags::ChannelNumber);
		tag_to_elem.insert(0x300A0282, &tags::ChannelNumber);

		ident_to_elem.insert("ChannelLength", &tags::ChannelLength);
		tag_to_elem.insert(0x300A0284, &tags::ChannelLength);

		ident_to_elem.insert("ChannelTotalTime", &tags::ChannelTotalTime);
		tag_to_elem.insert(0x300A0286, &tags::ChannelTotalTime);

		ident_to_elem.insert("SourceMovementType", &tags::SourceMovementType);
		tag_to_elem.insert(0x300A0288, &tags::SourceMovementType);

		ident_to_elem.insert("NumberOfPulses", &tags::NumberOfPulses);
		tag_to_elem.insert(0x300A028A, &tags::NumberOfPulses);

		ident_to_elem.insert("PulseRepetitionInterval", &tags::PulseRepetitionInterval);
		tag_to_elem.insert(0x300A028C, &tags::PulseRepetitionInterval);

		ident_to_elem.insert("SourceApplicatorNumber", &tags::SourceApplicatorNumber);
		tag_to_elem.insert(0x300A0290, &tags::SourceApplicatorNumber);

		ident_to_elem.insert("SourceApplicatorID", &tags::SourceApplicatorID);
		tag_to_elem.insert(0x300A0291, &tags::SourceApplicatorID);

		ident_to_elem.insert("SourceApplicatorType", &tags::SourceApplicatorType);
		tag_to_elem.insert(0x300A0292, &tags::SourceApplicatorType);

		ident_to_elem.insert("SourceApplicatorName", &tags::SourceApplicatorName);
		tag_to_elem.insert(0x300A0294, &tags::SourceApplicatorName);

		ident_to_elem.insert("SourceApplicatorLength", &tags::SourceApplicatorLength);
		tag_to_elem.insert(0x300A0296, &tags::SourceApplicatorLength);

		ident_to_elem.insert("SourceApplicatorManufacturer", &tags::SourceApplicatorManufacturer);
		tag_to_elem.insert(0x300A0298, &tags::SourceApplicatorManufacturer);

		ident_to_elem.insert("SourceApplicatorWallNominalThickness", &tags::SourceApplicatorWallNominalThickness);
		tag_to_elem.insert(0x300A029C, &tags::SourceApplicatorWallNominalThickness);

		ident_to_elem.insert("SourceApplicatorWallNominalTransmission", &tags::SourceApplicatorWallNominalTransmission);
		tag_to_elem.insert(0x300A029E, &tags::SourceApplicatorWallNominalTransmission);

		ident_to_elem.insert("SourceApplicatorStepSize", &tags::SourceApplicatorStepSize);
		tag_to_elem.insert(0x300A02A0, &tags::SourceApplicatorStepSize);

		ident_to_elem.insert("TransferTubeNumber", &tags::TransferTubeNumber);
		tag_to_elem.insert(0x300A02A2, &tags::TransferTubeNumber);

		ident_to_elem.insert("TransferTubeLength", &tags::TransferTubeLength);
		tag_to_elem.insert(0x300A02A4, &tags::TransferTubeLength);

		ident_to_elem.insert("ChannelShieldSequence", &tags::ChannelShieldSequence);
		tag_to_elem.insert(0x300A02B0, &tags::ChannelShieldSequence);

		ident_to_elem.insert("ChannelShieldNumber", &tags::ChannelShieldNumber);
		tag_to_elem.insert(0x300A02B2, &tags::ChannelShieldNumber);

		ident_to_elem.insert("ChannelShieldID", &tags::ChannelShieldID);
		tag_to_elem.insert(0x300A02B3, &tags::ChannelShieldID);

		ident_to_elem.insert("ChannelShieldName", &tags::ChannelShieldName);
		tag_to_elem.insert(0x300A02B4, &tags::ChannelShieldName);

		ident_to_elem.insert("ChannelShieldNominalThickness", &tags::ChannelShieldNominalThickness);
		tag_to_elem.insert(0x300A02B8, &tags::ChannelShieldNominalThickness);

		ident_to_elem.insert("ChannelShieldNominalTransmission", &tags::ChannelShieldNominalTransmission);
		tag_to_elem.insert(0x300A02BA, &tags::ChannelShieldNominalTransmission);

		ident_to_elem.insert("FinalCumulativeTimeWeight", &tags::FinalCumulativeTimeWeight);
		tag_to_elem.insert(0x300A02C8, &tags::FinalCumulativeTimeWeight);

		ident_to_elem.insert("BrachyControlPointSequence", &tags::BrachyControlPointSequence);
		tag_to_elem.insert(0x300A02D0, &tags::BrachyControlPointSequence);

		ident_to_elem.insert("ControlPointRelativePosition", &tags::ControlPointRelativePosition);
		tag_to_elem.insert(0x300A02D2, &tags::ControlPointRelativePosition);

		ident_to_elem.insert("ControlPoint3DPosition", &tags::ControlPoint3DPosition);
		tag_to_elem.insert(0x300A02D4, &tags::ControlPoint3DPosition);

		ident_to_elem.insert("CumulativeTimeWeight", &tags::CumulativeTimeWeight);
		tag_to_elem.insert(0x300A02D6, &tags::CumulativeTimeWeight);

		ident_to_elem.insert("CompensatorDivergence", &tags::CompensatorDivergence);
		tag_to_elem.insert(0x300A02E0, &tags::CompensatorDivergence);

		ident_to_elem.insert("CompensatorMountingPosition", &tags::CompensatorMountingPosition);
		tag_to_elem.insert(0x300A02E1, &tags::CompensatorMountingPosition);

		ident_to_elem.insert("SourceToCompensatorDistance", &tags::SourceToCompensatorDistance);
		tag_to_elem.insert(0x300A02E2, &tags::SourceToCompensatorDistance);

		ident_to_elem.insert("TotalCompensatorTrayWaterEquivalentThickness", &tags::TotalCompensatorTrayWaterEquivalentThickness);
		tag_to_elem.insert(0x300A02E3, &tags::TotalCompensatorTrayWaterEquivalentThickness);

		ident_to_elem.insert("IsocenterToCompensatorTrayDistance", &tags::IsocenterToCompensatorTrayDistance);
		tag_to_elem.insert(0x300A02E4, &tags::IsocenterToCompensatorTrayDistance);

		ident_to_elem.insert("CompensatorColumnOffset", &tags::CompensatorColumnOffset);
		tag_to_elem.insert(0x300A02E5, &tags::CompensatorColumnOffset);

		ident_to_elem.insert("IsocenterToCompensatorDistances", &tags::IsocenterToCompensatorDistances);
		tag_to_elem.insert(0x300A02E6, &tags::IsocenterToCompensatorDistances);

		ident_to_elem.insert("CompensatorRelativeStoppingPowerRatio", &tags::CompensatorRelativeStoppingPowerRatio);
		tag_to_elem.insert(0x300A02E7, &tags::CompensatorRelativeStoppingPowerRatio);

		ident_to_elem.insert("CompensatorMillingToolDiameter", &tags::CompensatorMillingToolDiameter);
		tag_to_elem.insert(0x300A02E8, &tags::CompensatorMillingToolDiameter);

		ident_to_elem.insert("IonRangeCompensatorSequence", &tags::IonRangeCompensatorSequence);
		tag_to_elem.insert(0x300A02EA, &tags::IonRangeCompensatorSequence);

		ident_to_elem.insert("CompensatorDescription", &tags::CompensatorDescription);
		tag_to_elem.insert(0x300A02EB, &tags::CompensatorDescription);

		ident_to_elem.insert("RadiationMassNumber", &tags::RadiationMassNumber);
		tag_to_elem.insert(0x300A0302, &tags::RadiationMassNumber);

		ident_to_elem.insert("RadiationAtomicNumber", &tags::RadiationAtomicNumber);
		tag_to_elem.insert(0x300A0304, &tags::RadiationAtomicNumber);

		ident_to_elem.insert("RadiationChargeState", &tags::RadiationChargeState);
		tag_to_elem.insert(0x300A0306, &tags::RadiationChargeState);

		ident_to_elem.insert("ScanMode", &tags::ScanMode);
		tag_to_elem.insert(0x300A0308, &tags::ScanMode);

		ident_to_elem.insert("ModulatedScanModeType", &tags::ModulatedScanModeType);
		tag_to_elem.insert(0x300A0309, &tags::ModulatedScanModeType);

		ident_to_elem.insert("VirtualSourceAxisDistances", &tags::VirtualSourceAxisDistances);
		tag_to_elem.insert(0x300A030A, &tags::VirtualSourceAxisDistances);

		ident_to_elem.insert("SnoutSequence", &tags::SnoutSequence);
		tag_to_elem.insert(0x300A030C, &tags::SnoutSequence);

		ident_to_elem.insert("SnoutPosition", &tags::SnoutPosition);
		tag_to_elem.insert(0x300A030D, &tags::SnoutPosition);

		ident_to_elem.insert("SnoutID", &tags::SnoutID);
		tag_to_elem.insert(0x300A030F, &tags::SnoutID);

		ident_to_elem.insert("NumberOfRangeShifters", &tags::NumberOfRangeShifters);
		tag_to_elem.insert(0x300A0312, &tags::NumberOfRangeShifters);

		ident_to_elem.insert("RangeShifterSequence", &tags::RangeShifterSequence);
		tag_to_elem.insert(0x300A0314, &tags::RangeShifterSequence);

		ident_to_elem.insert("RangeShifterNumber", &tags::RangeShifterNumber);
		tag_to_elem.insert(0x300A0316, &tags::RangeShifterNumber);

		ident_to_elem.insert("RangeShifterID", &tags::RangeShifterID);
		tag_to_elem.insert(0x300A0318, &tags::RangeShifterID);

		ident_to_elem.insert("RangeShifterType", &tags::RangeShifterType);
		tag_to_elem.insert(0x300A0320, &tags::RangeShifterType);

		ident_to_elem.insert("RangeShifterDescription", &tags::RangeShifterDescription);
		tag_to_elem.insert(0x300A0322, &tags::RangeShifterDescription);

		ident_to_elem.insert("NumberOfLateralSpreadingDevices", &tags::NumberOfLateralSpreadingDevices);
		tag_to_elem.insert(0x300A0330, &tags::NumberOfLateralSpreadingDevices);

		ident_to_elem.insert("LateralSpreadingDeviceSequence", &tags::LateralSpreadingDeviceSequence);
		tag_to_elem.insert(0x300A0332, &tags::LateralSpreadingDeviceSequence);

		ident_to_elem.insert("LateralSpreadingDeviceNumber", &tags::LateralSpreadingDeviceNumber);
		tag_to_elem.insert(0x300A0334, &tags::LateralSpreadingDeviceNumber);

		ident_to_elem.insert("LateralSpreadingDeviceID", &tags::LateralSpreadingDeviceID);
		tag_to_elem.insert(0x300A0336, &tags::LateralSpreadingDeviceID);

		ident_to_elem.insert("LateralSpreadingDeviceType", &tags::LateralSpreadingDeviceType);
		tag_to_elem.insert(0x300A0338, &tags::LateralSpreadingDeviceType);

		ident_to_elem.insert("LateralSpreadingDeviceDescription", &tags::LateralSpreadingDeviceDescription);
		tag_to_elem.insert(0x300A033A, &tags::LateralSpreadingDeviceDescription);

		ident_to_elem.insert("LateralSpreadingDeviceWaterEquivalentThickness", &tags::LateralSpreadingDeviceWaterEquivalentThickness);
		tag_to_elem.insert(0x300A033C, &tags::LateralSpreadingDeviceWaterEquivalentThickness);

		ident_to_elem.insert("NumberOfRangeModulators", &tags::NumberOfRangeModulators);
		tag_to_elem.insert(0x300A0340, &tags::NumberOfRangeModulators);

		ident_to_elem.insert("RangeModulatorSequence", &tags::RangeModulatorSequence);
		tag_to_elem.insert(0x300A0342, &tags::RangeModulatorSequence);

		ident_to_elem.insert("RangeModulatorNumber", &tags::RangeModulatorNumber);
		tag_to_elem.insert(0x300A0344, &tags::RangeModulatorNumber);

		ident_to_elem.insert("RangeModulatorID", &tags::RangeModulatorID);
		tag_to_elem.insert(0x300A0346, &tags::RangeModulatorID);

		ident_to_elem.insert("RangeModulatorType", &tags::RangeModulatorType);
		tag_to_elem.insert(0x300A0348, &tags::RangeModulatorType);

		ident_to_elem.insert("RangeModulatorDescription", &tags::RangeModulatorDescription);
		tag_to_elem.insert(0x300A034A, &tags::RangeModulatorDescription);

		ident_to_elem.insert("BeamCurrentModulationID", &tags::BeamCurrentModulationID);
		tag_to_elem.insert(0x300A034C, &tags::BeamCurrentModulationID);

		ident_to_elem.insert("PatientSupportType", &tags::PatientSupportType);
		tag_to_elem.insert(0x300A0350, &tags::PatientSupportType);

		ident_to_elem.insert("PatientSupportID", &tags::PatientSupportID);
		tag_to_elem.insert(0x300A0352, &tags::PatientSupportID);

		ident_to_elem.insert("PatientSupportAccessoryCode", &tags::PatientSupportAccessoryCode);
		tag_to_elem.insert(0x300A0354, &tags::PatientSupportAccessoryCode);

		ident_to_elem.insert("TrayAccessoryCode", &tags::TrayAccessoryCode);
		tag_to_elem.insert(0x300A0355, &tags::TrayAccessoryCode);

		ident_to_elem.insert("FixationLightAzimuthalAngle", &tags::FixationLightAzimuthalAngle);
		tag_to_elem.insert(0x300A0356, &tags::FixationLightAzimuthalAngle);

		ident_to_elem.insert("FixationLightPolarAngle", &tags::FixationLightPolarAngle);
		tag_to_elem.insert(0x300A0358, &tags::FixationLightPolarAngle);

		ident_to_elem.insert("MetersetRate", &tags::MetersetRate);
		tag_to_elem.insert(0x300A035A, &tags::MetersetRate);

		ident_to_elem.insert("RangeShifterSettingsSequence", &tags::RangeShifterSettingsSequence);
		tag_to_elem.insert(0x300A0360, &tags::RangeShifterSettingsSequence);

		ident_to_elem.insert("RangeShifterSetting", &tags::RangeShifterSetting);
		tag_to_elem.insert(0x300A0362, &tags::RangeShifterSetting);

		ident_to_elem.insert("IsocenterToRangeShifterDistance", &tags::IsocenterToRangeShifterDistance);
		tag_to_elem.insert(0x300A0364, &tags::IsocenterToRangeShifterDistance);

		ident_to_elem.insert("RangeShifterWaterEquivalentThickness", &tags::RangeShifterWaterEquivalentThickness);
		tag_to_elem.insert(0x300A0366, &tags::RangeShifterWaterEquivalentThickness);

		ident_to_elem.insert("LateralSpreadingDeviceSettingsSequence", &tags::LateralSpreadingDeviceSettingsSequence);
		tag_to_elem.insert(0x300A0370, &tags::LateralSpreadingDeviceSettingsSequence);

		ident_to_elem.insert("LateralSpreadingDeviceSetting", &tags::LateralSpreadingDeviceSetting);
		tag_to_elem.insert(0x300A0372, &tags::LateralSpreadingDeviceSetting);

		ident_to_elem.insert("IsocenterToLateralSpreadingDeviceDistance", &tags::IsocenterToLateralSpreadingDeviceDistance);
		tag_to_elem.insert(0x300A0374, &tags::IsocenterToLateralSpreadingDeviceDistance);

		ident_to_elem.insert("RangeModulatorSettingsSequence", &tags::RangeModulatorSettingsSequence);
		tag_to_elem.insert(0x300A0380, &tags::RangeModulatorSettingsSequence);

		ident_to_elem.insert("RangeModulatorGatingStartValue", &tags::RangeModulatorGatingStartValue);
		tag_to_elem.insert(0x300A0382, &tags::RangeModulatorGatingStartValue);

		ident_to_elem.insert("RangeModulatorGatingStopValue", &tags::RangeModulatorGatingStopValue);
		tag_to_elem.insert(0x300A0384, &tags::RangeModulatorGatingStopValue);

		ident_to_elem.insert("RangeModulatorGatingStartWaterEquivalentThickness", &tags::RangeModulatorGatingStartWaterEquivalentThickness);
		tag_to_elem.insert(0x300A0386, &tags::RangeModulatorGatingStartWaterEquivalentThickness);

		ident_to_elem.insert("RangeModulatorGatingStopWaterEquivalentThickness", &tags::RangeModulatorGatingStopWaterEquivalentThickness);
		tag_to_elem.insert(0x300A0388, &tags::RangeModulatorGatingStopWaterEquivalentThickness);

		ident_to_elem.insert("IsocenterToRangeModulatorDistance", &tags::IsocenterToRangeModulatorDistance);
		tag_to_elem.insert(0x300A038A, &tags::IsocenterToRangeModulatorDistance);

		ident_to_elem.insert("ScanSpotTimeOffset", &tags::ScanSpotTimeOffset);
		tag_to_elem.insert(0x300A038F, &tags::ScanSpotTimeOffset);

		ident_to_elem.insert("ScanSpotTuneID", &tags::ScanSpotTuneID);
		tag_to_elem.insert(0x300A0390, &tags::ScanSpotTuneID);

		ident_to_elem.insert("ScanSpotPrescribedIndices", &tags::ScanSpotPrescribedIndices);
		tag_to_elem.insert(0x300A0391, &tags::ScanSpotPrescribedIndices);

		ident_to_elem.insert("NumberOfScanSpotPositions", &tags::NumberOfScanSpotPositions);
		tag_to_elem.insert(0x300A0392, &tags::NumberOfScanSpotPositions);

		ident_to_elem.insert("ScanSpotReordered", &tags::ScanSpotReordered);
		tag_to_elem.insert(0x300A0393, &tags::ScanSpotReordered);

		ident_to_elem.insert("ScanSpotPositionMap", &tags::ScanSpotPositionMap);
		tag_to_elem.insert(0x300A0394, &tags::ScanSpotPositionMap);

		ident_to_elem.insert("ScanSpotReorderingAllowed", &tags::ScanSpotReorderingAllowed);
		tag_to_elem.insert(0x300A0395, &tags::ScanSpotReorderingAllowed);

		ident_to_elem.insert("ScanSpotMetersetWeights", &tags::ScanSpotMetersetWeights);
		tag_to_elem.insert(0x300A0396, &tags::ScanSpotMetersetWeights);

		ident_to_elem.insert("ScanningSpotSize", &tags::ScanningSpotSize);
		tag_to_elem.insert(0x300A0398, &tags::ScanningSpotSize);

		ident_to_elem.insert("NumberOfPaintings", &tags::NumberOfPaintings);
		tag_to_elem.insert(0x300A039A, &tags::NumberOfPaintings);

		ident_to_elem.insert("IonToleranceTableSequence", &tags::IonToleranceTableSequence);
		tag_to_elem.insert(0x300A03A0, &tags::IonToleranceTableSequence);

		ident_to_elem.insert("IonBeamSequence", &tags::IonBeamSequence);
		tag_to_elem.insert(0x300A03A2, &tags::IonBeamSequence);

		ident_to_elem.insert("IonBeamLimitingDeviceSequence", &tags::IonBeamLimitingDeviceSequence);
		tag_to_elem.insert(0x300A03A4, &tags::IonBeamLimitingDeviceSequence);

		ident_to_elem.insert("IonBlockSequence", &tags::IonBlockSequence);
		tag_to_elem.insert(0x300A03A6, &tags::IonBlockSequence);

		ident_to_elem.insert("IonControlPointSequence", &tags::IonControlPointSequence);
		tag_to_elem.insert(0x300A03A8, &tags::IonControlPointSequence);

		ident_to_elem.insert("IonWedgeSequence", &tags::IonWedgeSequence);
		tag_to_elem.insert(0x300A03AA, &tags::IonWedgeSequence);

		ident_to_elem.insert("IonWedgePositionSequence", &tags::IonWedgePositionSequence);
		tag_to_elem.insert(0x300A03AC, &tags::IonWedgePositionSequence);

		ident_to_elem.insert("ReferencedSetupImageSequence", &tags::ReferencedSetupImageSequence);
		tag_to_elem.insert(0x300A0401, &tags::ReferencedSetupImageSequence);

		ident_to_elem.insert("SetupImageComment", &tags::SetupImageComment);
		tag_to_elem.insert(0x300A0402, &tags::SetupImageComment);

		ident_to_elem.insert("MotionSynchronizationSequence", &tags::MotionSynchronizationSequence);
		tag_to_elem.insert(0x300A0410, &tags::MotionSynchronizationSequence);

		ident_to_elem.insert("ControlPointOrientation", &tags::ControlPointOrientation);
		tag_to_elem.insert(0x300A0412, &tags::ControlPointOrientation);

		ident_to_elem.insert("GeneralAccessorySequence", &tags::GeneralAccessorySequence);
		tag_to_elem.insert(0x300A0420, &tags::GeneralAccessorySequence);

		ident_to_elem.insert("GeneralAccessoryID", &tags::GeneralAccessoryID);
		tag_to_elem.insert(0x300A0421, &tags::GeneralAccessoryID);

		ident_to_elem.insert("GeneralAccessoryDescription", &tags::GeneralAccessoryDescription);
		tag_to_elem.insert(0x300A0422, &tags::GeneralAccessoryDescription);

		ident_to_elem.insert("GeneralAccessoryType", &tags::GeneralAccessoryType);
		tag_to_elem.insert(0x300A0423, &tags::GeneralAccessoryType);

		ident_to_elem.insert("GeneralAccessoryNumber", &tags::GeneralAccessoryNumber);
		tag_to_elem.insert(0x300A0424, &tags::GeneralAccessoryNumber);

		ident_to_elem.insert("SourceToGeneralAccessoryDistance", &tags::SourceToGeneralAccessoryDistance);
		tag_to_elem.insert(0x300A0425, &tags::SourceToGeneralAccessoryDistance);

		ident_to_elem.insert("ApplicatorGeometrySequence", &tags::ApplicatorGeometrySequence);
		tag_to_elem.insert(0x300A0431, &tags::ApplicatorGeometrySequence);

		ident_to_elem.insert("ApplicatorApertureShape", &tags::ApplicatorApertureShape);
		tag_to_elem.insert(0x300A0432, &tags::ApplicatorApertureShape);

		ident_to_elem.insert("ApplicatorOpening", &tags::ApplicatorOpening);
		tag_to_elem.insert(0x300A0433, &tags::ApplicatorOpening);

		ident_to_elem.insert("ApplicatorOpeningX", &tags::ApplicatorOpeningX);
		tag_to_elem.insert(0x300A0434, &tags::ApplicatorOpeningX);

		ident_to_elem.insert("ApplicatorOpeningY", &tags::ApplicatorOpeningY);
		tag_to_elem.insert(0x300A0435, &tags::ApplicatorOpeningY);

		ident_to_elem.insert("SourceToApplicatorMountingPositionDistance", &tags::SourceToApplicatorMountingPositionDistance);
		tag_to_elem.insert(0x300A0436, &tags::SourceToApplicatorMountingPositionDistance);

		ident_to_elem.insert("NumberOfBlockSlabItems", &tags::NumberOfBlockSlabItems);
		tag_to_elem.insert(0x300A0440, &tags::NumberOfBlockSlabItems);

		ident_to_elem.insert("BlockSlabSequence", &tags::BlockSlabSequence);
		tag_to_elem.insert(0x300A0441, &tags::BlockSlabSequence);

		ident_to_elem.insert("BlockSlabThickness", &tags::BlockSlabThickness);
		tag_to_elem.insert(0x300A0442, &tags::BlockSlabThickness);

		ident_to_elem.insert("BlockSlabNumber", &tags::BlockSlabNumber);
		tag_to_elem.insert(0x300A0443, &tags::BlockSlabNumber);

		ident_to_elem.insert("DeviceMotionControlSequence", &tags::DeviceMotionControlSequence);
		tag_to_elem.insert(0x300A0450, &tags::DeviceMotionControlSequence);

		ident_to_elem.insert("DeviceMotionExecutionMode", &tags::DeviceMotionExecutionMode);
		tag_to_elem.insert(0x300A0451, &tags::DeviceMotionExecutionMode);

		ident_to_elem.insert("DeviceMotionObservationMode", &tags::DeviceMotionObservationMode);
		tag_to_elem.insert(0x300A0452, &tags::DeviceMotionObservationMode);

		ident_to_elem.insert("DeviceMotionParameterCodeSequence", &tags::DeviceMotionParameterCodeSequence);
		tag_to_elem.insert(0x300A0453, &tags::DeviceMotionParameterCodeSequence);

		ident_to_elem.insert("DistalDepthFraction", &tags::DistalDepthFraction);
		tag_to_elem.insert(0x300A0501, &tags::DistalDepthFraction);

		ident_to_elem.insert("DistalDepth", &tags::DistalDepth);
		tag_to_elem.insert(0x300A0502, &tags::DistalDepth);

		ident_to_elem.insert("NominalRangeModulationFractions", &tags::NominalRangeModulationFractions);
		tag_to_elem.insert(0x300A0503, &tags::NominalRangeModulationFractions);

		ident_to_elem.insert("NominalRangeModulatedRegionDepths", &tags::NominalRangeModulatedRegionDepths);
		tag_to_elem.insert(0x300A0504, &tags::NominalRangeModulatedRegionDepths);

		ident_to_elem.insert("DepthDoseParametersSequence", &tags::DepthDoseParametersSequence);
		tag_to_elem.insert(0x300A0505, &tags::DepthDoseParametersSequence);

		ident_to_elem.insert("DeliveredDepthDoseParametersSequence", &tags::DeliveredDepthDoseParametersSequence);
		tag_to_elem.insert(0x300A0506, &tags::DeliveredDepthDoseParametersSequence);

		ident_to_elem.insert("DeliveredDistalDepthFraction", &tags::DeliveredDistalDepthFraction);
		tag_to_elem.insert(0x300A0507, &tags::DeliveredDistalDepthFraction);

		ident_to_elem.insert("DeliveredDistalDepth", &tags::DeliveredDistalDepth);
		tag_to_elem.insert(0x300A0508, &tags::DeliveredDistalDepth);

		ident_to_elem.insert("DeliveredNominalRangeModulationFractions", &tags::DeliveredNominalRangeModulationFractions);
		tag_to_elem.insert(0x300A0509, &tags::DeliveredNominalRangeModulationFractions);

		ident_to_elem.insert("DeliveredNominalRangeModulatedRegionDepths", &tags::DeliveredNominalRangeModulatedRegionDepths);
		tag_to_elem.insert(0x300A0510, &tags::DeliveredNominalRangeModulatedRegionDepths);

		ident_to_elem.insert("DeliveredReferenceDoseDefinition", &tags::DeliveredReferenceDoseDefinition);
		tag_to_elem.insert(0x300A0511, &tags::DeliveredReferenceDoseDefinition);

		ident_to_elem.insert("ReferenceDoseDefinition", &tags::ReferenceDoseDefinition);
		tag_to_elem.insert(0x300A0512, &tags::ReferenceDoseDefinition);

		ident_to_elem.insert("ReferencedRTPlanSequence", &tags::ReferencedRTPlanSequence);
		tag_to_elem.insert(0x300C0002, &tags::ReferencedRTPlanSequence);

		ident_to_elem.insert("ReferencedBeamSequence", &tags::ReferencedBeamSequence);
		tag_to_elem.insert(0x300C0004, &tags::ReferencedBeamSequence);

		ident_to_elem.insert("ReferencedBeamNumber", &tags::ReferencedBeamNumber);
		tag_to_elem.insert(0x300C0006, &tags::ReferencedBeamNumber);

		ident_to_elem.insert("ReferencedReferenceImageNumber", &tags::ReferencedReferenceImageNumber);
		tag_to_elem.insert(0x300C0007, &tags::ReferencedReferenceImageNumber);

		ident_to_elem.insert("StartCumulativeMetersetWeight", &tags::StartCumulativeMetersetWeight);
		tag_to_elem.insert(0x300C0008, &tags::StartCumulativeMetersetWeight);

		ident_to_elem.insert("EndCumulativeMetersetWeight", &tags::EndCumulativeMetersetWeight);
		tag_to_elem.insert(0x300C0009, &tags::EndCumulativeMetersetWeight);

		ident_to_elem.insert("ReferencedBrachyApplicationSetupSequence", &tags::ReferencedBrachyApplicationSetupSequence);
		tag_to_elem.insert(0x300C000A, &tags::ReferencedBrachyApplicationSetupSequence);

		ident_to_elem.insert("ReferencedBrachyApplicationSetupNumber", &tags::ReferencedBrachyApplicationSetupNumber);
		tag_to_elem.insert(0x300C000C, &tags::ReferencedBrachyApplicationSetupNumber);

		ident_to_elem.insert("ReferencedSourceNumber", &tags::ReferencedSourceNumber);
		tag_to_elem.insert(0x300C000E, &tags::ReferencedSourceNumber);

		ident_to_elem.insert("ReferencedFractionGroupSequence", &tags::ReferencedFractionGroupSequence);
		tag_to_elem.insert(0x300C0020, &tags::ReferencedFractionGroupSequence);

		ident_to_elem.insert("ReferencedFractionGroupNumber", &tags::ReferencedFractionGroupNumber);
		tag_to_elem.insert(0x300C0022, &tags::ReferencedFractionGroupNumber);

		ident_to_elem.insert("ReferencedVerificationImageSequence", &tags::ReferencedVerificationImageSequence);
		tag_to_elem.insert(0x300C0040, &tags::ReferencedVerificationImageSequence);

		ident_to_elem.insert("ReferencedReferenceImageSequence", &tags::ReferencedReferenceImageSequence);
		tag_to_elem.insert(0x300C0042, &tags::ReferencedReferenceImageSequence);

		ident_to_elem.insert("ReferencedDoseReferenceSequence", &tags::ReferencedDoseReferenceSequence);
		tag_to_elem.insert(0x300C0050, &tags::ReferencedDoseReferenceSequence);

		ident_to_elem.insert("ReferencedDoseReferenceNumber", &tags::ReferencedDoseReferenceNumber);
		tag_to_elem.insert(0x300C0051, &tags::ReferencedDoseReferenceNumber);

		ident_to_elem.insert("BrachyReferencedDoseReferenceSequence", &tags::BrachyReferencedDoseReferenceSequence);
		tag_to_elem.insert(0x300C0055, &tags::BrachyReferencedDoseReferenceSequence);

		ident_to_elem.insert("ReferencedStructureSetSequence", &tags::ReferencedStructureSetSequence);
		tag_to_elem.insert(0x300C0060, &tags::ReferencedStructureSetSequence);

		ident_to_elem.insert("ReferencedPatientSetupNumber", &tags::ReferencedPatientSetupNumber);
		tag_to_elem.insert(0x300C006A, &tags::ReferencedPatientSetupNumber);

		ident_to_elem.insert("ReferencedDoseSequence", &tags::ReferencedDoseSequence);
		tag_to_elem.insert(0x300C0080, &tags::ReferencedDoseSequence);

		ident_to_elem.insert("ReferencedToleranceTableNumber", &tags::ReferencedToleranceTableNumber);
		tag_to_elem.insert(0x300C00A0, &tags::ReferencedToleranceTableNumber);

		ident_to_elem.insert("ReferencedBolusSequence", &tags::ReferencedBolusSequence);
		tag_to_elem.insert(0x300C00B0, &tags::ReferencedBolusSequence);

		ident_to_elem.insert("ReferencedWedgeNumber", &tags::ReferencedWedgeNumber);
		tag_to_elem.insert(0x300C00C0, &tags::ReferencedWedgeNumber);

		ident_to_elem.insert("ReferencedCompensatorNumber", &tags::ReferencedCompensatorNumber);
		tag_to_elem.insert(0x300C00D0, &tags::ReferencedCompensatorNumber);

		ident_to_elem.insert("ReferencedBlockNumber", &tags::ReferencedBlockNumber);
		tag_to_elem.insert(0x300C00E0, &tags::ReferencedBlockNumber);

		ident_to_elem.insert("ReferencedControlPointIndex", &tags::ReferencedControlPointIndex);
		tag_to_elem.insert(0x300C00F0, &tags::ReferencedControlPointIndex);

		ident_to_elem.insert("ReferencedControlPointSequence", &tags::ReferencedControlPointSequence);
		tag_to_elem.insert(0x300C00F2, &tags::ReferencedControlPointSequence);

		ident_to_elem.insert("ReferencedStartControlPointIndex", &tags::ReferencedStartControlPointIndex);
		tag_to_elem.insert(0x300C00F4, &tags::ReferencedStartControlPointIndex);

		ident_to_elem.insert("ReferencedStopControlPointIndex", &tags::ReferencedStopControlPointIndex);
		tag_to_elem.insert(0x300C00F6, &tags::ReferencedStopControlPointIndex);

		ident_to_elem.insert("ReferencedRangeShifterNumber", &tags::ReferencedRangeShifterNumber);
		tag_to_elem.insert(0x300C0100, &tags::ReferencedRangeShifterNumber);

		ident_to_elem.insert("ReferencedLateralSpreadingDeviceNumber", &tags::ReferencedLateralSpreadingDeviceNumber);
		tag_to_elem.insert(0x300C0102, &tags::ReferencedLateralSpreadingDeviceNumber);

		ident_to_elem.insert("ReferencedRangeModulatorNumber", &tags::ReferencedRangeModulatorNumber);
		tag_to_elem.insert(0x300C0104, &tags::ReferencedRangeModulatorNumber);

		ident_to_elem.insert("OmittedBeamTaskSequence", &tags::OmittedBeamTaskSequence);
		tag_to_elem.insert(0x300C0111, &tags::OmittedBeamTaskSequence);

		ident_to_elem.insert("ReasonForOmission", &tags::ReasonForOmission);
		tag_to_elem.insert(0x300C0112, &tags::ReasonForOmission);

		ident_to_elem.insert("ReasonForOmissionDescription", &tags::ReasonForOmissionDescription);
		tag_to_elem.insert(0x300C0113, &tags::ReasonForOmissionDescription);

		ident_to_elem.insert("ApprovalStatus", &tags::ApprovalStatus);
		tag_to_elem.insert(0x300E0002, &tags::ApprovalStatus);

		ident_to_elem.insert("ReviewDate", &tags::ReviewDate);
		tag_to_elem.insert(0x300E0004, &tags::ReviewDate);

		ident_to_elem.insert("ReviewTime", &tags::ReviewTime);
		tag_to_elem.insert(0x300E0005, &tags::ReviewTime);

		ident_to_elem.insert("ReviewerName", &tags::ReviewerName);
		tag_to_elem.insert(0x300E0008, &tags::ReviewerName);

		ident_to_elem.insert("Arbitrary", &tags::Arbitrary);
		tag_to_elem.insert(0x40000010, &tags::Arbitrary);

		ident_to_elem.insert("TextComments", &tags::TextComments);
		tag_to_elem.insert(0x40004000, &tags::TextComments);

		ident_to_elem.insert("ResultsID", &tags::ResultsID);
		tag_to_elem.insert(0x40080040, &tags::ResultsID);

		ident_to_elem.insert("ResultsIDIssuer", &tags::ResultsIDIssuer);
		tag_to_elem.insert(0x40080042, &tags::ResultsIDIssuer);

		ident_to_elem.insert("ReferencedInterpretationSequence", &tags::ReferencedInterpretationSequence);
		tag_to_elem.insert(0x40080050, &tags::ReferencedInterpretationSequence);

		ident_to_elem.insert("ReportProductionStatusTrial", &tags::ReportProductionStatusTrial);
		tag_to_elem.insert(0x400800FF, &tags::ReportProductionStatusTrial);

		ident_to_elem.insert("InterpretationRecordedDate", &tags::InterpretationRecordedDate);
		tag_to_elem.insert(0x40080100, &tags::InterpretationRecordedDate);

		ident_to_elem.insert("InterpretationRecordedTime", &tags::InterpretationRecordedTime);
		tag_to_elem.insert(0x40080101, &tags::InterpretationRecordedTime);

		ident_to_elem.insert("InterpretationRecorder", &tags::InterpretationRecorder);
		tag_to_elem.insert(0x40080102, &tags::InterpretationRecorder);

		ident_to_elem.insert("ReferenceToRecordedSound", &tags::ReferenceToRecordedSound);
		tag_to_elem.insert(0x40080103, &tags::ReferenceToRecordedSound);

		ident_to_elem.insert("InterpretationTranscriptionDate", &tags::InterpretationTranscriptionDate);
		tag_to_elem.insert(0x40080108, &tags::InterpretationTranscriptionDate);

		ident_to_elem.insert("InterpretationTranscriptionTime", &tags::InterpretationTranscriptionTime);
		tag_to_elem.insert(0x40080109, &tags::InterpretationTranscriptionTime);

		ident_to_elem.insert("InterpretationTranscriber", &tags::InterpretationTranscriber);
		tag_to_elem.insert(0x4008010A, &tags::InterpretationTranscriber);

		ident_to_elem.insert("InterpretationText", &tags::InterpretationText);
		tag_to_elem.insert(0x4008010B, &tags::InterpretationText);

		ident_to_elem.insert("InterpretationAuthor", &tags::InterpretationAuthor);
		tag_to_elem.insert(0x4008010C, &tags::InterpretationAuthor);

		ident_to_elem.insert("InterpretationApproverSequence", &tags::InterpretationApproverSequence);
		tag_to_elem.insert(0x40080111, &tags::InterpretationApproverSequence);

		ident_to_elem.insert("InterpretationApprovalDate", &tags::InterpretationApprovalDate);
		tag_to_elem.insert(0x40080112, &tags::InterpretationApprovalDate);

		ident_to_elem.insert("InterpretationApprovalTime", &tags::InterpretationApprovalTime);
		tag_to_elem.insert(0x40080113, &tags::InterpretationApprovalTime);

		ident_to_elem.insert("PhysicianApprovingInterpretation", &tags::PhysicianApprovingInterpretation);
		tag_to_elem.insert(0x40080114, &tags::PhysicianApprovingInterpretation);

		ident_to_elem.insert("InterpretationDiagnosisDescription", &tags::InterpretationDiagnosisDescription);
		tag_to_elem.insert(0x40080115, &tags::InterpretationDiagnosisDescription);

		ident_to_elem.insert("InterpretationDiagnosisCodeSequence", &tags::InterpretationDiagnosisCodeSequence);
		tag_to_elem.insert(0x40080117, &tags::InterpretationDiagnosisCodeSequence);

		ident_to_elem.insert("ResultsDistributionListSequence", &tags::ResultsDistributionListSequence);
		tag_to_elem.insert(0x40080118, &tags::ResultsDistributionListSequence);

		ident_to_elem.insert("DistributionName", &tags::DistributionName);
		tag_to_elem.insert(0x40080119, &tags::DistributionName);

		ident_to_elem.insert("DistributionAddress", &tags::DistributionAddress);
		tag_to_elem.insert(0x4008011A, &tags::DistributionAddress);

		ident_to_elem.insert("InterpretationID", &tags::InterpretationID);
		tag_to_elem.insert(0x40080200, &tags::InterpretationID);

		ident_to_elem.insert("InterpretationIDIssuer", &tags::InterpretationIDIssuer);
		tag_to_elem.insert(0x40080202, &tags::InterpretationIDIssuer);

		ident_to_elem.insert("InterpretationTypeID", &tags::InterpretationTypeID);
		tag_to_elem.insert(0x40080210, &tags::InterpretationTypeID);

		ident_to_elem.insert("InterpretationStatusID", &tags::InterpretationStatusID);
		tag_to_elem.insert(0x40080212, &tags::InterpretationStatusID);

		ident_to_elem.insert("Impressions", &tags::Impressions);
		tag_to_elem.insert(0x40080300, &tags::Impressions);

		ident_to_elem.insert("ResultsComments", &tags::ResultsComments);
		tag_to_elem.insert(0x40084000, &tags::ResultsComments);

		ident_to_elem.insert("LowEnergyDetectors", &tags::LowEnergyDetectors);
		tag_to_elem.insert(0x40100001, &tags::LowEnergyDetectors);

		ident_to_elem.insert("HighEnergyDetectors", &tags::HighEnergyDetectors);
		tag_to_elem.insert(0x40100002, &tags::HighEnergyDetectors);

		ident_to_elem.insert("DetectorGeometrySequence", &tags::DetectorGeometrySequence);
		tag_to_elem.insert(0x40100004, &tags::DetectorGeometrySequence);

		ident_to_elem.insert("ThreatROIVoxelSequence", &tags::ThreatROIVoxelSequence);
		tag_to_elem.insert(0x40101001, &tags::ThreatROIVoxelSequence);

		ident_to_elem.insert("ThreatROIBase", &tags::ThreatROIBase);
		tag_to_elem.insert(0x40101004, &tags::ThreatROIBase);

		ident_to_elem.insert("ThreatROIExtents", &tags::ThreatROIExtents);
		tag_to_elem.insert(0x40101005, &tags::ThreatROIExtents);

		ident_to_elem.insert("ThreatROIBitmap", &tags::ThreatROIBitmap);
		tag_to_elem.insert(0x40101006, &tags::ThreatROIBitmap);

		ident_to_elem.insert("RouteSegmentID", &tags::RouteSegmentID);
		tag_to_elem.insert(0x40101007, &tags::RouteSegmentID);

		ident_to_elem.insert("GantryType", &tags::GantryType);
		tag_to_elem.insert(0x40101008, &tags::GantryType);

		ident_to_elem.insert("OOIOwnerType", &tags::OOIOwnerType);
		tag_to_elem.insert(0x40101009, &tags::OOIOwnerType);

		ident_to_elem.insert("RouteSegmentSequence", &tags::RouteSegmentSequence);
		tag_to_elem.insert(0x4010100A, &tags::RouteSegmentSequence);

		ident_to_elem.insert("PotentialThreatObjectID", &tags::PotentialThreatObjectID);
		tag_to_elem.insert(0x40101010, &tags::PotentialThreatObjectID);

		ident_to_elem.insert("ThreatSequence", &tags::ThreatSequence);
		tag_to_elem.insert(0x40101011, &tags::ThreatSequence);

		ident_to_elem.insert("ThreatCategory", &tags::ThreatCategory);
		tag_to_elem.insert(0x40101012, &tags::ThreatCategory);

		ident_to_elem.insert("ThreatCategoryDescription", &tags::ThreatCategoryDescription);
		tag_to_elem.insert(0x40101013, &tags::ThreatCategoryDescription);

		ident_to_elem.insert("ATDAbilityAssessment", &tags::ATDAbilityAssessment);
		tag_to_elem.insert(0x40101014, &tags::ATDAbilityAssessment);

		ident_to_elem.insert("ATDAssessmentFlag", &tags::ATDAssessmentFlag);
		tag_to_elem.insert(0x40101015, &tags::ATDAssessmentFlag);

		ident_to_elem.insert("ATDAssessmentProbability", &tags::ATDAssessmentProbability);
		tag_to_elem.insert(0x40101016, &tags::ATDAssessmentProbability);

		ident_to_elem.insert("Mass", &tags::Mass);
		tag_to_elem.insert(0x40101017, &tags::Mass);

		ident_to_elem.insert("Density", &tags::Density);
		tag_to_elem.insert(0x40101018, &tags::Density);

		ident_to_elem.insert("ZEffective", &tags::ZEffective);
		tag_to_elem.insert(0x40101019, &tags::ZEffective);

		ident_to_elem.insert("BoardingPassID", &tags::BoardingPassID);
		tag_to_elem.insert(0x4010101A, &tags::BoardingPassID);

		ident_to_elem.insert("CenterOfMass", &tags::CenterOfMass);
		tag_to_elem.insert(0x4010101B, &tags::CenterOfMass);

		ident_to_elem.insert("CenterOfPTO", &tags::CenterOfPTO);
		tag_to_elem.insert(0x4010101C, &tags::CenterOfPTO);

		ident_to_elem.insert("BoundingPolygon", &tags::BoundingPolygon);
		tag_to_elem.insert(0x4010101D, &tags::BoundingPolygon);

		ident_to_elem.insert("RouteSegmentStartLocationID", &tags::RouteSegmentStartLocationID);
		tag_to_elem.insert(0x4010101E, &tags::RouteSegmentStartLocationID);

		ident_to_elem.insert("RouteSegmentEndLocationID", &tags::RouteSegmentEndLocationID);
		tag_to_elem.insert(0x4010101F, &tags::RouteSegmentEndLocationID);

		ident_to_elem.insert("RouteSegmentLocationIDType", &tags::RouteSegmentLocationIDType);
		tag_to_elem.insert(0x40101020, &tags::RouteSegmentLocationIDType);

		ident_to_elem.insert("AbortReason", &tags::AbortReason);
		tag_to_elem.insert(0x40101021, &tags::AbortReason);

		ident_to_elem.insert("VolumeOfPTO", &tags::VolumeOfPTO);
		tag_to_elem.insert(0x40101023, &tags::VolumeOfPTO);

		ident_to_elem.insert("AbortFlag", &tags::AbortFlag);
		tag_to_elem.insert(0x40101024, &tags::AbortFlag);

		ident_to_elem.insert("RouteSegmentStartTime", &tags::RouteSegmentStartTime);
		tag_to_elem.insert(0x40101025, &tags::RouteSegmentStartTime);

		ident_to_elem.insert("RouteSegmentEndTime", &tags::RouteSegmentEndTime);
		tag_to_elem.insert(0x40101026, &tags::RouteSegmentEndTime);

		ident_to_elem.insert("TDRType", &tags::TDRType);
		tag_to_elem.insert(0x40101027, &tags::TDRType);

		ident_to_elem.insert("InternationalRouteSegment", &tags::InternationalRouteSegment);
		tag_to_elem.insert(0x40101028, &tags::InternationalRouteSegment);

		ident_to_elem.insert("ThreatDetectionAlgorithmandVersion", &tags::ThreatDetectionAlgorithmandVersion);
		tag_to_elem.insert(0x40101029, &tags::ThreatDetectionAlgorithmandVersion);

		ident_to_elem.insert("AssignedLocation", &tags::AssignedLocation);
		tag_to_elem.insert(0x4010102A, &tags::AssignedLocation);

		ident_to_elem.insert("AlarmDecisionTime", &tags::AlarmDecisionTime);
		tag_to_elem.insert(0x4010102B, &tags::AlarmDecisionTime);

		ident_to_elem.insert("AlarmDecision", &tags::AlarmDecision);
		tag_to_elem.insert(0x40101031, &tags::AlarmDecision);

		ident_to_elem.insert("NumberOfTotalObjects", &tags::NumberOfTotalObjects);
		tag_to_elem.insert(0x40101033, &tags::NumberOfTotalObjects);

		ident_to_elem.insert("NumberOfAlarmObjects", &tags::NumberOfAlarmObjects);
		tag_to_elem.insert(0x40101034, &tags::NumberOfAlarmObjects);

		ident_to_elem.insert("PTORepresentationSequence", &tags::PTORepresentationSequence);
		tag_to_elem.insert(0x40101037, &tags::PTORepresentationSequence);

		ident_to_elem.insert("ATDAssessmentSequence", &tags::ATDAssessmentSequence);
		tag_to_elem.insert(0x40101038, &tags::ATDAssessmentSequence);

		ident_to_elem.insert("TIPType", &tags::TIPType);
		tag_to_elem.insert(0x40101039, &tags::TIPType);

		ident_to_elem.insert("DICOSVersion", &tags::DICOSVersion);
		tag_to_elem.insert(0x4010103A, &tags::DICOSVersion);

		ident_to_elem.insert("OOIOwnerCreationTime", &tags::OOIOwnerCreationTime);
		tag_to_elem.insert(0x40101041, &tags::OOIOwnerCreationTime);

		ident_to_elem.insert("OOIType", &tags::OOIType);
		tag_to_elem.insert(0x40101042, &tags::OOIType);

		ident_to_elem.insert("OOISize", &tags::OOISize);
		tag_to_elem.insert(0x40101043, &tags::OOISize);

		ident_to_elem.insert("AcquisitionStatus", &tags::AcquisitionStatus);
		tag_to_elem.insert(0x40101044, &tags::AcquisitionStatus);

		ident_to_elem.insert("BasisMaterialsCodeSequence", &tags::BasisMaterialsCodeSequence);
		tag_to_elem.insert(0x40101045, &tags::BasisMaterialsCodeSequence);

		ident_to_elem.insert("PhantomType", &tags::PhantomType);
		tag_to_elem.insert(0x40101046, &tags::PhantomType);

		ident_to_elem.insert("OOIOwnerSequence", &tags::OOIOwnerSequence);
		tag_to_elem.insert(0x40101047, &tags::OOIOwnerSequence);

		ident_to_elem.insert("ScanType", &tags::ScanType);
		tag_to_elem.insert(0x40101048, &tags::ScanType);

		ident_to_elem.insert("ItineraryID", &tags::ItineraryID);
		tag_to_elem.insert(0x40101051, &tags::ItineraryID);

		ident_to_elem.insert("ItineraryIDType", &tags::ItineraryIDType);
		tag_to_elem.insert(0x40101052, &tags::ItineraryIDType);

		ident_to_elem.insert("ItineraryIDAssigningAuthority", &tags::ItineraryIDAssigningAuthority);
		tag_to_elem.insert(0x40101053, &tags::ItineraryIDAssigningAuthority);

		ident_to_elem.insert("RouteID", &tags::RouteID);
		tag_to_elem.insert(0x40101054, &tags::RouteID);

		ident_to_elem.insert("RouteIDAssigningAuthority", &tags::RouteIDAssigningAuthority);
		tag_to_elem.insert(0x40101055, &tags::RouteIDAssigningAuthority);

		ident_to_elem.insert("InboundArrivalType", &tags::InboundArrivalType);
		tag_to_elem.insert(0x40101056, &tags::InboundArrivalType);

		ident_to_elem.insert("CarrierID", &tags::CarrierID);
		tag_to_elem.insert(0x40101058, &tags::CarrierID);

		ident_to_elem.insert("CarrierIDAssigningAuthority", &tags::CarrierIDAssigningAuthority);
		tag_to_elem.insert(0x40101059, &tags::CarrierIDAssigningAuthority);

		ident_to_elem.insert("SourceOrientation", &tags::SourceOrientation);
		tag_to_elem.insert(0x40101060, &tags::SourceOrientation);

		ident_to_elem.insert("SourcePosition", &tags::SourcePosition);
		tag_to_elem.insert(0x40101061, &tags::SourcePosition);

		ident_to_elem.insert("BeltHeight", &tags::BeltHeight);
		tag_to_elem.insert(0x40101062, &tags::BeltHeight);

		ident_to_elem.insert("AlgorithmRoutingCodeSequence", &tags::AlgorithmRoutingCodeSequence);
		tag_to_elem.insert(0x40101064, &tags::AlgorithmRoutingCodeSequence);

		ident_to_elem.insert("TransportClassification", &tags::TransportClassification);
		tag_to_elem.insert(0x40101067, &tags::TransportClassification);

		ident_to_elem.insert("OOITypeDescriptor", &tags::OOITypeDescriptor);
		tag_to_elem.insert(0x40101068, &tags::OOITypeDescriptor);

		ident_to_elem.insert("TotalProcessingTime", &tags::TotalProcessingTime);
		tag_to_elem.insert(0x40101069, &tags::TotalProcessingTime);

		ident_to_elem.insert("DetectorCalibrationData", &tags::DetectorCalibrationData);
		tag_to_elem.insert(0x4010106C, &tags::DetectorCalibrationData);

		ident_to_elem.insert("AdditionalScreeningPerformed", &tags::AdditionalScreeningPerformed);
		tag_to_elem.insert(0x4010106D, &tags::AdditionalScreeningPerformed);

		ident_to_elem.insert("AdditionalInspectionSelectionCriteria", &tags::AdditionalInspectionSelectionCriteria);
		tag_to_elem.insert(0x4010106E, &tags::AdditionalInspectionSelectionCriteria);

		ident_to_elem.insert("AdditionalInspectionMethodSequence", &tags::AdditionalInspectionMethodSequence);
		tag_to_elem.insert(0x4010106F, &tags::AdditionalInspectionMethodSequence);

		ident_to_elem.insert("AITDeviceType", &tags::AITDeviceType);
		tag_to_elem.insert(0x40101070, &tags::AITDeviceType);

		ident_to_elem.insert("QRMeasurementsSequence", &tags::QRMeasurementsSequence);
		tag_to_elem.insert(0x40101071, &tags::QRMeasurementsSequence);

		ident_to_elem.insert("TargetMaterialSequence", &tags::TargetMaterialSequence);
		tag_to_elem.insert(0x40101072, &tags::TargetMaterialSequence);

		ident_to_elem.insert("SNRThreshold", &tags::SNRThreshold);
		tag_to_elem.insert(0x40101073, &tags::SNRThreshold);

		ident_to_elem.insert("ImageScaleRepresentation", &tags::ImageScaleRepresentation);
		tag_to_elem.insert(0x40101075, &tags::ImageScaleRepresentation);

		ident_to_elem.insert("ReferencedPTOSequence", &tags::ReferencedPTOSequence);
		tag_to_elem.insert(0x40101076, &tags::ReferencedPTOSequence);

		ident_to_elem.insert("ReferencedTDRInstanceSequence", &tags::ReferencedTDRInstanceSequence);
		tag_to_elem.insert(0x40101077, &tags::ReferencedTDRInstanceSequence);

		ident_to_elem.insert("PTOLocationDescription", &tags::PTOLocationDescription);
		tag_to_elem.insert(0x40101078, &tags::PTOLocationDescription);

		ident_to_elem.insert("AnomalyLocatorIndicatorSequence", &tags::AnomalyLocatorIndicatorSequence);
		tag_to_elem.insert(0x40101079, &tags::AnomalyLocatorIndicatorSequence);

		ident_to_elem.insert("AnomalyLocatorIndicator", &tags::AnomalyLocatorIndicator);
		tag_to_elem.insert(0x4010107A, &tags::AnomalyLocatorIndicator);

		ident_to_elem.insert("PTORegionSequence", &tags::PTORegionSequence);
		tag_to_elem.insert(0x4010107B, &tags::PTORegionSequence);

		ident_to_elem.insert("InspectionSelectionCriteria", &tags::InspectionSelectionCriteria);
		tag_to_elem.insert(0x4010107C, &tags::InspectionSelectionCriteria);

		ident_to_elem.insert("SecondaryInspectionMethodSequence", &tags::SecondaryInspectionMethodSequence);
		tag_to_elem.insert(0x4010107D, &tags::SecondaryInspectionMethodSequence);

		ident_to_elem.insert("PRCSToRCSOrientation", &tags::PRCSToRCSOrientation);
		tag_to_elem.insert(0x4010107E, &tags::PRCSToRCSOrientation);

		ident_to_elem.insert("MACParametersSequence", &tags::MACParametersSequence);
		tag_to_elem.insert(0x4FFE0001, &tags::MACParametersSequence);

		ident_to_elem.insert("SharedFunctionalGroupsSequence", &tags::SharedFunctionalGroupsSequence);
		tag_to_elem.insert(0x52009229, &tags::SharedFunctionalGroupsSequence);

		ident_to_elem.insert("PerFrameFunctionalGroupsSequence", &tags::PerFrameFunctionalGroupsSequence);
		tag_to_elem.insert(0x52009230, &tags::PerFrameFunctionalGroupsSequence);

		ident_to_elem.insert("WaveformSequence", &tags::WaveformSequence);
		tag_to_elem.insert(0x54000100, &tags::WaveformSequence);

		ident_to_elem.insert("ChannelMinimumValue", &tags::ChannelMinimumValue);
		tag_to_elem.insert(0x54000110, &tags::ChannelMinimumValue);

		ident_to_elem.insert("ChannelMaximumValue", &tags::ChannelMaximumValue);
		tag_to_elem.insert(0x54000112, &tags::ChannelMaximumValue);

		ident_to_elem.insert("WaveformBitsAllocated", &tags::WaveformBitsAllocated);
		tag_to_elem.insert(0x54001004, &tags::WaveformBitsAllocated);

		ident_to_elem.insert("WaveformSampleInterpretation", &tags::WaveformSampleInterpretation);
		tag_to_elem.insert(0x54001006, &tags::WaveformSampleInterpretation);

		ident_to_elem.insert("WaveformPaddingValue", &tags::WaveformPaddingValue);
		tag_to_elem.insert(0x5400100A, &tags::WaveformPaddingValue);

		ident_to_elem.insert("WaveformData", &tags::WaveformData);
		tag_to_elem.insert(0x54001010, &tags::WaveformData);

		ident_to_elem.insert("FirstOrderPhaseCorrectionAngle", &tags::FirstOrderPhaseCorrectionAngle);
		tag_to_elem.insert(0x56000010, &tags::FirstOrderPhaseCorrectionAngle);

		ident_to_elem.insert("SpectroscopyData", &tags::SpectroscopyData);
		tag_to_elem.insert(0x56000020, &tags::SpectroscopyData);

		ident_to_elem.insert("FloatPixelData", &tags::FloatPixelData);
		tag_to_elem.insert(0x7FE00008, &tags::FloatPixelData);

		ident_to_elem.insert("DoubleFloatPixelData", &tags::DoubleFloatPixelData);
		tag_to_elem.insert(0x7FE00009, &tags::DoubleFloatPixelData);

		ident_to_elem.insert("PixelData", &tags::PixelData);
		tag_to_elem.insert(0x7FE00010, &tags::PixelData);

		ident_to_elem.insert("CoefficientsSDVN", &tags::CoefficientsSDVN);
		tag_to_elem.insert(0x7FE00020, &tags::CoefficientsSDVN);

		ident_to_elem.insert("CoefficientsSDHN", &tags::CoefficientsSDHN);
		tag_to_elem.insert(0x7FE00030, &tags::CoefficientsSDHN);

		ident_to_elem.insert("CoefficientsSDDN", &tags::CoefficientsSDDN);
		tag_to_elem.insert(0x7FE00040, &tags::CoefficientsSDDN);

		ident_to_elem.insert("DigitalSignaturesSequence", &tags::DigitalSignaturesSequence);
		tag_to_elem.insert(0xFFFAFFFA, &tags::DigitalSignaturesSequence);

		ident_to_elem.insert("DataSetTrailingPadding", &tags::DataSetTrailingPadding);
		tag_to_elem.insert(0xFFFCFFFC, &tags::DataSetTrailingPadding);

		ident_to_elem.insert("Item", &tags::Item);
		tag_to_elem.insert(0xFFFEE000, &tags::Item);

		ident_to_elem.insert("ItemDelimitationItem", &tags::ItemDelimitationItem);
		tag_to_elem.insert(0xFFFEE00D, &tags::ItemDelimitationItem);

		ident_to_elem.insert("SequenceDelimitationItem", &tags::SequenceDelimitationItem);
		tag_to_elem.insert(0xFFFEE0DD, &tags::SequenceDelimitationItem);

		ident_to_elem.insert("FileMetaInformationGroupLength", &fme::FileMetaInformationGroupLength);
		tag_to_elem.insert(0x00020000, &fme::FileMetaInformationGroupLength);

		ident_to_elem.insert("FileMetaInformationVersion", &fme::FileMetaInformationVersion);
		tag_to_elem.insert(0x00020001, &fme::FileMetaInformationVersion);

		ident_to_elem.insert("MediaStorageSOPClassUID", &fme::MediaStorageSOPClassUID);
		tag_to_elem.insert(0x00020002, &fme::MediaStorageSOPClassUID);

		ident_to_elem.insert("MediaStorageSOPInstanceUID", &fme::MediaStorageSOPInstanceUID);
		tag_to_elem.insert(0x00020003, &fme::MediaStorageSOPInstanceUID);

		ident_to_elem.insert("TransferSyntaxUID", &fme::TransferSyntaxUID);
		tag_to_elem.insert(0x00020010, &fme::TransferSyntaxUID);

		ident_to_elem.insert("ImplementationClassUID", &fme::ImplementationClassUID);
		tag_to_elem.insert(0x00020012, &fme::ImplementationClassUID);

		ident_to_elem.insert("ImplementationVersionName", &fme::ImplementationVersionName);
		tag_to_elem.insert(0x00020013, &fme::ImplementationVersionName);

		ident_to_elem.insert("SourceApplicationEntityTitle", &fme::SourceApplicationEntityTitle);
		tag_to_elem.insert(0x00020016, &fme::SourceApplicationEntityTitle);

		ident_to_elem.insert("SendingApplicationEntityTitle", &fme::SendingApplicationEntityTitle);
		tag_to_elem.insert(0x00020017, &fme::SendingApplicationEntityTitle);

		ident_to_elem.insert("ReceivingApplicationEntityTitle", &fme::ReceivingApplicationEntityTitle);
		tag_to_elem.insert(0x00020018, &fme::ReceivingApplicationEntityTitle);

		ident_to_elem.insert("PrivateInformationCreatorUID", &fme::PrivateInformationCreatorUID);
		tag_to_elem.insert(0x00020100, &fme::PrivateInformationCreatorUID);

		ident_to_elem.insert("PrivateInformation", &fme::PrivateInformation);
		tag_to_elem.insert(0x00020102, &fme::PrivateInformation);

		ident_to_elem.insert("FileSetID", &dse::FileSetID);
		tag_to_elem.insert(0x00041130, &dse::FileSetID);

		ident_to_elem.insert("FileSetDescriptorFileID", &dse::FileSetDescriptorFileID);
		tag_to_elem.insert(0x00041141, &dse::FileSetDescriptorFileID);

		ident_to_elem.insert("SpecificCharacterSetOfFileSetDescriptorFile", &dse::SpecificCharacterSetOfFileSetDescriptorFile);
		tag_to_elem.insert(0x00041142, &dse::SpecificCharacterSetOfFileSetDescriptorFile);

		ident_to_elem.insert("OffsetOfTheFirstDirectoryRecordOfTheRootDirectoryEntity", &dse::OffsetOfTheFirstDirectoryRecordOfTheRootDirectoryEntity);
		tag_to_elem.insert(0x00041200, &dse::OffsetOfTheFirstDirectoryRecordOfTheRootDirectoryEntity);

		ident_to_elem.insert("OffsetOfTheLastDirectoryRecordOfTheRootDirectoryEntity", &dse::OffsetOfTheLastDirectoryRecordOfTheRootDirectoryEntity);
		tag_to_elem.insert(0x00041202, &dse::OffsetOfTheLastDirectoryRecordOfTheRootDirectoryEntity);

		ident_to_elem.insert("FileSetConsistencyFlag", &dse::FileSetConsistencyFlag);
		tag_to_elem.insert(0x00041212, &dse::FileSetConsistencyFlag);

		ident_to_elem.insert("DirectoryRecordSequence", &dse::DirectoryRecordSequence);
		tag_to_elem.insert(0x00041220, &dse::DirectoryRecordSequence);

		ident_to_elem.insert("OffsetOfTheNextDirectoryRecord", &dse::OffsetOfTheNextDirectoryRecord);
		tag_to_elem.insert(0x00041400, &dse::OffsetOfTheNextDirectoryRecord);

		ident_to_elem.insert("RecordInUseFlag", &dse::RecordInUseFlag);
		tag_to_elem.insert(0x00041410, &dse::RecordInUseFlag);

		ident_to_elem.insert("OffsetOfReferencedLowerLevelDirectoryEntity", &dse::OffsetOfReferencedLowerLevelDirectoryEntity);
		tag_to_elem.insert(0x00041420, &dse::OffsetOfReferencedLowerLevelDirectoryEntity);

		ident_to_elem.insert("DirectoryRecordType", &dse::DirectoryRecordType);
		tag_to_elem.insert(0x00041430, &dse::DirectoryRecordType);

		ident_to_elem.insert("PrivateRecordUID", &dse::PrivateRecordUID);
		tag_to_elem.insert(0x00041432, &dse::PrivateRecordUID);

		ident_to_elem.insert("ReferencedFileID", &dse::ReferencedFileID);
		tag_to_elem.insert(0x00041500, &dse::ReferencedFileID);

		ident_to_elem.insert("MRDRDirectoryRecordOffset", &dse::MRDRDirectoryRecordOffset);
		tag_to_elem.insert(0x00041504, &dse::MRDRDirectoryRecordOffset);

		ident_to_elem.insert("ReferencedSOPClassUIDInFile", &dse::ReferencedSOPClassUIDInFile);
		tag_to_elem.insert(0x00041510, &dse::ReferencedSOPClassUIDInFile);

		ident_to_elem.insert("ReferencedSOPInstanceUIDInFile", &dse::ReferencedSOPInstanceUIDInFile);
		tag_to_elem.insert(0x00041511, &dse::ReferencedSOPInstanceUIDInFile);

		ident_to_elem.insert("ReferencedTransferSyntaxUIDInFile", &dse::ReferencedTransferSyntaxUIDInFile);
		tag_to_elem.insert(0x00041512, &dse::ReferencedTransferSyntaxUIDInFile);

		ident_to_elem.insert("ReferencedRelatedGeneralSOPClassUIDInFile", &dse::ReferencedRelatedGeneralSOPClassUIDInFile);
		tag_to_elem.insert(0x0004151A, &dse::ReferencedRelatedGeneralSOPClassUIDInFile);

		ident_to_elem.insert("NumberOfReferences", &dse::NumberOfReferences);
		tag_to_elem.insert(0x00041600, &dse::NumberOfReferences);


		TagLookup {
			ident_to_elem: ident_to_elem,
			tag_to_elem: tag_to_elem,
		}
	}
}

