use crate::traits::{
    TokenType,
    Token
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
    NodeProgram,
    NodeExpr
};

use std::collections::HashMap;

pub mod function_call;
pub mod var_dec;
pub mod assignment;

pub fn parse<'a, I>(tokens: &mut std::iter::Peekable<I>)
    -> Result<NodeProgram>
    where  I: Iterator<Item = &'a mut TokenType>,
{
    let mut tree = NodeProgram {
        body: vec![]
    };
    let mut varIdent: HashMap<String, i32> = HashMap::new();
    let mut funIdent: HashMap<String, i32> = HashMap::new();
    while let Some(t) = tokens.peek() {
        let mut token = &t.token;
        match token {
            Token::Exit => {
                tokens.next();
                tree.body.push(function_call::parseExit(tokens)?);     
            },
            Token::Print => {
                tokens.next();
                tree.body.push(function_call::parsePrint(tokens)?);
            },
            Token::Var => {
                tokens.next();
                tree.body.push(var_dec::parse(tokens, &mut varIdent)?);
                if let Some(v) = tree.body.last() {
                    match v {
                        NodeStatment::Var(ident, _) => {
                            match ident {
                                NodeExpr::Ident(i) => {
                                    varIdent.insert(i.clone(), 0);
                                },
                                _ => {},
                            };
                        },
                        _ => {},
                    };
                }
            },
            Token::Ident(id) => {
                if let Some(_) = varIdent.get(id) {
                    tree.body.push(assignment::parse(tokens, &mut varIdent)?);
                } else {
                    return Err(HasteErrors::Str("Oh no that ident not found"))
                }
            },
            Token::Semi => {
                tokens.next();                                              
            },
            _ => {},
        }
    }
    println!("{:#?}", varIdent);
    return Ok(tree);
}
