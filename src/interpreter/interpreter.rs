use crate::error::error;
use crate::node::{Expression, Identifier, Literal, Operator, MethodCall, Program};
use std::collections::{HashMap};
use std::sync::Mutex;
use once_cell::sync::Lazy;

use super::methods::get_method;
use super::value::{Value, Convert};

static mut VARIABLES: Lazy<Mutex<HashMap<String, Value>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});

pub struct Interpreter {
    ast: Box<Expression>
}


impl Interpreter {
    pub fn new(ast: Box<Expression>) -> Self {
        Interpreter { ast }
    }

    pub fn evaluate(&self, node: Option<&Box<Expression>>) {
        let node_content = node.unwrap_or(&self.ast).as_ref();
        match node_content {
            Expression::Program(program) => {
                self.evaluate_program(program);
            },
            Expression::BinaryOperation(_, _, _) => {
                self.evaluate_expression(node_content);
            },
            Expression::Statement(_) | Expression::Declaration(_, _) | Expression::MethodCall(_) => self.evaluate_statement(node_content),
            _ => error("Unexpected AST node.".to_string())
        }
    }

    fn evaluate_program(&self, program: &Program) {
        let statements = &program.body;

        for statement in statements {
            self.evaluate(Some(statement));
        }
    }

    fn evaluate_statement(&self, expression: &Expression) {
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

    fn evaluate_assignment(&self, identifier: &Identifier, expression: &Expression) {

        let value = self.evaluate_expression(expression);
        unsafe {
            VARIABLES.lock().unwrap().insert(identifier.name.to_string(), value);
        }
    }

    fn evaluate_method_call(&self, node: &MethodCall) -> Value {

        // Prepraring for having multiple arguments
        let args : Vec<Value> = node.arguments
            .iter()
            .map(|expr| self.evaluate_expression(expr))
            .collect();

        return get_method(node.identifier.name.clone(), args);
    }

    fn evaluate_expression(&self, node: &Expression) -> Value {
        if let Expression::Identifier(identifier) = node {
            let value = identifier.name.to_string();
            let result: Value;
            unsafe {
                result = VARIABLES.lock().unwrap().get(&value).unwrap().clone();
            }
            return result;
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
        else if let Expression::UnaryOperation(_, expr) = node {
            // Assuming is minus unary operator
            return Value::Float(-1.0) * self.evaluate_expression(expr);
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
            };
        }
        else {
            error("Unrecognized node".to_string());
        }
    }

}