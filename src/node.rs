use crate::{error, lexer::{NumeralType, Token, TokenType}};

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Sum,
    Min,
    Mul,
    Div,
    Exp,
    Eq,
    Neq
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Min
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub name: String
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program
{
    pub body: Vec<Box<Expression>>
}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodCall
{
    pub identifier: Identifier,
    pub arguments: Vec<Box<Expression>>,
}

pub type Block = Vec<Box<Expression>>;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal(Literal),
    BinaryOperation(Box<Expression>, Operator, Box<Expression>),
    UnaryOperation(UnaryOperator, Box<Expression>),
    Program(Program), // Change to block?
    Statement(Box<Expression>),
    MethodCall(MethodCall),
    Identifier(Identifier),
    Declaration(Identifier, Box<Expression>),
    Block(Block),
    IfConditional(Box<Expression>, Block, Option<Block>)
}

fn token_value_to_operator(value: String) -> Operator {
    return match value.as_str() {
        "+" => Operator::Sum,
        "-" => Operator::Min,
        "*" => Operator::Mul,
        "/" => Operator::Div,
        "^" => Operator::Exp,
        "==" => Operator::Eq,
        "!=" => Operator::Neq,
        _  => error(format!("Unrecognized operator {}", value))
    };
}

pub fn build_method_call_node(method_name: String, args: Vec<Box<Expression>>) -> Box<Expression> {
    return Box::new(Expression::MethodCall(MethodCall {
        identifier: Identifier { name: method_name },
        arguments: args
    }));
}

pub fn build_numerical_literal_node(literal: Literal) -> Box<Expression> {
    return Box::new(Expression::Literal(literal));
}

pub fn build_conditional_node(condition: Box<Expression>, if_block: Block, else_block: Option<Block>) -> Box<Expression> {
    return Box::new(Expression::IfConditional(condition, if_block, else_block));
}

pub fn build_binary_op_node(operator: Operator, left: Box<Expression>, right: Box<Expression>) -> Box<Expression> {
    return Box::new(Expression::BinaryOperation(left, operator, right));
}

pub fn build_assignment_node(identifier: String, expr: Box<Expression>) -> Box<Expression> {
    return Box::new(Expression::Declaration(Identifier { name: identifier }, expr));
}


pub fn build_node(token: &Token, left: Option<Box<Expression>>, right: Option<Box<Expression>>) -> Box<Expression> {
    let value = String::from(token.value.as_ref().unwrap());
    return
        match token.token_type {
            TokenType::NumeralLiteral(numeral_type) => match(numeral_type) {
                NumeralType::Integer => build_numerical_literal_node(Literal::Integer(value.parse::<i64>().unwrap())),
                NumeralType::Float => build_numerical_literal_node(Literal::Float(value.parse::<f64>().unwrap()))
            },
            TokenType::StringLiteral => build_numerical_literal_node(Literal::String(value.parse::<String>().unwrap())),
            TokenType::BooleanLiteral => build_numerical_literal_node(Literal::Boolean(value.parse::<bool>().unwrap())),
            TokenType::Operator => build_binary_op_node(token_value_to_operator(value), left.unwrap(), right.unwrap()),
            TokenType::Assignment => build_assignment_node(value, left.unwrap()),
            TokenType::Symbol => Box::new(Expression::Identifier(Identifier { name: value })),
            _ => panic!("Unexpected token type to process when building node.")
        };
}

// TODO: Check for the actual operator
pub fn build_unary_node(_: &Token, node: Box<Expression>) -> Box<Expression> {
    return Box::new(Expression::UnaryOperation(UnaryOperator::Min, node));
}

pub fn build_program_node(body: Vec<Box<Expression>>) -> Box<Expression> {
    return Box::new(Expression::Program(Program { body }));
}

pub fn build_statement_node(expr: Box<Expression>) -> Box<Expression> {
    return Box::new(Expression::Statement(expr))
}