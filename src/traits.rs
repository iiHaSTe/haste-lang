pub trait Process<T> {
    fn peek(&mut self, offset: Option<usize>) -> Option<T>;
    fn consume(&mut self) -> T;
}

#[derive(Clone)]
#[derive(Debug)]
pub enum Token {
    // built in functions
    Exit,
    Print,
    // keywords
    Var,
    Ident(String),
    // built in data types
    IntLit(String),
    StringLit(String, bool),
    Boolean(bool),
    FloatLit(String),
    // genirics data types
    // tokens
    Eq,
    Semi,
    LeftPrac,
    RightPrac
}
