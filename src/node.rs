use crate::{error, lexer::{AdditiveOperatorSubtype, NumeralType, OperatorType, Token, TokenType, UnaryOperatorSubtype}};

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

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration
{
    pub identifier: Identifier,
    pub arguments: Vec<Identifier>,
    pub block: Block
}

pub type Block = Vec<Box<Expression>>;

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal(Literal),
    BinaryOperation(Box<Expression>, OperatorType, Box<Expression>),
    UnaryOperation(OperatorType, Box<Expression>),
    Program(Program), // Change to block?
    Statement(Box<Expression>),
    MethodCall(MethodCall),
    Identifier(Identifier),
    Declaration(Identifier, Box<Expression>),
    Block(Block),
    FunctionDeclaration(FunctionDeclaration),
    IfConditional(Box<Expression>, Block, Option<Block>)
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

pub fn build_binary_op_node(operator: OperatorType, left: Box<Expression>, right: Box<Expression>) -> Box<Expression> {
    return Box::new(Expression::BinaryOperation(left, operator, right));
}

pub fn build_assignment_node(identifier: String, expr: Box<Expression>) -> Box<Expression> {
    return Box::new(Expression::Declaration(Identifier { name: identifier }, expr));
}

pub fn build_function_declaration_node(identifier: String, args: Vec<String>, block: Block) -> Box<Expression> {
    return Box::new(Expression::FunctionDeclaration(FunctionDeclaration {
        identifier: Identifier { name: identifier },
        // TODO: How can I prevent cloning?
        arguments: args.iter().map(|arg| Identifier { name: arg.clone() }).collect(),
        block,
    }));
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
            TokenType::Operator => {
                if let Some(operator_type) = token.operator_type.clone() {
                    build_binary_op_node(operator_type, left.unwrap(), right.unwrap())
                } else {
                    panic!("Unexpected operator type.")
                }
            },
            TokenType::Assignment => build_assignment_node(value, left.unwrap()),
            TokenType::Symbol => Box::new(Expression::Identifier(Identifier { name: value })),
            _ => panic!("Unexpected token type to process when building node.")
        };
}

pub fn build_unary_node(operation_type: UnaryOperatorSubtype, node: Box<Expression>) -> Box<Expression> {
    return Box::new(Expression::UnaryOperation(OperatorType::Unary(operation_type), node));
}

pub fn build_program_node(body: Vec<Box<Expression>>) -> Box<Expression> {
    return Box::new(Expression::Program(Program { body }));
}

pub fn build_statement_node(expr: Box<Expression>) -> Box<Expression> {
    return Box::new(Expression::Statement(expr))
}