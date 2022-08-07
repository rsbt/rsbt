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
mod torrent;

pub(crate) use rsbt_bencode_nom7::lib::{Arc, Vec};

pub use torrent::Torrent;
