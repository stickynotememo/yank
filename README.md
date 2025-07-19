# yank
 [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
A command line tool to copy and paste files in a desktop-like fashion.

Files can be copied and saved to the clipboard, and later pasted. Unlike `cp`, it's not required to know the destination path beforehand.

## Installation
Markdownlint can be installed using cargo:
```cargo install yank```
~~or on the AUR: ```yay -S yank```~~

## Usage
```bash
yank [FILE] # to copy a file.
yank -x [FILE] # cut the file
cd destination_folder
yank # to paste, optionally use -p flag to specify paste filename
```

## Notes
[crates.io/yank](https://crates.io/crates/yank) exists and has a similar function, but is no longer maintained. I was not aware of this crate before developing this tool.
