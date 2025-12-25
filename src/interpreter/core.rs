use std::rc::Rc;

use super::methods::get_method;
use super::value::Value;
use crate::interpreter::{execution_context::ExecutionContext, runtime_errors::RuntimeError};
use crate::lexer::{
    AdditiveOperatorSubtype, BooleanOperatorSubtype, CompOperatorSubtype, MultiplicativeOperatorSubtype, OperatorType, UnaryOperatorSubtype
};
use crate::node::{
    Block, Expression, FunctionDeclaration, Identifier, Literal, FunctionCall, Program,
};
pub enum ControlFlow {
    Continue,
    Break
}

pub struct Interpreter {
    execution_context: ExecutionContext,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            execution_context: ExecutionContext::new(),
        }
    }

    pub fn run(&mut self, node: Option<&Expression>) -> Result<(), RuntimeError> {
        self.evaluate(node)?;
        Ok(())
    }

    pub fn evaluate(&mut self, node: Option<&Expression>) -> Result<ControlFlow, RuntimeError> {
        if let Some(node_content) = node {
            match node_content {
                Expression::Program(program) => {
                    Ok(self.evaluate_program(program)?)
                },
                Expression::BinaryOperation(_, _, _) => {
                    self.evaluate_expression(node_content)?;
                    Ok(ControlFlow::Continue)
                }
                Expression::Statement(_)
                | Expression::Declaration(_, _)
                | Expression::FunctionCall(_) => Ok(self.evaluate_statement(node_content)?),
                Expression::IfConditional(expression, if_block, else_block) => {
                    self.evaluate_conditional(expression, if_block, else_block)?;
                    Ok(ControlFlow::Continue)
                }
                Expression::Return(_) => {
                    self.evaluate_return(node_content)?;
                    
                    return Ok(ControlFlow::Break);
                },
                Expression::FunctionDeclaration(function_declaration) => {
                    self.evaluate_function_definition(function_declaration)?;
                    Ok(ControlFlow::Continue)
                }
                _ => panic!("Unexpected AST node"),
            }
        }
        else {
            // When the program is finished the flow breaks.
            Ok(ControlFlow::Break)
        }
    }

    fn evaluate_program(&mut self, program: &Program) -> Result<ControlFlow, RuntimeError> {
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

    fn evaluate_block(&mut self, block: &Block) -> Result<ControlFlow, RuntimeError> {
        let (parent_scope, _) = self.execution_context.enter_new_scope();
        let mut break_invoked = false;
        for statement in block {
            let statement = self.evaluate(Some(statement))?;
            match statement {
                ControlFlow::Break => {
                    break_invoked = true;
                    break;
                },
                ControlFlow::Continue => ()
            }
        }
        self.execution_context.restore_scope(parent_scope);
        Ok(if break_invoked { ControlFlow::Break } else { ControlFlow::Continue })
    }

    fn evaluate_statement(&mut self, expression: &Expression) -> Result<ControlFlow, RuntimeError> {
        match expression {
            Expression::Statement(expr) => {
                let eval = self.evaluate(Some(expr.as_ref()))?;
                Ok(eval)
            }
            Expression::Declaration(identifier, expr) => {
                self.evaluate_assignment(identifier, expr)?;
                Ok(ControlFlow::Continue)
            }
            Expression::FunctionCall(method_call) => {
                self.evaluate_function_call(method_call)?;
                Ok(ControlFlow::Continue)
            }
            _ => return Err(self.error_with_stack("Unexpected AST node")),
        }
    }

    fn evaluate_conditional(
        &mut self,
        expression: &Expression,
        if_block: &Block,
        else_block: &Option<Block>,
    ) -> Result<(), RuntimeError> {
        let expression_result = self.evaluate_expression(expression)?;
        if expression_result.to_bool() {
            self.evaluate_block(if_block)?;
        } else if let Some(else_block) = else_block {
            self.evaluate_block(else_block)?;
        }
        Ok(())
    }

    fn evaluate_assignment(
        &mut self,
        identifier: &Identifier,
        expression: &Expression,
    ) -> Result<(), RuntimeError> {
        let value = self.evaluate_expression(expression)?;
        self.execution_context
            .define_variable_in_scope(&identifier.name, value)?;
        Ok(())
    }

    fn evaluate_function_definition(
        &mut self,
        node: &FunctionDeclaration,
    ) -> Result<(), RuntimeError> {
        self.execution_context
            .define_function_in_scope(&node.identifier.name, node.clone())?;
        Ok(())
    }

    fn evaluate_function_call(&mut self, node: &FunctionCall) -> Result<Rc<Value>, RuntimeError> {
        let method_name = &node.identifier.name;
        if let Some(function) = self.execution_context.lookup_function_in_scope(method_name) {
            let FunctionDeclaration {
                arguments: param_names,
                block,
                ..
            } = function;
            let evaluated_args = self.evaluate_arguments(&node.arguments)?;

            if param_names.len() != evaluated_args.len() {
                return Err(self.error_with_stack(&format!(
                    "Function '{}' expected {} arguments, got {}",
                    method_name,
                    param_names.len(),
                    evaluated_args.len()
                )));
            }

            let (parent_scope, _) = self.execution_context.enter_new_scope();

            // Function arguments are not passed at reference. cloning values.
            for (param, value) in param_names.into_iter().zip(evaluated_args.into_iter()) {
                self.execution_context
                    .define_variable_in_scope(&param.name, value.as_ref().clone().into_rc())?;
            }

            self.execution_context
                .push_frame(method_name.clone(), Some(node.location));
            self.execution_context.enter_function();

            self.evaluate_block(&block)?;

            let return_value = self
                .execution_context
                .exit_function_with_return()
                .unwrap_or(Value::Integer(0));

            self.execution_context.pop_frame();
            self.execution_context.restore_scope(parent_scope);

            Ok(return_value.into_rc())
        } else {
            self.execution_context
                .push_frame(method_name.clone(), Some(node.location));

            let args = self.evaluate_arguments(&node.arguments)?;
            let result = get_method(method_name.clone(), args);

            self.execution_context.pop_frame();
            
            result.map_err(|err| self.execution_context.attach_stack(err))
        }
    }

    fn evaluate_arguments(&mut self, args: &[Expression]) -> Result<Vec<Rc<Value>>, RuntimeError> {
        let mut results = Vec::with_capacity(args.len());
        for expr in args {
            results.push(self.evaluate_expression(expr)?);
        }
        Ok(results)
    }

    fn evaluate_expression(&mut self, node: &Expression) -> Result<Rc<Value>, RuntimeError> {
        match node {
            Expression::Identifier(identifier) => {
                let identifier = identifier.name.clone();
                let result = self.execution_context.lookup_variable_in_scope(&identifier);

                // Cloning variable. Considering a way to pass the reference so that cloning is
                // not necessary. Variables should not be cloned.
                result.ok_or_else(|| {
                    self.error_with_stack(&format!("Undefined variable {}", identifier))
                })
            }
            Expression::Literal(literal) => Ok(match literal {
                Literal::Boolean(b) => Value::Boolean(*b).into_rc(),
                Literal::Integer(i) => Value::Integer(*i).into_rc(),
                Literal::Float(f) => Value::Float(*f).into_rc(),
                Literal::String(s) => Value::String(s.clone()).into_rc(), // Cheap Rc clone
            }),
            Expression::FunctionCall(method_call) => self.evaluate_function_call(method_call),
            Expression::UnaryOperation(operator, expr) => {
                let val = self.evaluate_expression(expr)?;
                match operator {
                    OperatorType::Unary(UnaryOperatorSubtype::Min) => {
                        Ok(Value::Float(-1.0).mul_value(val.as_ref()).into_rc())
                    }
                    OperatorType::Unary(UnaryOperatorSubtype::Not) => {
                        let bool_value = val.to_bool();
                        Ok(Value::Boolean(!bool_value).into_rc())
                    }
                    _ => unreachable!(),
                }
            }
            Expression::BinaryOperation(left, op, right) => {
                let left_val = self.evaluate_expression(left)?;

                // Evaluate lazily
                if let OperatorType::Boolean(BooleanOperatorSubtype::And) = op {
                    if !left_val.to_bool() {
                        return Ok(Value::Boolean(false).into_rc());
                    }
                    let right_val = self.evaluate_expression(right)?;
                    return Ok(left_val.and_value(&right_val).into_rc());
                }

                if let OperatorType::Boolean(BooleanOperatorSubtype::Or) = op {
                    if left_val.to_bool() {
                        return Ok(Value::Boolean(true).into_rc());
                    }
                    let right_val = self.evaluate_expression(right)?;
                    return Ok(left_val.or_value(&right_val).into_rc());
                }

                let right_val = self.evaluate_expression(right)?;

                let res = match op {
                    OperatorType::Exponential => left_val.power(right_val.as_ref()),
                    OperatorType::Multiplicative(MultiplicativeOperatorSubtype::Mul) => {
                        left_val.mul_value(right_val.as_ref())
                    }
                    OperatorType::Multiplicative(MultiplicativeOperatorSubtype::Div) => {
                        left_val.div_value(right_val.as_ref())
                    }
                    OperatorType::Additive(AdditiveOperatorSubtype::Sub) => {
                        left_val.sub_value(right_val.as_ref())
                    }
                    OperatorType::Additive(AdditiveOperatorSubtype::Add) => {
                        left_val.add_value(right_val.as_ref())
                    }
                    OperatorType::Comp(comp_type) => match comp_type {
                        CompOperatorSubtype::Eq => left_val.eq_value(&right_val),
                        CompOperatorSubtype::Neq => left_val.neq_value(&right_val),
                        CompOperatorSubtype::Gt => left_val.gt_value(&right_val),
                        CompOperatorSubtype::Lt => left_val.lt_value(&right_val),
                        CompOperatorSubtype::Gte => left_val.gte_value(&right_val),
                        CompOperatorSubtype::Lte => left_val.lte_value(&right_val),
                    },
                    OperatorType::Boolean(_) => unreachable!(),
                    OperatorType::Unary(_) => {
                        return Err(self.error_with_stack("Unary operation unexpected"));
                    }
                };
                Ok(res.into_rc())
            }
            _ => Err(self.error_with_stack(&format!("Unrecognized node {:?}", node))),
        }
    }

    fn error_with_stack(&mut self, msg: &str) -> RuntimeError {
        self.execution_context.attach_stack(RuntimeError::new(msg))
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
