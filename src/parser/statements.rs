use crate::lexer::TokenType;
use crate::node::{build_assignment_node, build_conditional_node, build_return_node, Expression};

use super::{error_eof, error_unrecognized_token, Parser, ParserError};

impl<'a> Parser<'a> {
    pub(crate) fn consume_statement_terminator(
        &mut self,
        stmt: &Expression,
    ) -> Result<(), ParserError<'a>> {
        match stmt {
            Expression::IfConditional(_, _, _)
            | Expression::FunctionDeclaration(_)
            | Expression::ClassDeclaration(_) => Ok(()),
            _ => {
                self.digest(TokenType::EndOfstatement)?;
                Ok(())
            }
        }
    }

    pub(crate) fn parse_statement(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        let token = self.peek(None).ok_or_else(error_eof)?;

        let statement = match token.token_type {
            TokenType::NumeralLiteral(_)
            | TokenType::BooleanLiteral
            | TokenType::Operator
            | TokenType::Symbol
            | TokenType::StringLiteral
            | TokenType::ParenthesisL => Ok(self.parse_expression(0)?),
            TokenType::Declaration => Ok(self.parse_declaration()?),
            TokenType::FunctionDeclaration => Ok(self.parse_function_declaration()?),
            TokenType::ClassDeclaration => Ok(self.parse_class_declaration()?),
            TokenType::ConditionalIf => Ok(self.parse_conditional()?),
            TokenType::Return => Ok(self.parse_return()?),
            _ => Err(error_unrecognized_token(token)),
        }?;

        Ok(statement)
    }

    pub(crate) fn parse_declaration(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        self.digest(TokenType::Declaration)?;
        let symbol_token = self.digest(TokenType::Symbol)?;
        let symbol_name = symbol_token.value.ok_or_else(error_eof)?.to_string();
        self.digest(TokenType::Assignment)?;
        let expr = self.parse_expression(0)?;
        Ok(build_assignment_node(symbol_name, expr))
    }

    pub(crate) fn parse_return(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        self.digest(TokenType::Return)?;
        Ok(build_return_node(self.parse_expression(0)?))
    }

    pub(crate) fn parse_conditional(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        self.digest(TokenType::ConditionalIf)?;
        self.digest(TokenType::ParenthesisL)?;
        let expr = self.parse_expression(0)?;
        self.digest(TokenType::ParenthesisR)?;

        let if_block = self.parse_statement_or_block()?;

        let else_block = if self.peek_type_is(TokenType::ConditionalElse) {
            self.digest(TokenType::ConditionalElse)?;
            Some(self.parse_statement_or_block()?)
        } else {
            None
        };

        Ok(build_conditional_node(expr, if_block, else_block))
    }
}
