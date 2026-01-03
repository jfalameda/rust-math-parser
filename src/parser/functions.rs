use crate::lexer::TokenType;
use crate::node::{build_function_call_node, build_function_declaration_node, Expression};

use super::{error_eof, error_unexpected_empty_value, Parser, ParserError};

impl<'a> Parser<'a> {
    pub(crate) fn parse_function_declaration(
        &mut self,
    ) -> Result<Box<Expression>, ParserError<'a>> {
        self.digest(TokenType::FunctionDeclaration)?;

        let identifier_token = self.digest(TokenType::Symbol)?;
        let identifier = identifier_token
            .value
            .ok_or_else(error_unexpected_empty_value)?
            .to_string();

        self.digest(TokenType::ParenthesisL)?;

        let mut args = vec![];

        while let Some(token) = self.peek(None) {
            if token.token_type == TokenType::ParenthesisR {
                break;
            }

            args.push(
                self.digest(TokenType::Symbol)?
                    .value
                    .ok_or_else(error_unexpected_empty_value)?
                    .to_string(),
            );

            if let Some(next) = self.peek(None) {
                if next.token_type != TokenType::ParenthesisR {
                    self.digest(TokenType::ArgumentSeparator)?;
                }
            } else {
                return Err(error_eof());
            }
        }

        self.digest(TokenType::ParenthesisR)?;

        let block = self.parse_block_with_delimiters()?;

        Ok(build_function_declaration_node(identifier, args, block))
    }

    pub(crate) fn parse_function_call(
        &mut self,
        method_name: String,
        location: usize,
    ) -> Result<Box<Expression>, ParserError<'a>> {
        self.digest(TokenType::ParenthesisL)?;
        let args = self.parse_method_args()?;
        self.digest(TokenType::ParenthesisR)?;

        Ok(build_function_call_node(method_name, args, location))
    }

    pub(crate) fn parse_method_args(&mut self) -> Result<Vec<Expression>, ParserError<'a>> {
        let mut args = vec![];

        while let Some(token) = self.peek(None) {
            if token.token_type == TokenType::ParenthesisR {
                break;
            }

            args.push(*self.parse_expression(0)?);

            if let Some(next) = self.peek(None) {
                if next.token_type != TokenType::ParenthesisR {
                    self.digest(TokenType::ArgumentSeparator)?;
                }
            } else {
                return Err(error_eof());
            }
        }

        Ok(args)
    }
}
