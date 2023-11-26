pub mod expretion;
pub mod statement;
pub mod node_types;

use crate::traits::TokenType;
use crate::errors::Result;

pub fn parse_program(tokens: &mut Vec<TokenType>) -> Result<node_types::NodeProgram> {
    let mut token_peekable = tokens.into_iter().peekable();
    return statement::parse(&mut token_peekable);
}
