# dcmpipe_tests #

This crate has no content other than tests. The reason for splitting tests into a separate crate is due to needing to depend on both `dcmpipe_lib` and `dcmpipe_dict` in order to test the dicom parser with the standard dicom dictionary.

## Test Fixtures ##
Test fixtures are files are needed to run several tests here. These fixtures are a suite of dicom files gathered from several sources:
 - [gdcm](http://gdcm.sourceforge.net/), notably
   - [gdcmData](https://sourceforge.net/projects/gdcm/files/gdcmData/)
   - [gdcmConformanceTests](https://sourceforge.net/projects/gdcm/files/gdcmConformanceTests/)
 - [David Clunie](https://www.dclunie.com/), at the bottom of the page under `Images`

These fixtures should be downloaded and extracted into this crate

[{F22119}](https://speck.phacility.com/F22119)
