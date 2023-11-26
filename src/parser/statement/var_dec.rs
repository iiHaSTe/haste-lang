use crate::traits::{
    Token,
    TokenType
};
use crate::errors::{
    Result,
    HasteErrors,
    variables::{
        VarStatmentError,
        VarErrorTypes
    }
};
use crate::parser::node_types::{
    NodeStatment,
    NodeExpr
};
use crate::parser::expretion::parse as parse_expr;

use std::collections::HashMap;

pub fn parse<'a, I>(tokens: &mut std::iter::Peekable<I>, idents: &mut HashMap<String, i32>)
    -> Result<NodeStatment>
    where  I: Iterator<Item = &'a mut TokenType>,
{
    let mut statment: NodeStatment;
    let mut ident: NodeExpr;
    let mut aa: NodeExpr = NodeExpr::Null;
    if let Some(t) = tokens.peek() {
        let mut token = &t.token;
        match token {
            Token::Ident(_) => {
                ident = parse_expr(tokens)?;
                tokens.next();
            },
            _ => return Err(HasteErrors::Vars(
                VarStatmentError::new(
                    VarErrorTypes::NoIdent,
                    t.line, t.column,
                )
            )),
        }
    } else {
        return Err(HasteErrors::Vars(
            VarStatmentError::new(
                VarErrorTypes::EndOfTheFile,
                0, 0,
            )
        ));
    }
    if let Some(token_type) = tokens.peek() {
        match token_type.token {
            Token::Eq => {
                tokens.next();
                aa = parse_expr(tokens)?;
                tokens.next();
            },
            Token::Semi => {
                tokens.next();
            },
            _ => {
                return Err(HasteErrors::Vars(
                    VarStatmentError::new(
                        VarErrorTypes::InvalidToken,
                        token_type.line, token_type.column,
                    )
                ));
            }
        }
    } else {
        return Err(HasteErrors::Vars(
            VarStatmentError::new(
                VarErrorTypes::EndOfTheFile,
                0, 0,
            )
        ));
    }
    statment = NodeStatment::Var(ident, aa);
    return Ok(statment);
}

