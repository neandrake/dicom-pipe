# dcmpipe_cli #

A set of command-line tool examples developed to test out the `dcmpipe_lib` library, both functionality and ergonomics of the API.

```lang=console
$ ./dcmpipe_cli
Explore DICOM

Usage: dcmpipe_cli <COMMAND>

Commands:
  print    Parses a single file and prints the DICOM elements to stdout
  browse   Browse a DICOM dataset in a text-based user interface
  index    Manage a database index of DICOM on disk
  archive  Archives DICOM datasets from a source folder into a destination folder
  scp      Starts an SCP service
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Print
```lang=console
$ ./dcmpipe_cli help print
Parses a single file and prints the DICOM elements to stdout

Usage: dcmpipe_cli print <FILE>

Arguments:
  <FILE>  The file to process as a DICOM dataset

Options:
  -h, --help  Print help
```

## Browse
```lang=console
$ ./dcmpipe_cli help browse
Browse a DICOM dataset in a text-based user interface

Usage: dcmpipe_cli browse <FILE>

Arguments:
  <FILE>  The file to process as a DICOM dataset

Options:
  -h, --help  Print help
```

## Index
```lang=console
$ ./dcmpipe_cli help index
Manage a database index of DICOM on disk.

Recursively scans a folder for DICOM datasets, indexing them into a database.

Usage: dcmpipe_cli index --db <DB> <COMMAND>

Commands:
  scan    Recursively scans a folder for DICOM datasets, indexing them into a database
  verify  Verify records in the database reference valid files on-disk
  help    Print this message or the help of the given subcommand(s)

Options:
  -d, --db <DB>
          The db URI of the index

  -h, --help
          Print help (see a summary with '-h')
```

## Archive
```lang=console
$ ./dcmpipe_cli help archive
Archives DICOM datasets from a source folder into a destination folder.

The source folder is assumed to be unstructured whereas the DICOM datasets will be copied into the destination folder in a consistent structure: - One series per folder - Each DICOM file will be named in the format `[SOP_UID].dcm`

Usage: dcmpipe_cli archive <SOURCE> <DESTINATION>

Arguments:
  <SOURCE>
          The source folder of DICOM datasets to process

  <DESTINATION>
          The destination folder to archive datasets into

Options:
  -h, --help
          Print help (see a summary with '-h')
```

## SCP
```lang=console
$ ./dcmpipe_cli help scp
Starts an SCP service

Usage: dcmpipe_cli scp [OPTIONS] --host <HOST> --aetitle <AETITLE> --max-assoc <MAX_ASSOC>

Options:
  -h, --host <HOST>
          The host/port to bind the service on

  -a, --aetitle <AETITLE>
          The AE Title to run as

  -m, --max-pdu-size <MAX_PDU_SIZE>
          The maximum PDU size to receive.

          Size is specified in bytes and should be no more than `u32::MAX`. If not specified then no maximum is configured.

  -m, --max-assoc <MAX_ASSOC>
          The maximum number of concurrent associations

  -d, --db <DB>
          The database URL for resolving DIMSE queries.

          If not specified then Query/Retrieve will not return results, and Store will not persist received series.

  -a, --accept-aet <ACCEPT_AET>
          Specifies an accepted AE Title for associations. Can be specified multiple times.

          The format for each accepted AE Title is `key=val` where `key` is a valid AE Title such as `MY_AE`, and `val` is the IP + port such as `127.0.0.1:4001`.

          If no accepted AE Titles are specified then all AE Titles are accepted, but cannot be connected to, such as for handling C-MOVE requests.

  -h, --help
          Print help (see a summary with '-h')
```

## SCU
```lang=console
$ ./dcmpipe_cli help scu
Issue commands as an SCU

Usage: dcmpipe_cli scu [OPTIONS] --host <HOST> --my-ae <MY_AE> --host-ae <HOST_AE> <COMMAND>

Commands:
  echo   Issue a C-ECHO command
  find   Issue a C-FIND command
  store  Issue a C-STORE command
  move   Issue a C-MOVE command
  get    Issue a C-GET command
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --host <HOST>
          The host/port to connect to

  -m, --my-ae <MY_AE>
          The AE Title to represent this SCU

  -h, --host-ae <HOST_AE>
          The target AE Title on the host

  -m, --max-pdu-size <MAX_PDU_SIZE>
          The maximum PDU size to receive.

          Size is specified in bytes and should be no more than `u32::MAX`. If not specified then no maximum is configured.

  -h, --help
          Print help (see a summary with '-h')
```
