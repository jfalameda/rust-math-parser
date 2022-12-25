use crate::lexer::{Token, TokenType};

#[derive(PartialEq)]
pub enum NodeType {
    Literal,
    BinaryOperation,
    UnaryOperation
}

pub struct Node {
    pub node_type: NodeType,
    pub value: String,
    pub left_handside: Option<Box<Node>>,
    pub right_handside: Option<Box<Node>>
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

pub fn build_node(token: &Token, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Option<Box<Node>> {
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

pub fn build_unary_node(token: &Token, node: Option<Box<Node>>) -> Option<Box<Node>> {
    return Some(Box::new(Node {
        node_type: NodeType::UnaryOperation,
        value: String::from(token.value.as_ref().unwrap()),
        left_handside: node,
        right_handside: None
    }));
}