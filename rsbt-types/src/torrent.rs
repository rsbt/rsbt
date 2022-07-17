use rsbt_bencode_nom7::{Bencode, BencodeError, BencodeParse};

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
    #[bencode(parse_with = "parse_vec_sha1")]
    pieces: Vec<Sha1<'a>>,
    // files: Files<'a>,
}

#[derive(Debug)]
pub struct Sha1<'a>(&'a [u8; 20]);

fn parse_vec_sha1<'a>(bencode: Bencode<'a>) -> Result<Vec<Sha1<'a>>, BencodeError> {
    todo!()
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
