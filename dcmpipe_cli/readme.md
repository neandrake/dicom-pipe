# dcmpipe_cli #

A set of command-line tools for using `dcmpipe_lib` for testing functionality and the API.

```lang=console
Explore DICOM

USAGE:
    dcmpipe_cli <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    archive    Archives DICOM datasets from a source folder into a destination folder
    browse     Opens a DICOM dataset in a TUI for browsing and editing
    help       Prints this message or the help of the given subcommand(s)
    index      Manage a database index of DICOM on disk
    scan       Recursively scans a folder of DICOM datasets and prints results of parsing
    print      Parses a single file and prints the DICOM elements to stdout
```
