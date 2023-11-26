use crate::traits::{
    TokenType,
    Token
};
use crate::errors::{
    Result,
    HasteErrors,
    expretions::{
        ExpretionError,
        ExpretionErrorTypes
    }
};
use super::node_types::NodeStatment;
use crate::parser::node_types::NodeExpr;


pub fn parse<'a, I>(tokens: &mut std::iter::Peekable<I>)
    -> Result<NodeExpr>
    where I:  Iterator<Item = &'a mut TokenType>
{
    return match tokens.peek() {
        Some(token_type) => {
            match &token_type.token {
                Token::Ident(value) => Ok(NodeExpr::Ident(value.to_string())),
                Token::IntLit(value) => Ok(NodeExpr::IntLit(value.to_string())),
                Token::StringLit(value, r#type) => Ok(NodeExpr::StringLit(value.clone(), *r#type)),
                Token::Boolean(value) => Ok(NodeExpr::Boolean(*value)),
                Token::FloatLit(value) => Ok(NodeExpr::FloatLit(value.to_string())),
                _ => Err(HasteErrors::Expr(
                        ExpretionError::new(
                        ExpretionErrorTypes::InvalidExpretion,
                        token_type.line, token_type.column,
                    )
                ))
            }
        },
        None => Err(HasteErrors::Str("[Syntax Error] the file ends before finding a valid Token")),
    }
}
