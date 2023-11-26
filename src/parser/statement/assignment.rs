use crate::parser::node_types::NodeStatment;
use crate::parser::node_types::NodeExpr;
use crate::traits::{
    TokenType,
    Token
};
use crate::errors::{
    Result,
    HasteErrors
};
use crate::parser::expretion;

use std::collections::HashMap;


pub fn parse<'a, I>(tokens: &mut std::iter::Peekable<I>, idents: &mut HashMap<String, i32>)
    -> Result<NodeStatment>
    where  I: Iterator<Item = &'a mut TokenType>
{
    let mut ident = expretion::parse(tokens)?;
    tokens.next();
    let mut value: NodeExpr = NodeExpr::Null;
    if let Some(i) = tokens.peek() {
        match i.token {
            Token::Eq => {
                tokens.next();
            },
            _ => {
                return Err(HasteErrors::Str("wrong assignment"));
            },
        }
    } else {
        return Err(HasteErrors::Str("Ended the file"));
    }
    if let Some(ex) = tokens.peek() {
        value = expretion::parse(tokens)?;
        tokens.next();
    } else {
        return Err(HasteErrors::Str("Ended the file"));
    }
    if let Some(a) = tokens.peek() {
        match a.token {
            Token::Semi => {
                tokens.next();
            },
            _ => return Err(HasteErrors::Str("Semi"))
        }
    } else {
        return Err(HasteErrors::Str("Ended the file"));
    }
    return Ok(NodeStatment::Assignment(ident, value));
}
