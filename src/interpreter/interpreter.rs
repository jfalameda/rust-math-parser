use crate::error::error;
use crate::interpreter::scope::{ScopeArena, ScopeId};
use crate::lexer::{AdditiveOperatorSubtype, CompOperatorSubtype, MultiplicativeOperatorSubtype, OperatorType, UnaryOperatorSubtype};
use crate::node::{Block, Expression, FunctionDeclaration, Identifier, Literal, MethodCall, Program};

use super::methods::get_method;
use super::value::{Value, Convert};

pub struct ExecutionContext {
    in_function: bool,
    returned_value: Option<Value>,
}

impl ExecutionContext {
    pub fn enter_function(&mut self) {
        self.in_function = true;
    }

    pub fn exit_with_return(&mut self) -> Option<Value> {
        let return_value = self.returned_value.clone();
        self.returned_value = None;
        self.in_function = false;

        return_value
    }
}

pub struct Interpreter {
    scope_arena: ScopeArena,
    current_scope: ScopeId,

    // TODO: Quick solution. Refactor later.
    execution_context: ExecutionContext
}


impl Interpreter {
    pub fn new() -> Self {
        let mut scope_arena = ScopeArena::new();
        let current_scope = scope_arena.new_scope(None);

        let execution_context = ExecutionContext {
            in_function: false,
            returned_value: None
        };

        Interpreter {
            scope_arena,
            current_scope,
            execution_context
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
                Expression::Return(_) => self.evaluate_return(node_content),
                Expression::FunctionDeclaration(function_declaration) => {
                    self.evaluate_function_definition(function_declaration);
                },
                _ => error("Unexpected AST node."),
            }
        }
    }

    fn evaluate_program(&mut self, program: &Program) {
        let statements = &program.body;

        self.evaluate_block(statements);
    }

    fn evaluate_return(&mut self, expression: &Expression) {
        if self.execution_context.in_function {
            if let Expression::Return(inner_expression) = expression {
                // Evaluate the inner expression and store the result
                self.execution_context.returned_value = Some(self.evaluate_expression(inner_expression));
            } else {
                error("Expected a return expression.");
            }
        } else {
            error("Attempting to return outside a function block.");
        }
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
            _ => error("Unexpected AST node")
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
        self.scope_arena.define_variable(self.current_scope, identifier.name.to_string(), value);
    }

    fn evaluate_function_definition(&mut self, node: &FunctionDeclaration) {
        self.scope_arena.define_function(self.current_scope, node.identifier.name.clone(), node.clone());
    }

    fn evaluate_method_call(&mut self, node: &MethodCall) -> Value {
        let method_name = &node.identifier.name;

        let (function_opt, arg_exprs) = {
            let func = self.scope_arena.lookup_function(self.current_scope, method_name).cloned();
            let args = node.arguments.clone();
            (func, args)
        };

        if let Some(function) = function_opt {
            let param_names = function.arguments;

            // Evaluate arguments now (mutable borrow allowed)
            let evaluated_args = self.evaluate_arguments(&arg_exprs);

            // Validate arity
            // TODO: Create separate function
            if param_names.len() != evaluated_args.len() {
                error(&format!(
                    "Function '{}' expected {} arguments, got {}",
                    method_name,
                    param_names.len(),
                    evaluated_args.len()
                ));
            }

            // Create a new scope for the function call
            // TODO: Consider wrapper to create scopes
            let parent_scope = self.current_scope;
            let function_scope = self.scope_arena.new_scope(Some(parent_scope));
            self.current_scope = function_scope;

            // Inject arguments as local variables
            for (param, value) in param_names.into_iter().zip(evaluated_args.into_iter()) {
                self.scope_arena.define_variable(self.current_scope, param.name, value);
            }

            self.execution_context.enter_function();

            // Evaluate function body in the new scope
            self.evaluate_block(&function.block);


            self.current_scope = parent_scope;

            self.execution_context
                .exit_with_return()
                .unwrap_or(Value::Integer(0))
        } else {
            let args = self.evaluate_arguments(&node.arguments.clone());
            get_method(method_name.clone(), args)
        }
    }

    fn evaluate_arguments(&mut self, args: &Vec<Box<Expression>>) -> Vec<Value> {
        let args : Vec<Value> = args
                .iter()
                .map(|expr| self.evaluate_expression(expr))
                .collect();
        return args;
    }

    fn evaluate_expression(&mut self, node: &Expression) -> Value {
        if let Expression::Identifier(identifier) = node {
            let identifier = identifier.name.to_string();
            let result = self.scope_arena.lookup_variable(self.current_scope, &identifier);

            // Do we need to clone? What is the cost of it
            // TODO: Improve code
            return result.unwrap_or_else(|| error(format!("Undefined variable {}", identifier).as_str())).clone();
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
                OperatorType::Unary(UnaryOperatorSubtype::Min)=> Value::Float(-1.0) * self.evaluate_expression(expr),
                OperatorType::Unary(UnaryOperatorSubtype::Not)=> Value::Boolean(false) * self.evaluate_expression(expr),
                _ => error("Unexpected operator")
            }
        }
        else if let Expression::BinaryOperation(left, op, right) = node {

            let left = self.evaluate_expression(left);
            let right = self.evaluate_expression(right);
    
            // TODO: Is this needed now? I think the Value string ops covers it
            if matches!(left, Value::String(_)) || matches!(right, Value::String(_)) {
                if matches!(op, OperatorType::Additive(AdditiveOperatorSubtype::Add)) {
                    let mut left_str = String::convert(left.to_string()).unwrap();
                    let right = String::convert(right.to_string()).unwrap();
                    left_str.push_str(&right);
                    return Value::String(left_str);
                }
            }
            
            return match op {
                OperatorType::Exponential => left.power(right),
                OperatorType::Multiplicative(MultiplicativeOperatorSubtype::Mul) => left * right,
                OperatorType::Multiplicative(MultiplicativeOperatorSubtype::Div) => left / right,
                OperatorType::Additive(AdditiveOperatorSubtype::Sub) => left - right,
                OperatorType::Additive(AdditiveOperatorSubtype::Add) => left + right,
                OperatorType::Comp(comp_type)  => {
                    match comp_type {
                        CompOperatorSubtype::Eq => left.eq_value(&right),
                        CompOperatorSubtype::Neq => left.neq_value(&right),
                        CompOperatorSubtype::Gt => left.gt_value(&right),
                        CompOperatorSubtype::Lt => left.lt_value(&right),
                        CompOperatorSubtype::Gte => left.gte_value(&right),
                        CompOperatorSubtype::Lte => left.lte_value(&right),
                    }
                }
                OperatorType::Unary(_) => error("Unary operatios unexpected")
            };
        }
        else {
            error(format!("Unrecognized node {:?}", node).as_str());
        }
    }

}