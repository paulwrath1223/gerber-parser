# Gerber-parser

A simple `gerber` parser written in rust to be used with the `gerber-types` crate. 

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};
use gerber_parser::parser::parse_gerber;

// open a .gbr file from system
let file = File::open(path).unwrap();
let reader = BufReader::new(file);

// Now we parse the file to a GerberDoc 
let gerber_doc: GerberDoc = parse_gerber(reader);

// it is possible to convert to an 'atomic' representation purely 
// in terms of Vec<Command> of the gerber-types crate
let commands:Vec<Command> = gerber_doc.to_commands();
```

### Current State

⚠️ Note: this package is still in development and does not cover the full Gerber spec

Currently missing

* All `AM` commands
* `LM`, `LR`, `LS`, `IP` commands (note: these are deprecated in the spec)
* `AB` commands

Partial:

* The `TF` and `TA` commands only support a limited range of arguments; custom attributes will result in an error

### General to-do

* Do proper coordinate check (compatibility with `format specification`)
* Test with output from more programs (I use f360 and that seems to work)
