use std::fmt;
use std::error;

#[derive(Debug, Clone)]
pub enum GenErrorTypes {
    InvalidMethodArgs
}

#[derive(Debug, Clone)]
pub struct GenError {
    line: usize,
    column: usize,
    error_type: GenErrorTypes,
}

impl fmt::Display for GenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::from("Wrong `var` declaration.\n[Syntax Error]");
        match self.error_type {
            GenErrorTypes::InvalidMethodArgs =>
                res.push_str("(Invalid Method) Invalid method statment or arguments types are not matched at`")
        }
        write!(f, "{}\nLine: {}, Column: {}", res, self.line, self.column)
    }
}

impl error::Error for GenError {}

impl GenError {
    pub fn new(error_type: GenErrorTypes, line: usize, column: usize) -> Self {
        Self {
            line, column, error_type
        }
    }
}
