use crate::lib::{Box, Vec};

#[derive(Debug, Clone, PartialEq)]
pub enum Bencode<'a> {
    String(&'a [u8]),
    Integer(i64),
    List(Vec<Bencode<'a>>),
    Dictionary {
        input: &'a [u8],
        entries: Vec<(&'a str, Bencode<'a>)>,
    },
}

impl<'a> TryFrom<&'a [u8]> for Bencode<'a> {
    type Error = BencodeError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        crate::parse_bencoded(value)
            .map(|(_, value)| value)
            .map_err(|err| BencodeError::Parse(err.to_string()))
    }
}

pub type BencodeResult<T> = Result<T, BencodeError>;
pub type BoxedParser<'a, 'b> = Box<dyn FnOnce(Bencode<'a>) -> BencodeResult<()> + 'b>;

pub fn parse_bencoded_entries<'a, 'b, P, E>(parsers: P, entries: E) -> BencodeResult<()>
where
    P: IntoIterator<Item = (&'b str, BoxedParser<'a, 'b>)>,
    E: IntoIterator<Item = (&'a str, Bencode<'a>)>,
{
    let mut parsers = parsers.into_iter().peekable();
    'for_loop: for (key, value) in entries {
        loop {
            if let Some((parser_key, _)) = parsers.peek() {
                match key.cmp(parser_key) {
                    core::cmp::Ordering::Equal => {
                        if let Some((_, parser_fn)) = parsers.next() {
                            parser_fn(value)?;
                            break;
                        }
                    }
                    core::cmp::Ordering::Less => break,
                    core::cmp::Ordering::Greater => {
                        parsers.next();
                    }
                }
            } else {
                break 'for_loop;
            }
        }
    }
    Ok(())
}

pub trait Bencoded<'a>: Sized + core::fmt::Debug {
    fn try_from_bencoded(bencode: Bencode<'a>) -> Result<Self, BencodeError>;

    fn parse_bencoded_slice(slice: &'a [u8]) -> Result<Self, BencodeError> {
        Self::try_from_bencoded(slice.try_into()?)
    }

    fn init_fields<'c>(
        parsers: &mut Vec<(&'c str, BoxedParser<'a, 'c>)>,
        name: &'c str,
        value: &'c mut Option<Self>,
    ) {
        parsers.push((
            name,
            Box::new(move |bencode| {
                let field: Option<Self> = Bencoded::try_from_bencoded(bencode)?;
                *value = field;
                Ok(())
            }),
        ));
    }
}

impl<'a, T> Bencoded<'a> for Option<T>
where
    T: Bencoded<'a>,
{
    fn try_from_bencoded(bencode: Bencode<'a>) -> Result<Self, BencodeError> {
        T::try_from_bencoded(bencode).map(Some)
    }
}

impl<'a> Bencoded<'a> for &'a str {
    fn try_from_bencoded(bencode: Bencode<'a>) -> Result<Self, BencodeError> {
        if let Bencode::String(result) = bencode {
            core::str::from_utf8(result).map_err(From::from)
        } else {
            Err(BencodeError::NoMatch)
        }
    }
}

impl<'a> Bencoded<'a> for i64 {
    fn try_from_bencoded(bencode: Bencode<'a>) -> Result<Self, BencodeError> {
        if let Bencode::Integer(result) = bencode {
            Ok(result)
        } else {
            Err(BencodeError::NoMatch)
        }
    }
}

impl<'a> Bencoded<'a> for usize {
    fn try_from_bencoded(bencode: Bencode<'a>) -> Result<Self, BencodeError> {
        i64::try_from_bencoded(bencode)?
            .try_into()
            .map_err(From::from)
    }
}

impl<'a, T> Bencoded<'a> for Vec<T>
where
    T: Bencoded<'a>,
{
    fn try_from_bencoded(bencode: Bencode<'a>) -> Result<Self, BencodeError> {
        if let Bencode::List(result) = bencode {
            result.into_iter().map(T::try_from_bencoded).collect()
        } else {
            Err(BencodeError::NoMatch)
        }
    }
}

impl<'a, T> Bencoded<'a> for Vec<(&'a str, T)>
where
    T: Bencoded<'a>,
{
    fn try_from_bencoded(bencode: Bencode<'a>) -> Result<Self, BencodeError> {
        if let Bencode::Dictionary { entries, .. } = bencode {
            entries
                .into_iter()
                .map(|(key, value)| T::try_from_bencoded(value).map(|value| (key, value)))
                .collect()
        } else {
            Err(BencodeError::NoMatch)
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BencodeError {
    #[error("bencode parse error: {0}")]
    Parse(String),
    #[error("expected another type")]
    NoMatch,
    #[error("field {0} not found")]
    NoField(String),
    #[error(transparent)]
    Utf8Error(#[from] core::str::Utf8Error),
    #[error(transparent)]
    TryFromInt(#[from] core::num::TryFromIntError),
    #[error(transparent)]
    TryFromSlice(#[from] core::array::TryFromSliceError),
}
