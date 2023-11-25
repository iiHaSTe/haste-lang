#![allow(non_snake_case)]

use std::thread;
use std::time::Duration;
use crate::traits::{
    Process,
    Token,
    TokenType
};

#[derive(Debug)]
pub struct Tokenizer<'a> {
    index: usize,
    content: &'a str,
    contentLen: usize,
    token_type: TokenType
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
            contentLen: content.chars().count(),
            token_type: TokenType {
                line: 1,
                column: 0,
                token: Token::Vide
            }
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
    pub fn init_token(&mut self, token: Token) -> TokenType {
        self.token_type.token = token;
        return self.token_type.clone();
    }

    pub fn tokenize(&mut self) -> Vec<TokenType> {
        let mut tokens: Vec<TokenType> = vec![];
        let mut buffer: String = String::from("");
        while let Some(ch) = self.peek(None) {
            self.token_type.token = Token::Vide;
            self.token_type.column += 1;
            match ch {
                c if c.is_whitespace() => {
                    if c == '\n' {
                        self.token_type.line += 1;
                        self.token_type.column = 0;
                        self.consume();
                        continue;
                    }
                    self.token_type.column += 1;
                    self.consume();
                    continue;
                },
                c if c.is_alphabetic() || c == '_' => {
                    buffer.push(self.consume());
                    while let Some(cc) = self.peek(None) {
                        if cc.is_alphanumeric() || cc == '_' {
        self.token_type.column += 1;
                            buffer.push(self.consume());
                        } else {
                            break;
                        }
                    }
                    match &*buffer {
                        "exit" =>
                            tokens.push(self.init_token(Token::Exit)),
                        "print" =>
                            tokens.push(self.init_token(Token::Print)),
                        "var" =>
                            tokens.push(self.init_token(Token::Keyword(buffer.to_string()))),
                        b if b == "true" || b == "false" =>
                            tokens.push(self.init_token(Token::Boolean(b == "true"))),
                        v =>
                            tokens.push(self.init_token(Token::Ident(v.to_string()))),
                    }
                    buffer.clear();
                },
                c if c.is_numeric() => {
                    buffer.push(self.consume());
                    let mut isFloat = false;
                    while let Some(cc) = self.peek(None) {
                        if cc.is_numeric() { 
        self.token_type.column += 1;
                            buffer.push(self.consume());
                        } else if cc == '.' {
        self.token_type.column += 1;
                            buffer.push(self.consume());
                            isFloat = true;
                        } else {
                            break;
                        }
                    }
                    tokens.push(self.init_token(
                        if isFloat {
                            Token::FloatLit(buffer.clone())
                        } else {
                            Token::IntLit(buffer.clone())
                        }
                    ));
                    buffer.clear();
                },
                '=' => {
                    self.consume();
                    tokens.push(self.init_token(Token::Eq));
                },
                '"' | '\'' => {
                    self.consume();
                    while let Some(c) = self.peek(None) {
        self.token_type.column += 1;
                        if c == '"' || c == '\'' {
                            self.consume();
                            break;
                        }
                        self.str_tokener(&mut buffer, &c);
                    }
                    tokens.push(self.init_token(Token::StringLit(buffer.clone(), false)));
                    buffer.clear();
                },
                '`' => {
                    self.consume();
                    while let Some(c) = self.peek(None) {
        self.token_type.column += 1;
                        if c != '`' {
                            self.str_tokener(&mut buffer, &c);
                        } else {
                            self.consume();
                            break;
                        }
                    }
                    tokens.push(self.init_token(Token::StringLit(buffer.clone(), true)));
                    buffer.clear();
                },
                ';' => {
                    self.consume();
                    tokens.push(self.init_token(Token::Semi));
                },
                '(' => {
                    self.consume();
                    tokens.push(self.init_token(Token::LeftPrac));
                },
                ')' => {
                    self.consume();
                    tokens.push(self.init_token(Token::RightPrac));
                },
                '#' => {
                    self.consume();
                    while let Some(ch) = self.peek(None) {
                        if ch == '\n' {
                            self.token_type.line += 1;
                            self.token_type.column = 1;
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
