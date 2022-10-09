#![cfg_attr(not(feature = "std"), no_std)]
/*!
# rsbt-defs description

## Features

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
rsbt-defs = "0.1"
```

*/
#[cfg(feature = "alloc")]
extern crate alloc;

pub mod lib {
    #[cfg(feature = "alloc")]
    pub use alloc::{boxed::Box, format, string::String, sync::Arc, vec::Vec};
    #[cfg(not(feature = "alloc"))]
    pub use std::{boxed::Box, format, string::String, sync::Arc, vec::Vec};
}
