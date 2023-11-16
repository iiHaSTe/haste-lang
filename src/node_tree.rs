use crate::traits::{
    Process,
    Token
};

#[derive(Debug)]
pub enum NodeLine {
    Exit(u8),
    Print(Token)
}

#[derive(Debug)]
pub struct NodeTree {
    pub body: Vec<NodeLine>
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
                Token::Semi => {
                    self.consume();
                },
                _ => {},
            }
        }
        return tree;
    }
    fn parsePrint(&mut self) -> NodeLine {
        if let Some(token) = self.peek(None) {
            return match token {
                Token::IntLit(_value) => {
                    NodeLine::Print(self.consume().clone())
                },
                Token::StringLit(_value) => {
                    NodeLine::Print(self.consume().clone())
                },
                Token::FloatLit(_value) => {
                    NodeLine::Print(self.consume().clone())
                },
                Token::Boolean(_value) => {
                    NodeLine::Print(self.consume().clone())
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
    fn parseExit(&mut self) -> NodeLine {
        if let Some(token) = self.peek(None) {
            return match token {
                Token::IntLit(value) => {
                    self.consume();
                    NodeLine::Exit(value.parse::<u8>().unwrap())
                },
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

