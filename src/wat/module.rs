use std::{cell::RefCell, rc::Rc};

use nom::{
    branch::alt,
    combinator::map,
    multi::many0,
    sequence::{preceded, tuple},
    IResult,
};

use crate::{
    ast::{Export, ExportDesc, Func, Module},
    wat::{instruction, token, types, values},
};

use super::{
    context::Context,
    token::{bws, ws},
};

// 関数をパースする
fn func<'a>(input: &'a str, ctx: &mut Rc<RefCell<Context>>) -> IResult<&'a str, Func> {
    fn inner<'a>(input: &'a str, ctx: &mut Rc<RefCell<Context>>) -> IResult<&'a str, Func> {
        // 関数の識別子をパースする
        let (input, id) = preceded(token::func, values::id)(input)?;

        // Context型に関数の識別子を登録する
        ctx.borrow_mut().insert_func_id(Some(id.to_string()));

        // 引数と戻り値の型をパースする
        let (input, func_type) = types::type_use(input, ctx)?;

        // 関数内に命令をパースする
        let (input, body) = instruction::instructions(input, ctx)?;

        // 関数の構造体を作成する
        let f = Func {
            f_type: func_type as u32,
            locals: vec![],
            body,
        };

        Ok((input, f))
    }

    // Contextを適応する
    let in_pt = |i| inner(i, ctx);

    // 括弧をパースする
    let (input, func) = token::pt(in_pt)(input)?;
    ctx.borrow_mut().insert_func(&func);

    Ok((input, func))
}

// エクスポートをパースする
fn export<'a>(input: &'a str, ctx: &mut Rc<RefCell<Context>>) -> IResult<&'a str, Export> {
    // 関数の識別子をパースする
    let index = token::pt(preceded(token::func, types::index));

    // エクスポートをパースする
    let mut exp = token::pt(preceded(token::export, tuple((values::literal, index))));

    let (input, (name, idx)) = exp(input)?;

    let export = Export {
        name: name.clone(),
        desc: ExportDesc::Func(ctx.borrow().get_func_idx(&idx) as u32),
    };

    //
    ctx.borrow_mut().insert_export(&Some(name), &export);

    Ok((input, export))
}

// モジュールをパースする
pub fn module(input: &str) -> IResult<&str, Module> {
    // Contextを作成する
    let ctx = Rc::new(RefCell::new(Context::new()));

    // 関数にContextの適応とパースの作成
    let func_ctx = |i| func(i, &mut ctx.clone());

    // エクスポートにContextの適応とパースの作成
    let export_ctx = |i| export(i, &mut ctx.clone());

    // 関数とエクスポートをパースする
    let module_field = bws(many0(bws(alt((
        map(func_ctx, |_| ()),
        map(export_ctx, |_| ()),
    )))));

    // モジュールをパースする
    preceded(ws, token::pt(preceded(token::module, module_field)))(input)?;

    // パースした結果をContextから取り出し、Module構造体を作成する
    let module = Module {
        types: ctx.borrow().types.list.clone(),
        funcs: ctx.borrow().funcs.list.clone(),
        exports: ctx.borrow().exports.list.clone(),
    };

    Ok(("", module))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        ast::{Instruction, ValueType::I32},
        wat::context::Field,
    };

    #[test]
    fn func_parse() {
        let mut ctx = Rc::new(RefCell::new(Context::new()));
        let wat = "(func $add (param $lhs i32) (param $rhs i32) (result i32)
          local.get $lhs
          local.get $rhs
          i32.add)";
        let expected = Func {
            f_type: 0,
            locals: vec![],
            body: vec![
                Instruction::LocalGet(0),
                Instruction::LocalGet(1),
                Instruction::I32Add,
            ],
        };
        assert_eq!(func(&wat, &mut ctx), Ok(("", expected.clone())));
        assert_eq!(
            ctx,
            Rc::new(RefCell::new(Context {
                locals: vec![Some("$lhs".to_string()), Some("$rhs".to_string())],
                types: Field {
                    ids: vec![None],
                    list: vec![(vec![I32, I32], vec![I32])],
                },
                funcs: Field {
                    ids: vec![Some("$add".to_string())],
                    list: vec![expected]
                },
                exports: Field::new()
            }))
        )
    }
}
