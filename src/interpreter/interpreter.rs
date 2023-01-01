use crate::node::{Expression, Identifier, Literal, Operator, MethodCall, Program};
use std::collections::{HashMap};
use std::sync::Mutex;
use once_cell::sync::Lazy;

static mut F32_VARIABLES: Lazy<Mutex<HashMap<String, f32>>> = Lazy::new(|| {
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
                //self.evaluate_program();
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
            F32_VARIABLES.lock().unwrap().insert(identifier.name.to_string(), value);
        }
    }

    fn evaluate_method_call(&self, node: &MethodCall) -> f32 {

        // Prepraring for having multiple arguments
        let arg = node.arguments.get(0).unwrap();

        let expr_result = self.evaluate_expression(arg);

        return match node.identifier.name.as_str() {
            "print" => { 
                print!("{}", expr_result);
                0.0
            },
            "println" => { 
                println!("{}", expr_result);
                0.0
            },
            "sin" => f32::sin(expr_result),
            "cos" => f32::cos(expr_result),
            _ => panic!("Unrecognized method name")
        }
    }

    fn evaluate_expression(&self, node: &Expression) -> f32 {
        if let Expression::Identifier(identifier) = node {
            let value = identifier.name.to_string();
            let result: f32;
            unsafe {
                result = F32_VARIABLES.lock().unwrap().get(&value).unwrap().clone();
            }
            return result;
        }
        else if let Expression::Literal(literal) = node {
            // Preparing for having multiple types
            return match literal {
                Literal::Float(f) => f.clone()
            };
        }
        else if let Expression::MethodCall(method_call) = node {
            return self.evaluate_method_call(method_call);
        }
        else if let Expression::UnaryOperation(_, expr) = node {
            // Assuming is minus unary operator
            return -1.0 * self.evaluate_expression(expr);
        }
        else if let Expression::BinaryOperation(left, op, right) = node {

            let left = self.evaluate_expression(left);
            let right = self.evaluate_expression(right);
            
            return match op {
                Operator::Exp => left.powf(right),
                Operator::Mul => left * right,
                Operator::Div => left * right,
                Operator::Min => left - right,
                Operator::Sum => left + right,
            };
        }
        else {
            panic!("Unrecognized node");
        }
    }

}