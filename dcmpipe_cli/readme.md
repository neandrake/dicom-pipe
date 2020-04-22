# dcmpipe_cli

A set of command-line tools for using `dcmpipe_lib` to get an idea of the API.

## Commands
 - `dump` - Parses a single file and prints the DICOM elements to stdout.
   - `--stream` - If specified, instead of reading the entire dataset into memory will print elements as they are
                  parsed.
 - `edit` - Opens a DICOM dataset in a TUI for browsing and editing.
 - `scan` - Scans a folder recursively for DICOM datasets and prints results of found DICOM.
 
 