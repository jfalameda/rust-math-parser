use crate::lexer::{self, Token, TokenType};
use crate::node::{Expression, build_node, build_unary_node, build_program_node, build_statement_node, build_method_call_node, build_assignment_node};

pub struct Parser {
    pos : usize,
    tokens : Vec<lexer::Token> 
}

fn token_type_string(token_type: &TokenType) -> String {
    return match token_type {
        TokenType::Assignment => "=".to_string(),
        TokenType::Declaration => "let".to_string(),
        TokenType::EndOfstatement => ";".to_string(),
        TokenType::Eof => "EOF".to_string(),
        TokenType::NumeralLiteral => "literal".to_string(),
        TokenType::Operator => "operator".to_string(),
        TokenType::ParenthesisL => "(".to_string(),
        TokenType::ParenthesisR => ")".to_string(),
        TokenType::Symbol => "symbol".to_string(),
        TokenType::ArgumentSeparator => ",".to_string(),
        TokenType::StringLiteral => "\"".to_string(),
    }
}

fn error_unexpected_token(token: &Token, expected_token_type: &TokenType) -> ! {
    let expected = token_type_string(expected_token_type);
    let found = token_type_string(&token.token_type);
    eprintln!("Syntax error: Expected token {} at line {} and character {}, instead found {}", expected, token.line, token.start, found);
    std::process::exit(1);
}

fn error_unrecognized_token(token: &Token,) -> ! {
    let found = token_type_string(&token.token_type);
    eprintln!("Syntax error: Unrecognized token {} at line {} and character {}", found, token.line, token.start);
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
        if token_type.is_some() {
            let token_ref = token_type.as_ref().unwrap();
            if token.token_type != *token_ref {
                error_unexpected_token(&token, token_ref);
            }
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

    fn get_current_operator_predecence(&self) -> i32 {
        let op1 = self.peek(None);
        if op1.is_some() && op1.as_ref().unwrap().token_type == TokenType::Operator {
            return op1.unwrap().operator_predecende();
        }
        return 0;
    }


    pub fn parse(&mut self) -> Box<Expression> {
        return build_program_node(self.parse_program());
    }

    fn parse_program(&mut self) -> Vec<Box<Expression>> {
        let statement = self.parse_statement();
        self.digest(Some(TokenType::EndOfstatement));

        let mut body = vec![statement];

        while let Some(token) = self.peek(None) {
            if token.token_type == TokenType::Eof {
                self.digest(None);
                break;
            }
            let right = self.parse_statement();
            body.push(build_statement_node(right));
            self.digest(Some(TokenType::EndOfstatement));
        }

        return body;
    }

    fn parse_statement(&mut self) -> Box<Expression> {
        let token = self.peek(None).unwrap();

        match token.token_type {
            TokenType::NumeralLiteral | TokenType::Operator | TokenType::Symbol | TokenType::StringLiteral=> {
                return self.parse_expression(0);
            }
            TokenType::Declaration => {
                self.digest(None);
                let symbol = self.digest(Some(TokenType::Symbol)).unwrap();
                self.digest(Some(TokenType::Assignment));
                let expr = self.parse_expression(0);
                return build_assignment_node(symbol.value.unwrap(), expr);
            }
            _ => error_unrecognized_token(&token)
        }
    }

    fn parse_expression(&mut self, precedence: i32) -> Box<Expression> {
        let token = self.peek(None).unwrap();
        let next = self.peek(Some(self.pos+1)).unwrap();
        if token.token_type == TokenType::Symbol && next.token_type == TokenType::ParenthesisL {
            return self.parse_method_call();
        }
        else {
            // Start parsing the expression with the lowest precedence and descend
            return self.parse_binary_expression(precedence);
        }
    }

    fn parse_method_call(&mut self) -> Box<Expression> {
        let method_name = self.digest(None).unwrap();
        self.digest(Some(TokenType::ParenthesisL));
        let args = self.parse_method_args();
        self.digest(Some(TokenType::ParenthesisR));
        
        return build_method_call_node(method_name.value.unwrap(), args);
    }

    fn parse_method_args(&mut self) -> Vec<Box<Expression>> {
        let mut args = vec![];

        while let Some(token) = self.peek(None) {
            if token.token_type == TokenType::ParenthesisR {
                break;
            }
            else {
                args.push(self.parse_expression(0));
                let next = self.peek(None).unwrap();
                if next.token_type != TokenType::ParenthesisR {
                    self.digest(Some(TokenType::ArgumentSeparator));
                }
            }
        }

        return args;
    }

    fn parse_binary_expression(&mut self, precedence: i32) -> Box<Expression> {
        let mut left = self.parse_term();
        
        while precedence < self.get_current_operator_predecence() {
            let token = self.peek(None).unwrap();
            if token.token_type == TokenType::Operator {
                let op_precedence = self.get_current_operator_predecence();
                self.digest(None);
                let node = self.parse_expression(op_precedence);
                left = build_node(&token, Some(left), Some(node));
            }
            else {
                break;
            }
        }

        return left;
    }

    fn parse_term(&mut self) -> Box<Expression> {
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
            TokenType::StringLiteral => {
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
