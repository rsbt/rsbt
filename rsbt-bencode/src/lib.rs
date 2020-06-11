/*!
# rsbt-bencode description

## Features

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
rsbt-bencode = "0.1"
```

*/

use std::array::TryFromSliceError;
use thiserror::Error;

mod parser;
mod types;

pub use types::Handshake;

pub(crate) const SHA1_SIZE: usize = 20;

#[derive(Error, Debug)]
pub enum RsbtBencodeError {
    #[error("parser failed")]
    Parser,
}

impl<'a> From<nom::Err<&'a [u8]>> for RsbtBencodeError {
    fn from(_: nom::Err<&'a [u8]>) -> Self {
        RsbtBencodeError::Parser
    }
}

impl<'a> From<nom::Err<(&'a [u8], nom::error::ErrorKind)>> for RsbtBencodeError {
    fn from(_: nom::Err<(&'a [u8], nom::error::ErrorKind)>) -> Self {
        RsbtBencodeError::Parser
    }
}

impl From<TryFromSliceError> for RsbtBencodeError {
    fn from(value: TryFromSliceError) -> Self {
        RsbtBencodeError::Parser
    }
}
