# dcmpipe_archiver

This is a command-line tool for scanning directories of DICOM and either
 - Inserting into mongo database as per-series records, via the `scan` command
 - Importing the files into an on-disk location, where the destination will be maintained with an expected
   layout/structure. This should probably also update mongo database.
 
