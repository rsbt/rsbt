use crate::{types::Handshake, RsbtBencodeError};
use nom::*;

use std::convert::TryInto;

fn arr_20(s: &[u8]) -> Result<[u8; 20], RsbtBencodeError> {
    s.try_into().map_err(RsbtBencodeError::from)
}

fn arr_8(s: &[u8]) -> Result<[u8; 8], RsbtBencodeError> {
    s.try_into().map_err(RsbtBencodeError::from)
}

named!(
    parser_handshake<Handshake>,
    do_parse!(
        protocol_prefix: map_res!(take!(20), arr_20)
            >> reserved: map_res!(take!(8), arr_8)
            >> info_hash: map_res!(take!(20), arr_20)
            >> peer_id: map_res!(take!(20), arr_20)
            >> (Handshake {
                protocol_prefix,
                reserved,
                info_hash,
                peer_id,
            })
    )
);

pub fn parse_handshake(bytes: &[u8]) -> Result<Handshake, RsbtBencodeError> {
    parser_handshake(bytes)
        .map(|x| x.1)
        .map_err(RsbtBencodeError::from)
}
