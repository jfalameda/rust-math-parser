use crate::lexer::TokenType;
use crate::node::{build_class_declaration_node, build_class_instantiation_node, Expression};

use super::{error_eof, error_unexpected_empty_value, Parser, ParserError};

impl<'a> Parser<'a> {
    pub(crate) fn parse_class_declaration(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        self.digest(TokenType::ClassDeclaration)?;
        let identifier_token = self.digest(TokenType::Symbol)?;
        let class_name = identifier_token
            .value
            .ok_or_else(error_unexpected_empty_value)?
            .to_string();
        self.digest(TokenType::BlockStart)?;

        let mut members = vec![];
        let mut methods = vec![];

        while let Some(token) = self.peek(None) {
            if token.token_type == TokenType::Declaration {
                members.push(self.parse_declaration()?);
                self.digest(TokenType::EndOfstatement)?;
            } else if token.token_type == TokenType::FunctionDeclaration {
                methods.push(self.parse_function_declaration()?);
            } else {
                break;
            }
        }

        self.digest(TokenType::BlockEnd)?;

        Ok(build_class_declaration_node(class_name, members, methods))
    }

    pub(crate) fn parse_class_instantiation(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        self.digest(TokenType::New)?;
        let class_name_token = self.digest(TokenType::Symbol)?;
        let class_name = class_name_token.value.ok_or_else(error_eof)?.to_string();
        let class_location = class_name_token.line;
        self.digest(TokenType::ParenthesisL)?;
        let args = self.parse_method_args()?;
        self.digest(TokenType::ParenthesisR)?;

        Ok(build_class_instantiation_node(
            class_name,
            args,
            class_location,
        ))
    }
}
