use crate::interpreter::{
    execution_context::ExecutionContext,
    runtime_errors::{RuntimeError},
};
use crate::lexer::{AdditiveOperatorSubtype, CompOperatorSubtype, MultiplicativeOperatorSubtype, OperatorType, UnaryOperatorSubtype};
use crate::node::{Block, Expression, FunctionDeclaration, Identifier, Literal, MethodCall, Program};
use super::methods::get_method;
use super::value::{Value};

pub struct Interpreter {
    execution_context: ExecutionContext,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            execution_context: ExecutionContext::new(),
        }
    }

    pub fn evaluate(&mut self, node: Option<&Box<Expression>>) -> Result<(), RuntimeError> {
        if let Some(node_content) = node {
            match node_content.as_ref() {
                Expression::Program(program) => self.evaluate_program(program)?,
                Expression::BinaryOperation(_, _, _) => { self.evaluate_expression(node_content)?; },
                Expression::Statement(_) 
                | Expression::Declaration(_, _) 
                | Expression::MethodCall(_) => self.evaluate_statement(node_content)?,
                Expression::IfConditional(expression, if_block, else_block) => {
                    self.evaluate_conditional(expression, if_block, else_block)?
                },
                Expression::Return(_) => self.evaluate_return(node_content)?,
                Expression::FunctionDeclaration(function_declaration) => {
                    self.evaluate_function_definition(function_declaration)?
                },
                _ => panic!("Unexpected AST node"),
            }
        }
        Ok(())
    }

    fn evaluate_program(&mut self, program: &Program) -> Result<(), RuntimeError> {
        let statements = &program.body;
        self.evaluate_block(statements)
    }

    fn evaluate_return(&mut self, expression: &Expression) -> Result<(), RuntimeError> {
        if self.execution_context.is_in_function() {
            if let Expression::Return(inner_expression) = expression {
                let value = self.evaluate_expression(inner_expression)?;
                self.execution_context.set_return_value(value);
            } else {
                return Err(self.error_with_stack("Expected a return expression"));
            }
        } else {
            return Err(self.error_with_stack("Attempting to return outside a function block"));
        }
        Ok(())
    }

    fn evaluate_block(&mut self, block: &Block) -> Result<(), RuntimeError> {
        let (parent_scope, _) = self.execution_context.enter_new_scope();
        for statement in block {
            self.evaluate(Some(statement))?;
        }
        self.execution_context.restore_scope(parent_scope);
        Ok(())
    }

    fn evaluate_statement(&mut self, expression: &Expression) -> Result<(), RuntimeError> {
        match expression {
            Expression::Statement(expr) => self.evaluate(Some(expr))?,
            Expression::Declaration(identifier, expr) => self.evaluate_assignment(identifier, expr)?,
            Expression::MethodCall(method_call) => { self.evaluate_method_call(method_call)?; },
            _ => return Err(self.error_with_stack("Unexpected AST node")),
        }
        Ok(())
    }

    fn evaluate_conditional(&mut self, expression: &Expression, if_block: &Block, else_block: &Option<Block>) -> Result<(), RuntimeError> {
        let expression_result = self.evaluate_expression(expression)?;
        if expression_result.to_bool() {
            self.evaluate_block(&if_block)?;
        } else if let Some(else_block) = else_block {
            self.evaluate_block(&else_block)?;
        }
        Ok(())
    }

    fn evaluate_assignment(&mut self, identifier: &Identifier, expression: &Expression) -> Result<(), RuntimeError> {
        let value = self.evaluate_expression(expression)?;
        self.execution_context.define_variable_in_scope(&identifier.name, value)?;
        Ok(())
    }

    fn evaluate_function_definition(&mut self, node: &FunctionDeclaration) -> Result<(), RuntimeError> {
        self.execution_context.define_function_in_scope(&node.identifier.name, node.clone())?;
        Ok(())
    }

    fn evaluate_method_call(&mut self, node: &MethodCall) -> Result<Value, RuntimeError> {
        let method_name = &node.identifier.name;

        let (function_opt, arg_exprs) = {
            let func = self.execution_context.lookup_function_in_scope(method_name);
            let args = node.arguments.clone();
            (func, args)
        };

        if let Some(function) = function_opt {
            let param_names = function.arguments;
            let evaluated_args = self.evaluate_arguments(&arg_exprs)?;

            // Validate arity
            if param_names.len() != evaluated_args.len() {
                return Err(self.error_with_stack(&format!(
                        "Function '{}' expected {} arguments, got {}",
                        method_name,
                        param_names.len(),
                        evaluated_args.len()
                    ))
                );
            }

            // Enter a new scope for the function
            let (parent_scope, _) = self.execution_context.enter_new_scope();

            // Inject arguments as local variables
            for (param, value) in param_names.into_iter().zip(evaluated_args.into_iter()) {
                self.execution_context.define_variable_in_scope(&param.name, value)?;
            }

            self.execution_context.push_frame(method_name.clone(),Some(node.location));
            self.execution_context.enter_function();

            // Execute function block
            self.evaluate_block(&function.block)?;

            // Get return value
            let return_value = self
                .execution_context
                .exit_function_with_return()
                .unwrap_or(Value::Integer(0));

            // Restore previous scope
            self.execution_context.pop_frame();
            self.execution_context.restore_scope(parent_scope);

            Ok(return_value)
        } else {
            // If function not found in scope, attempt to call built-in method
            let args = self.evaluate_arguments(&node.arguments.clone())?;
            let result = get_method(method_name.clone(), args);
            result.map_err(|err| self.execution_context.attach_stack(err))
        }
    }


    fn evaluate_arguments(&mut self, args: &Vec<Box<Expression>>) -> Result<Vec<Value>, RuntimeError> {
        let mut results = Vec::with_capacity(args.len());
        for expr in args {
            results.push(self.evaluate_expression(expr)?);
        }
        Ok(results)
    }

    fn evaluate_expression(&mut self, node: &Expression) -> Result<Value, RuntimeError> {
        match node {
            Expression::Identifier(identifier) => {
                let identifier = identifier.name.clone();
                let result = self.execution_context.lookup_variable_in_scope(&identifier);

                // Cloning variable. Considering a way to pass the reference so that cloning is
                // not necessary. Variables should not be cloned.
                result
                    .cloned()
                    .ok_or_else(|| self.error_with_stack(&format!("Undefined variable {}", identifier)))
            },
            Expression::Literal(literal) => Ok(match literal {
                Literal::Boolean(b) => Value::Boolean(*b),
                Literal::Integer(i) => Value::Integer(*i),
                Literal::Float(f) => Value::Float(*f),
                Literal::String(s) => Value::String(s.clone()), // Cheap Rc clone
            }),
            Expression::MethodCall(method_call) => self.evaluate_method_call(method_call),
            Expression::UnaryOperation(operator, expr) => {
                let val = self.evaluate_expression(expr)?;
                match operator {
                    OperatorType::Unary(UnaryOperatorSubtype::Min) => Ok(Value::Float(-1.0) * val),
                    OperatorType::Unary(UnaryOperatorSubtype::Not) => {
                        let bool_value = val.to_bool();
                        Ok(Value::Boolean(!bool_value))
                    }
                    _ => unreachable!(),
                }
            },
            Expression::BinaryOperation(left, op, right) => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;

                // String concatenation
                if matches!(left_val, Value::String(_)) || matches!(right_val, Value::String(_)) {
                    if matches!(op, OperatorType::Additive(AdditiveOperatorSubtype::Add)) {
                        // Convert left_val to String
                        let left_str = left_val.to_string();

                        // Return as Value::String(Rc<str>)
                        return Ok(left_str + right_val);
                    }
                }

                let res = match op {
                    OperatorType::Exponential => left_val.power(right_val),
                    OperatorType::Multiplicative(MultiplicativeOperatorSubtype::Mul) => left_val * right_val,
                    OperatorType::Multiplicative(MultiplicativeOperatorSubtype::Div) => left_val / right_val,
                    OperatorType::Additive(AdditiveOperatorSubtype::Sub) => left_val - right_val,
                    OperatorType::Additive(AdditiveOperatorSubtype::Add) => left_val + right_val,
                    OperatorType::Comp(comp_type) => match comp_type {
                        CompOperatorSubtype::Eq => left_val.eq_value(&right_val),
                        CompOperatorSubtype::Neq => left_val.neq_value(&right_val),
                        CompOperatorSubtype::Gt => left_val.gt_value(&right_val),
                        CompOperatorSubtype::Lt => left_val.lt_value(&right_val),
                        CompOperatorSubtype::Gte => left_val.gte_value(&right_val),
                        CompOperatorSubtype::Lte => left_val.lte_value(&right_val),
                    },
                    OperatorType::Unary(_) => {
                        return Err(self.error_with_stack("Unary operation unexpected"));
                    }
                };
                Ok(res)
            },
            _ => Err(self.error_with_stack(&format!("Unrecognized node {:?}", node))),
        }
    }

    fn error_with_stack(&mut self, msg: &str) -> RuntimeError {
        self.execution_context.attach_stack(
            RuntimeError::new(msg)
        )
    }
}
