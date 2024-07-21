#[derive(Debug, Clone)]
pub enum Type {
    Num,
    String,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Ident(pub String);

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Ident,
    pub args: Vec<(Ident, Type)>,
    pub body: Vec<Box<Expr>>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    BinOp(Box<Expr>, Op, Box<Expr>),
    TrailingOp(Box<Expr>, Op),
    LeadingOp(Op, Box<Expr>),
    Paren(Box<Expr>),
    Function(Function),
    Call(Ident, Vec<Box<Expr>>),
    Var(Ident),
    Let(Ident, Box<Expr>),
    Noop,
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    QuestionMark,
}
