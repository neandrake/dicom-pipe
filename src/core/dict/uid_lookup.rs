//! This is an auto-generated file. Do not make modifications here.

use core::dict::uids;
use core::uid::UID;

use std::collections::hash_map::HashMap;

pub struct UidLookup {
	ident_to_uid: HashMap<&'static str, &'static UID>,
	id_to_uid: HashMap<&'static str, &'static UID>,
}

impl UidLookup {
	pub fn by_ident(&self, ident: &str) -> Option<&'static UID> {
		self.ident_to_uid.get(ident).map(|uid| *uid)
	}

	pub fn by_id(&self, id: &str) -> Option<&'static UID> {
		self.id_to_uid.get(id).map(|uid| *uid)
	}

	pub fn new() -> UidLookup {
		let mut ident_to_uid: HashMap<&'static str, &'static UID> = HashMap::new();
		let mut id_to_uid: HashMap<&'static str, &'static UID> = HashMap::new();

		ident_to_uid.insert("VerificationSOPClass", &uids::VerificationSOPClass);
		id_to_uid.insert("1.2.840.10008.1.1", &uids::VerificationSOPClass);

		ident_to_uid.insert("ImplicitVRLittleEndian", &uids::ImplicitVRLittleEndian);
		id_to_uid.insert("1.2.840.10008.1.2", &uids::ImplicitVRLittleEndian);

		ident_to_uid.insert("ExplicitVRLittleEndian", &uids::ExplicitVRLittleEndian);
		id_to_uid.insert("1.2.840.10008.1.2.1", &uids::ExplicitVRLittleEndian);

		ident_to_uid.insert("DeflatedExplicitVRLittleEndian", &uids::DeflatedExplicitVRLittleEndian);
		id_to_uid.insert("1.2.840.10008.1.2.1.99", &uids::DeflatedExplicitVRLittleEndian);

		ident_to_uid.insert("ExplicitVRBigEndian", &uids::ExplicitVRBigEndian);
		id_to_uid.insert("1.2.840.10008.1.2.2", &uids::ExplicitVRBigEndian);

		ident_to_uid.insert("JPEGBaselineProcess1", &uids::JPEGBaselineProcess1);
		id_to_uid.insert("1.2.840.10008.1.2.4.50", &uids::JPEGBaselineProcess1);

		ident_to_uid.insert("JPEGExtendedProcess2_and_4", &uids::JPEGExtendedProcess2_and_4);
		id_to_uid.insert("1.2.840.10008.1.2.4.51", &uids::JPEGExtendedProcess2_and_4);

		ident_to_uid.insert("JPEGExtendedProcess3_and_5", &uids::JPEGExtendedProcess3_and_5);
		id_to_uid.insert("1.2.840.10008.1.2.4.52", &uids::JPEGExtendedProcess3_and_5);

		ident_to_uid.insert("JPEGSpectralSelectionNonHierarchicalProcess6_and_8", &uids::JPEGSpectralSelectionNonHierarchicalProcess6_and_8);
		id_to_uid.insert("1.2.840.10008.1.2.4.53", &uids::JPEGSpectralSelectionNonHierarchicalProcess6_and_8);

		ident_to_uid.insert("JPEGSpectralSelectionNonHierarchicalProcess7_and_9", &uids::JPEGSpectralSelectionNonHierarchicalProcess7_and_9);
		id_to_uid.insert("1.2.840.10008.1.2.4.54", &uids::JPEGSpectralSelectionNonHierarchicalProcess7_and_9);

		ident_to_uid.insert("JPEGFullProgressionNonHierarchicalProcess10_and_12", &uids::JPEGFullProgressionNonHierarchicalProcess10_and_12);
		id_to_uid.insert("1.2.840.10008.1.2.4.55", &uids::JPEGFullProgressionNonHierarchicalProcess10_and_12);

		ident_to_uid.insert("JPEGFullProgressionNonHierarchicalProcess11_and_13", &uids::JPEGFullProgressionNonHierarchicalProcess11_and_13);
		id_to_uid.insert("1.2.840.10008.1.2.4.56", &uids::JPEGFullProgressionNonHierarchicalProcess11_and_13);

		ident_to_uid.insert("JPEGLosslessNonHierarchicalProcess14", &uids::JPEGLosslessNonHierarchicalProcess14);
		id_to_uid.insert("1.2.840.10008.1.2.4.57", &uids::JPEGLosslessNonHierarchicalProcess14);

		ident_to_uid.insert("JPEGLosslessNonHierarchicalProcess15", &uids::JPEGLosslessNonHierarchicalProcess15);
		id_to_uid.insert("1.2.840.10008.1.2.4.58", &uids::JPEGLosslessNonHierarchicalProcess15);

		ident_to_uid.insert("JPEGExtendedHierarchicalProcess16_and_18", &uids::JPEGExtendedHierarchicalProcess16_and_18);
		id_to_uid.insert("1.2.840.10008.1.2.4.59", &uids::JPEGExtendedHierarchicalProcess16_and_18);

		ident_to_uid.insert("JPEGExtendedHierarchicalProcess17_and_19", &uids::JPEGExtendedHierarchicalProcess17_and_19);
		id_to_uid.insert("1.2.840.10008.1.2.4.60", &uids::JPEGExtendedHierarchicalProcess17_and_19);

		ident_to_uid.insert("JPEGSpectralSelectionHierarchicalProcess20_and_22", &uids::JPEGSpectralSelectionHierarchicalProcess20_and_22);
		id_to_uid.insert("1.2.840.10008.1.2.4.61", &uids::JPEGSpectralSelectionHierarchicalProcess20_and_22);

		ident_to_uid.insert("JPEGSpectralSelectionHierarchicalProcess21_and_23", &uids::JPEGSpectralSelectionHierarchicalProcess21_and_23);
		id_to_uid.insert("1.2.840.10008.1.2.4.62", &uids::JPEGSpectralSelectionHierarchicalProcess21_and_23);

		ident_to_uid.insert("JPEGFullProgressionHierarchicalProcess24_and_26", &uids::JPEGFullProgressionHierarchicalProcess24_and_26);
		id_to_uid.insert("1.2.840.10008.1.2.4.63", &uids::JPEGFullProgressionHierarchicalProcess24_and_26);

		ident_to_uid.insert("JPEGFullProgressionHierarchicalProcess25_and_27", &uids::JPEGFullProgressionHierarchicalProcess25_and_27);
		id_to_uid.insert("1.2.840.10008.1.2.4.64", &uids::JPEGFullProgressionHierarchicalProcess25_and_27);

		ident_to_uid.insert("JPEGLosslessHierarchicalProcess28", &uids::JPEGLosslessHierarchicalProcess28);
		id_to_uid.insert("1.2.840.10008.1.2.4.65", &uids::JPEGLosslessHierarchicalProcess28);

		ident_to_uid.insert("JPEGLosslessHierarchicalProcess29", &uids::JPEGLosslessHierarchicalProcess29);
		id_to_uid.insert("1.2.840.10008.1.2.4.66", &uids::JPEGLosslessHierarchicalProcess29);

		ident_to_uid.insert("JPEGLosslessNonHierarchicalFirstOrderPredictionProcess14SelectionValue1", &uids::JPEGLosslessNonHierarchicalFirstOrderPredictionProcess14SelectionValue1);
		id_to_uid.insert("1.2.840.10008.1.2.4.70", &uids::JPEGLosslessNonHierarchicalFirstOrderPredictionProcess14SelectionValue1);

		ident_to_uid.insert("JPEGLSLosslessImageCompression", &uids::JPEGLSLosslessImageCompression);
		id_to_uid.insert("1.2.840.10008.1.2.4.80", &uids::JPEGLSLosslessImageCompression);

		ident_to_uid.insert("JPEGLSLossyNearLosslessImageCompression", &uids::JPEGLSLossyNearLosslessImageCompression);
		id_to_uid.insert("1.2.840.10008.1.2.4.81", &uids::JPEGLSLossyNearLosslessImageCompression);

		ident_to_uid.insert("JPEG2000ImageCompressionLosslessOnly", &uids::JPEG2000ImageCompressionLosslessOnly);
		id_to_uid.insert("1.2.840.10008.1.2.4.90", &uids::JPEG2000ImageCompressionLosslessOnly);

		ident_to_uid.insert("JPEG2000ImageCompression", &uids::JPEG2000ImageCompression);
		id_to_uid.insert("1.2.840.10008.1.2.4.91", &uids::JPEG2000ImageCompression);

		ident_to_uid.insert("JPEG2000Part2MulticomponentImageCompressionLosslessOnly", &uids::JPEG2000Part2MulticomponentImageCompressionLosslessOnly);
		id_to_uid.insert("1.2.840.10008.1.2.4.92", &uids::JPEG2000Part2MulticomponentImageCompressionLosslessOnly);

		ident_to_uid.insert("JPEG2000Part2MulticomponentImageCompression", &uids::JPEG2000Part2MulticomponentImageCompression);
		id_to_uid.insert("1.2.840.10008.1.2.4.93", &uids::JPEG2000Part2MulticomponentImageCompression);

		ident_to_uid.insert("JPIPReferenced", &uids::JPIPReferenced);
		id_to_uid.insert("1.2.840.10008.1.2.4.94", &uids::JPIPReferenced);

		ident_to_uid.insert("JPIPReferencedDeflate", &uids::JPIPReferencedDeflate);
		id_to_uid.insert("1.2.840.10008.1.2.4.95", &uids::JPIPReferencedDeflate);

		ident_to_uid.insert("MPEG2MainProfileMainLevel", &uids::MPEG2MainProfileMainLevel);
		id_to_uid.insert("1.2.840.10008.1.2.4.100", &uids::MPEG2MainProfileMainLevel);

		ident_to_uid.insert("MPEG2MainProfileHighLevel", &uids::MPEG2MainProfileHighLevel);
		id_to_uid.insert("1.2.840.10008.1.2.4.101", &uids::MPEG2MainProfileHighLevel);

		ident_to_uid.insert("MPEG4AVCH264HighProfileLevel41", &uids::MPEG4AVCH264HighProfileLevel41);
		id_to_uid.insert("1.2.840.10008.1.2.4.102", &uids::MPEG4AVCH264HighProfileLevel41);

		ident_to_uid.insert("MPEG4AVCH264BDcompatibleHighProfileLevel41", &uids::MPEG4AVCH264BDcompatibleHighProfileLevel41);
		id_to_uid.insert("1.2.840.10008.1.2.4.103", &uids::MPEG4AVCH264BDcompatibleHighProfileLevel41);

		ident_to_uid.insert("MPEG4AVCH264HighProfileLevel42For2DVideo", &uids::MPEG4AVCH264HighProfileLevel42For2DVideo);
		id_to_uid.insert("1.2.840.10008.1.2.4.104", &uids::MPEG4AVCH264HighProfileLevel42For2DVideo);

		ident_to_uid.insert("MPEG4AVCH264HighProfileLevel42For3DVideo", &uids::MPEG4AVCH264HighProfileLevel42For3DVideo);
		id_to_uid.insert("1.2.840.10008.1.2.4.105", &uids::MPEG4AVCH264HighProfileLevel42For3DVideo);

		ident_to_uid.insert("MPEG4AVCH264StereoHighProfileLevel42", &uids::MPEG4AVCH264StereoHighProfileLevel42);
		id_to_uid.insert("1.2.840.10008.1.2.4.106", &uids::MPEG4AVCH264StereoHighProfileLevel42);

		ident_to_uid.insert("HEVCH265MainProfileLevel51", &uids::HEVCH265MainProfileLevel51);
		id_to_uid.insert("1.2.840.10008.1.2.4.107", &uids::HEVCH265MainProfileLevel51);

		ident_to_uid.insert("HEVCH265Main10ProfileLevel51", &uids::HEVCH265Main10ProfileLevel51);
		id_to_uid.insert("1.2.840.10008.1.2.4.108", &uids::HEVCH265Main10ProfileLevel51);

		ident_to_uid.insert("RLELossless", &uids::RLELossless);
		id_to_uid.insert("1.2.840.10008.1.2.5", &uids::RLELossless);

		ident_to_uid.insert("RFC2557MIMEencapsulation", &uids::RFC2557MIMEencapsulation);
		id_to_uid.insert("1.2.840.10008.1.2.6.1", &uids::RFC2557MIMEencapsulation);

		ident_to_uid.insert("XMLEncoding", &uids::XMLEncoding);
		id_to_uid.insert("1.2.840.10008.1.2.6.2", &uids::XMLEncoding);

		ident_to_uid.insert("MediaStorageDirectoryStorage", &uids::MediaStorageDirectoryStorage);
		id_to_uid.insert("1.2.840.10008.1.3.10", &uids::MediaStorageDirectoryStorage);

		ident_to_uid.insert("TalairachBrainAtlasFrameofReference", &uids::TalairachBrainAtlasFrameofReference);
		id_to_uid.insert("1.2.840.10008.1.4.1.1", &uids::TalairachBrainAtlasFrameofReference);

		ident_to_uid.insert("SPM2T1FrameofReference", &uids::SPM2T1FrameofReference);
		id_to_uid.insert("1.2.840.10008.1.4.1.2", &uids::SPM2T1FrameofReference);

		ident_to_uid.insert("SPM2T2FrameofReference", &uids::SPM2T2FrameofReference);
		id_to_uid.insert("1.2.840.10008.1.4.1.3", &uids::SPM2T2FrameofReference);

		ident_to_uid.insert("SPM2PDFrameofReference", &uids::SPM2PDFrameofReference);
		id_to_uid.insert("1.2.840.10008.1.4.1.4", &uids::SPM2PDFrameofReference);

		ident_to_uid.insert("SPM2EPIFrameofReference", &uids::SPM2EPIFrameofReference);
		id_to_uid.insert("1.2.840.10008.1.4.1.5", &uids::SPM2EPIFrameofReference);

		ident_to_uid.insert("SPM2FILT1FrameofReference", &uids::SPM2FILT1FrameofReference);
		id_to_uid.insert("1.2.840.10008.1.4.1.6", &uids::SPM2FILT1FrameofReference);

		ident_to_uid.insert("SPM2PETFrameofReference", &uids::SPM2PETFrameofReference);
		id_to_uid.insert("1.2.840.10008.1.4.1.7", &uids::SPM2PETFrameofReference);

		ident_to_uid.insert("SPM2TRANSMFrameofReference", &uids::SPM2TRANSMFrameofReference);
		id_to_uid.insert("1.2.840.10008.1.4.1.8", &uids::SPM2TRANSMFrameofReference);

		ident_to_uid.insert("SPM2SPECTFrameofReference", &uids::SPM2SPECTFrameofReference);
		id_to_uid.insert("1.2.840.10008.1.4.1.9", &uids::SPM2SPECTFrameofReference);

		ident_to_uid.insert("SPM2GRAYFrameofReference", &uids::SPM2GRAYFrameofReference);
		id_to_uid.insert("1.2.840.10008.1.4.1.10", &uids::SPM2GRAYFrameofReference);

		ident_to_uid.insert("SPM2WHITEFrameofReference", &uids::SPM2WHITEFrameofReference);
		id_to_uid.insert("1.2.840.10008.1.4.1.11", &uids::SPM2WHITEFrameofReference);

		ident_to_uid.insert("SPM2CSFFrameofReference", &uids::SPM2CSFFrameofReference);
		id_to_uid.insert("1.2.840.10008.1.4.1.12", &uids::SPM2CSFFrameofReference);

		ident_to_uid.insert("SPM2BRAINMASKFrameofReference", &uids::SPM2BRAINMASKFrameofReference);
		id_to_uid.insert("1.2.840.10008.1.4.1.13", &uids::SPM2BRAINMASKFrameofReference);

		ident_to_uid.insert("SPM2AVG305T1FrameofReference", &uids::SPM2AVG305T1FrameofReference);
		id_to_uid.insert("1.2.840.10008.1.4.1.14", &uids::SPM2AVG305T1FrameofReference);

		ident_to_uid.insert("SPM2AVG152T1FrameofReference", &uids::SPM2AVG152T1FrameofReference);
		id_to_uid.insert("1.2.840.10008.1.4.1.15", &uids::SPM2AVG152T1FrameofReference);

		ident_to_uid.insert("SPM2AVG152T2FrameofReference", &uids::SPM2AVG152T2FrameofReference);
		id_to_uid.insert("1.2.840.10008.1.4.1.16", &uids::SPM2AVG152T2FrameofReference);

		ident_to_uid.insert("SPM2AVG152PDFrameofReference", &uids::SPM2AVG152PDFrameofReference);
		id_to_uid.insert("1.2.840.10008.1.4.1.17", &uids::SPM2AVG152PDFrameofReference);

		ident_to_uid.insert("SPM2SINGLESUBJT1FrameofReference", &uids::SPM2SINGLESUBJT1FrameofReference);
		id_to_uid.insert("1.2.840.10008.1.4.1.18", &uids::SPM2SINGLESUBJT1FrameofReference);

		ident_to_uid.insert("ICBM452T1FrameofReference", &uids::ICBM452T1FrameofReference);
		id_to_uid.insert("1.2.840.10008.1.4.2.1", &uids::ICBM452T1FrameofReference);

		ident_to_uid.insert("ICBMSingleSubjectMRIFrameofReference", &uids::ICBMSingleSubjectMRIFrameofReference);
		id_to_uid.insert("1.2.840.10008.1.4.2.2", &uids::ICBMSingleSubjectMRIFrameofReference);

		ident_to_uid.insert("HotIronColorPaletteSOPInstance", &uids::HotIronColorPaletteSOPInstance);
		id_to_uid.insert("1.2.840.10008.1.5.1", &uids::HotIronColorPaletteSOPInstance);

		ident_to_uid.insert("PETColorPaletteSOPInstance", &uids::PETColorPaletteSOPInstance);
		id_to_uid.insert("1.2.840.10008.1.5.2", &uids::PETColorPaletteSOPInstance);

		ident_to_uid.insert("HotMetalBlueColorPaletteSOPInstance", &uids::HotMetalBlueColorPaletteSOPInstance);
		id_to_uid.insert("1.2.840.10008.1.5.3", &uids::HotMetalBlueColorPaletteSOPInstance);

		ident_to_uid.insert("PET20StepColorPaletteSOPInstance", &uids::PET20StepColorPaletteSOPInstance);
		id_to_uid.insert("1.2.840.10008.1.5.4", &uids::PET20StepColorPaletteSOPInstance);

		ident_to_uid.insert("SpringColorPaletteSOPInstance", &uids::SpringColorPaletteSOPInstance);
		id_to_uid.insert("1.2.840.10008.1.5.5", &uids::SpringColorPaletteSOPInstance);

		ident_to_uid.insert("SummerColorPaletteSOPInstance", &uids::SummerColorPaletteSOPInstance);
		id_to_uid.insert("1.2.840.10008.1.5.6", &uids::SummerColorPaletteSOPInstance);

		ident_to_uid.insert("FallColorPaletteSOPInstance", &uids::FallColorPaletteSOPInstance);
		id_to_uid.insert("1.2.840.10008.1.5.7", &uids::FallColorPaletteSOPInstance);

		ident_to_uid.insert("WinterColorPaletteSOPInstance", &uids::WinterColorPaletteSOPInstance);
		id_to_uid.insert("1.2.840.10008.1.5.8", &uids::WinterColorPaletteSOPInstance);

		ident_to_uid.insert("BasicStudyContentNotificationSOPClass", &uids::BasicStudyContentNotificationSOPClass);
		id_to_uid.insert("1.2.840.10008.1.9", &uids::BasicStudyContentNotificationSOPClass);

		ident_to_uid.insert("Papyrus3ImplicitVRLittleEndian", &uids::Papyrus3ImplicitVRLittleEndian);
		id_to_uid.insert("1.2.840.10008.1.20", &uids::Papyrus3ImplicitVRLittleEndian);

		ident_to_uid.insert("StorageCommitmentPushModelSOPClass", &uids::StorageCommitmentPushModelSOPClass);
		id_to_uid.insert("1.2.840.10008.1.20.1", &uids::StorageCommitmentPushModelSOPClass);

		ident_to_uid.insert("StorageCommitmentPushModelSOPInstance", &uids::StorageCommitmentPushModelSOPInstance);
		id_to_uid.insert("1.2.840.10008.1.20.1.1", &uids::StorageCommitmentPushModelSOPInstance);

		ident_to_uid.insert("StorageCommitmentPullModelSOPClass", &uids::StorageCommitmentPullModelSOPClass);
		id_to_uid.insert("1.2.840.10008.1.20.2", &uids::StorageCommitmentPullModelSOPClass);

		ident_to_uid.insert("StorageCommitmentPullModelSOPInstance", &uids::StorageCommitmentPullModelSOPInstance);
		id_to_uid.insert("1.2.840.10008.1.20.2.1", &uids::StorageCommitmentPullModelSOPInstance);

		ident_to_uid.insert("ProceduralEventLoggingSOPClass", &uids::ProceduralEventLoggingSOPClass);
		id_to_uid.insert("1.2.840.10008.1.40", &uids::ProceduralEventLoggingSOPClass);

		ident_to_uid.insert("ProceduralEventLoggingSOPInstance", &uids::ProceduralEventLoggingSOPInstance);
		id_to_uid.insert("1.2.840.10008.1.40.1", &uids::ProceduralEventLoggingSOPInstance);

		ident_to_uid.insert("SubstanceAdministrationLoggingSOPClass", &uids::SubstanceAdministrationLoggingSOPClass);
		id_to_uid.insert("1.2.840.10008.1.42", &uids::SubstanceAdministrationLoggingSOPClass);

		ident_to_uid.insert("SubstanceAdministrationLoggingSOPInstance", &uids::SubstanceAdministrationLoggingSOPInstance);
		id_to_uid.insert("1.2.840.10008.1.42.1", &uids::SubstanceAdministrationLoggingSOPInstance);

		ident_to_uid.insert("DICOMUIDRegistry", &uids::DICOMUIDRegistry);
		id_to_uid.insert("1.2.840.10008.2.6.1", &uids::DICOMUIDRegistry);

		ident_to_uid.insert("DICOMControlledTerminology", &uids::DICOMControlledTerminology);
		id_to_uid.insert("1.2.840.10008.2.16.4", &uids::DICOMControlledTerminology);

		ident_to_uid.insert("AdultMouseAnatomyOntology", &uids::AdultMouseAnatomyOntology);
		id_to_uid.insert("1.2.840.10008.2.16.5", &uids::AdultMouseAnatomyOntology);

		ident_to_uid.insert("UberonOntology", &uids::UberonOntology);
		id_to_uid.insert("1.2.840.10008.2.16.6", &uids::UberonOntology);

		ident_to_uid.insert("IntegratedTaxonomicInformationSystemITISTaxonomicSerialNumberTSN", &uids::IntegratedTaxonomicInformationSystemITISTaxonomicSerialNumberTSN);
		id_to_uid.insert("1.2.840.10008.2.16.7", &uids::IntegratedTaxonomicInformationSystemITISTaxonomicSerialNumberTSN);

		ident_to_uid.insert("MouseGenomeInitiativeMGI", &uids::MouseGenomeInitiativeMGI);
		id_to_uid.insert("1.2.840.10008.2.16.8", &uids::MouseGenomeInitiativeMGI);

		ident_to_uid.insert("PubChemCompoundCID", &uids::PubChemCompoundCID);
		id_to_uid.insert("1.2.840.10008.2.16.9", &uids::PubChemCompoundCID);

		ident_to_uid.insert("DICOMApplicationContextName", &uids::DICOMApplicationContextName);
		id_to_uid.insert("1.2.840.10008.3.1.1.1", &uids::DICOMApplicationContextName);

		ident_to_uid.insert("DetachedPatientManagementSOPClass", &uids::DetachedPatientManagementSOPClass);
		id_to_uid.insert("1.2.840.10008.3.1.2.1.1", &uids::DetachedPatientManagementSOPClass);

		ident_to_uid.insert("DetachedPatientManagementMetaSOPClass", &uids::DetachedPatientManagementMetaSOPClass);
		id_to_uid.insert("1.2.840.10008.3.1.2.1.4", &uids::DetachedPatientManagementMetaSOPClass);

		ident_to_uid.insert("DetachedVisitManagementSOPClass", &uids::DetachedVisitManagementSOPClass);
		id_to_uid.insert("1.2.840.10008.3.1.2.2.1", &uids::DetachedVisitManagementSOPClass);

		ident_to_uid.insert("DetachedStudyManagementSOPClass", &uids::DetachedStudyManagementSOPClass);
		id_to_uid.insert("1.2.840.10008.3.1.2.3.1", &uids::DetachedStudyManagementSOPClass);

		ident_to_uid.insert("StudyComponentManagementSOPClass", &uids::StudyComponentManagementSOPClass);
		id_to_uid.insert("1.2.840.10008.3.1.2.3.2", &uids::StudyComponentManagementSOPClass);

		ident_to_uid.insert("ModalityPerformedProcedureStepSOPClass", &uids::ModalityPerformedProcedureStepSOPClass);
		id_to_uid.insert("1.2.840.10008.3.1.2.3.3", &uids::ModalityPerformedProcedureStepSOPClass);

		ident_to_uid.insert("ModalityPerformedProcedureStepRetrieveSOPClass", &uids::ModalityPerformedProcedureStepRetrieveSOPClass);
		id_to_uid.insert("1.2.840.10008.3.1.2.3.4", &uids::ModalityPerformedProcedureStepRetrieveSOPClass);

		ident_to_uid.insert("ModalityPerformedProcedureStepNotificationSOPClass", &uids::ModalityPerformedProcedureStepNotificationSOPClass);
		id_to_uid.insert("1.2.840.10008.3.1.2.3.5", &uids::ModalityPerformedProcedureStepNotificationSOPClass);

		ident_to_uid.insert("DetachedResultsManagementSOPClass", &uids::DetachedResultsManagementSOPClass);
		id_to_uid.insert("1.2.840.10008.3.1.2.5.1", &uids::DetachedResultsManagementSOPClass);

		ident_to_uid.insert("DetachedResultsManagementMetaSOPClass", &uids::DetachedResultsManagementMetaSOPClass);
		id_to_uid.insert("1.2.840.10008.3.1.2.5.4", &uids::DetachedResultsManagementMetaSOPClass);

		ident_to_uid.insert("DetachedStudyManagementMetaSOPClass", &uids::DetachedStudyManagementMetaSOPClass);
		id_to_uid.insert("1.2.840.10008.3.1.2.5.5", &uids::DetachedStudyManagementMetaSOPClass);

		ident_to_uid.insert("DetachedInterpretationManagementSOPClass", &uids::DetachedInterpretationManagementSOPClass);
		id_to_uid.insert("1.2.840.10008.3.1.2.6.1", &uids::DetachedInterpretationManagementSOPClass);

		ident_to_uid.insert("StorageServiceClass", &uids::StorageServiceClass);
		id_to_uid.insert("1.2.840.10008.4.2", &uids::StorageServiceClass);

		ident_to_uid.insert("BasicFilmSessionSOPClass", &uids::BasicFilmSessionSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.1", &uids::BasicFilmSessionSOPClass);

		ident_to_uid.insert("BasicFilmBoxSOPClass", &uids::BasicFilmBoxSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.2", &uids::BasicFilmBoxSOPClass);

		ident_to_uid.insert("BasicGrayscaleImageBoxSOPClass", &uids::BasicGrayscaleImageBoxSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.4", &uids::BasicGrayscaleImageBoxSOPClass);

		ident_to_uid.insert("BasicColorImageBoxSOPClass", &uids::BasicColorImageBoxSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.4.1", &uids::BasicColorImageBoxSOPClass);

		ident_to_uid.insert("ReferencedImageBoxSOPClass", &uids::ReferencedImageBoxSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.4.2", &uids::ReferencedImageBoxSOPClass);

		ident_to_uid.insert("BasicGrayscalePrintManagementMetaSOPClass", &uids::BasicGrayscalePrintManagementMetaSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.9", &uids::BasicGrayscalePrintManagementMetaSOPClass);

		ident_to_uid.insert("ReferencedGrayscalePrintManagementMetaSOPClass", &uids::ReferencedGrayscalePrintManagementMetaSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.9.1", &uids::ReferencedGrayscalePrintManagementMetaSOPClass);

		ident_to_uid.insert("PrintJobSOPClass", &uids::PrintJobSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.14", &uids::PrintJobSOPClass);

		ident_to_uid.insert("BasicAnnotationBoxSOPClass", &uids::BasicAnnotationBoxSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.15", &uids::BasicAnnotationBoxSOPClass);

		ident_to_uid.insert("PrinterSOPClass", &uids::PrinterSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.16", &uids::PrinterSOPClass);

		ident_to_uid.insert("PrinterConfigurationRetrievalSOPClass", &uids::PrinterConfigurationRetrievalSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.16.376", &uids::PrinterConfigurationRetrievalSOPClass);

		ident_to_uid.insert("PrinterSOPInstance", &uids::PrinterSOPInstance);
		id_to_uid.insert("1.2.840.10008.5.1.1.17", &uids::PrinterSOPInstance);

		ident_to_uid.insert("PrinterConfigurationRetrievalSOPInstance", &uids::PrinterConfigurationRetrievalSOPInstance);
		id_to_uid.insert("1.2.840.10008.5.1.1.17.376", &uids::PrinterConfigurationRetrievalSOPInstance);

		ident_to_uid.insert("BasicColorPrintManagementMetaSOPClass", &uids::BasicColorPrintManagementMetaSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.18", &uids::BasicColorPrintManagementMetaSOPClass);

		ident_to_uid.insert("ReferencedColorPrintManagementMetaSOPClass", &uids::ReferencedColorPrintManagementMetaSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.18.1", &uids::ReferencedColorPrintManagementMetaSOPClass);

		ident_to_uid.insert("VOILUTBoxSOPClass", &uids::VOILUTBoxSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.22", &uids::VOILUTBoxSOPClass);

		ident_to_uid.insert("PresentationLUTSOPClass", &uids::PresentationLUTSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.23", &uids::PresentationLUTSOPClass);

		ident_to_uid.insert("ImageOverlayBoxSOPClass", &uids::ImageOverlayBoxSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.24", &uids::ImageOverlayBoxSOPClass);

		ident_to_uid.insert("BasicPrintImageOverlayBoxSOPClass", &uids::BasicPrintImageOverlayBoxSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.24.1", &uids::BasicPrintImageOverlayBoxSOPClass);

		ident_to_uid.insert("PrintQueueSOPInstance", &uids::PrintQueueSOPInstance);
		id_to_uid.insert("1.2.840.10008.5.1.1.25", &uids::PrintQueueSOPInstance);

		ident_to_uid.insert("PrintQueueManagementSOPClass", &uids::PrintQueueManagementSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.26", &uids::PrintQueueManagementSOPClass);

		ident_to_uid.insert("StoredPrintStorageSOPClass", &uids::StoredPrintStorageSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.27", &uids::StoredPrintStorageSOPClass);

		ident_to_uid.insert("HardcopyGrayscaleImageStorageSOPClass", &uids::HardcopyGrayscaleImageStorageSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.29", &uids::HardcopyGrayscaleImageStorageSOPClass);

		ident_to_uid.insert("HardcopyColorImageStorageSOPClass", &uids::HardcopyColorImageStorageSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.30", &uids::HardcopyColorImageStorageSOPClass);

		ident_to_uid.insert("PullPrintRequestSOPClass", &uids::PullPrintRequestSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.31", &uids::PullPrintRequestSOPClass);

		ident_to_uid.insert("PullStoredPrintManagementMetaSOPClass", &uids::PullStoredPrintManagementMetaSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.32", &uids::PullStoredPrintManagementMetaSOPClass);

		ident_to_uid.insert("MediaCreationManagementSOPClassUID", &uids::MediaCreationManagementSOPClassUID);
		id_to_uid.insert("1.2.840.10008.5.1.1.33", &uids::MediaCreationManagementSOPClassUID);

		ident_to_uid.insert("DisplaySystemSOPClass", &uids::DisplaySystemSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.1.40", &uids::DisplaySystemSOPClass);

		ident_to_uid.insert("DisplaySystemSOPInstance", &uids::DisplaySystemSOPInstance);
		id_to_uid.insert("1.2.840.10008.5.1.1.40.1", &uids::DisplaySystemSOPInstance);

		ident_to_uid.insert("ComputedRadiographyImageStorage", &uids::ComputedRadiographyImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.1", &uids::ComputedRadiographyImageStorage);

		ident_to_uid.insert("DigitalXRayImageStorageForPresentation", &uids::DigitalXRayImageStorageForPresentation);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.1.1", &uids::DigitalXRayImageStorageForPresentation);

		ident_to_uid.insert("DigitalXRayImageStorageForProcessing", &uids::DigitalXRayImageStorageForProcessing);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.1.1.1", &uids::DigitalXRayImageStorageForProcessing);

		ident_to_uid.insert("DigitalMammographyXRayImageStorageForPresentation", &uids::DigitalMammographyXRayImageStorageForPresentation);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.1.2", &uids::DigitalMammographyXRayImageStorageForPresentation);

		ident_to_uid.insert("DigitalMammographyXRayImageStorageForProcessing", &uids::DigitalMammographyXRayImageStorageForProcessing);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.1.2.1", &uids::DigitalMammographyXRayImageStorageForProcessing);

		ident_to_uid.insert("DigitalIntraOralXRayImageStorageForPresentation", &uids::DigitalIntraOralXRayImageStorageForPresentation);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.1.3", &uids::DigitalIntraOralXRayImageStorageForPresentation);

		ident_to_uid.insert("DigitalIntraOralXRayImageStorageForProcessing", &uids::DigitalIntraOralXRayImageStorageForProcessing);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.1.3.1", &uids::DigitalIntraOralXRayImageStorageForProcessing);

		ident_to_uid.insert("CTImageStorage", &uids::CTImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.2", &uids::CTImageStorage);

		ident_to_uid.insert("EnhancedCTImageStorage", &uids::EnhancedCTImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.2.1", &uids::EnhancedCTImageStorage);

		ident_to_uid.insert("LegacyConvertedEnhancedCTImageStorage", &uids::LegacyConvertedEnhancedCTImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.2.2", &uids::LegacyConvertedEnhancedCTImageStorage);

		ident_to_uid.insert("UltrasoundMultiframeImageStorage_Retired", &uids::UltrasoundMultiframeImageStorage_Retired);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.3", &uids::UltrasoundMultiframeImageStorage_Retired);

		ident_to_uid.insert("UltrasoundMultiframeImageStorage", &uids::UltrasoundMultiframeImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.3.1", &uids::UltrasoundMultiframeImageStorage);

		ident_to_uid.insert("MRImageStorage", &uids::MRImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.4", &uids::MRImageStorage);

		ident_to_uid.insert("EnhancedMRImageStorage", &uids::EnhancedMRImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.4.1", &uids::EnhancedMRImageStorage);

		ident_to_uid.insert("MRSpectroscopyStorage", &uids::MRSpectroscopyStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.4.2", &uids::MRSpectroscopyStorage);

		ident_to_uid.insert("EnhancedMRColorImageStorage", &uids::EnhancedMRColorImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.4.3", &uids::EnhancedMRColorImageStorage);

		ident_to_uid.insert("LegacyConvertedEnhancedMRImageStorage", &uids::LegacyConvertedEnhancedMRImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.4.4", &uids::LegacyConvertedEnhancedMRImageStorage);

		ident_to_uid.insert("NuclearMedicineImageStorage_Retired", &uids::NuclearMedicineImageStorage_Retired);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.5", &uids::NuclearMedicineImageStorage_Retired);

		ident_to_uid.insert("UltrasoundImageStorage_Retired", &uids::UltrasoundImageStorage_Retired);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.6", &uids::UltrasoundImageStorage_Retired);

		ident_to_uid.insert("UltrasoundImageStorage", &uids::UltrasoundImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.6.1", &uids::UltrasoundImageStorage);

		ident_to_uid.insert("EnhancedUSVolumeStorage", &uids::EnhancedUSVolumeStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.6.2", &uids::EnhancedUSVolumeStorage);

		ident_to_uid.insert("SecondaryCaptureImageStorage", &uids::SecondaryCaptureImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.7", &uids::SecondaryCaptureImageStorage);

		ident_to_uid.insert("MultiframeSingleBitSecondaryCaptureImageStorage", &uids::MultiframeSingleBitSecondaryCaptureImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.7.1", &uids::MultiframeSingleBitSecondaryCaptureImageStorage);

		ident_to_uid.insert("MultiframeGrayscaleByteSecondaryCaptureImageStorage", &uids::MultiframeGrayscaleByteSecondaryCaptureImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.7.2", &uids::MultiframeGrayscaleByteSecondaryCaptureImageStorage);

		ident_to_uid.insert("MultiframeGrayscaleWordSecondaryCaptureImageStorage", &uids::MultiframeGrayscaleWordSecondaryCaptureImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.7.3", &uids::MultiframeGrayscaleWordSecondaryCaptureImageStorage);

		ident_to_uid.insert("MultiframeTrueColorSecondaryCaptureImageStorage", &uids::MultiframeTrueColorSecondaryCaptureImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.7.4", &uids::MultiframeTrueColorSecondaryCaptureImageStorage);

		ident_to_uid.insert("StandaloneOverlayStorage", &uids::StandaloneOverlayStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.8", &uids::StandaloneOverlayStorage);

		ident_to_uid.insert("StandaloneCurveStorage", &uids::StandaloneCurveStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.9", &uids::StandaloneCurveStorage);

		ident_to_uid.insert("WaveformStorageTrial", &uids::WaveformStorageTrial);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.9.1", &uids::WaveformStorageTrial);

		ident_to_uid.insert("Tag_12leadECGWaveformStorage", &uids::Tag_12leadECGWaveformStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.9.1.1", &uids::Tag_12leadECGWaveformStorage);

		ident_to_uid.insert("GeneralECGWaveformStorage", &uids::GeneralECGWaveformStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.9.1.2", &uids::GeneralECGWaveformStorage);

		ident_to_uid.insert("AmbulatoryECGWaveformStorage", &uids::AmbulatoryECGWaveformStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.9.1.3", &uids::AmbulatoryECGWaveformStorage);

		ident_to_uid.insert("HemodynamicWaveformStorage", &uids::HemodynamicWaveformStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.9.2.1", &uids::HemodynamicWaveformStorage);

		ident_to_uid.insert("CardiacElectrophysiologyWaveformStorage", &uids::CardiacElectrophysiologyWaveformStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.9.3.1", &uids::CardiacElectrophysiologyWaveformStorage);

		ident_to_uid.insert("BasicVoiceAudioWaveformStorage", &uids::BasicVoiceAudioWaveformStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.9.4.1", &uids::BasicVoiceAudioWaveformStorage);

		ident_to_uid.insert("GeneralAudioWaveformStorage", &uids::GeneralAudioWaveformStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.9.4.2", &uids::GeneralAudioWaveformStorage);

		ident_to_uid.insert("ArterialPulseWaveformStorage", &uids::ArterialPulseWaveformStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.9.5.1", &uids::ArterialPulseWaveformStorage);

		ident_to_uid.insert("RespiratoryWaveformStorage", &uids::RespiratoryWaveformStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.9.6.1", &uids::RespiratoryWaveformStorage);

		ident_to_uid.insert("StandaloneModalityLUTStorage", &uids::StandaloneModalityLUTStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.10", &uids::StandaloneModalityLUTStorage);

		ident_to_uid.insert("StandaloneVOILUTStorage", &uids::StandaloneVOILUTStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.11", &uids::StandaloneVOILUTStorage);

		ident_to_uid.insert("GrayscaleSoftcopyPresentationStateStorageSOPClass", &uids::GrayscaleSoftcopyPresentationStateStorageSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.11.1", &uids::GrayscaleSoftcopyPresentationStateStorageSOPClass);

		ident_to_uid.insert("ColorSoftcopyPresentationStateStorageSOPClass", &uids::ColorSoftcopyPresentationStateStorageSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.11.2", &uids::ColorSoftcopyPresentationStateStorageSOPClass);

		ident_to_uid.insert("PseudoColorSoftcopyPresentationStateStorageSOPClass", &uids::PseudoColorSoftcopyPresentationStateStorageSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.11.3", &uids::PseudoColorSoftcopyPresentationStateStorageSOPClass);

		ident_to_uid.insert("BlendingSoftcopyPresentationStateStorageSOPClass", &uids::BlendingSoftcopyPresentationStateStorageSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.11.4", &uids::BlendingSoftcopyPresentationStateStorageSOPClass);

		ident_to_uid.insert("XAXRFGrayscaleSoftcopyPresentationStateStorage", &uids::XAXRFGrayscaleSoftcopyPresentationStateStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.11.5", &uids::XAXRFGrayscaleSoftcopyPresentationStateStorage);

		ident_to_uid.insert("GrayscalePlanarMPRVolumetricPresentationStateStorage", &uids::GrayscalePlanarMPRVolumetricPresentationStateStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.11.6", &uids::GrayscalePlanarMPRVolumetricPresentationStateStorage);

		ident_to_uid.insert("CompositingPlanarMPRVolumetricPresentationStateStorage", &uids::CompositingPlanarMPRVolumetricPresentationStateStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.11.7", &uids::CompositingPlanarMPRVolumetricPresentationStateStorage);

		ident_to_uid.insert("XRayAngiographicImageStorage", &uids::XRayAngiographicImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.12.1", &uids::XRayAngiographicImageStorage);

		ident_to_uid.insert("EnhancedXAImageStorage", &uids::EnhancedXAImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.12.1.1", &uids::EnhancedXAImageStorage);

		ident_to_uid.insert("XRayRadiofluoroscopicImageStorage", &uids::XRayRadiofluoroscopicImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.12.2", &uids::XRayRadiofluoroscopicImageStorage);

		ident_to_uid.insert("EnhancedXRFImageStorage", &uids::EnhancedXRFImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.12.2.1", &uids::EnhancedXRFImageStorage);

		ident_to_uid.insert("XRayAngiographicBiPlaneImageStorage", &uids::XRayAngiographicBiPlaneImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.12.3", &uids::XRayAngiographicBiPlaneImageStorage);

		ident_to_uid.insert("XRay3DAngiographicImageStorage", &uids::XRay3DAngiographicImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.13.1.1", &uids::XRay3DAngiographicImageStorage);

		ident_to_uid.insert("XRay3DCraniofacialImageStorage", &uids::XRay3DCraniofacialImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.13.1.2", &uids::XRay3DCraniofacialImageStorage);

		ident_to_uid.insert("BreastTomosynthesisImageStorage", &uids::BreastTomosynthesisImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.13.1.3", &uids::BreastTomosynthesisImageStorage);

		ident_to_uid.insert("BreastProjectionXRayImageStorageForPresentation", &uids::BreastProjectionXRayImageStorageForPresentation);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.13.1.4", &uids::BreastProjectionXRayImageStorageForPresentation);

		ident_to_uid.insert("BreastProjectionXRayImageStorageForProcessing", &uids::BreastProjectionXRayImageStorageForProcessing);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.13.1.5", &uids::BreastProjectionXRayImageStorageForProcessing);

		ident_to_uid.insert("IntravascularOpticalCoherenceTomographyImageStorageForPresentation", &uids::IntravascularOpticalCoherenceTomographyImageStorageForPresentation);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.14.1", &uids::IntravascularOpticalCoherenceTomographyImageStorageForPresentation);

		ident_to_uid.insert("IntravascularOpticalCoherenceTomographyImageStorageForProcessing", &uids::IntravascularOpticalCoherenceTomographyImageStorageForProcessing);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.14.2", &uids::IntravascularOpticalCoherenceTomographyImageStorageForProcessing);

		ident_to_uid.insert("NuclearMedicineImageStorage", &uids::NuclearMedicineImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.20", &uids::NuclearMedicineImageStorage);

		ident_to_uid.insert("ParametricMapStorage", &uids::ParametricMapStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.30", &uids::ParametricMapStorage);

		ident_to_uid.insert("RawDataStorage", &uids::RawDataStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.66", &uids::RawDataStorage);

		ident_to_uid.insert("SpatialRegistrationStorage", &uids::SpatialRegistrationStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.66.1", &uids::SpatialRegistrationStorage);

		ident_to_uid.insert("SpatialFiducialsStorage", &uids::SpatialFiducialsStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.66.2", &uids::SpatialFiducialsStorage);

		ident_to_uid.insert("DeformableSpatialRegistrationStorage", &uids::DeformableSpatialRegistrationStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.66.3", &uids::DeformableSpatialRegistrationStorage);

		ident_to_uid.insert("SegmentationStorage", &uids::SegmentationStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.66.4", &uids::SegmentationStorage);

		ident_to_uid.insert("SurfaceSegmentationStorage", &uids::SurfaceSegmentationStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.66.5", &uids::SurfaceSegmentationStorage);

		ident_to_uid.insert("TractographyResultsStorage", &uids::TractographyResultsStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.66.6", &uids::TractographyResultsStorage);

		ident_to_uid.insert("RealWorldValueMappingStorage", &uids::RealWorldValueMappingStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.67", &uids::RealWorldValueMappingStorage);

		ident_to_uid.insert("SurfaceScanMeshStorage", &uids::SurfaceScanMeshStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.68.1", &uids::SurfaceScanMeshStorage);

		ident_to_uid.insert("SurfaceScanPointCloudStorage", &uids::SurfaceScanPointCloudStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.68.2", &uids::SurfaceScanPointCloudStorage);

		ident_to_uid.insert("VLImageStorageTrial", &uids::VLImageStorageTrial);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.77.1", &uids::VLImageStorageTrial);

		ident_to_uid.insert("VLMultiframeImageStorageTrial", &uids::VLMultiframeImageStorageTrial);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.77.2", &uids::VLMultiframeImageStorageTrial);

		ident_to_uid.insert("VLEndoscopicImageStorage", &uids::VLEndoscopicImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.77.1.1", &uids::VLEndoscopicImageStorage);

		ident_to_uid.insert("VideoEndoscopicImageStorage", &uids::VideoEndoscopicImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.77.1.1.1", &uids::VideoEndoscopicImageStorage);

		ident_to_uid.insert("VLMicroscopicImageStorage", &uids::VLMicroscopicImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.77.1.2", &uids::VLMicroscopicImageStorage);

		ident_to_uid.insert("VideoMicroscopicImageStorage", &uids::VideoMicroscopicImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.77.1.2.1", &uids::VideoMicroscopicImageStorage);

		ident_to_uid.insert("VLSlideCoordinatesMicroscopicImageStorage", &uids::VLSlideCoordinatesMicroscopicImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.77.1.3", &uids::VLSlideCoordinatesMicroscopicImageStorage);

		ident_to_uid.insert("VLPhotographicImageStorage", &uids::VLPhotographicImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.77.1.4", &uids::VLPhotographicImageStorage);

		ident_to_uid.insert("VideoPhotographicImageStorage", &uids::VideoPhotographicImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.77.1.4.1", &uids::VideoPhotographicImageStorage);

		ident_to_uid.insert("OphthalmicPhotography8BitImageStorage", &uids::OphthalmicPhotography8BitImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.77.1.5.1", &uids::OphthalmicPhotography8BitImageStorage);

		ident_to_uid.insert("OphthalmicPhotography16BitImageStorage", &uids::OphthalmicPhotography16BitImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.77.1.5.2", &uids::OphthalmicPhotography16BitImageStorage);

		ident_to_uid.insert("StereometricRelationshipStorage", &uids::StereometricRelationshipStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.77.1.5.3", &uids::StereometricRelationshipStorage);

		ident_to_uid.insert("OphthalmicTomographyImageStorage", &uids::OphthalmicTomographyImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.77.1.5.4", &uids::OphthalmicTomographyImageStorage);

		ident_to_uid.insert("WideFieldOphthalmicPhotographyStereographicProjectionImageStorage", &uids::WideFieldOphthalmicPhotographyStereographicProjectionImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.77.1.5.5", &uids::WideFieldOphthalmicPhotographyStereographicProjectionImageStorage);

		ident_to_uid.insert("WideFieldOphthalmicPhotography3DCoordinatesImageStorage", &uids::WideFieldOphthalmicPhotography3DCoordinatesImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.77.1.5.6", &uids::WideFieldOphthalmicPhotography3DCoordinatesImageStorage);

		ident_to_uid.insert("VLWholeSlideMicroscopyImageStorage", &uids::VLWholeSlideMicroscopyImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.77.1.6", &uids::VLWholeSlideMicroscopyImageStorage);

		ident_to_uid.insert("LensometryMeasurementsStorage", &uids::LensometryMeasurementsStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.78.1", &uids::LensometryMeasurementsStorage);

		ident_to_uid.insert("AutorefractionMeasurementsStorage", &uids::AutorefractionMeasurementsStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.78.2", &uids::AutorefractionMeasurementsStorage);

		ident_to_uid.insert("KeratometryMeasurementsStorage", &uids::KeratometryMeasurementsStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.78.3", &uids::KeratometryMeasurementsStorage);

		ident_to_uid.insert("SubjectiveRefractionMeasurementsStorage", &uids::SubjectiveRefractionMeasurementsStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.78.4", &uids::SubjectiveRefractionMeasurementsStorage);

		ident_to_uid.insert("VisualAcuityMeasurementsStorage", &uids::VisualAcuityMeasurementsStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.78.5", &uids::VisualAcuityMeasurementsStorage);

		ident_to_uid.insert("SpectaclePrescriptionReportStorage", &uids::SpectaclePrescriptionReportStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.78.6", &uids::SpectaclePrescriptionReportStorage);

		ident_to_uid.insert("OphthalmicAxialMeasurementsStorage", &uids::OphthalmicAxialMeasurementsStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.78.7", &uids::OphthalmicAxialMeasurementsStorage);

		ident_to_uid.insert("IntraocularLensCalculationsStorage", &uids::IntraocularLensCalculationsStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.78.8", &uids::IntraocularLensCalculationsStorage);

		ident_to_uid.insert("MacularGridThicknessandVolumeReportStorage", &uids::MacularGridThicknessandVolumeReportStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.79.1", &uids::MacularGridThicknessandVolumeReportStorage);

		ident_to_uid.insert("OphthalmicVisualFieldStaticPerimetryMeasurementsStorage", &uids::OphthalmicVisualFieldStaticPerimetryMeasurementsStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.80.1", &uids::OphthalmicVisualFieldStaticPerimetryMeasurementsStorage);

		ident_to_uid.insert("OphthalmicThicknessMapStorage", &uids::OphthalmicThicknessMapStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.81.1", &uids::OphthalmicThicknessMapStorage);

		ident_to_uid.insert("CornealTopographyMapStorage", &uids::CornealTopographyMapStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.82.1", &uids::CornealTopographyMapStorage);

		ident_to_uid.insert("TextSRStorageTrial", &uids::TextSRStorageTrial);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.88.1", &uids::TextSRStorageTrial);

		ident_to_uid.insert("AudioSRStorageTrial", &uids::AudioSRStorageTrial);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.88.2", &uids::AudioSRStorageTrial);

		ident_to_uid.insert("DetailSRStorageTrial", &uids::DetailSRStorageTrial);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.88.3", &uids::DetailSRStorageTrial);

		ident_to_uid.insert("ComprehensiveSRStorageTrial", &uids::ComprehensiveSRStorageTrial);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.88.4", &uids::ComprehensiveSRStorageTrial);

		ident_to_uid.insert("BasicTextSRStorage", &uids::BasicTextSRStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.88.11", &uids::BasicTextSRStorage);

		ident_to_uid.insert("EnhancedSRStorage", &uids::EnhancedSRStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.88.22", &uids::EnhancedSRStorage);

		ident_to_uid.insert("ComprehensiveSRStorage", &uids::ComprehensiveSRStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.88.33", &uids::ComprehensiveSRStorage);

		ident_to_uid.insert("Comprehensive3DSRStorage", &uids::Comprehensive3DSRStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.88.34", &uids::Comprehensive3DSRStorage);

		ident_to_uid.insert("ExtensibleSRStorage", &uids::ExtensibleSRStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.88.35", &uids::ExtensibleSRStorage);

		ident_to_uid.insert("ProcedureLogStorage", &uids::ProcedureLogStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.88.40", &uids::ProcedureLogStorage);

		ident_to_uid.insert("MammographyCADSRStorage", &uids::MammographyCADSRStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.88.50", &uids::MammographyCADSRStorage);

		ident_to_uid.insert("KeyObjectSelectionDocumentStorage", &uids::KeyObjectSelectionDocumentStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.88.59", &uids::KeyObjectSelectionDocumentStorage);

		ident_to_uid.insert("ChestCADSRStorage", &uids::ChestCADSRStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.88.65", &uids::ChestCADSRStorage);

		ident_to_uid.insert("XRayRadiationDoseSRStorage", &uids::XRayRadiationDoseSRStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.88.67", &uids::XRayRadiationDoseSRStorage);

		ident_to_uid.insert("RadiopharmaceuticalRadiationDoseSRStorage", &uids::RadiopharmaceuticalRadiationDoseSRStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.88.68", &uids::RadiopharmaceuticalRadiationDoseSRStorage);

		ident_to_uid.insert("ColonCADSRStorage", &uids::ColonCADSRStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.88.69", &uids::ColonCADSRStorage);

		ident_to_uid.insert("ImplantationPlanSRStorage", &uids::ImplantationPlanSRStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.88.70", &uids::ImplantationPlanSRStorage);

		ident_to_uid.insert("AcquisitionContextSRStorage", &uids::AcquisitionContextSRStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.88.71", &uids::AcquisitionContextSRStorage);

		ident_to_uid.insert("SimplifiedAdultEchoSRStorage", &uids::SimplifiedAdultEchoSRStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.88.72", &uids::SimplifiedAdultEchoSRStorage);

		ident_to_uid.insert("ContentAssessmentResultsStorage", &uids::ContentAssessmentResultsStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.90.1", &uids::ContentAssessmentResultsStorage);

		ident_to_uid.insert("EncapsulatedPDFStorage", &uids::EncapsulatedPDFStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.104.1", &uids::EncapsulatedPDFStorage);

		ident_to_uid.insert("EncapsulatedCDAStorage", &uids::EncapsulatedCDAStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.104.2", &uids::EncapsulatedCDAStorage);

		ident_to_uid.insert("PositronEmissionTomographyImageStorage", &uids::PositronEmissionTomographyImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.128", &uids::PositronEmissionTomographyImageStorage);

		ident_to_uid.insert("LegacyConvertedEnhancedPETImageStorage", &uids::LegacyConvertedEnhancedPETImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.128.1", &uids::LegacyConvertedEnhancedPETImageStorage);

		ident_to_uid.insert("StandalonePETCurveStorage", &uids::StandalonePETCurveStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.129", &uids::StandalonePETCurveStorage);

		ident_to_uid.insert("EnhancedPETImageStorage", &uids::EnhancedPETImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.130", &uids::EnhancedPETImageStorage);

		ident_to_uid.insert("BasicStructuredDisplayStorage", &uids::BasicStructuredDisplayStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.131", &uids::BasicStructuredDisplayStorage);

		ident_to_uid.insert("CTDefinedProcedureProtocolStorage", &uids::CTDefinedProcedureProtocolStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.200.1", &uids::CTDefinedProcedureProtocolStorage);

		ident_to_uid.insert("CTPerformedProcedureProtocolStorage", &uids::CTPerformedProcedureProtocolStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.200.2", &uids::CTPerformedProcedureProtocolStorage);

		ident_to_uid.insert("RTImageStorage", &uids::RTImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.481.1", &uids::RTImageStorage);

		ident_to_uid.insert("RTDoseStorage", &uids::RTDoseStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.481.2", &uids::RTDoseStorage);

		ident_to_uid.insert("RTStructureSetStorage", &uids::RTStructureSetStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.481.3", &uids::RTStructureSetStorage);

		ident_to_uid.insert("RTBeamsTreatmentRecordStorage", &uids::RTBeamsTreatmentRecordStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.481.4", &uids::RTBeamsTreatmentRecordStorage);

		ident_to_uid.insert("RTPlanStorage", &uids::RTPlanStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.481.5", &uids::RTPlanStorage);

		ident_to_uid.insert("RTBrachyTreatmentRecordStorage", &uids::RTBrachyTreatmentRecordStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.481.6", &uids::RTBrachyTreatmentRecordStorage);

		ident_to_uid.insert("RTTreatmentSummaryRecordStorage", &uids::RTTreatmentSummaryRecordStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.481.7", &uids::RTTreatmentSummaryRecordStorage);

		ident_to_uid.insert("RTIonPlanStorage", &uids::RTIonPlanStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.481.8", &uids::RTIonPlanStorage);

		ident_to_uid.insert("RTIonBeamsTreatmentRecordStorage", &uids::RTIonBeamsTreatmentRecordStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.481.9", &uids::RTIonBeamsTreatmentRecordStorage);

		ident_to_uid.insert("DICOSCTImageStorage", &uids::DICOSCTImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.501.1", &uids::DICOSCTImageStorage);

		ident_to_uid.insert("DICOSDigitalXRayImageStorageForPresentation", &uids::DICOSDigitalXRayImageStorageForPresentation);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.501.2.1", &uids::DICOSDigitalXRayImageStorageForPresentation);

		ident_to_uid.insert("DICOSDigitalXRayImageStorageForProcessing", &uids::DICOSDigitalXRayImageStorageForProcessing);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.501.2.2", &uids::DICOSDigitalXRayImageStorageForProcessing);

		ident_to_uid.insert("DICOSThreatDetectionReportStorage", &uids::DICOSThreatDetectionReportStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.501.3", &uids::DICOSThreatDetectionReportStorage);

		ident_to_uid.insert("DICOS2DAITStorage", &uids::DICOS2DAITStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.501.4", &uids::DICOS2DAITStorage);

		ident_to_uid.insert("DICOS3DAITStorage", &uids::DICOS3DAITStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.501.5", &uids::DICOS3DAITStorage);

		ident_to_uid.insert("DICOSQuadrupoleResonanceQRStorage", &uids::DICOSQuadrupoleResonanceQRStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.501.6", &uids::DICOSQuadrupoleResonanceQRStorage);

		ident_to_uid.insert("EddyCurrentImageStorage", &uids::EddyCurrentImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.601.1", &uids::EddyCurrentImageStorage);

		ident_to_uid.insert("EddyCurrentMultiframeImageStorage", &uids::EddyCurrentMultiframeImageStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.1.601.2", &uids::EddyCurrentMultiframeImageStorage);

		ident_to_uid.insert("PatientRootQueryRetrieveInformationModelFIND", &uids::PatientRootQueryRetrieveInformationModelFIND);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.2.1.1", &uids::PatientRootQueryRetrieveInformationModelFIND);

		ident_to_uid.insert("PatientRootQueryRetrieveInformationModelMOVE", &uids::PatientRootQueryRetrieveInformationModelMOVE);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.2.1.2", &uids::PatientRootQueryRetrieveInformationModelMOVE);

		ident_to_uid.insert("PatientRootQueryRetrieveInformationModelGET", &uids::PatientRootQueryRetrieveInformationModelGET);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.2.1.3", &uids::PatientRootQueryRetrieveInformationModelGET);

		ident_to_uid.insert("StudyRootQueryRetrieveInformationModelFIND", &uids::StudyRootQueryRetrieveInformationModelFIND);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.2.2.1", &uids::StudyRootQueryRetrieveInformationModelFIND);

		ident_to_uid.insert("StudyRootQueryRetrieveInformationModelMOVE", &uids::StudyRootQueryRetrieveInformationModelMOVE);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.2.2.2", &uids::StudyRootQueryRetrieveInformationModelMOVE);

		ident_to_uid.insert("StudyRootQueryRetrieveInformationModelGET", &uids::StudyRootQueryRetrieveInformationModelGET);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.2.2.3", &uids::StudyRootQueryRetrieveInformationModelGET);

		ident_to_uid.insert("PatientStudyOnlyQueryRetrieveInformationModelFIND", &uids::PatientStudyOnlyQueryRetrieveInformationModelFIND);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.2.3.1", &uids::PatientStudyOnlyQueryRetrieveInformationModelFIND);

		ident_to_uid.insert("PatientStudyOnlyQueryRetrieveInformationModelMOVE", &uids::PatientStudyOnlyQueryRetrieveInformationModelMOVE);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.2.3.2", &uids::PatientStudyOnlyQueryRetrieveInformationModelMOVE);

		ident_to_uid.insert("PatientStudyOnlyQueryRetrieveInformationModelGET", &uids::PatientStudyOnlyQueryRetrieveInformationModelGET);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.2.3.3", &uids::PatientStudyOnlyQueryRetrieveInformationModelGET);

		ident_to_uid.insert("CompositeInstanceRootRetrieveMOVE", &uids::CompositeInstanceRootRetrieveMOVE);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.2.4.2", &uids::CompositeInstanceRootRetrieveMOVE);

		ident_to_uid.insert("CompositeInstanceRootRetrieveGET", &uids::CompositeInstanceRootRetrieveGET);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.2.4.3", &uids::CompositeInstanceRootRetrieveGET);

		ident_to_uid.insert("CompositeInstanceRetrieveWithoutBulkDataGET", &uids::CompositeInstanceRetrieveWithoutBulkDataGET);
		id_to_uid.insert("1.2.840.10008.5.1.4.1.2.5.3", &uids::CompositeInstanceRetrieveWithoutBulkDataGET);

		ident_to_uid.insert("DefinedProcedureProtocolInformationModelFIND", &uids::DefinedProcedureProtocolInformationModelFIND);
		id_to_uid.insert("1.2.840.10008.5.1.4.20.1", &uids::DefinedProcedureProtocolInformationModelFIND);

		ident_to_uid.insert("DefinedProcedureProtocolInformationModelMOVE", &uids::DefinedProcedureProtocolInformationModelMOVE);
		id_to_uid.insert("1.2.840.10008.5.1.4.20.2", &uids::DefinedProcedureProtocolInformationModelMOVE);

		ident_to_uid.insert("DefinedProcedureProtocolInformationModelGET", &uids::DefinedProcedureProtocolInformationModelGET);
		id_to_uid.insert("1.2.840.10008.5.1.4.20.3", &uids::DefinedProcedureProtocolInformationModelGET);

		ident_to_uid.insert("ModalityWorklistInformationModelFIND", &uids::ModalityWorklistInformationModelFIND);
		id_to_uid.insert("1.2.840.10008.5.1.4.31", &uids::ModalityWorklistInformationModelFIND);

		ident_to_uid.insert("GeneralPurposeWorklistManagementMetaSOPClass", &uids::GeneralPurposeWorklistManagementMetaSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.4.32", &uids::GeneralPurposeWorklistManagementMetaSOPClass);

		ident_to_uid.insert("GeneralPurposeWorklistInformationModelFIND", &uids::GeneralPurposeWorklistInformationModelFIND);
		id_to_uid.insert("1.2.840.10008.5.1.4.32.1", &uids::GeneralPurposeWorklistInformationModelFIND);

		ident_to_uid.insert("GeneralPurposeScheduledProcedureStepSOPClass", &uids::GeneralPurposeScheduledProcedureStepSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.4.32.2", &uids::GeneralPurposeScheduledProcedureStepSOPClass);

		ident_to_uid.insert("GeneralPurposePerformedProcedureStepSOPClass", &uids::GeneralPurposePerformedProcedureStepSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.4.32.3", &uids::GeneralPurposePerformedProcedureStepSOPClass);

		ident_to_uid.insert("InstanceAvailabilityNotificationSOPClass", &uids::InstanceAvailabilityNotificationSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.4.33", &uids::InstanceAvailabilityNotificationSOPClass);

		ident_to_uid.insert("RTBeamsDeliveryInstructionStorageTrial", &uids::RTBeamsDeliveryInstructionStorageTrial);
		id_to_uid.insert("1.2.840.10008.5.1.4.34.1", &uids::RTBeamsDeliveryInstructionStorageTrial);

		ident_to_uid.insert("RTConventionalMachineVerificationTrial", &uids::RTConventionalMachineVerificationTrial);
		id_to_uid.insert("1.2.840.10008.5.1.4.34.2", &uids::RTConventionalMachineVerificationTrial);

		ident_to_uid.insert("RTIonMachineVerificationTrial", &uids::RTIonMachineVerificationTrial);
		id_to_uid.insert("1.2.840.10008.5.1.4.34.3", &uids::RTIonMachineVerificationTrial);

		ident_to_uid.insert("UnifiedWorklistandProcedureStepServiceClassTrial", &uids::UnifiedWorklistandProcedureStepServiceClassTrial);
		id_to_uid.insert("1.2.840.10008.5.1.4.34.4", &uids::UnifiedWorklistandProcedureStepServiceClassTrial);

		ident_to_uid.insert("UnifiedProcedureStepPushSOPClassTrial", &uids::UnifiedProcedureStepPushSOPClassTrial);
		id_to_uid.insert("1.2.840.10008.5.1.4.34.4.1", &uids::UnifiedProcedureStepPushSOPClassTrial);

		ident_to_uid.insert("UnifiedProcedureStepWatchSOPClassTrial", &uids::UnifiedProcedureStepWatchSOPClassTrial);
		id_to_uid.insert("1.2.840.10008.5.1.4.34.4.2", &uids::UnifiedProcedureStepWatchSOPClassTrial);

		ident_to_uid.insert("UnifiedProcedureStepPullSOPClassTrial", &uids::UnifiedProcedureStepPullSOPClassTrial);
		id_to_uid.insert("1.2.840.10008.5.1.4.34.4.3", &uids::UnifiedProcedureStepPullSOPClassTrial);

		ident_to_uid.insert("UnifiedProcedureStepEventSOPClassTrial", &uids::UnifiedProcedureStepEventSOPClassTrial);
		id_to_uid.insert("1.2.840.10008.5.1.4.34.4.4", &uids::UnifiedProcedureStepEventSOPClassTrial);

		ident_to_uid.insert("UPSGlobalSubscriptionSOPInstance", &uids::UPSGlobalSubscriptionSOPInstance);
		id_to_uid.insert("1.2.840.10008.5.1.4.34.5", &uids::UPSGlobalSubscriptionSOPInstance);

		ident_to_uid.insert("UPSFilteredGlobalSubscriptionSOPInstance", &uids::UPSFilteredGlobalSubscriptionSOPInstance);
		id_to_uid.insert("1.2.840.10008.5.1.4.34.5.1", &uids::UPSFilteredGlobalSubscriptionSOPInstance);

		ident_to_uid.insert("UnifiedWorklistandProcedureStepServiceClass", &uids::UnifiedWorklistandProcedureStepServiceClass);
		id_to_uid.insert("1.2.840.10008.5.1.4.34.6", &uids::UnifiedWorklistandProcedureStepServiceClass);

		ident_to_uid.insert("UnifiedProcedureStepPushSOPClass", &uids::UnifiedProcedureStepPushSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.4.34.6.1", &uids::UnifiedProcedureStepPushSOPClass);

		ident_to_uid.insert("UnifiedProcedureStepWatchSOPClass", &uids::UnifiedProcedureStepWatchSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.4.34.6.2", &uids::UnifiedProcedureStepWatchSOPClass);

		ident_to_uid.insert("UnifiedProcedureStepPullSOPClass", &uids::UnifiedProcedureStepPullSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.4.34.6.3", &uids::UnifiedProcedureStepPullSOPClass);

		ident_to_uid.insert("UnifiedProcedureStepEventSOPClass", &uids::UnifiedProcedureStepEventSOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.4.34.6.4", &uids::UnifiedProcedureStepEventSOPClass);

		ident_to_uid.insert("RTBeamsDeliveryInstructionStorage", &uids::RTBeamsDeliveryInstructionStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.34.7", &uids::RTBeamsDeliveryInstructionStorage);

		ident_to_uid.insert("RTConventionalMachineVerification", &uids::RTConventionalMachineVerification);
		id_to_uid.insert("1.2.840.10008.5.1.4.34.8", &uids::RTConventionalMachineVerification);

		ident_to_uid.insert("RTIonMachineVerification", &uids::RTIonMachineVerification);
		id_to_uid.insert("1.2.840.10008.5.1.4.34.9", &uids::RTIonMachineVerification);

		ident_to_uid.insert("RTBrachyApplicationSetupDeliveryInstructionStorage", &uids::RTBrachyApplicationSetupDeliveryInstructionStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.34.10", &uids::RTBrachyApplicationSetupDeliveryInstructionStorage);

		ident_to_uid.insert("GeneralRelevantPatientInformationQuery", &uids::GeneralRelevantPatientInformationQuery);
		id_to_uid.insert("1.2.840.10008.5.1.4.37.1", &uids::GeneralRelevantPatientInformationQuery);

		ident_to_uid.insert("BreastImagingRelevantPatientInformationQuery", &uids::BreastImagingRelevantPatientInformationQuery);
		id_to_uid.insert("1.2.840.10008.5.1.4.37.2", &uids::BreastImagingRelevantPatientInformationQuery);

		ident_to_uid.insert("CardiacRelevantPatientInformationQuery", &uids::CardiacRelevantPatientInformationQuery);
		id_to_uid.insert("1.2.840.10008.5.1.4.37.3", &uids::CardiacRelevantPatientInformationQuery);

		ident_to_uid.insert("HangingProtocolStorage", &uids::HangingProtocolStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.38.1", &uids::HangingProtocolStorage);

		ident_to_uid.insert("HangingProtocolInformationModelFIND", &uids::HangingProtocolInformationModelFIND);
		id_to_uid.insert("1.2.840.10008.5.1.4.38.2", &uids::HangingProtocolInformationModelFIND);

		ident_to_uid.insert("HangingProtocolInformationModelMOVE", &uids::HangingProtocolInformationModelMOVE);
		id_to_uid.insert("1.2.840.10008.5.1.4.38.3", &uids::HangingProtocolInformationModelMOVE);

		ident_to_uid.insert("HangingProtocolInformationModelGET", &uids::HangingProtocolInformationModelGET);
		id_to_uid.insert("1.2.840.10008.5.1.4.38.4", &uids::HangingProtocolInformationModelGET);

		ident_to_uid.insert("ColorPaletteStorage", &uids::ColorPaletteStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.39.1", &uids::ColorPaletteStorage);

		ident_to_uid.insert("ColorPaletteQueryRetrieveInformationModelFIND", &uids::ColorPaletteQueryRetrieveInformationModelFIND);
		id_to_uid.insert("1.2.840.10008.5.1.4.39.2", &uids::ColorPaletteQueryRetrieveInformationModelFIND);

		ident_to_uid.insert("ColorPaletteQueryRetrieveInformationModelMOVE", &uids::ColorPaletteQueryRetrieveInformationModelMOVE);
		id_to_uid.insert("1.2.840.10008.5.1.4.39.3", &uids::ColorPaletteQueryRetrieveInformationModelMOVE);

		ident_to_uid.insert("ColorPaletteQueryRetrieveInformationModelGET", &uids::ColorPaletteQueryRetrieveInformationModelGET);
		id_to_uid.insert("1.2.840.10008.5.1.4.39.4", &uids::ColorPaletteQueryRetrieveInformationModelGET);

		ident_to_uid.insert("ProductCharacteristicsQuerySOPClass", &uids::ProductCharacteristicsQuerySOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.4.41", &uids::ProductCharacteristicsQuerySOPClass);

		ident_to_uid.insert("SubstanceApprovalQuerySOPClass", &uids::SubstanceApprovalQuerySOPClass);
		id_to_uid.insert("1.2.840.10008.5.1.4.42", &uids::SubstanceApprovalQuerySOPClass);

		ident_to_uid.insert("GenericImplantTemplateStorage", &uids::GenericImplantTemplateStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.43.1", &uids::GenericImplantTemplateStorage);

		ident_to_uid.insert("GenericImplantTemplateInformationModelFIND", &uids::GenericImplantTemplateInformationModelFIND);
		id_to_uid.insert("1.2.840.10008.5.1.4.43.2", &uids::GenericImplantTemplateInformationModelFIND);

		ident_to_uid.insert("GenericImplantTemplateInformationModelMOVE", &uids::GenericImplantTemplateInformationModelMOVE);
		id_to_uid.insert("1.2.840.10008.5.1.4.43.3", &uids::GenericImplantTemplateInformationModelMOVE);

		ident_to_uid.insert("GenericImplantTemplateInformationModelGET", &uids::GenericImplantTemplateInformationModelGET);
		id_to_uid.insert("1.2.840.10008.5.1.4.43.4", &uids::GenericImplantTemplateInformationModelGET);

		ident_to_uid.insert("ImplantAssemblyTemplateStorage", &uids::ImplantAssemblyTemplateStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.44.1", &uids::ImplantAssemblyTemplateStorage);

		ident_to_uid.insert("ImplantAssemblyTemplateInformationModelFIND", &uids::ImplantAssemblyTemplateInformationModelFIND);
		id_to_uid.insert("1.2.840.10008.5.1.4.44.2", &uids::ImplantAssemblyTemplateInformationModelFIND);

		ident_to_uid.insert("ImplantAssemblyTemplateInformationModelMOVE", &uids::ImplantAssemblyTemplateInformationModelMOVE);
		id_to_uid.insert("1.2.840.10008.5.1.4.44.3", &uids::ImplantAssemblyTemplateInformationModelMOVE);

		ident_to_uid.insert("ImplantAssemblyTemplateInformationModelGET", &uids::ImplantAssemblyTemplateInformationModelGET);
		id_to_uid.insert("1.2.840.10008.5.1.4.44.4", &uids::ImplantAssemblyTemplateInformationModelGET);

		ident_to_uid.insert("ImplantTemplateGroupStorage", &uids::ImplantTemplateGroupStorage);
		id_to_uid.insert("1.2.840.10008.5.1.4.45.1", &uids::ImplantTemplateGroupStorage);

		ident_to_uid.insert("ImplantTemplateGroupInformationModelFIND", &uids::ImplantTemplateGroupInformationModelFIND);
		id_to_uid.insert("1.2.840.10008.5.1.4.45.2", &uids::ImplantTemplateGroupInformationModelFIND);

		ident_to_uid.insert("ImplantTemplateGroupInformationModelMOVE", &uids::ImplantTemplateGroupInformationModelMOVE);
		id_to_uid.insert("1.2.840.10008.5.1.4.45.3", &uids::ImplantTemplateGroupInformationModelMOVE);

		ident_to_uid.insert("ImplantTemplateGroupInformationModelGET", &uids::ImplantTemplateGroupInformationModelGET);
		id_to_uid.insert("1.2.840.10008.5.1.4.45.4", &uids::ImplantTemplateGroupInformationModelGET);

		ident_to_uid.insert("NativeDICOMModel", &uids::NativeDICOMModel);
		id_to_uid.insert("1.2.840.10008.7.1.1", &uids::NativeDICOMModel);

		ident_to_uid.insert("AbstractMultiDimensionalImageModel", &uids::AbstractMultiDimensionalImageModel);
		id_to_uid.insert("1.2.840.10008.7.1.2", &uids::AbstractMultiDimensionalImageModel);

		ident_to_uid.insert("DICOMContentMappingResource", &uids::DICOMContentMappingResource);
		id_to_uid.insert("1.2.840.10008.8.1.1", &uids::DICOMContentMappingResource);

		ident_to_uid.insert("dicomDeviceName", &uids::dicomDeviceName);
		id_to_uid.insert("1.2.840.10008.15.0.3.1", &uids::dicomDeviceName);

		ident_to_uid.insert("dicomDescription", &uids::dicomDescription);
		id_to_uid.insert("1.2.840.10008.15.0.3.2", &uids::dicomDescription);

		ident_to_uid.insert("dicomManufacturer", &uids::dicomManufacturer);
		id_to_uid.insert("1.2.840.10008.15.0.3.3", &uids::dicomManufacturer);

		ident_to_uid.insert("dicomManufacturerModelName", &uids::dicomManufacturerModelName);
		id_to_uid.insert("1.2.840.10008.15.0.3.4", &uids::dicomManufacturerModelName);

		ident_to_uid.insert("dicomSoftwareVersion", &uids::dicomSoftwareVersion);
		id_to_uid.insert("1.2.840.10008.15.0.3.5", &uids::dicomSoftwareVersion);

		ident_to_uid.insert("dicomVendorData", &uids::dicomVendorData);
		id_to_uid.insert("1.2.840.10008.15.0.3.6", &uids::dicomVendorData);

		ident_to_uid.insert("dicomAETitle", &uids::dicomAETitle);
		id_to_uid.insert("1.2.840.10008.15.0.3.7", &uids::dicomAETitle);

		ident_to_uid.insert("dicomNetworkConnectionReference", &uids::dicomNetworkConnectionReference);
		id_to_uid.insert("1.2.840.10008.15.0.3.8", &uids::dicomNetworkConnectionReference);

		ident_to_uid.insert("dicomApplicationCluster", &uids::dicomApplicationCluster);
		id_to_uid.insert("1.2.840.10008.15.0.3.9", &uids::dicomApplicationCluster);

		ident_to_uid.insert("dicomAssociationInitiator", &uids::dicomAssociationInitiator);
		id_to_uid.insert("1.2.840.10008.15.0.3.10", &uids::dicomAssociationInitiator);

		ident_to_uid.insert("dicomAssociationAcceptor", &uids::dicomAssociationAcceptor);
		id_to_uid.insert("1.2.840.10008.15.0.3.11", &uids::dicomAssociationAcceptor);

		ident_to_uid.insert("dicomHostname", &uids::dicomHostname);
		id_to_uid.insert("1.2.840.10008.15.0.3.12", &uids::dicomHostname);

		ident_to_uid.insert("dicomPort", &uids::dicomPort);
		id_to_uid.insert("1.2.840.10008.15.0.3.13", &uids::dicomPort);

		ident_to_uid.insert("dicomSOPClass", &uids::dicomSOPClass);
		id_to_uid.insert("1.2.840.10008.15.0.3.14", &uids::dicomSOPClass);

		ident_to_uid.insert("dicomTransferRole", &uids::dicomTransferRole);
		id_to_uid.insert("1.2.840.10008.15.0.3.15", &uids::dicomTransferRole);

		ident_to_uid.insert("dicomTransferSyntax", &uids::dicomTransferSyntax);
		id_to_uid.insert("1.2.840.10008.15.0.3.16", &uids::dicomTransferSyntax);

		ident_to_uid.insert("dicomPrimaryDeviceType", &uids::dicomPrimaryDeviceType);
		id_to_uid.insert("1.2.840.10008.15.0.3.17", &uids::dicomPrimaryDeviceType);

		ident_to_uid.insert("dicomRelatedDeviceReference", &uids::dicomRelatedDeviceReference);
		id_to_uid.insert("1.2.840.10008.15.0.3.18", &uids::dicomRelatedDeviceReference);

		ident_to_uid.insert("dicomPreferredCalledAETitle", &uids::dicomPreferredCalledAETitle);
		id_to_uid.insert("1.2.840.10008.15.0.3.19", &uids::dicomPreferredCalledAETitle);

		ident_to_uid.insert("dicomTLSCyphersuite", &uids::dicomTLSCyphersuite);
		id_to_uid.insert("1.2.840.10008.15.0.3.20", &uids::dicomTLSCyphersuite);

		ident_to_uid.insert("dicomAuthorizedNodeCertificateReference", &uids::dicomAuthorizedNodeCertificateReference);
		id_to_uid.insert("1.2.840.10008.15.0.3.21", &uids::dicomAuthorizedNodeCertificateReference);

		ident_to_uid.insert("dicomThisNodeCertificateReference", &uids::dicomThisNodeCertificateReference);
		id_to_uid.insert("1.2.840.10008.15.0.3.22", &uids::dicomThisNodeCertificateReference);

		ident_to_uid.insert("dicomInstalled", &uids::dicomInstalled);
		id_to_uid.insert("1.2.840.10008.15.0.3.23", &uids::dicomInstalled);

		ident_to_uid.insert("dicomStationName", &uids::dicomStationName);
		id_to_uid.insert("1.2.840.10008.15.0.3.24", &uids::dicomStationName);

		ident_to_uid.insert("dicomDeviceSerialNumber", &uids::dicomDeviceSerialNumber);
		id_to_uid.insert("1.2.840.10008.15.0.3.25", &uids::dicomDeviceSerialNumber);

		ident_to_uid.insert("dicomInstitutionName", &uids::dicomInstitutionName);
		id_to_uid.insert("1.2.840.10008.15.0.3.26", &uids::dicomInstitutionName);

		ident_to_uid.insert("dicomInstitutionAddress", &uids::dicomInstitutionAddress);
		id_to_uid.insert("1.2.840.10008.15.0.3.27", &uids::dicomInstitutionAddress);

		ident_to_uid.insert("dicomInstitutionDepartmentName", &uids::dicomInstitutionDepartmentName);
		id_to_uid.insert("1.2.840.10008.15.0.3.28", &uids::dicomInstitutionDepartmentName);

		ident_to_uid.insert("dicomIssuerOfPatientID", &uids::dicomIssuerOfPatientID);
		id_to_uid.insert("1.2.840.10008.15.0.3.29", &uids::dicomIssuerOfPatientID);

		ident_to_uid.insert("dicomPreferredCallingAETitle", &uids::dicomPreferredCallingAETitle);
		id_to_uid.insert("1.2.840.10008.15.0.3.30", &uids::dicomPreferredCallingAETitle);

		ident_to_uid.insert("dicomSupportedCharacterSet", &uids::dicomSupportedCharacterSet);
		id_to_uid.insert("1.2.840.10008.15.0.3.31", &uids::dicomSupportedCharacterSet);

		ident_to_uid.insert("dicomConfigurationRoot", &uids::dicomConfigurationRoot);
		id_to_uid.insert("1.2.840.10008.15.0.4.1", &uids::dicomConfigurationRoot);

		ident_to_uid.insert("dicomDevicesRoot", &uids::dicomDevicesRoot);
		id_to_uid.insert("1.2.840.10008.15.0.4.2", &uids::dicomDevicesRoot);

		ident_to_uid.insert("dicomUniqueAETitlesRegistryRoot", &uids::dicomUniqueAETitlesRegistryRoot);
		id_to_uid.insert("1.2.840.10008.15.0.4.3", &uids::dicomUniqueAETitlesRegistryRoot);

		ident_to_uid.insert("dicomDevice", &uids::dicomDevice);
		id_to_uid.insert("1.2.840.10008.15.0.4.4", &uids::dicomDevice);

		ident_to_uid.insert("dicomNetworkAE", &uids::dicomNetworkAE);
		id_to_uid.insert("1.2.840.10008.15.0.4.5", &uids::dicomNetworkAE);

		ident_to_uid.insert("dicomNetworkConnection", &uids::dicomNetworkConnection);
		id_to_uid.insert("1.2.840.10008.15.0.4.6", &uids::dicomNetworkConnection);

		ident_to_uid.insert("dicomUniqueAETitle", &uids::dicomUniqueAETitle);
		id_to_uid.insert("1.2.840.10008.15.0.4.7", &uids::dicomUniqueAETitle);

		ident_to_uid.insert("dicomTransferCapability", &uids::dicomTransferCapability);
		id_to_uid.insert("1.2.840.10008.15.0.4.8", &uids::dicomTransferCapability);

		ident_to_uid.insert("UniversalCoordinatedTime", &uids::UniversalCoordinatedTime);
		id_to_uid.insert("1.2.840.10008.15.1.1", &uids::UniversalCoordinatedTime);


		UidLookup {
			ident_to_uid: ident_to_uid,
			id_to_uid: id_to_uid,
		}
	}
}
