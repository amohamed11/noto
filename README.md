# noto
A minimal note-taking cli tool for jogging down your thoughts quickly.

## Overview

noto creates new notes using timestamps (eg. 20211231045135.md) in a pre-configured directory and opens said note in your desired editor.  
By default noto has a minimal template for new notes (just title & tags), but can add your own templates for new notes easily.

```
noto 0.1
A minimal note-taking cli tool for jogging down your thoughts quickly.

USAGE:
    noto [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -c, --config <CONFIG>    Path for configuration file. Default is ~/.config/noto/config
    -h, --help               Print help information
    -V, --version            Print version information

SUBCOMMANDS:
    help          Print this message or the help of the given subcommand(s)
    new           create a new note with current timestamp for id
    set-editor    Set editor to use for opening the new note
    set-folder    Set the folder for storing your notes. Make sure the folder path is absolute
```

Checkout `TODO.md` for planned features & current progress.

## Installation

For now you have to install from repo/source instead of crates.io.  
**Git**  
`cargo install --git https://github.com/amohamed11/noto.git`  

**Source**  
`git clone https://github.com/amohamed11/noto.git`  
`cargo install --path noto/`
