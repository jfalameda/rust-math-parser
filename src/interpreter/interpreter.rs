use crate::node::{Expression, Identifier, Literal, Operator, MethodCall, Program};
use std::collections::{HashMap};
use std::sync::Mutex;
use once_cell::sync::Lazy;

use super::value::Value;

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
            _ => panic!("Unexpected AST node")
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
            _ => panic!("Unexpected AST node")
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

        return match node.identifier.name.as_str() {
            "print" => {
                args.iter().for_each(|arg| {
                    print!("{}", arg.to_string());
                });
               
                Value::Empty
            },
            "println" => { 
                args.iter().for_each(|arg| {
                    print!("{}", arg.to_string());
                });
                println!("");

                Value::Empty
            },
            "str_concat" => {
                let mut concat_str = String::from("");

                args.iter().for_each(|arg| {
                    concat_str.push_str(&arg.to_string());
                });

                return Value::String(concat_str);
            },
            "sin" => {
                let number = args.get(0).unwrap();
                return Value::Float(f32::sin(number.to_number()));
            }
            "cos" => {
                let number = args.get(0).unwrap();
                return Value::Float(f32::cos(number.to_number()));
            }
            _ => panic!("Unrecognized method name")
        }
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
                Literal::Float(f) => Value::Float(*f),
                Literal::String(s) => Value::String(s.to_string())
            };
        }
        else if let Expression::MethodCall(method_call) = node {
            return self.evaluate_method_call(method_call);
        }
        else if let Expression::UnaryOperation(_, expr) = node {
            // Assuming is minus unary operator
            return Value::Float(-1.0 * self.evaluate_expression(expr).to_number());
        }
        else if let Expression::BinaryOperation(left, op, right) = node {

            let left = self.evaluate_expression(left);
            let right = self.evaluate_expression(right);
    
            if matches!(left, Value::String(_)) || matches!(right, Value::String(_)) {
                if matches!(op, Operator::Sum) {
                    let mut left_str = left.to_string();
                    left_str.push_str(&right.to_string());
                    return Value::String(left_str);
                }
            }
            
            return Value::Float(match op {
                Operator::Exp => left.to_number().powf(right.to_number()),
                Operator::Mul => left.to_number() * right.to_number(),
                Operator::Div => left.to_number() * right.to_number(),
                Operator::Min => left.to_number() - right.to_number(),
                Operator::Sum => left.to_number() + right.to_number(),
            });
        }
        else {
            panic!("Unrecognized node");
        }
    }

}