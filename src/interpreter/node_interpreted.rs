use crate::node::{Node, NodeType};
use std::collections::{HashMap};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static mut F32_VARIABLES: Lazy<Mutex<HashMap<String, f32>>> = Lazy::new(|| {
    let m = HashMap::new();
    Mutex::new(m)
});

pub trait NodeInterpreted {
    fn evaluate(&self);

    fn evaluate_program(&self);

    fn evaluate_statement(&self);

    fn evaluate_assignment(&self);

    fn evaluate_method_call(&self, node: Rc<RefCell<&Node>>) -> f32;

    fn evaluate_expression(&self, node: Rc<RefCell<&Node>>) -> f32;
}

impl NodeInterpreted for Node {
    fn evaluate(&self) {
        let node_content = &self;
        match node_content.node_type {
            NodeType::Program => {
                self.evaluate_program();
            },
            NodeType::BinaryOperation => {
                self.evaluate_program();
            },
            NodeType::Statement | NodeType::Assigment | NodeType::MethodCall => self.evaluate_statement(),
            _ => panic!("Unexpected AST node")
        }
    }

    fn evaluate_program(&self) {
        let node_content = self.left_handside.as_ref().unwrap();

        node_content.evaluate();
    }

    fn evaluate_statement(&self) {
        match self.node_type {
            NodeType::Statement => {
                if self.left_handside.is_some() {
                    let left_ref = self.left_handside.as_ref().unwrap();
                    left_ref.evaluate();
                }
                if self.right_handside.is_some() {
                    let right_ref = self.right_handside.as_ref().unwrap();
                    right_ref.evaluate();
                }
            }
            NodeType::Assigment => {
                self.evaluate_assignment();
            },
            NodeType::MethodCall => {
                let node_ref = self;
                let rc_node = Rc::new(RefCell::new(node_ref));
                self.evaluate_method_call(rc_node.clone());
            },
            _ => panic!("Unexpected AST node")
        }
    }

    fn evaluate_assignment(&self) {
        let node = self;
        let symbol_name = node.left_handside.as_ref().unwrap().value.to_string();
        let value_node = node.right_handside.as_ref().unwrap().as_ref();
        let rc_node = Rc::new(RefCell::new(value_node));
        let value = self.evaluate_expression(rc_node.clone());
        unsafe {
            F32_VARIABLES.lock().unwrap().insert(symbol_name, value);
        }
    }

    fn evaluate_method_call(&self, node: Rc<RefCell<&Node>>) -> f32 {
        let node = node.borrow();
        let rc_left = Rc::new(RefCell::new(node.left_handside.as_ref().unwrap().as_ref()));
        let expr_result = self.evaluate_expression(rc_left.clone());
        return match node.value.as_str() {
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

    fn evaluate_expression(&self, node: Rc<RefCell<&Node>>) -> f32 {
        let node_mut = node.borrow();
        if node_mut.node_type == NodeType::Symbol {
            let value = node_mut.value.to_string();
            let result: f32;
            unsafe {
                result = F32_VARIABLES.lock().unwrap().get(&value).unwrap().clone();
            }
            return result;
        }
        else if node_mut.node_type == NodeType::Literal {
            return node_mut.value.parse::<f32>().unwrap();
        }
        else if node_mut.node_type == NodeType::MethodCall {
            return self.evaluate_method_call(node.clone());
        }
        else if node_mut.node_type == NodeType::UnaryOperation {
            let rc_left = Rc::new(RefCell::new(node_mut.left_handside.as_ref().unwrap().as_ref()));
            return -1.0 * self.evaluate_expression(rc_left.clone());
        }
        else {
            let rc_left = Rc::new(RefCell::new(node_mut.left_handside.as_ref().unwrap().as_ref()));
            let rc_right = Rc::new(RefCell::new(node_mut.right_handside.as_ref().unwrap().as_ref()));
            let left = self.evaluate_expression(rc_left.clone());
            let right = self.evaluate_expression(rc_right.clone());
            
            if node_mut.value == "^" {
                return left.powf(right);
            }
            if node_mut.value == "*" {
                return left * right;
            }
            if node_mut.value == "/" {
                return left / right;
            }
            if node_mut.value == "-" {
                return left - right;
            }
            if node_mut.value == "+" {
                return left + right;
            }
        }
        return 0.0;
    }

}