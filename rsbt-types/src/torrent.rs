use core::{cell::RefCell, convert::TryInto};

use rsbt_bencode_nom7::{Bencode, BencodeError, BencodeParse, Bencoded};

use crate::{Arc, Vec};

/// <div class="section" id="metainfo-files">
/// <h1>metainfo files</h1>
/// <p>Metainfo files (also known as .torrent files) are bencoded dictionaries
/// with the following keys:</p>
/// <dl class="docutils">
/// <dt>announce</dt>
/// <dd>The URL of the tracker.</dd>
/// <dt>info</dt>
/// <dd>This maps to a dictionary, with keys described below.</dd>
/// </dl>
/// <p>All strings in a .torrent file that contains text must be UTF-8
/// encoded.</p>
/// </div>
///
/// See [BEP 3, metainfo files](https://www.bittorrent.org/beps/bep_0003.html)
#[derive(Debug, BencodeParse)]
pub struct Torrent<'a> {
    announce: &'a str,
    info: Info<'a>,
}

/// <div class="section" id="info-dictionary">
/// <h2>info dictionary</h2>
/// <p>The <tt class="docutils literal">name</tt> key maps to a UTF-8 encoded string which is the
/// suggested name to save the file (or directory) as. It is purely advisory.</p>
/// <p><tt class="docutils literal">piece length</tt> maps to the number of bytes in each piece
/// the file is split into. For the purposes of transfer, files are
/// split into fixed-size pieces which are all the same length except for
/// possibly the last one which may be truncated. <tt class="docutils literal">piece
/// length</tt> is almost always a power of two, most commonly 2 18 =
/// 256 K (BitTorrent prior to version 3.2 uses 2 20 = 1 M as
/// default).</p>
/// <p><tt class="docutils literal">pieces</tt> maps to a string whose length is a multiple of
/// 20. It is to be subdivided into strings of length 20, each of which is
/// the SHA1 hash of the piece at the corresponding index.</p>
/// <p>There is also a key <tt class="docutils literal">length</tt> or a key <tt class="docutils literal">files</tt>,
/// but not both or neither. If <tt class="docutils literal">length</tt> is present then the
/// download represents a single file, otherwise it represents a set of
/// files which go in a directory structure.</p>
/// <p>In the single file case, <tt class="docutils literal">length</tt> maps to the length of
/// the file in bytes.</p>
/// <p>For the purposes of the other keys, the multi-file case is treated as
/// only having a single file by concatenating the files in the order they
/// appear in the files list. The files list is the value
/// <tt class="docutils literal">files</tt> maps to, and is a list of dictionaries containing
/// the following keys:</p>
/// <p><tt class="docutils literal">length</tt> - The length of the file, in bytes.</p>
/// <p><tt class="docutils literal">path</tt> - A list of UTF-8 encoded strings corresponding to subdirectory
/// names, the last of which is the actual file name (a zero length list
/// is an error case).</p>
/// <p>In the single file case, the name key is the name of a file, in the
/// muliple file case, it's the name of a directory.</p>
/// </div>
#[derive(BencodeParse, Debug)]
pub struct Info<'a> {
    name: &'a str,
    #[bencode(rename = "piece length")]
    piece_length: usize,
    pieces: Pieces<'a>,
    files: Files<'a>,
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
#[derive(Debug)]
pub enum Files<'a> {
    // #[bencode(rename = "length")]
    SingleFile(usize),
    // #[bencode(rename = "files")]
    Files(Vec<File<'a>>),
}

impl<'a> ::rsbt_bencode_nom7::Bencoded<'a> for Files<'a> {
    fn init_fields<'c>(
        parsers: &mut Vec<(
            &'c str,
            Box<dyn FnOnce(Bencode<'a>) -> Result<(), BencodeError> + 'c>,
        )>,
        _: &'c str,
        value: &'c mut Option<Self>,
    ) -> Result<(), BencodeError> {
        let f = Arc::new(RefCell::new(move |field| *value = field));
        let f_clone = f.clone();
        parsers.push((
            "length",
            Box::new(move |bencode| {
                let field: Option<_> = Bencoded::try_from_bencoded(bencode)?;
                f_clone.borrow_mut()(field.map(Self::SingleFile));
                Ok(())
            }),
        ));
        parsers.push((
            "files",
            Box::new(move |bencode| {
                let field: Option<_> = Bencoded::try_from_bencoded(bencode)?;
                f.borrow_mut()(field.map(Self::Files));
                Ok(())
            }),
        ));

        Ok(())
    }

    fn try_from_bencoded(
        bencode: ::rsbt_bencode_nom7::Bencode<'a>,
    ) -> Result<Self, ::rsbt_bencode_nom7::BencodeError> {
        use ::rsbt_bencode_nom7::Bencode::*;
        match bencode {
            Dictionary(entries) => {
                todo!()
            }
            String(_) | Integer(_) | List(_) => Err(::rsbt_bencode_nom7::BencodeError::NoMatch),
        }
    }
}

#[derive(Debug, BencodeParse)]
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
