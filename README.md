messer
------

# Usage
```
$ messer --help
Utility for replacing all occurrences of source text with replacement text in all files from provided directory

Usage: messer [OPTIONS] -s <source-text> -r <replacement-text> <directory>

Arguments:
  <directory>  Directory with text files to be edited

Options:
  -s <source-text>           Source text to search for
  -r <replacement-text>      Replacement text
  -i                         Perform a case-insensitive search
  -h, --help                 Print help
  -V, --version              Print version
  

$ messer /my/dir -s "Text to replace" -r "Replacement" -i
```