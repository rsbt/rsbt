use crate::{
    types::{BencodeBlob, BencodeValue},
    RsbtBencodeError,
};
use nom::character::complete::digit1;
use nom::*;
use std::str::from_utf8;

macro_rules! recognize_map (
    ($i:expr, $submac:ident!( $($args:tt)* ), $g:expr) => (
        {
            use nom::Offset;
            use nom::Slice;
            $submac!($i, $($args)*).map(
                |(i, res)|
                    (i, $g(
                        ($i).slice( .. (&$i).offset(&i) ),
                        res
                    ))
            )
        }
    );
);

named!(
    integer_literal,
    recognize!(do_parse!(opt!(char!('-')) >> digit1 >> ()))
);

named!(
    integer<i64>,
    map_res!(map_res!(integer_literal, from_utf8), |s: &str| {
        s.parse::<i64>()
    })
);

named!(
    bencode_string<BencodeValue>,
    do_parse!(len: integer >> char!(':') >> s: take!(len) >> (BencodeValue::String(s.into())))
);

named!(
    bencode_string_s<String>,
    do_parse!(len: integer >> char!(':') >> s: map_res!(take!(len), from_utf8) >> (s.into()))
);

named!(
    bencode_integer<BencodeValue>,
    delimited!(char!('i'), map!(integer, BencodeValue::Integer), char!('e'))
);

named!(
    bencode_list<BencodeValue>,
    delimited!(
        char!('l'),
        map!(many0!(parser_bencode), |x: Vec<BencodeBlob>| {
            BencodeValue::List(x)
        }),
        char!('e')
    )
);

named!(
    bencode_dictionary<BencodeValue>,
    delimited!(
        char!('d'),
        map!(
            many0!(tuple!(bencode_string_s, parser_bencode)),
            BencodeValue::Dictionary
        ),
        char!('e')
    )
);

named!(
    parser_bencode<BencodeBlob>,
    recognize_map!(
        alt!(bencode_string | bencode_integer | bencode_list | bencode_dictionary),
        |i: &[u8], r| BencodeBlob {
            source: i.to_vec(),
            value: r
        }
    )
);

pub fn parse_bencode(bytes: &[u8]) -> Result<BencodeBlob, RsbtBencodeError> {
    parser_bencode(bytes)
        .map(|x| x.1)
        .map_err(RsbtBencodeError::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::BencodeBlob;

    fn blob(source: &[u8], value: BencodeValue) -> BencodeBlob {
        BencodeBlob {
            source: source.to_vec(),
            value,
        }
    }

    #[test]
    fn check_bencode_string() {
        assert_eq!(
            bencode_string(b"5:UTF-8"),
            Ok((&vec![][..], BencodeValue::String(b"UTF-8".to_vec())))
        );
    }

    #[test]
    fn check_bencode_integer() {
        assert_eq!(
            bencode_integer(b"i3e"),
            Ok((&vec![][..], BencodeValue::Integer(3)))
        );
        assert_eq!(
            bencode_integer(b"i-3e"),
            Ok((&vec![][..], BencodeValue::Integer(-3)))
        );
    }

    #[test]
    fn check_bencode_list() {
        assert_eq!(
            bencode_list(b"l5:UTF-8i3ee"),
            Ok((
                &vec![][..],
                BencodeValue::List(vec![
                    blob(b"5:UTF-8", BencodeValue::String(b"UTF-8".to_vec())),
                    blob(b"i3e", BencodeValue::Integer(3))
                ])
            ))
        );
    }

    #[test]
    fn check_bencode_dictionary() {
        assert_eq!(
            bencode_dictionary(b"d3:cow3:moo4:spam4:eggse"),
            Ok((
                &vec![][..],
                BencodeValue::Dictionary(
                    vec![
                        (
                            "cow".into(),
                            blob(b"3:moo", BencodeValue::String(b"moo".to_vec()))
                        ),
                        (
                            "spam".into(),
                            blob(b"4:eggs", BencodeValue::String(b"eggs".to_vec()))
                        )
                    ]
                    .into_iter()
                    .collect()
                )
            ))
        );

        assert_eq!(
            bencode_dictionary(b"d4:spaml1:a1:bee"),
            Ok((
                &vec![][..],
                BencodeValue::Dictionary(
                    vec![(
                        "spam".into(),
                        blob(
                            b"l1:a1:be",
                            BencodeValue::List(vec![
                                blob(b"1:a", BencodeValue::String(b"a".to_vec())),
                                blob(b"1:b", BencodeValue::String(b"b".to_vec()))
                            ])
                        )
                    ),]
                    .into_iter()
                    .collect()
                )
            ))
        );

        assert_eq!(
            bencode_dictionary(b"de"),
            Ok((&vec![][..], BencodeValue::Dictionary(vec![])))
        );
    }
}
