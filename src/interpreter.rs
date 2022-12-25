use crate::node::{Node, NodeType};

pub struct Interpreter {
    ast: Option<Box<Node>>,
    variables: Vec<(String, f32)>
}

impl Interpreter {
    pub fn new(ast: Option<Box<Node>>) -> Self {
        Interpreter {
            ast,
            variables: vec![]
        }
    }

    pub fn evaluate(&mut self) {
        self.evaluate_program();
    }

    fn evaluate_program(&mut self) {
        let node_content = self.ast.as_ref().unwrap();
        let node = &self.ast.as_ref().unwrap().left_handside;
        match node_content.node_type {
            NodeType::Program => {
                self.evaluate_statement(node)
            }
            _ => panic!("Unexpected AST node")
        }
    }

    pub fn evaluate_statement(&mut self, node: &Option<Box<Node>>) {
        let node_content = node.as_ref().unwrap();
        match node_content.node_type {
            NodeType::Statement => {
                if node_content.left_handside.is_some() {
                    self.evaluate_statement(&node_content.left_handside);
                }
                if node_content.right_handside.is_some() {
                    self.evaluate_statement(&node_content.right_handside);
                }
            }
            NodeType::Assigment => {
                self.evaluate_assignment(node)
            },
            NodeType::MethodCall => {
                self.evaluate_method_call(node)
            }
            _ => panic!("Unexpected AST node")
        }
    }

    pub fn evaluate_assignment(&mut self, node: &Option<Box<Node>>) {
        let symbol_name = node.as_ref().unwrap().left_handside.as_ref().unwrap().value.to_string();
        let value = (symbol_name, node.as_ref().unwrap().right_handside.as_ref().unwrap().evaluate());
        self.variables.push(value);
    }

    pub fn evaluate_method_call(&self, node: &Option<Box<Node>>) {
        let node_content = node.as_ref().unwrap();
        let expr_result = self.evaluate_expression(&node_content.left_handside);
        match node_content.value.as_str() {
            "print" => print!("{}", expr_result),
            _ => panic!("Unrecognized method name")
        }
    }

    pub fn evaluate_expression(&self, node: &Option<Box<Node>>) -> f32 {
        let node = node.as_ref().unwrap();
        if node.node_type == NodeType::Literal {
            return node.value.parse::<f32>().unwrap();
        }
        else if node.node_type == NodeType::UnaryOperation {
            return -1.0 * node.left_handside.as_ref().unwrap().evaluate();
        }
        else {
            let left = self.evaluate_expression(&node.left_handside);
            let right = self.evaluate_expression(&node.right_handside);
            
            if node.value == "^" {
                return left.powf(right);
            }
            if node.value == "*" {
                return left * right;
            }
            if node.value == "/" {
                return left / right;
            }
            if node.value == "-" {
                return left - right;
            }
            if node.value == "+" {
                return left + right;
            }
        }
        return 0.0;
    }

}