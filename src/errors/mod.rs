pub mod variables;
pub mod expretions;
pub mod generals;

use std::fmt;


#[derive(Debug, Clone)]
pub enum HasteErrors {
    Vars(variables::VarStatmentError),
    Expr(expretions::ExpretionError),
    Gen(generals::GenError),
    Str(&'static str)
}

impl fmt::Display for HasteErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            HasteErrors::Vars(e) => write!(f, "{}", e),
            HasteErrors::Gen(e) => write!(f, "{}", e),
            HasteErrors::Expr(e) => write!(f, "{}", e),
            HasteErrors::Str(e) => write!(f, "{}", e),
        };
    }
}

pub type Result<T> = std::result::Result<T, HasteErrors>;
