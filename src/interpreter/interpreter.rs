use crate::node::{Expression, Identifier, Literal, Operator, MethodCall, Program};
use std::collections::btree_map::Values;
use std::collections::{HashMap};
use std::io::{self, BufRead, Write};
use std::io::stdout;
use std::sync::Mutex;
use once_cell::sync::Lazy;

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
                    let str = String::convert(arg.to_string()).unwrap();
                    print!("{}", str);
                });
               
                Value::Empty
            },
            "to_number" => {
                let value = args.get(0).unwrap();

                value.convert_to_number()
            }
            "readln" => {
                args.iter().for_each(|arg| {
                    let str = String::convert(arg.to_string()).unwrap();
                    print!("{}", str);
                });

                stdout().flush()
                    .expect("Unable to flush");

                let mut line = String::new();
                let stdin = io::stdin();
                stdin.lock().read_line(&mut line).unwrap();

                // Remove last character
                line.pop();
                
                Value::String(line)
            }
            "println" => {
                args.iter().for_each(|arg| {
                    let str = String::convert(arg.to_string()).unwrap();
                    print!("{}", str);
                });
                println!("");

                Value::Empty
            },
            "str_concat" => {
                let mut concat_str = String::from("");

                args.iter().for_each(|arg| {
                    let str = String::convert(arg.to_string()).unwrap();
                    concat_str.push_str(&str);
                });

                return Value::String(concat_str);
            },
            "sin" => {
                let number = args.get(0).unwrap();
                let number = f32::convert(number.to_number()).unwrap();
                return Value::Float(f32::sin(number));
            }
            "cos" => {
                let number = args.get(0).unwrap();
                let number = f32::convert(number.to_number()).unwrap();
                return Value::Float(f32::cos(number));
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
            panic!("Unrecognized node");
        }
    }

}