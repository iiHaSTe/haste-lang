use crate::traits::Token;
use crate::traits::TokenType;
use crate::errors::{
    Result as Result__,
    HasteErrors,
    generals::{
        GenError,
        GenErrorTypes
    },
    expretions::{
        ExpretionError,
        ExpretionErrorTypes
    },
    variables::{
        VarStatmentError,
        VarErrorTypes
    }
};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub enum NodeExpr {
    Ident(String),
    IntLit(String),
    StringLit(String, bool),
    FloatLit(String),
    Boolean(bool),
    Null,
}

#[derive(Debug)]
pub enum NodeStatment {
    Exit(NodeExpr),
    Print(NodeExpr),
    Var(NodeExpr, NodeExpr)
}

#[derive(Debug)]
pub struct NodeTree {
    pub body: Vec<NodeStatment>
}

#[derive(Debug)]
pub struct TreeParser<'a> {
    content: &'a Vec<TokenType>,
}

impl<'a> TreeParser<'a> {
    pub fn new(content: &'a Vec<TokenType>) -> Self {
        return TreeParser { 
            content: content
        };
    }
    pub fn parse(&mut self) -> NodeTree {
        let mut tokens = self.content.into_iter().peekable();
        return self.parse_statment(&mut tokens);
    }
    fn parse_statment<I>(&mut self, tokens: &mut std::iter::Peekable<I>)
        -> NodeTree
        where I: Iterator<Item = &'a TokenType>
    {
        let mut tree = NodeTree {
            body: vec![]
        };
        while let Some(t) = tokens.peek() {
            let mut token = &t.token;
            match token {
                Token::Exit => {
                    tokens.next();
                    tree.body.push(match self.parseExit(tokens) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    });
                },
                Token::Print => {
                    tokens.next();
                    tree.body.push(match self.parsePrint(tokens) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    });
                },
                Token::Var => {
                    tokens.next();
                    tree.body.push(match self.parseVar(tokens) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    });
                },
                Token::Semi => {
                    tokens.next();
                },
                _ => {},
            }
        }
        return tree;
    }
    fn parse_expr<I>(&mut self, tokens: &mut std::iter::Peekable<I>)
        -> Result__<NodeExpr>
        where I: Iterator<Item = &'a TokenType>
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
    fn parseVar<I>(&mut self, tokens: &mut std::iter::Peekable<I>)
        -> Result__<NodeStatment>
        where I: Iterator<Item = &'a TokenType>
    {
        let mut statment: NodeStatment;
        let mut ident: NodeExpr;
        let mut aa: NodeExpr = NodeExpr::Null;
        if let Some(t) = tokens.peek() {
            let mut token = &t.token;
            match token {
                Token::Ident(_) => {
                    ident = self.parse_expr(tokens)?;
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
                    aa = self.parse_expr(tokens)?;
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
    fn parsePrint<I>(&mut self, tokens: &mut std::iter::Peekable<I>)
        -> Result__<NodeStatment>
        where I: Iterator<Item = &'a TokenType>
    {
        return if let Some(t) = tokens.peek() {
            let mut token = &t.token;
            let mut res: Result__<NodeStatment>;
            match token {
                Token::IntLit(_)
                    | Token::StringLit(_, _)
                    | Token::FloatLit(_)
                    | Token::Boolean(_) 
                    | Token::Ident(_) => {
                    res = Ok(NodeStatment::Print(self.parse_expr(tokens)?));
                },
                _ => {
                    res = Err(HasteErrors::Str(r#"[Syntax Error] "print" method only accept 'int', 'float', 'boolean' and 'string'"#));
                }
            }
            tokens.next();
            res
        } else {
            Err(HasteErrors::Str(r#"[Syntax Error] you just end every think and forgot to pass an argument to "print" method!!"#))
        };
    }
    fn parseExit<I>(&mut self, tokens: &mut std::iter::Peekable<I>)
        -> Result__<NodeStatment>
        where I: Iterator<Item = &'a TokenType>
    {
        return if let Some(t) = tokens.peek() {
            let mut token = &t.token;
            let mut res: Result__<NodeStatment>;
            match token {
                Token::IntLit(_) | Token::Ident(_) => {
                    res = Ok(NodeStatment::Exit(self.parse_expr(tokens)?));
                },
                _ => {
                    res = Err(HasteErrors::Str(r#"[Syntax Error] It has to be a byte inside "Exit" method"#));
                }
            }
            tokens.next();
            res
        } else {
            Err(HasteErrors::Str(r#"[Syntax Error] you just end every think and forgot to pass an argument to "exit" method!!"#))
        }
    }
}
