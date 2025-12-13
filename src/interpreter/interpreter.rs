use crate::error::error;
use crate::interpreter::scope::{ScopeArena, ScopeId};
use crate::node::{Block, Expression, Identifier, Literal, MethodCall, Operator, Program, UnaryOperator};

use super::methods::get_method;
use super::value::{Value, Convert};

pub struct Interpreter {
    scope_arena: ScopeArena,
    current_scope: ScopeId
}


impl Interpreter {
    pub fn new() -> Self {
        let mut scope_arena = ScopeArena::new();
        let current_scope = scope_arena.new_scope(None);
        Interpreter {
            scope_arena,
            current_scope
        }
    }

    pub fn evaluate(&mut self, node: Option<&Box<Expression>>) {
        if let Some(node_content) = node {
            match node_content.as_ref() {
                Expression::Program(program) => self.evaluate_program(program),
                Expression::BinaryOperation(_, _, _) => { self.evaluate_expression(node_content); },
                Expression::Statement(_) 
                | Expression::Declaration(_, _) 
                | Expression::MethodCall(_) => self.evaluate_statement(node_content),
                Expression::IfConditional(expression, if_block, else_block) => {
                    self.evaluate_conditional(expression, if_block, else_block)
                },
                _ => error("Unexpected AST node.".to_string()),
            }
        }
    }

    fn evaluate_program(&mut self, program: &Program) {
        let statements = &program.body;

        self.evaluate_block(statements);
    }

    fn evaluate_block(&mut self, block: &Block) {
        let parent_scope = self.current_scope;

        let child_scope = self.scope_arena.new_scope(Some(parent_scope));

        // Enter new scope
        self.current_scope = child_scope;

        for statement in block {
            self.evaluate(Some(statement));
        }

        // Exit scope
        self.current_scope = parent_scope;
    }

    fn evaluate_statement(&mut self, expression: &Expression) {
        match expression {
            Expression::Statement(expr) => {
                self.evaluate(Some(expr));
            }
            Expression::Declaration(identifier, expr) => {
                self.evaluate_assignment(identifier, expr);
            },
            Expression::MethodCall(method_call) => {
                self.evaluate_method_call(method_call);
            },
            _ => error("Unexpected AST node".to_string())
        }
    }

    fn evaluate_conditional(&mut self, expression: &Expression, if_block: &Block, else_block: &Option<Block>) {
        let expression_result = self.evaluate_expression(expression);
        if expression_result.to_bool() {
            self.evaluate_block(&if_block);
        }
        else if let Some(else_block) = else_block {
            self.evaluate_block(&else_block);
        }
    }

    fn evaluate_assignment(&mut self, identifier: &Identifier, expression: &Expression) {

        let value = self.evaluate_expression(expression);
        self.scope_arena.define(self.current_scope, identifier.name.to_string(), value);
    }

    fn evaluate_method_call(&mut self, node: &MethodCall) -> Value {

        // Prepraring for having multiple arguments
        let args : Vec<Value> = node.arguments
            .iter()
            .map(|expr| self.evaluate_expression(expr))
            .collect();

        return get_method(node.identifier.name.clone(), args);
    }

    fn evaluate_expression(&mut self, node: &Expression) -> Value {
        if let Expression::Identifier(identifier) = node {
            let identifier = identifier.name.to_string();
            let result = self.scope_arena.lookup(self.current_scope, &identifier);

            // Do we need to clone? What is the cost of it
            return result.unwrap_or_else(|| error("Unrecognized node".to_string())).clone();
        }
        else if let Expression::Literal(literal) = node {
            // Preparing for having multiple types
            return match literal {
                Literal::Boolean(b) => Value::Boolean(*b),
                Literal::Integer(i) => Value::Integer(*i),
                Literal::Float(f) => Value::Float(*f),
                Literal::String(s) => Value::String(s.to_string())
            };
        }
        else if let Expression::MethodCall(method_call) = node {
            return self.evaluate_method_call(method_call);
        }
        else if let Expression::UnaryOperation(operator, expr) = node {
            // Assuming is minus unary operator
            return match operator {
                UnaryOperator::Min => Value::Float(-1.0) * self.evaluate_expression(expr),
                UnaryOperator::Not => Value::Boolean(false) * self.evaluate_expression(expr)
            }
        }
        else if let Expression::BinaryOperation(left, op, right) = node {

            let left = self.evaluate_expression(left);
            let right = self.evaluate_expression(right);
    
            if matches!(left, Value::String(_)) || matches!(right, Value::String(_)) {
                if matches!(op, Operator::Sum) {
                    let mut left_str = String::convert(left.to_string()).unwrap();
                    let right = String::convert(right.to_string()).unwrap();
                    left_str.push_str(&right);
                    return Value::String(left_str);
                }
            }
            
            return match op {
                Operator::Exp => left.power(right),
                Operator::Mul => left * right,
                Operator::Div => left / right,
                Operator::Min => left - right,
                Operator::Sum => left + right,
                Operator::Eq  => left.eq_value(&right),
                Operator::Neq => left.neq_value(&right),

            };
        }
        else {
            error("Unrecognized node".to_string());
        }
    }

}