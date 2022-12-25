use crate::lexer::{self, Token, TokenType};
use crate::node::{Node, build_node, build_unary_node};

pub struct Parser {
    pos : usize,
    tokens : Vec<lexer::Token> 
}

fn error_unrecognized_token(token: &Token) {
    eprintln!("Syntax error: unrecognized token type {} at character {}", token.value.as_ref().unwrap(), token.start);
    std::process::exit(1);
}

impl Parser {
    pub fn new(tokens: Vec<lexer::Token>) -> Self {
        Parser {
            pos: 0,
            tokens: tokens
        }
    }

    fn digest(&mut self, token_type : Option<lexer::TokenType>) -> Option<lexer::Token> {
        if self.tokens.len() == 0 {
            return None;
        }
        let token = self.tokens.remove(0);
        if token_type.is_some() && token.token_type != token_type.unwrap() {
            error_unrecognized_token(&token);
            std::process::exit(1)
        }
        return Some(token);
    }

    fn peek(&self, pos: Option<usize>) -> Option<lexer::Token> {
        let pos = pos.unwrap_or(self.pos);
        if pos < self.tokens.len() {
            return Some(self.tokens[pos].clone_token());
        }
        return None;
    }

    pub fn evaluate(&mut self) -> f32 {
        let ast = self.parse_expression(0);
        return ast.unwrap().evaluate();
    }

    fn get_current_operator_predecence(&self) -> i32 {
        let op1 = self.peek(None);
        if op1.is_some() && op1.as_ref().unwrap().token_type == TokenType::Operator {
            return op1.unwrap().operator_predecende();
        }
        return 0;
    }

    fn parse_expression(&mut self, precedence: i32) -> Option<Box<Node>> {
        // Start parsing the expression with the lowest precedence and descend
        return self.parse_binary_expression(precedence);
    }


    fn parse_binary_expression(&mut self, precedence: i32) -> Option<Box<Node>> {
        let mut left = self.parse_term();
        
        while precedence < self.get_current_operator_predecence() {
            let token = self.peek(None).unwrap();
            if token.token_type == TokenType::Eof {
                self.digest(None);
                break;
            }
            else if token.token_type == TokenType::Operator {
                let op_precedence = self.get_current_operator_predecence();
                self.digest(None);
                let node = self.parse_expression(op_precedence);
                left = build_node(&token, left, node);
            }
            else {
                break;
            }
        }

        return left;
    }

    fn parse_term(&mut self) -> Option<Box<Node>> {
        let token = &self.digest(None).unwrap();

        match token.token_type {
            TokenType::Operator => {
                if token.value.as_ref().unwrap() == "-" {
                    let literal = self.parse_term();
                    return build_unary_node(token, literal);
                }
                else {
                    error_unrecognized_token(token);
                    return None;
                }
            }
            TokenType::NumeralLiteral => {
                return build_node(token, None, None)
            }
            TokenType::ParenthesisL => {
                let expr = self.parse_expression(0);
                self.digest(Some(TokenType::ParenthesisR));
                return expr;
            }
            _ => {
                error_unrecognized_token(token);
                return None;
            }
        }
        
    }

}
