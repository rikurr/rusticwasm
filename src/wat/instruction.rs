use std::{cell::RefCell, rc::Rc};

use nom::{
    branch::alt, bytes::streaming::tag, combinator::map, multi::many1, sequence::preceded, IResult,
};

use crate::ast::Instruction;

use super::{context::Context, token::bws, types::index};

// local.getとその先に続く文字列からIndexを取得する
fn local_get<'a>(input: &'a str, ctx: &Rc<RefCell<Context>>) -> IResult<&'a str, Instruction> {
    let local_get = bws(tag("local.get"));
    let (input, i) = preceded(local_get, index)(input)?;

    // IndexからContextのIndexを取得する
    let i = ctx.borrow().get_local_idx(&i);

    Ok((input, Instruction::LocalGet(i)))
}

// 文字列からInstruction型へ変換する
fn i32_add(input: &str) -> IResult<&str, Instruction> {
    map(bws(tag("i32.add")), |_| Instruction::I32Add)(input)
}

// 文字列からInstructionのVec型へ変換する
pub fn instructions<'a>(
    input: &'a str,
    ctx: &mut Rc<RefCell<Context>>,
) -> IResult<&'a str, Vec<Instruction>> {
    let lg = |i| local_get(i, ctx);
    let instruction = alt((lg, i32_add));
    many1(bws(instruction))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Instruction;

    #[test]
    fn local_get_parse() {
        let ctx = Rc::new(RefCell::new(Context {
            locals: vec![Some("$lhs".to_string())],
            ..Context::new()
        }));
        assert_eq!(
            local_get("local.get 1", &ctx),
            Ok(("", Instruction::LocalGet(1)))
        );
        assert_eq!(
            local_get("local.get $lhs", &ctx),
            Ok(("", Instruction::LocalGet(0)))
        );
    }

    #[test]
    fn i32_add_parse() {
        assert_eq!(i32_add("i32.add"), Ok(("", Instruction::I32Add)));
        assert!(i32_add("local.get").is_err());
    }

    #[test]
    fn instructions_parse() {
        let mut ctx = Rc::new(RefCell::new(Context::new()));

        assert_eq!(
            instructions("local.get 1", &mut ctx),
            Ok((
                "",
                vec![
                    Instruction::LocalGet(0),
                    Instruction::I32Add,
                    Instruction::LocalGet(1)
                ]
            ))
        );
    }
}
