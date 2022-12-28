mod lexer;
mod parser;
mod node;
mod interpreter;
use std::{env, fs};

use interpreter::node_interpreted::NodeInterpreted;

fn error(error_message: String) -> ! {
    eprintln!("{}", error_message.to_string());
    std::process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut program_file = args.get(1);
    if program_file.is_none() {
        //error("Program file is mandatory.".to_string());
    }
    let program = "program.rmp".to_string();
    let file = args.get(1).unwrap_or(&program);
    let program = fs::read_to_string(file)
        .expect("Invalid program name.");

    let program = String::from(program);
    let mut token_parser = lexer::TokenParser::new(program);
    let tokens = token_parser.parse();
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse();
    ast.unwrap().evaluate();

}
