#![allow(non_snake_case, unreachable_patterns)]

pub mod tokens;
pub mod traits;
pub mod node_tree;
pub mod generator;

use std::fs;
use std::fs::File;
use std::io::prelude::*;

use crate::tokens::Tokenizer;
use crate::node_tree::TreeParser;
use crate::generator::javascript::JSGenerator;

fn main() -> std::io::Result<()> {
    let binding = fs::read("./main.haste")?;
    let input = String::from_utf8_lossy(&binding);
    
    let mut tokenizer = Tokenizer::new(&input);
    let tokens = tokenizer.tokenize();

    let mut parser = TreeParser::new(&tokens);
    let tree = parser.parse();

    let generator = JSGenerator::new(&tree);
    let javascriptCode = generator.generate();
    
    {
        let mut file = File::create("./dist/out.js")?;
        file.write_all(javascriptCode.as_bytes())?;
    }
    {
        let mut file = File::create("./log/tokens.rs")?;
        file.write_all(format!("{:#?}", tokens).as_bytes())?;
    }
    {
        let mut file = File::create("./log/tree.rs")?;
        file.write_all(format!("{:#?}", tree).as_bytes())?;
    }
    Ok(())
}
