use crate::{Box, Vec};

#[derive(Debug, Clone, PartialEq)]
pub enum Bencode<'a> {
    String(&'a [u8]),
    Integer(i64),
    List(Vec<Bencode<'a>>),
    Dictionary(Vec<(&'a str, Bencode<'a>)>),
}

impl<'a> TryFrom<&'a [u8]> for Bencode<'a> {
    type Error = BencodeError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        crate::parse_bencoded(value)
            .map(|(_, value)| value)
            .map_err(|err| BencodeError::Parse(err.to_string()))
    }
}

type BencodeResult<T> = Result<T, BencodeError>;
type BoxedParser<'a, I> = Box<dyn FnOnce(I) -> BencodeResult<I> + 'a>;

pub trait Bencoded<'a>: Sized {
    fn try_from_bencoded(bencode: Bencode<'a>) -> Result<Self, BencodeError>;

    fn parse_bencoded_slice(slice: &'a [u8]) -> Result<Self, BencodeError> {
        Self::try_from_bencoded(slice.try_into()?)
    }

    fn field<I>(entries: &mut I, name: &str) -> Result<Self, BencodeError>
    where
        I: Iterator<Item = (&'a str, Bencode<'a>)>,
    {
        let field: Option<Self> = Bencoded::field(entries, name)?;
        field.ok_or_else(|| BencodeError::NoField(name.into()))
    }

    fn field_parsers<'b, A, I>(
        name: &'static str,
        apply_fn: A,
    ) -> Vec<(&'static str, BoxedParser<'b, I>)>
    where
        I: Iterator<Item = (&'a str, Bencode<'a>)>,
        A: FnOnce(Option<Self>) + 'b,
    {
        let mut res: Vec<(&'static str, BoxedParser<'b, I>)> = Vec::new();
        res.push((name, Box::new(|mut entries| {
            let field: Option<Self> = Bencoded::field(&mut entries, name)?;
            apply_fn(field);
            Ok(entries)
        })));
        res
    }
}

impl<'a, T> Bencoded<'a> for Option<T>
where
    T: Bencoded<'a>,
{
    fn try_from_bencoded(bencode: Bencode<'a>) -> Result<Self, BencodeError> {
        T::try_from_bencoded(bencode).map(Some)
    }

    fn field<I>(entries: &mut I, name: &str) -> Result<Self, BencodeError>
    where
        I: Iterator<Item = (&'a str, Bencode<'a>)>,
    {
        let mut field = None;
        for (key, value) in entries.by_ref() {
            field = match key.cmp(name) {
                core::cmp::Ordering::Less => continue,
                core::cmp::Ordering::Equal => Self::try_from_bencoded(value)?,
                _ => break,
            };
            break;
        }
        Ok(field)
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
        if let Bencode::Dictionary(result) = bencode {
            result
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
    #[error(transparent)]
    Utf8Error(#[from] core::str::Utf8Error),
    #[error(transparent)]
    TryFromInt(#[from] core::num::TryFromIntError),
    #[error("field {0} not found")]
    NoField(String),
}
