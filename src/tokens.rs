#![allow(non_snake_case)]




use crate::traits::{
    Process,
    Token
};

#[derive(Debug)]
pub struct Tokenizer<'a> {
    index: usize,
    content: &'a str,
    contentLen: usize
}

impl<'a> Process<char> for Tokenizer<'a> {
    fn peek(&mut self, offset: Option<usize>) -> Option<char> {
        if self.index + offset.unwrap_or(0) >= self.contentLen {
            return None;
        }
        return Some(self.content.chars().nth(self.index + offset.unwrap_or(0)).unwrap());
    }
    fn consume(&mut self) -> char {
        let tmp = self.index.clone();
        self.index += 1;
        return self.content.chars().nth(tmp).unwrap();
    }
}

impl<'a> Tokenizer<'a> {
    pub fn new(content: &'a str) -> Self {
        Tokenizer {
            index: 0,
            content,
            contentLen: content.chars().count()
        }
    }
    fn str_tokener(&mut self, buffer: &mut String, c: &char) -> bool {
        return if *c == '\\' {
            buffer.push(self.consume());
            if let Some(_cc) = self.peek(None) {
                buffer.push(self.consume());
                true
            } else {
                eprintln!("[Syntax Error] Files ends before the string ends");
                false
            }
        } else {
            buffer.push(self.consume());
            true
        };
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut buffer: String = String::from("");
        while let Some(ch) = self.peek(None) {
            match ch {
                c if c.is_whitespace() => {
                    self.consume();
                    continue;
                },
                c if c.is_alphabetic() || c == '_' => {
                    buffer.push(self.consume());
                    while let Some(cc) = self.peek(None) {
                        if cc.is_alphanumeric() || cc == '_' {
                            buffer.push(self.consume());
                        } else {
                            break;
                        }
                    }
                    match &*buffer {
                        "exit" =>
                            tokens.push(Token::Exit),
                        "print" =>
                            tokens.push(Token::Print),
                        "var" =>
                            tokens.push(Token::Var),
                        b if b == "true" || b == "false" =>
                            tokens.push(Token::Boolean(b == "true")),
                        v =>
                            tokens.push(Token::Ident(v.to_string())),
                    }
                    buffer.clear();
                },
                c if c.is_numeric() => {
                    buffer.push(self.consume());
                    let mut isFloat = false;
                    while let Some(cc) = self.peek(None) {
                        if cc.is_numeric() { 
                            buffer.push(self.consume());
                        } else if cc == '.' {
                            buffer.push(self.consume());
                            isFloat = true;
                        } else {
                            break;
                        }
                    }
                    tokens.push(
                        if isFloat {
                            Token::FloatLit(buffer.clone())
                        } else {
                            Token::IntLit(buffer.clone())
                        }
                    );
                    buffer.clear();
                },
                '=' => {
                    self.consume();
                    tokens.push(Token::Eq);
                },
                '"' | '\'' => {
                    self.consume();
                    while let Some(c) = self.peek(None) {
                        if c == '"' || c == '\'' {
                            self.consume();
                            break;
                        }
                        self.str_tokener(&mut buffer, &c);
                    }
                    tokens.push(Token::StringLit(buffer.clone(), false));
                    buffer.clear();
                },
                '`' => {
                    self.consume();
                    while let Some(c) = self.peek(None) {
                        if c != '`' {
                            self.str_tokener(&mut buffer, &c);
                        } else {
                            self.consume();
                            break;
                        }
                    }
                    tokens.push(Token::StringLit(buffer.clone(), true));
                    buffer.clear();
                },
                ';' => {
                    self.consume();
                    tokens.push(Token::Semi);
                },
                '(' => {
                    self.consume();
                    tokens.push(Token::LeftPrac);
                },
                ')' => {
                    self.consume();
                    tokens.push(Token::RightPrac);
                },
                '#' => {
                    self.consume();
                    while let Some(ch) = self.peek(None) {
                        if ch == '\n' {
                            self.consume();
                            break;
                        }
                        self.consume();
                    }
                },
                _ => {
                    eprintln!("[Syntax Error] they're is no Token known as ({ch})");
                    break;
                }
            }
        }
        return tokens;
    }
}
