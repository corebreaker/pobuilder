[![Crates.io]](https://crates.io/crates/pobuilder)
[![Docs.rs]](https://docs.rs/pobuilder/)
[![CircleCI]](https://circleci.com/gh/corebreaker/pobuilder/tree/main)
[![Coverage Status]](https://coveralls.io/github/corebreaker/pobuilder?branch=main)

# `pobuilder`

Rust library for forging and for parsing translation catalogs forked from the crate [poreader].

Only PO and Xliff are planned to be supported. For anything else, just convert it with [Translate Toolkit].
There is no point in replacing that excellent library;
the main reason for Rust parser and writer is to them as part of build
process of Rust programs, especially in procedural macros, which need to be written in Rust.

## Documentation

On [Docs.rs].

## Installation

It uses [Cargo], Rust's package manager.
You can depend on this library by adding `pobuilder` to your Cargo dependencies:

```toml
[dependencies]
pobuilder = "0.1"
```

Or, to use the Git repo directly:
```toml
[dependencies.pobuilder]
git = "https://github.com/corebreaker/pobuilder.git"
```

Or, finally by Cargo CLI:
```sh
cargo add pobuilder
```

## How to use

Start by creating a PO reader from a new PO parser, then iterate on the reader:
```rust
use pobuilder::PoParser;

use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    io::{Result, Error as IoError, ErrorKind},
    error::Error as StdError,
    env::args,
    fs::File,
};

struct NoArg;
impl StdError for NoArg {}

impl Display for NoArg {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { Debug::fmt(self, f) }
}

impl Debug for NoArg {
    fn fmt(&self, f: &mut Formatter) -> FmtResult { write!(f, "No file specified") }
}

fn main() -> Result<()> {
    // Filename
    let filename = match args().skip(1).next() {
        Some(v) => v,
        None => { return Err(IoError::new(ErrorKind::Other, NoArg)); }
    };

    // Open a file
    let file = File::open(filename)?;

    // Create PO parser
    let parser = PoParser::new()?;
    
    // Parse the opened file
    let pofile = parser.parse(file)?;

    // Read PO file by iterating on units
    for unit in pofile {
        let unit = unit?;

        // Show `msgid`
        println!(" - {}", unit.message().get_id())
    }

    Ok(())
}
```

# Status of the project

This project is in early development stage. It's a fork of the crate [poreader], which is a parser for PO files.

The writer was suggested by the crate [poreader], in an issue, [here][Writer-Issue], but it was never implemented.


[Cargo]: http://crates.io
[Docs.rs]: https://img.shields.io/docsrs/pobuilder?style=for-the-badge
[Crates.io]: https://img.shields.io/crates/v/pobuilder?style=for-the-badge
[CircleCI]: https://img.shields.io/circleci/build/github/corebreaker/pobuilder/main?style=for-the-badge
[Coverage Status]: https://img.shields.io/coveralls/github/corebreaker/pobuilder?style=for-the-badge
[poreader]: https://crates.io/crates/poreader
[Writer-Issue]: https://github.com/corebreaker/poreader/issues/15
