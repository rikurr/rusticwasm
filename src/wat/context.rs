#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Context {
    pub locals: Vec<Option<String>>,
    pub types: Field<Type>,
    pub funcs: Field<Func>,
    pub exports: Field<Export>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Field<T> {
    pub ids: Vec<String>,
    pub list: Vec<T>,
}
