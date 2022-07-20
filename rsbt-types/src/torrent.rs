use core::convert::TryInto;

use rsbt_bencode_nom7::{Bencode, BencodeError, BencodeParse, Bencoded};

use crate::Vec;

#[derive(Debug, BencodeParse)]
pub struct Torrent<'a> {
    announce: &'a str,
    info: Info<'a>,
}

#[derive(BencodeParse, Debug)]
pub struct Info<'a> {
    name: &'a str,
    #[bencode(rename = "piece length")]
    piece_length: usize,
    pieces: Pieces<'a>,
    // files: Files<'a>,
}

#[derive(Debug)]
pub struct Sha1<'a>(&'a [u8; 20]);

#[derive(Debug)]
pub struct Pieces<'a>(Vec<Sha1<'a>>);

impl<'a> Bencoded<'a> for Pieces<'a> {
    fn try_from_bencoded(bencode: Bencode<'a>) -> Result<Self, BencodeError> {
        if let Bencode::String(list) = bencode {
            let mut chunks = list.chunks_exact(20);
            chunks
                .by_ref()
                .map(|x| x.try_into().map(Sha1))
                .collect::<Result<Vec<_>, _>>()
                .map_err(BencodeError::from)
                .and_then(|result| {
                    if chunks.remainder().is_empty() {
                        Ok(Self(result))
                    } else {
                        Err(BencodeError::NoMatch)
                    }
                })
        } else {
            Err(BencodeError::NoMatch)
        }
    }
}
// #[derive(BencodeParse)]
// pub enum Files<'a> {
//     #[bencode(rename = "length")]
//     SingleFile(usize),
//     #[bencode(rename = "files")]
//     Files(Vec<File<'a>>),
// }

// #[derive(BencodeParse)]
pub struct File<'a> {
    length: usize,
    path: Vec<&'a str>,
}

mod tests {
    use super::*;
    use rsbt_bencode_nom7::Bencoded;

    #[test]
    fn parse_torrent() {
        let torrent = Torrent::parse_bencoded_slice(
            &include_bytes!("../../rsbt-bencode/tests/big-buck-bunny.torrent")[..],
        )
        .expect("bencoded torrent");
        dbg!(torrent);
    }
}
