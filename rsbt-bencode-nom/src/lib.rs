#![cfg_attr(not(feature = "std"), no_std)]
/*!
# rsbt-bencode-nom description

## Features

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
rsbt-bencode-nom = "0.1"
```

*/

mod parser;
mod types;

pub(crate) use rsbt_defs::lib;

#[cfg(feature = "derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate rsbt_bencode_derive;

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use rsbt_bencode_derive::*;

pub use parser::parse_bencoded;
pub use types::{
    parse_bencoded_entries, Bencode, BencodeError, BencodeResult, Bencoded, BoxedParser,
};
