use crate::traits::{
    Process,
    Token
};

#[derive(Debug)]
pub enum NodeExpr {
    Ident(Token),
    IntLit(Token),
    StringLit(Token),
    FloatLit(Token),
    Boolean(Token),
    Null,
}

#[derive(Debug)]
pub enum NodeStatment {
    Exit(NodeExpr),
    Print(NodeExpr),
    Var(Token, NodeExpr)
}

#[derive(Debug)]
pub struct NodeTree {
    pub body: Vec<NodeStatment>
}

#[derive(Debug)]
pub struct TreeParser<'a> {
    index: usize,
    content: &'a Vec<Token>,
    contentLen: usize
}

impl<'a> Process<&'a Token> for TreeParser<'a> {
    fn peek(&mut self, offset: Option<usize>) -> Option<&'a Token> {
        if self.index + offset.unwrap_or(0) >= self.contentLen {
            return None;
        }
        return Some(self.content.get(self.index + offset.unwrap_or(0)).unwrap());
    }
    fn consume(&mut self) -> &'a Token {
        let tmp = self.index.clone();
        self.index += 1;
        return self.content.get(tmp).unwrap();
    }
}

impl<'a> TreeParser<'a> {
    pub fn new(content: &'a Vec<Token>) -> Self {
        return TreeParser { 
            index: 0,
            content: content,
            contentLen: content.len()
        };
    }
    pub fn parse(&mut self) -> NodeTree {
        let mut tree = NodeTree {
            body: vec![]
        };
        while let Some(token) = self.peek(None) {
            match token {
                Token::Exit => {
                    self.consume();
                    tree.body.push(self.parseExit())
                },
                Token::Print => {
                    self.consume();
                    tree.body.push(self.parsePrint());
                },
                Token::Var => {
                    self.consume();
                    tree.body.push(self.parseVar());
                },
                Token::Semi => {
                    self.consume();
                },
                _ => {},
            }
        }
        return tree;
    }
    fn parseVar(&mut self) -> NodeStatment {
        let mut statment: NodeStatment;
        if let (Some(t1), Some(t2), Some(t3)) = (self.peek(None), self.peek(Some(1)), self.peek(Some(2))) {
            match (t1, t2, t3) {
                (Token::Ident(_), Token::Semi, _) =>
                    statment = NodeStatment::Var(self.consume().clone(), NodeExpr::Null),
                (Token::Ident(_), Token::Eq, token) =>
                    statment = NodeStatment::Var(self.consume().clone(), {
                        self.consume();
                        match token {
                            Token::Ident(_) => NodeExpr::Ident(self.consume().clone()),
                            Token::IntLit(_) => NodeExpr::IntLit(self.consume().clone()),
                            Token::Boolean(_) => NodeExpr::Boolean(self.consume().clone()),
                            Token::StringLit(_) => NodeExpr::StringLit(self.consume().clone()),
                            Token::FloatLit(_) => NodeExpr::FloatLit(self.consume().clone()),
                            _ => {
                                eprintln!("[Syntax Error] wrong token in var statment");
                                std::process::exit(1);
                            }
                        }
                    }),
                _ => {
                    println!("{:#?} {:#?} {:#?}", t1, t2, t3);
                    eprintln!("[Syntax Error] You don't know how to define a variable ??");
                    std::process::exit(1);
                }
            }
        } else {
            eprintln!("[Syntax Error] So smart! even you ended the file without declare the variable correctly!!");
            std::process::exit(1);
        }
        return statment;
    }
    fn parsePrint(&mut self) -> NodeStatment {
        if let Some(token) = self.peek(None) {
            return match token {
                Token::IntLit(_value) => {
                    NodeStatment::Print(
                        NodeExpr::IntLit(self.consume().clone())
                    )
                },
                Token::StringLit(_value) => {
                    NodeStatment::Print(
                        NodeExpr::StringLit(self.consume().clone())
                    )
                },
                Token::FloatLit(_value) => {
                    NodeStatment::Print(
                        NodeExpr::FloatLit(self.consume().clone())
                    )
                },
                Token::Boolean(_value) => {
                    NodeStatment::Print(
                        NodeExpr::Boolean(self.consume().clone())
                    )
                },
                Token::Ident(_value) => {
                    NodeStatment::Print(
                        NodeExpr::Ident(self.consume().clone())
                    )
                },
                _ => {
                    eprintln!(r#"[Syntax Error] "print" method only accept 'int', 'float', 'boolean' and 'string'"#);
                    std::process::exit(1);
                }
            }
        } else {
            eprintln!(r#"[Syntax Error] you just end every think and forgot to pass an argument to "print" method!!"#);
            std::process::exit(1);
        }
    }
    fn parseExit(&mut self) -> NodeStatment {
        if let Some(token) = self.peek(None) {
            return match token {
                Token::IntLit(_value) => {
                    NodeStatment::Exit(
                        NodeExpr::IntLit(self.consume().clone())
                    )
                },
                Token::Ident(_value) => {
                    NodeStatment::Exit(
                        NodeExpr::Ident(self.consume().clone())
                    )
                }
                _ => {
                    eprintln!(r#"[Syntax Error] It has to be a byte inside "Exit" method"#);
                    std::process::exit(1);
                }
            };
        } else {
            eprintln!(r#"[Syntax Error] you just end every think and forgot to pass an argument to "exit" method!!"#);
            std::process::exit(1);
        }
    }
}

