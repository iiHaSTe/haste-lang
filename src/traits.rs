pub trait Process<T> {
    fn peek(&mut self, offset: Option<usize>) -> Option<T>;
    fn consume(&mut self) -> T;
}

#[derive(Debug, Clone)]
pub enum Token {
    Vide,
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

#[derive(Debug, Clone)]
pub struct TokenType {
    pub token: Token,
    pub line: usize,
    pub column: usize
}
