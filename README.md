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
  -v, --verbose              Print name of each changed file
  -i, --ignore-case          Perform a case-insensitive search
  -h, --help                 Print help
  -V, --version              Print version
  

$ messer /my/dir -s "Text to replace" -r "Replacement" -iv
Changed file: ./example1.md
Changed file: ./example2.txt
Changed file: ./example3.sh
```