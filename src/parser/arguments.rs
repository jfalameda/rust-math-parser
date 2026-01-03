use crate::lexer::TokenType;
use crate::ast::Expression;

use super::{error_eof, Parser, ParserError};

impl<'a> Parser<'a> {
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
