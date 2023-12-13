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
  -I, --interactive          Prompt before every file change
  -h, --help                 Print help
  -V, --version              Print version
  

$ messer /my/dir -s "Text to replace" -r "Replacement" -iv
messer: changed './example1.md'
messer: changed './dir1/dir2/example3.sh'

$ messer /my/dir -s "Text to replace" -r "Replacement" -ivI
messer: change file './example1.md' [y/N]? y
messer: changed './example1.md'
```