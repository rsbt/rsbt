#![cfg_attr(not(feature = "std"), no_std)]
/*!
# rsbt-types description

## Features

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
rsbt-types = "0.1"
```

*/
#[cfg(feature = "alloc")]
extern crate alloc;

mod torrent;

#[cfg(feature = "alloc")]
pub(crate) use alloc::vec::Vec;
#[cfg(not(feature = "alloc"))]
pub(crate) use std::vec::Vec;
