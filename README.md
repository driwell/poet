## What is Poet?

It's a specialized tool to process dialogue topics extracted from Skyrim for DBVO voice creators.

**Poet** provides multiple functionalities that aim to automate the entire process between the extraction and the generations of the voiced dialogue lines.

Being a specialized tool instead of a generic parser it's able to modify the text automatically and provides advice in order to mitigate the common limitations of DBVO, a few examples are:

1. Removes leading non alphabetic characters.

2. Outputs the list of modified lines and tells the user to create a .esp patch to make them work.

3. When strange characters are encountered beyond the common cases that can be fixed automatically those will also be listed separately so the creator can decide on how to modify them.

## How to use?

The application includes documentation which can be accessed through the `--help` command.

`poet [-h, --help]`

```
Parses exported topics

Usage: poet [OPTIONS] <path>

Arguments:
  <path>  File path

Options:
  -a, --all                  Print all lines
  -f, --find <PATTERN>       Print lines with a PATTERN
  -r, --replace <OLD> <NEW>  Print lines with OLD replaced by NEW
  -h, --help                 Print help
  -V, --version              Print version
```
