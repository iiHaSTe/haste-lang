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
use crate::parser::node_types::NodeStatment;
use crate::parser::expretion;

pub fn parseExit<'a, I>(tokens: &mut std::iter::Peekable<I>)
    -> Result<NodeStatment>
    where  I: Iterator<Item = &'a mut TokenType>,
{
    return if let Some(t) = tokens.peek() {
        let mut token = &t.token;
        let mut res: Result<NodeStatment>;
        match token {
            Token::IntLit(_) | Token::Ident(_) => {
                res = Ok(NodeStatment::Exit(expretion::parse(tokens)?));
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

pub fn parsePrint<'a, I>(tokens: &mut std::iter::Peekable<I>)
        -> Result<NodeStatment>
        where  I: Iterator<Item = &'a mut TokenType>
    {
        return if let Some(t) = tokens.peek() {
            let mut token = &t.token;
            let mut res: Result<NodeStatment>;
            match token {
                Token::IntLit(_)
                    | Token::StringLit(_, _)
                    | Token::FloatLit(_)
                    | Token::Boolean(_) 
                    | Token::Ident(_) => {
                    res = Ok(NodeStatment::Print(expretion::parse(tokens)?));
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
