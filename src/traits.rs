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
    // built in data types
    IntLit(String),
    StringLit(String),
    Boolean(bool),
    FloatLit(String),
    // genirics data types
    // tokens
    Semi,
    LeftPrac,
    RightPrac
}
