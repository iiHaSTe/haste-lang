use std::fmt;
use std::error;

#[derive(Debug, Clone)]
pub enum VarErrorTypes {
    NoIdent,
    NoSemi,
    NoValue,
    EndOfTheFile,
    InvalidToken
}

#[derive(Debug, Clone)]
pub struct VarStatmentError {
    line: usize,
    column: usize,
    error_type: VarErrorTypes,
}

impl fmt::Display for VarStatmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::from("Wrong `var` declaration.\n[Syntax Error]");
        match self.error_type {
            VarErrorTypes::NoIdent =>
                res.push_str("(No identefier) Use an identefier: `var ident;`"),
            VarErrorTypes::NoSemi =>
                res.push_str("(No Semicolon) End the expretion with Semicolon `;`"),
            VarErrorTypes::NoValue =>
                res.push_str("(No asignment) Value after `=` are important `var ident = \"I love Turtles\";`"),
            VarErrorTypes::EndOfTheFile =>
                res.push_str("(No complition) You ended the file before you finnish the `var` statment"),
            VarErrorTypes::InvalidToken =>
                res.push_str("(Invalid token) Token here is invalid while making a variable `var ident = \"I love Turtles\";`")
        }
        write!(f, "{}\nLine: {}, Column: {}", res, self.line, self.column)
    }
}

impl error::Error for VarStatmentError {}

impl VarStatmentError {
    pub fn new(error_type: VarErrorTypes, line: usize, column: usize) -> Self {
        Self {
            line, column, error_type
        }
    }
}
