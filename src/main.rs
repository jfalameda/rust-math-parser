use parser::{interpreter::Interpreter, lexer, parser as ast_parser};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut program_file = args.get(1);
    let file = "program.rmp".to_string();

    if cfg!(debug_assertions) {
        program_file = Some(&file);
    } else if program_file.is_none() {
        eprintln!("Program file is mandatory.");
        std::process::exit(1);
    }

    let file_name = program_file.unwrap();
    let program = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(_) => {
            eprintln!("Invalid program file: {}", file_name);
            std::process::exit(1);
        }
    };

    // Lexical analysis
    let mut token_parser = lexer::TokenParser::new(program);
    let tokens = match token_parser.parse() {
        Ok(t) => t,
        Err(err) => {
            eprintln!("Lexer error: {}", err);
            std::process::exit(1);
        }
    };

    // Parsing
    let mut parser = ast_parser::Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!("Parser error: {}", err);
            std::process::exit(1);
        }
    };

    // Interpreting
    let mut interpreter = Interpreter::new();
    if let Err(err) = interpreter.run(Some(ast.as_ref())) {
        eprintln!("\nProgram exited \n {}", err);
        std::process::exit(1);
    }
}
