mod lexer;
mod parser;
mod node;

fn main() {
    let program = String::from("let a = (2-3)*4+3^5+4+(-3+4^4)+5+6+2-1; print(a);");
    let mut token_parser = lexer::TokenParser::new(program);
    let tokens = token_parser.parse();
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse();

    //print!("Result: {}", result);
}
