
use std::fmt;
use std::error;

#[derive(Debug, Clone)]
pub enum ExpretionErrorTypes {
    InvalidExpretion
}

#[derive(Debug, Clone)]
pub struct ExpretionError {
    line: usize,
    column: usize,
    error_type: ExpretionErrorTypes,
}

impl fmt::Display for ExpretionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::from("Wrong `var` declaration.\n[Syntax Error]");
        match self.error_type {
            ExpretionErrorTypes::InvalidExpretion =>
                res.push_str("(Invalid expretion) Invalid expretion at`")
        }
        write!(f, "{}\nLine: {}, Column: {}", res, self.line, self.column)
    }
}

impl error::Error for ExpretionError {}

impl ExpretionError {
    pub fn new(error_type: ExpretionErrorTypes, line: usize, column: usize) -> Self {
        Self {
            line, column, error_type
        }
    }
}
