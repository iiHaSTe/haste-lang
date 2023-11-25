pub mod running;

use std::env;

/*
 * A main command
 * * * arguments and flags
 */
trait ICommand {
    
}

pub fn run_cli() {
    let args: Vec<String> = env::args().collect();
    println!("{:#?}", args);
}

