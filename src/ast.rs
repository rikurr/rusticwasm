// WebAssembly Text Format (WAT) AST
// WebAssembly binaryにも対応できる想定
// https://webassembly.github.io/spec/core/syntax/index.html

// https://webassembly.github.io/spec/core/syntax/modules.html
#[derive(Debug, PartialEq)]
pub struct Module {
    types: Vec<Type>,
    funcs: Vec<Func>,
    exports: Vec<Export>,
}

// 現在はNumber Typesのみサポート
// https://webassembly.github.io/spec/core/syntax/types.html#value-types
#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}
pub type StackType = Vec<ValueType>;

// 最初のStackTypeは全ての引数の型、2つ目のStackTypeは全ての戻り値の型が入る
// https://webassembly.github.io/spec/core/syntax/types.html#function-types
pub type FuncType = (StackType, StackType);

// FuncTypeのエイリアス
// https://webassembly.github.io/spec/core/syntax/modules.html#types
pub type Type = FuncType;

// https://webassembly.github.io/spec/core/syntax/modules.html#functions
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Func {
    pub f_type: u32,
    pub locals: Vec<ValueType>,
    pub body: Vec<Instruction>,
}

// https://webassembly.github.io/spec/core/syntax/instructions.html
// https://webassembly.github.io/spec/core/text/instructions.html
#[derive(Debug, PartialEq, Clone, Eq)]
pub enum Instruction {
    LocalGet(String),
    LocalSet(String),
    I32Add,
}

// https://webassembly.github.io/spec/core/syntax/modules.html#exports
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Export {
    pub name: String,
    pub desc: ExportDesc,
}

// https://webassembly.github.io/spec/core/syntax/modules.html#syntax-exportdesc
#[derive(Debug, PartialEq, Clone, Eq)]
pub enum ExportDesc {
    Func(u32),
    Table(u32),
    Memory(u32),
    Global(u32),
}
