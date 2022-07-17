use nom::{
    branch::alt,
    bytes::complete::take,
    character::complete as character,
    combinator::{map, map_res},
    error::Error,
    multi::many0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::Bencode;

fn parse_string(input: &[u8]) -> IResult<&[u8], &[u8], Error<&[u8]>> {
    let (input, len) = character::u64(input)?;
    let (input, _) = character::char(':')(input)?;
    take(len)(input)
}

fn parse_integer(input: &[u8]) -> IResult<&[u8], i64, Error<&[u8]>> {
    delimited(character::char('i'), character::i64, character::char('e'))(input)
}

fn parse_list(input: &[u8]) -> IResult<&[u8], Vec<Bencode>, Error<&[u8]>> {
    delimited(
        character::char('l'),
        many0(parse_bencoded),
        character::char('e'),
    )(input)
}

type BencodeEntry<'a> = (&'a str, Bencode<'a>);

fn parse_dictionary(input: &[u8]) -> IResult<&[u8], Vec<BencodeEntry<'_>>, Error<&[u8]>> {
    delimited(
        character::char('d'),
        many0(tuple((
            map_res(parse_string, core::str::from_utf8),
            parse_bencoded,
        ))),
        character::char('e'),
    )(input)
}

pub fn parse_bencoded(input: &[u8]) -> IResult<&[u8], Bencode, Error<&[u8]>> {
    alt((
        map(parse_string, Bencode::String),
        map(parse_integer, Bencode::Integer),
        map(parse_list, Bencode::List),
        map(parse_dictionary, Bencode::Dictionary),
    ))(input)
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_string() {
        assert_eq!(
            (&[][..], &b"abc"[..]),
            super::parse_string(b"3:abc").unwrap()
        );
        assert_eq!(
            (&[][..], &b"abcdefghjkl"[..]),
            super::parse_string(b"11:abcdefghjkl").unwrap()
        );
        assert_eq!((&[][..], &b"a"[..]), super::parse_string(b"1:a").unwrap());
        assert_eq!((&[][..], &[][..]), super::parse_string(b"0:").unwrap());
        assert!(matches!(super::parse_string(b"-3:qqq"), Err(_)));
        assert!(matches!(super::parse_string(b"-0:"), Err(_)));
    }

    #[test]
    fn parse_integer() {
        assert_eq!((&[][..], 123), super::parse_integer(b"i123e").unwrap());
        assert_eq!((&[][..], -123), super::parse_integer(b"i-123e").unwrap());
        assert_eq!((&[][..], 0), super::parse_integer(b"i0e").unwrap());
    }

    #[test]
    fn parse_list() {
        use super::Bencode::*;

        assert_eq!((&[][..], Vec::new()), super::parse_list(b"le").unwrap());
        assert_eq!(
            (&[][..], [Integer(123)].to_vec()),
            super::parse_list(b"li123ee").unwrap()
        );
        assert_eq!(
            (&[][..], [String(b"abcdefghjkl")].to_vec()),
            super::parse_list(b"l11:abcdefghjkle").unwrap()
        );
        assert_eq!(
            (&[][..], [Integer(123), String(b"abcdefghjkl")].to_vec()),
            super::parse_list(b"li123e11:abcdefghjkle").unwrap()
        );
    }
}
