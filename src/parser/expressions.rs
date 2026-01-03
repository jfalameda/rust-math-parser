use crate::lexer::{
    AdditiveOperatorSubtype, OperatorType, PostfixOperatorType, TokenType, UnaryOperatorSubtype,
};
use crate::node::{build_node, build_postfix_node, build_unary_node, Expression};

use super::{
    error_eof, error_unexpected_empty_value, error_unexpected_token, error_unrecognized_token,
    Parser, ParserError,
};

impl<'a> Parser<'a> {
    pub(crate) fn parse_expression(
        &mut self,
        precedence: i32,
    ) -> Result<Box<Expression>, ParserError<'a>> {
        let token = self.peek(None).ok_or_else(error_eof)?;

        if token.token_type == TokenType::New {
            self.parse_class_instantiation()
        } else {
            self.parse_binary_expression(precedence)
        }
    }

    pub(crate) fn parse_binary_expression(
        &mut self,
        precedence: i32,
    ) -> Result<Box<Expression>, ParserError<'a>> {
        let mut left = self.parse_term()?;

        loop {
            let op_index = self.pos;
            let op_token = match self.peek(None) {
                Some(t) if t.token_type == TokenType::Operator => t,
                _ => break,
            };

            let (op_precedence, is_right) = op_token.operator_predecende();

            if op_precedence < precedence {
                break;
            }

            self.digest(TokenType::Operator)?;

            let next_precedence = if is_right {
                op_precedence
            } else {
                op_precedence + 1
            };

            let right = self.parse_expression(next_precedence)?;
            let operator_token = self
                .tokens
                .get(op_index)
                .expect("Operator token missing after digest");
            left = build_node(operator_token, Some(left), Some(right));
        }

        Ok(left)
    }

    pub(crate) fn parse_term(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        let left = self.parse_prefix_term()?;
        self.parse_postfix_chain(left)
    }

    pub(crate) fn parse_prefix_term(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        let token = self.peek(None).ok_or_else(error_eof)?;

        match token.token_type {
            TokenType::Operator => match token.operator_type {
                Some(OperatorType::Additive(AdditiveOperatorSubtype::Sub)) => {
                    self.digest(TokenType::Operator)?;
                    let literal = self.parse_term()?;
                    Ok(build_unary_node(UnaryOperatorSubtype::Min, literal))
                }
                Some(OperatorType::Unary(UnaryOperatorSubtype::Not)) => {
                    self.digest(TokenType::Operator)?;
                    let literal = self.parse_term()?;
                    Ok(build_unary_node(UnaryOperatorSubtype::Not, literal))
                }
                Some(_) | None => Err(error_unrecognized_token(&token)),
            },
            _ => self.parse_primary(),
        }
    }

    pub(crate) fn parse_primary(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        let token_type = self
            .peek(None)
            .map(|token| &token.token_type)
            .ok_or_else(error_unexpected_empty_value)?;

        match token_type {
            TokenType::Symbol => {
                let next_is_call = matches!(
                    self.tokens.get(self.pos + 1),
                    Some(next) if next.token_type == TokenType::ParenthesisL
                );
                self.digest(TokenType::Symbol)?;
                let symbol_index = self.pos - 1;
                if next_is_call {
                    let token = self.tokens.get(symbol_index).ok_or_else(error_eof)?;
                    let method_name = token.value.ok_or_else(error_eof)?.to_string();
                    let location = token.line;
                    self.parse_function_call(method_name, location)
                } else {
                    let token = self.tokens.get(symbol_index).ok_or_else(error_eof)?;
                    Ok(build_node(token, None, None))
                }
            }
            TokenType::StringLiteral | TokenType::BooleanLiteral | TokenType::NumeralLiteral(_) => {
                Ok(self.parse_literal()?)
            }
            TokenType::ParenthesisL => {
                self.digest(TokenType::ParenthesisL)?;
                let expr = self.parse_expression(0)?;
                self.digest(TokenType::ParenthesisR)?;
                Ok(expr)
            }
            _ => Err(error_unrecognized_token(
                self.peek(None).ok_or_else(error_eof)?,
            )),
        }
    }

    pub(crate) fn parse_literal(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        let token = self.peek(None).ok_or_else(error_unexpected_empty_value)?;

        match token.token_type {
            TokenType::StringLiteral => {
                let token = self.digest(TokenType::StringLiteral)?;
                Ok(build_node(token, None, None))
            }
            TokenType::BooleanLiteral => {
                let token = self.digest(TokenType::BooleanLiteral)?;
                Ok(build_node(token, None, None))
            }
            TokenType::NumeralLiteral(numeral_type) => {
                let token = self.digest(TokenType::NumeralLiteral(numeral_type))?;
                Ok(build_node(token, None, None))
            }
            _ => Err(error_unrecognized_token(token)),
        }
    }

    pub(crate) fn parse_postfix_chain(
        &mut self,
        mut left: Box<Expression>,
    ) -> Result<Box<Expression>, ParserError<'a>> {
        while self.peek_type_is(TokenType::MemberAccess) {
            self.digest(TokenType::MemberAccess)?;

            if !self.peek_type_is(TokenType::Symbol) {
                let token = self.peek(None).ok_or_else(error_eof)?;

                return Err(error_unexpected_token(token, &TokenType::Symbol));
            }

            let right = self.parse_term()?;
            left = build_postfix_node(PostfixOperatorType::MemberAccess, left, right);
        }

        Ok(left)
    }
}
