use crate::{parser::parse_handshake, RsbtBencodeError, SHA1_SIZE};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Handshake {
    pub protocol_prefix: [u8; 20],
    pub reserved: [u8; 8],
    pub info_hash: [u8; SHA1_SIZE],
    pub peer_id: [u8; 20],
}

impl TryFrom<Vec<u8>> for Handshake {
    type Error = RsbtBencodeError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        parse_handshake(&value)
    }
}

#[cfg(test)]
mod tests {
    use super::{Handshake, SHA1_SIZE};
    use std::mem::size_of;

    fn size_check<T>(required: usize) {
        assert_eq!(size_of::<T>(), required);
    }

    #[test]
    fn size_checks() {
        size_check::<Handshake>(20 + 8 + SHA1_SIZE + 20)
    }
}
