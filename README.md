## What is Poet?

It's a specialized tool to process dialogue topics extracted from Skyrim for DBVO voice creators.

**Poet** provides multiple functionalities that aim to automate the entire process between the extractions and the generations of the voiced dialogue lines.

Being a specialized tool instead of a generic parser it's able to modify the text automatically and provides advice in order to mitigate the common limitations of DBVO, a few examples are:

1. Removes leading non alphabetic characters.

2. Outputs the list of modified lines and tells the user to create a .esp patch to make them work.

3. When strange characters are encountered beyond the common cases that can be fixed automatically those will also be listed separately so the creator can decide on how to modify them.

## How to use?

`poet [OPTIONS] FILE_PATH`

**clean** - Generates file with sensible dialogue replacements. When applicable, it will generate three extra files.

1. [FILE_PATH_clean.topic] File with sensible replacements. Ready to generate dialogue lines with.

2. [FILE_PATH_alias_player.txt] File with topics where there's an occurrence of <Alias=Player>.

3. [FILE_PATH_unresolved.txt] File with topics which poet wasn't able to automatically resolve. Manual action is required.

4. [FILE_PATH_clean.log] Messages that require attention. Errors, warnings or advices will be in this file.

`poet [-c --clean] FILE_PATH`
