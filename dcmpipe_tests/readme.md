# dcmpipe_tests #

This crate has no content other than tests. The reason for splitting tests into a separate crate is due to needing to depend on both `dcmpipe_lib` and `dcmpipe_dict` in order to test the dicom parser with the standard dicom dictionary.

## Test Fixtures ##
Test fixtures are files are needed to run several tests here. Due to the large size of these test datasets they are not included in the repository. To set up these test fixtures:
1. [Download the fixutres (700mb .7z)](https://drive.google.com/file/d/1VI89r3leiLPm9-8sClyy0o15KtGZeqaD/view?usp=sharing)
2. From the `dcmpipe_tests` folder extract the downloaded archive
3. The resulting extraction should create a `dcmpipe_tests/fixtures` folder with subfolders containing the test datasets.

---
_These fixtures are a suite of dicom files gathered from several sources:_
 - [gdcm](http://gdcm.sourceforge.net/), notably
   - [gdcmData](https://sourceforge.net/projects/gdcm/files/gdcmData/)
   - [gdcmConformanceTests](https://sourceforge.net/projects/gdcm/files/gdcmConformanceTests/)
 - [David Clunie](https://www.dclunie.com/), at the bottom of the page under `Images`
