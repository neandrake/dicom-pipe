= dcmpipe_cli =

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
    dump       Parses a single file and prints the DICOM elements to stdout
    edit       Opens a DICOM dataset in a TUI for browsing and editing
    help       Prints this message or the help of the given subcommand(s)
    index      Index a directory of unstructured DICOM files
    scan       Scans a folder recursively for DICOM datasets and prints results of found DICOM
```
