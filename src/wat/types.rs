use std::{cell::RefCell, rc::Rc};

use nom::{
    branch::alt,
    bytes::streaming::tag,
    combinator::{map, opt, value},
    multi::many0,
    sequence::{preceded, tuple},
    IResult,
};

use crate::ast::{FuncType, ValueType};
use crate::wat::{token, types};

use super::{context::Context, token::bws, values};

// 符号無し整数値か"$add"のような識別子
pub enum Index {
    Idx(usize),
    Id(String),
}

// 文字列をIndex型に変換する
pub fn index(input: &str) -> IResult<&str, Index> {
    let idx = map(values::u32, |u| Index::Idx(u as usize));
    let id = map(values::id, |id| Index::Id(id.to_string()));
    alt((idx, id))(input)
}

pub fn value_type(input: &str) -> IResult<&str, ValueType> {
    let types = alt((
        value(ValueType::I32, tag("i32")),
        value(ValueType::I64, tag("i64")),
        value(ValueType::F32, tag("f32")),
        value(ValueType::F64, tag("f64")),
    ));
    bws(types)(input)
}

pub fn func_type<'a>(input: &'a str, ctx: &mut Rc<RefCell<Context>>) -> IResult<&'a str, FuncType> {
    #[derive(Clone)]
    enum PR {
        R(ValueType),                 // Return type
        P(ValueType, Option<String>), // Parameter type
    }

    // Parse a parameter with an optional identifier
    let param = map(
        preceded(
            token::ws,
            token::pt(tuple((token::param, opt(values::id), types::value_type))),
        ),
        |p| PR::P(p.2, p.1.map(|id| id.to_string())),
    );

    // Parse a result type
    let result = map(
        preceded(
            token::ws,
            token::pt(preceded(token::result, types::value_type)),
        ),
        PR::R,
    );

    // Parse a type "t" that is either a parameter "param" or a result type "result".
    let t = alt((param, result));
    let (input, many_t) = many0(t)(input)?;

    // Get all result types from the list
    // of parsed types.
    let results = many_t
        .iter()
        .filter_map(|t| match t {
            PR::R(r) => Some(*r),
            _ => None,
        })
        .collect::<Vec<ValueType>>();

    let params = many_t
        .iter()
        .filter_map(|t| match t {
            PR::P(p, id) => {
                ctx.borrow_mut().locals.push(id.clone());
                Some(*p)
            }
            _ => None,
        })
        .collect::<Vec<ValueType>>();

    let ft = (params, results);
    Ok((input, ft))
}

pub fn type_use<'a>(input: &'a str, ctx: &mut Rc<RefCell<Context>>) -> IResult<&'a str, usize> {
    let mut ft = |i| func_type(i, ctx);
    let (input, ft) = ft(input)?;

    let index = ctx.borrow_mut().upsert_func_type(&ft);

    Ok((input, index))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn func_type_parse_1() {
        let mut ctx = Rc::new(RefCell::new(Context::new()));
        assert_eq!(
            func_type("(param $lhs i32)", &mut ctx),
            Ok(("", (vec![ValueType::I32], vec![])))
        );
        assert_eq!(
            Context {
                locals: vec![Some("$lhs".to_string())],
                ..Context::new()
            },
            *ctx.borrow_mut()
        );
    }

    #[test]
    fn func_type_parse_2() {
        let mut ctx = Rc::new(RefCell::new(Context::new()));
        assert_eq!(
            func_type("(param $lhs i32) (param $rhs i32) ", &mut ctx),
            Ok((" ", (vec![ValueType::I32, ValueType::I32], vec![])))
        );
        assert_eq!(
            Context {
                locals: vec![Some("$lhs".to_string()), Some("$rhs".to_string())],
                ..Context::new()
            },
            *ctx.borrow_mut()
        );
    }

    #[test]
    fn func_type_parse_3() {
        let mut ctx = Rc::new(RefCell::new(Context::new()));
        assert_eq!(
            func_type("(xparam $lhs u32)", &mut ctx),
            Ok(("(xparam $lhs u32)", (vec![], vec![])))
        );
    }

    #[test]
    fn func_type_parse_4() {
        let mut ctx = Rc::new(RefCell::new(Context::new()));
        assert_eq!(
            func_type("param $lhs u32", &mut ctx),
            Ok(("param $lhs u32", (vec![], vec![])))
        );
    }

    #[test]
    fn func_type_parse_5() {
        let mut ctx = Rc::new(RefCell::new(Context::new()));
        assert_eq!(
            func_type("(param xlhs u32)", &mut ctx),
            Ok(("(param xlhs u32)", (vec![], vec![])))
        );
    }

    #[test]
    fn func_type_parse_6() {
        let mut ctx = Rc::new(RefCell::new(Context::new()));
        assert_eq!(
            func_type("(param $lhs i32)", &mut ctx),
            Ok(("", (vec![ValueType::I32], vec![])))
        );
    }

    #[test]
    fn func_type_parse_7() {
        let mut ctx = Rc::new(RefCell::new(Context::new()));
        assert_eq!(
            func_type("(param $lhs i32) (param $rhs i32) (result i64)", &mut ctx),
            Ok((
                "",
                (vec![ValueType::I32, ValueType::I32], vec![ValueType::I64])
            ))
        );
    }

    #[test]
    fn func_type_parse_8() {
        let mut ctx = Rc::new(RefCell::new(Context::new()));
        assert_eq!(
            func_type("(param i32) (param i32) (result i64)", &mut ctx),
            Ok((
                "",
                (vec![ValueType::I32, ValueType::I32], vec![ValueType::I64])
            ))
        );
    }

    #[test]
    fn value_type_parse() {
        assert_eq!(value_type("i32"), Ok(("", ValueType::I32)));
        assert_eq!(value_type("i64"), Ok(("", ValueType::I64)));
        assert_eq!(value_type("f32"), Ok(("", ValueType::F32)));
        assert_eq!(value_type("f64"), Ok(("", ValueType::F64)));
        assert!(value_type("x32").is_err());
    }
}
