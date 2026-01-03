use parser::{
    ast::Expression,
    interpreter::Interpreter,
    lexer::{Token, TokenParser},
    parser::Parser as AstParser,
};
use std::{env, fs, process};

fn main() {
    if let Err(message) = run() {
        eprintln!("{}", message);
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let file_name = resolve_program_path(&args)?;

    let program = fs::read_to_string(&file_name)
        .map_err(|_| format!("Invalid program file: {}", file_name))?;

    lex(&program)
        .and_then(parse_ast)
        .and_then(|ast| interpret(ast.as_ref()))
}

fn resolve_program_path(args: &[String]) -> Result<String, String> {
    if let Some(program_file_name) = args.get(1) {
        Ok(program_file_name.clone())
    } else if cfg!(debug_assertions) {
        Ok("program.rmp".to_string())
    } else {
        Err("Program file is mandatory.".to_string())
    }
}

fn lex<'a>(program: &'a str) -> Result<Vec<Token<'a>>, String> {
    let mut token_parser = TokenParser::new(program);
    token_parser
        .parse()
        .map_err(|err| format!("Lexer error: {}", err))
}

fn parse_ast<'a>(tokens: Vec<Token<'a>>) -> Result<Box<Expression>, String> {
    let mut parser = AstParser::new(tokens);
    parser
        .parse()
        .map_err(|err| format!("Parser error: {}", err))
}

fn interpret(ast: &Expression) -> Result<(), String> {
    let mut interpreter = Interpreter::new();
    interpreter
        .run(Some(ast))
        .map_err(|err| format!("\nProgram exited \n {}", err))
}
