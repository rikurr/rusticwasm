use crate::wat::token::bws;
use nom::{
    branch::alt,
    bytes::{
        complete::{is_a, is_not},
        streaming::tag,
    },
    character::complete::{alphanumeric1, char, digit1},
    combinator::{map, recognize},
    multi::many1,
    sequence::{delimited, pair},
    IResult,
};

// 仕様
// https://webassembly.github.io/spec/core/text/values.html

// $で始まる識別子をパースする
// パースされた識別子に対して、前後の空白文字を除去する
// https://webassembly.github.io/spec/core/text/values.html#text-id
pub fn id(input: &str) -> IResult<&str, &str> {
    // WATの仕様では、識別子で以下の特殊文字を含むことができる
    let additional_chars = "!#$%&′∗+−./:<=>?@∖^_`|~";
    let id_char = alt((alphanumeric1, is_a(additional_chars)));
    let id = recognize(pair(tag("$"), many1(id_char)));
    bws(id)(input)
}

// 文字列の数値をu32をパースする
pub fn u32(input: &str) -> IResult<&str, u32> {
    map(digit1, |d: &str| {
        d.parse().expect("Integer format not supported")
    })(input)
}

// 引用符で囲まれた文字列をパースする
// 前後の空白文字を除去する
pub fn literal(input: &str) -> IResult<&str, String> {
    map(
        bws(delimited(char('"'), is_not("\""), char('"'))),
        |s: &str| s.to_string(),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn id_parse() {
        assert_eq!(id("$valid_id%#! foo "), Ok(("foo ", "$valid_id%#!")));
        assert_eq!(id("  $valid_id%#! foo "), Ok(("foo ", "$valid_id%#!")));
        assert!(id("valid_id%#! foo ").is_err());
    }

    #[test]
    fn u32_parse() {
        assert_eq!(u32("12"), Ok(("", 12)));
        assert!(u32("hello").is_err());
    }

    #[test]
    fn literal_parse() {
        assert_eq!(
            literal("\"valid#+123\""),
            Ok(("", "valid#+123".to_string()))
        );
        assert!(literal("invalid").is_err());
    }
}
