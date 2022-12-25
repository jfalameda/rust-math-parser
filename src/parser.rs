use crate::lexer::{self, Token, TokenType, OperatorType};

#[derive(PartialEq)]
enum NodeType {
    Literal,
    BinaryOperation,
    UnaryOperation
}

struct Node {
    node_type: NodeType,
    value: String,
    left_handside: Option<Box<Node>>,
    right_handside: Option<Box<Node>>
}

impl Node {
    pub fn print(&self, level: i32) {
        let level_indent = (0..level).map(|_| "..").collect::<String>();
        if self.node_type == NodeType::BinaryOperation {

            let left = self.left_handside.as_ref();
            let right = self.right_handside.as_ref();

            if left.is_some() {
                left.unwrap().print(level+1);
            }
            if self.node_type == NodeType::BinaryOperation {
                println!("{} {}", level_indent, self.value);
            }
            if right.is_some() {
                right.unwrap().print(level+1);
            }
        }
        else {
            println!("{} {}", level_indent, self.value);
        }
        return;
    }

    pub fn evaluate(&self) -> f32 {
        if self.node_type == NodeType::Literal {
            return self.value.parse::<f32>().unwrap();
        }
        else if self.node_type == NodeType::UnaryOperation {
            return -1.0 * self.left_handside.as_ref().unwrap().evaluate();
        }
        else {
            let left = self.left_handside.as_ref().unwrap().evaluate();
            let right = self.right_handside.as_ref().unwrap().evaluate();
            
            if self.value == "^" {
                return left.powf(right);
            }
            if self.value == "*" {
                return left * right;
            }
            if self.value == "/" {
                return left / right;
            }
            if self.value == "-" {
                return left - right;
            }
            if self.value == "+" {
                return left + right;
            }
        }
        return 0.0;
    }
}

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

    fn operator_predecende(&self, operator: String) -> i32 {
        match operator.as_str() {
            "^" => 3,
            "*" | "/" => 2,
            "+" | "-" => 1,
            _ => 0
        }
    }

    fn get_current_operator_predecence(&self) -> i32 {
        let op1 = self.peek(None);
        if op1.is_some() && op1.as_ref().unwrap().token_type == TokenType::Operator {
            return self.operator_predecende(String::from(op1.as_ref().unwrap().value.as_ref().unwrap()));
        }
        return 0;
    }

    fn build_node(&self, token: &Token, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Option<Box<Node>> {
        return Some(Box::new(Node {
            node_type: match token.token_type {
                TokenType::NumeralLiteral => NodeType::Literal,
                TokenType::Operator => NodeType::BinaryOperation,
                _ => panic!("Unexpected token type to process when building node.")
            },
            value: String::from(token.value.as_ref().unwrap()),
            left_handside: left,
            right_handside: right
        }));
    }

    fn build_unary_node(&self, token: &Token, node: Option<Box<Node>>) -> Option<Box<Node>> {
        return Some(Box::new(Node {
            node_type: NodeType::UnaryOperation,
            value: String::from(token.value.as_ref().unwrap()),
            left_handside: node,
            right_handside: None
        }));
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
                left = self.build_node(&token, left, node);
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
                    return self.build_unary_node(token, literal);
                }
                else {
                    error_unrecognized_token(token);
                    return None;
                }
            }
            TokenType::NumeralLiteral => {
                return self.build_node(token, None, None)
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
