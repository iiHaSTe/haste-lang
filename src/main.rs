#![allow(non_snake_case, unused_mut, unreachable_patterns)]

pub mod tokens;
pub mod traits;
pub mod parser;
pub mod generator;
pub mod errors;
pub mod cli;

use std::fs;
use std::fs::File;
use std::io::prelude::*;

use crate::tokens::Tokenizer;
use crate::generator::javascript::JSGenerator;
use crate::cli::run_cli;

fn main() -> std::io::Result<()> {
    // run_cli();
    let binding = fs::read("./main.haste")?;
    let input = String::from_utf8_lossy(&binding);
    
    let mut tokenizer = Tokenizer::new(&input);
    let mut tokens = tokenizer.tokenize();
    {
        let mut file = File::create("./log/tokens.rs")?;
        file.write_all(format!("{:#?}", tokens).as_bytes())?;
    }
    
    let tree = match parser::parse_program(&mut tokens) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        },
    };
    {
        let mut file = File::create("./log/tree.rs")?;
        file.write_all(format!("{:#?}", tree).as_bytes())?;
    }
    
    let generator = JSGenerator::new(&tree);
    let javascriptCode = generator.generate_programe();
    {
        let mut file = File::create("./dist/out.js")?;
        file.write_all(javascriptCode.as_bytes())?;
    }
    Ok(())
}
