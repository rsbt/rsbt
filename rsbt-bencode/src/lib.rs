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
pub(crate) const BLOCK_SIZE: usize = 1 << 14;

#[inline]
pub(crate) fn count_parts(total: usize, part_size: usize) -> usize {
    total / part_size + if total % part_size != 0 { 1 } else { 0 }
}

#[inline]
pub fn index_in_bitarray(index: usize) -> (usize, u8) {
    (index / 8, 128 >> (index % 8))
}

#[inline]
pub fn bit_by_index(index: usize, data: &[u8]) -> Option<(usize, u8)> {
    let (index_byte, index_bit) = index_in_bitarray(index);
    data.get(index_byte).and_then(|&v| {
        if v & index_bit == index_bit {
            Some((index_byte, index_bit))
        } else {
            None
        }
    })
}

#[derive(Error, Debug)]
pub enum RsbtBencodeError {
    #[error("parser failed")]
    Parser,
    #[error("conversion failed: {0}")]
    FailureReason(String),
    #[error("conversion failed: {0}")]
    Bencode(TryFromBencode),
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

impl From<TryFromBencode> for RsbtBencodeError {
    fn from(value: TryFromBencode) -> Self {
        RsbtBencodeError::Bencode(value)
    }
}

#[derive(Error, Debug)]
pub enum TryFromBencode {
    #[error("not a string bencode")]
    NotString,
    #[error("not an integer bencode")]
    NotInteger,
    #[error("not a list bencode")]
    NotList,
    #[error("not a dictionary bencode")]
    NotDictionary,
    #[error("not valid utf-8 {0}")]
    NotUtf8(std::str::Utf8Error),
    #[error("not valid ip {0}")]
    NotValidIp(std::net::AddrParseError),
}
