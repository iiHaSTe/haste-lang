use crate::traits::{
    Token
};

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
    content: &'a Vec<Token>,
}

impl<'a> TreeParser<'a> {
    pub fn new(content: &'a Vec<Token>) -> Self {
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
        where I: Iterator<Item = &'a Token>
    {
        let mut tree = NodeTree {
            body: vec![]
        };
        while let Some(token) = tokens.peek() {
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
        -> Result<NodeExpr, &'static str>
        where I: Iterator<Item = &'a Token>
    {
        return match tokens.peek() {
            Some(token) => {
                match token {
                Token::Ident(value) => Ok(NodeExpr::Ident(value.to_string())),
                Token::IntLit(value) => Ok(NodeExpr::IntLit(value.to_string())),
                Token::StringLit(value, r#type) => Ok(NodeExpr::StringLit(value.clone(), *r#type)),
                Token::Boolean(value) => Ok(NodeExpr::Boolean(*value)),
                Token::FloatLit(value) => Ok(NodeExpr::FloatLit(value.to_string())),
                _ => Err("[Syntax Error] Current token is not a valid Expretion")
                }
            },
            None => Err("[Syntax Error] the file ends before finding a valid Token"),
        }
    }
    fn parseVar<I>(&mut self, tokens: &mut std::iter::Peekable<I>)
        -> Result<NodeStatment, &'static str>
        where I: Iterator<Item = &'a Token>
    {
        let mut statment: NodeStatment;
        let mut ident: NodeExpr;
        let mut aa: NodeExpr = NodeExpr::Null;
        if let Some(token) = tokens.peek() {
            match token {
                Token::Ident(_) => {
                    ident = self.parse_expr(tokens)?;
                    tokens.next();
                },
                _ => return Err(r#"[1Syntax Error] Wrong 'let' syntax.
try `let ident;` or `let ident = "Something";`"#),
            }
        } else {
            return Err(r#"[2Syntax Error] You ended the file b4 complete 'let' syntax.
try `let ident;` or `let ident = "Something";`"#);
        }
        if let Some(token) = tokens.peek() {
            match token {
                Token::Eq => {
                    tokens.next();
                    aa = self.parse_expr(tokens)?;
                    tokens.next();
                },
                Token::Semi => {
                    tokens.next();
                },
                _ => {
                    return Err(r#"[3Syntax Error] 'let' syntax is invalid.
try `let ident;` or `let ident = "Something";`, {:#?}"#);
                }
            }
        } else {
            return Err(r#"[4Syntax Error] You ended the file b4 complete 'let' syntax.
try `let ident;` or `let ident = "Something";`"#);
        }

        statment = NodeStatment::Var(ident, aa);
        return Ok(statment);
    }
    fn parsePrint<I>(&mut self, tokens: &mut std::iter::Peekable<I>)
        -> Result<NodeStatment, &'static str>
        where I: Iterator<Item = &'a Token>
    {
        return if let Some(token) = tokens.peek() {
            let mut res: Result<NodeStatment, &'static str>;
            match token {
                Token::IntLit(_)
                    | Token::StringLit(_, _)
                    | Token::FloatLit(_)
                    | Token::Boolean(_) 
                    | Token::Ident(_) => {
                    res = Ok(NodeStatment::Print(self.parse_expr(tokens)?));
                },
                _ => {
                    res = Err(r#"[Syntax Error] "print" method only accept 'int', 'float', 'boolean' and 'string'"#);
                }
            }
            tokens.next();
            res
        } else {
            Err(r#"[Syntax Error] you just end every think and forgot to pass an argument to "print" method!!"#)
        };
    }
    fn parseExit<I>(&mut self, tokens: &mut std::iter::Peekable<I>)
        -> Result<NodeStatment, &'static str>
        where I: Iterator<Item = &'a Token>
    {
        return if let Some(token) = tokens.peek() {
            let mut res: Result<NodeStatment, &'static str>;
            match token {
                Token::IntLit(_) | Token::Ident(_) => {
                    res = Ok(NodeStatment::Exit(self.parse_expr(tokens)?));
                },
                _ => {
                    res = Err(r#"[Syntax Error] It has to be a byte inside "Exit" method"#);
                }
            }
            tokens.next();
            res
        } else {
            Err(r#"[Syntax Error] you just end every think and forgot to pass an argument to "exit" method!!"#)
        }
    }
}
