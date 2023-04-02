use super::types::Index;
use crate::ast::{Export, Func, FuncType, Type};

// 仕様
// https://webassembly.github.io/spec/core/valid/conventions.html?highlight=context#contexts

// パースしたローカル変数、型、関数、エクスポートを保持する
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Context {
    pub locals: Vec<Option<String>>,
    pub types: Field<Type>,
    pub funcs: Field<Func>,
    pub exports: Field<Export>,
}

// 識別子と型と関数のペアを保持する
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Field<T> {
    pub ids: Vec<Option<String>>,
    pub list: Vec<T>,
}

impl<T> Field<T> {
    fn new() -> Self {
        Self {
            ids: Vec::new(),
            list: Vec::new(),
        }
    }

    // フィールドに型と識別子を追加する
    fn add(&mut self, id: Option<String>, item: T) {
        self.add_id(id);
        self.add_item(item);
    }

    // フィールドに型を追加する
    fn add_item(&mut self, item: T) {
        self.list.push(item);
    }

    // フィールドに識別子を追加する
    fn add_id(&mut self, id: Option<String>) {
        self.ids.push(id);
    }
}

impl Context {
    pub fn new() -> Self {
        Self {
            locals: Vec::new(),
            types: Field::new(),
            funcs: Field::new(),
            exports: Field::new(),
        }
    }

    // 関数のリストから関数のインデックスを取得する
    // まだ関数が存在しない場合は、panicする
    pub fn get_func_idx(&self, idx: &Index) -> usize {
        match idx {
            Index::Idx(idx) => *idx,
            Index::Id(id) => self
                .funcs
                .ids
                .iter()
                .position(|x| x == &Some(id.to_owned()))
                .expect("Func id has to exists"),
        }
    }

    // ローカル変数をリストに追加する
    // すでに同じローカル変数が存在する場合は、panicする
    pub fn insert_local_id(&mut self, id: &Option<String>) {
        if id.is_some() && self.locals.contains(id) {
            panic!("Local id has to be unique");
        } else {
            self.locals.push(id.clone());
        }
    }

    // ローカル変数のリストからローカル変数のインデックスを取得する
    pub fn get_local_idx(&self, index: &Index) -> usize {
        match index {
            Index::Idx(idx) => *idx,
            Index::Id(id) => self
                .locals
                .iter()
                .position(|x| x == &Some(id.clone()))
                .expect("Identifier not found"),
        }
    }
    // 関数のリストに関数の識別子を追加する
    // 追加した関数のインデックスを返す
    pub fn insert_func_id(&mut self, id: Option<String>) -> usize {
        self.funcs.add_id(id);
        self.funcs.ids.len() - 1
    }

    // 型のリストに関数型を追加する
    pub fn insert_id_func_type(&mut self, id: Option<String>, t: &FuncType) {
        self.types.add(id, t.clone());
    }

    // 型のリストから関数型のインデックスを取得する
    pub fn get_idx_from_func_type(&self, ft: &FuncType) -> Option<usize> {
        self.types.list.iter().position(|t| t == ft)
    }

    // 型のリストに関数型を追加する
    // すでに同じ関数型が存在する場合は、panicする
    pub fn insert_func_type_get_idx(&mut self, ft: &FuncType) -> usize {
        self.insert_id_func_type(None, ft);
        self.get_idx_from_func_type(ft).expect("Type has to exist")
    }

    // 型のリストに関数型を追加する
    // すでに同じ関数型が存在する場合は、その関数型のインデックスを返す
    pub fn upsert_func_type(&mut self, ft: &FuncType) -> usize {
        match self.get_idx_from_func_type(ft) {
            None => self.insert_func_type_get_idx(ft),
            Some(i) => i,
        }
    }

    // 関数型のリストに関数を追加する
    pub fn insert_func(&mut self, func: &Func) {
        self.funcs.add_item(func.clone());
    }

    // エクスポートのリストにエクスポートを追加する
    pub fn insert_export(&mut self, id: &Option<String>, export: &Export) {
        self.exports.add((*id).clone(), export.clone());
    }
}
