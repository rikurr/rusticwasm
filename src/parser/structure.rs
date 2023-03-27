#[derive(Debug, PartialEq)]
pub struct Module {
    types: Vec<Type>,
    funcs: Vec<Func>,
    exports: Vec<Export>,
}

// Type
#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}

pub type StackType = Vec<ValueType>;
pub type FuncType = (StackType, StackType);
pub type Type = FuncType;

// Func
#[derive(Debug, PartialEq)]
pub struct Func {
    pub name: String,
    pub params: Vec<Param>,
    pub result: Option<ValueType>,
    pub body: Vec<Instruction>,
}

#[derive(Debug, PartialEq)]
pub struct Param {
    pub name: Option<String>,
    pub ty: ValueType,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    LocalGet(String),
    LocalSet(String),
    I32Add,
}

// Export
#[derive(Debug, PartialEq)]
pub struct Export {
    pub name: String,
    pub desc: ExportDesc,
}

#[derive(Debug, PartialEq)]
pub enum ExportDesc {
    Func(u32),
    Table(u32),
    Memory(u32),
    Global(u32),
}
