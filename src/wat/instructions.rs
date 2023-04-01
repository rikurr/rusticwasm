// use std::{borrow::Borrow, cell::RefCell, rc::Rc};

// use nom::{bytes::streaming::tag, sequence::preceded, IResult};

// use crate::ast::Instruction;

// use super::{context::Context, token::bws, types::index};

// // local.getとその先に続く文字列をパースする
// fn local_get<'a>(input: &'a str, ctx: &Rc<RefCell<Context>>) -> IResult<&'a str, Instruction> {
//     let local_get = bws(tag("local.get"));
//     let (input, index) = preceded(local_get, index)(input)?;
//     let i = ctx.borrow().get_local_idx(&index);
//     Ok((input, Instr::LocalGet(i)))
// }
