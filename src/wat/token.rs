use std::ops::RangeFrom;

use nom::{
    bytes::streaming::tag,
    character::complete::{char, multispace0},
    error::ParseError,
    sequence::delimited,
    AsChar, IResult, InputIter, InputTakeAtPosition, Parser, Slice,
};

// https://webassembly.github.io/spec/core/text/lexical.html#tokens

// 前後の"("と")"を削除し、innerで指定したパーサーの結果を返す
pub fn pt<I, O, E: ParseError<I>, G>(inner: G) -> impl FnMut(I) -> IResult<I, O, E>
where
    G: Parser<I, O, E>,
    I: Slice<RangeFrom<usize>> + InputIter,
    <I as InputIter>::Item: AsChar,
{
    delimited(char('('), inner, char(')'))
}

// 先頭と末尾の空白文字列を削除し、innerで指定したパーサーでその間の文字列を返す
pub fn bws<I, O, E: ParseError<I>, G>(inner: G) -> impl FnMut(I) -> IResult<I, O, E>
where
    G: Parser<I, O, E>,
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    delimited(multispace0, inner, multispace0)
}

// 先頭から0個以上の空白文字列を削除し、以降の文字列を返す
pub fn ws(input: &str) -> IResult<&str, &str> {
    multispace0(input)
}

// "func"をパースする
// "func"の前後の0個以上の空白文字列を削除し、以降の文字列を返す
pub fn func(input: &str) -> IResult<&str, &str> {
    bws(tag("func"))(input)
}

// "param"をパースする
// "param"の前後の0個以上の空白文字列を削除し、以降の文字列を返す
pub fn param(input: &str) -> IResult<&str, &str> {
    bws(tag("param"))(input)
}

// "result"をパースする
// "result"の前後の0個以上の空白文字列を削除し、以降の文字列を返す
pub fn result(input: &str) -> IResult<&str, &str> {
    bws(tag("result"))(input)
}

// "export"をパースする
// "export"の前後の0個以上の空白文字列を削除し、以降の文字列を返す
pub fn export(input: &str) -> IResult<&str, &str> {
    bws(tag("export"))(input)
}

// "module"をパースする
// "module"の前後の0個以上の空白文字列を削除し、以降の文字列を返す
pub fn module(input: &str) -> IResult<&str, &str> {
    bws(tag("module"))(input)
}

#[cfg(test)]
mod tests {
    use nom::sequence::preceded;

    use crate::{ast::ValueType, wat::types::value_type};

    use super::*;

    #[test]
    fn pt_parse() {
        assert_eq!(pt(module)("(module)"), Ok(("", "module")));
        assert_eq!(pt(module)("( module )"), Ok(("", "module")));
        assert_eq!(
            pt(preceded(result, value_type))("(result i32)"),
            Ok(("", ValueType::I32))
        );
    }

    #[test]
    fn func_parse() {
        assert_eq!(func("func"), Ok(("", "func")));
        assert_eq!(func("func foobar"), Ok(("foobar", "func")));
        assert_eq!(
            func("func $add (param $lhs i32) (param $rhs i32) (result i32)"),
            Ok((
                "$add (param $lhs i32) (param $rhs i32) (result i32)",
                "func"
            ))
        );
        assert!(func("notfunc").is_err());
    }

    #[test]
    fn ws_parse() {
        assert_eq!(ws("  foo"), Ok(("foo", "  ")));
        assert_eq!(ws(" \nfoo"), Ok(("foo", " \n")));
        assert_eq!(ws("foo"), Ok(("foo", "")));
    }
    #[test]
    fn param_parse() {
        assert_eq!(param("param"), Ok(("", "param")));
        assert_eq!(param("param $lhs i32"), Ok(("$lhs i32", "param")));
    }
    #[test]
    fn result_parse() {
        assert_eq!(result("result"), Ok(("", "result")));
        assert_eq!(result("result i32"), Ok(("i32", "result")));
    }
    #[test]
    fn export_parse() {
        assert_eq!(export(" export "), Ok(("", "export")));
        assert!(export("noexport").is_err());
    }
    #[test]
    fn module_parse() {
        assert_eq!(module(" module "), Ok(("", "module")));
        assert!(module("nomodule").is_err());
    }

    #[test]
    fn bws_parse() {
        assert_eq!(bws(param)(" param "), Ok(("", "param")));
        assert_eq!(bws(param)(" param123"), Ok(("123", "param")));
        assert_eq!(bws(param)("param"), Ok(("", "param")));
        assert_eq!(bws(param)("param      $lhs i32"), Ok(("$lhs i32", "param")));
        assert!(bws(param)("p a r a m").is_err());
    }
}
