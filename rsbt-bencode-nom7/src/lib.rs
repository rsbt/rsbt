#![cfg_attr(not(feature = "std"), no_std)]
/*!
# rsbt-bencode-nom7 description

## Features

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
rsbt-bencode-nom7 = "0.1"
```

*/

#[cfg(feature = "alloc")]
extern crate alloc;

mod parser;
mod types;

#[cfg(feature = "alloc")]
pub(crate) use alloc::{boxed::Box, vec::Vec};
#[cfg(not(feature = "alloc"))]
pub(crate) use std::{boxed::Box, vec::Vec};

#[cfg(feature = "derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate rsbt_bencode_derive;

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use rsbt_bencode_derive::*;

pub use parser::parse_bencoded;
pub use types::{Bencode, BencodeError, Bencoded};
