use crate::lexer::{self, Token, TokenType};
use crate::node::{Node, build_node, build_unary_node, build_program_node, build_statement_node, build_method_call_node};

pub struct Parser {
    pos : usize,
    tokens : Vec<lexer::Token> 
}

fn error_unrecognized_token(token: &Token) -> ! {
    eprintln!("Syntax error: Unexpected token {} at character {}", token.value.as_ref().unwrap(), token.start);
    std::process::exit(1);
}

impl Parser {
    pub fn new(tokens: Vec<lexer::Token>) -> Self {
        Parser {
            pos: 0,
            tokens
        }
    }

    fn digest(&mut self, token_type : Option<lexer::TokenType>) -> Option<lexer::Token> {
        if self.tokens.len() == 0 {
            return None;
        }
        let token = self.tokens.remove(0);
        if token_type.is_some() && token.token_type != token_type.unwrap() {
            error_unrecognized_token(&token);
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

    pub fn parse(&mut self) -> Option<Box<Node>> {
        return build_program_node(self.parse_program());
    }

    fn parse_program(&mut self) -> Option<Box<Node>> {
        let mut statement = self.parse_statement();
        self.digest(Some(TokenType::EndOfstatement));

        while let Some(token) = self.peek(None) {
            if token.token_type == TokenType::Eof {
                self.digest(None);
                break;
            }
            let right = self.parse_statement();
            statement = build_statement_node(statement, right);
            self.digest(Some(TokenType::EndOfstatement));
        }

        return statement;
    }

    fn parse_statement(&mut self) -> Option<Box<Node>> {
        let token = self.peek(None).unwrap();

        match token.token_type {
            TokenType::NumeralLiteral | TokenType::Operator | TokenType::Symbol => {
                return self.parse_expression(0);
            }
            TokenType::Declaration => {
                self.digest(None);
                let symbol = self.digest(Some(TokenType::Symbol)).unwrap();
                let asignment_token = self.digest(Some(TokenType::Assignment)).unwrap();
                let expr = self.parse_expression(0);
                let symbol_node = build_node(&symbol, None, None);
                return build_node(&asignment_token, symbol_node, expr);
            }
            _ => error_unrecognized_token(&token)
        }
    }

    fn get_current_operator_predecence(&self) -> i32 {
        let op1 = self.peek(None);
        if op1.is_some() && op1.as_ref().unwrap().token_type == TokenType::Operator {
            return op1.unwrap().operator_predecende();
        }
        return 0;
    }

    fn parse_expression(&mut self, precedence: i32) -> Option<Box<Node>> {
        let token = self.peek(None).unwrap();
        let next = self.peek(Some(self.pos+1)).unwrap();
        if token.token_type == TokenType::Symbol && next.token_type == TokenType::ParenthesisL {
            let method_name = self.digest(None).unwrap();
            self.digest(Some(TokenType::ParenthesisL));
            let expr = self.parse_expression(0);
            self.digest(Some(TokenType::ParenthesisR));
            
            return build_method_call_node(method_name.value.unwrap(), expr);
        }
        else {
            // Start parsing the expression with the lowest precedence and descend
            return self.parse_binary_expression(precedence);
        }
    }

    fn parse_binary_expression(&mut self, precedence: i32) -> Option<Box<Node>> {
        let mut left = self.parse_term();
        
        while precedence < self.get_current_operator_predecence() {
            let token = self.peek(None).unwrap();
            if token.token_type == TokenType::Operator {
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
                }
            }
            TokenType::Symbol => {
                return build_node(token, None, None);
            }
            TokenType::NumeralLiteral => {
                return build_node(token, None, None)
            }
            TokenType::ParenthesisL => {
                let expr = self.parse_statement();
                self.digest(Some(TokenType::ParenthesisR));
                return expr;
            }
            _ => {
                error_unrecognized_token(token);
            }
        }
        
    }

}
