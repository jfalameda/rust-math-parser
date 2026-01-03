use crate::lexer::TokenType;
use crate::node::{build_statement_node, Block};

use super::{Parser, ParserError};

impl<'a> Parser<'a> {
    pub(crate) fn parse_block_with_delimiters(&mut self) -> Result<Block, ParserError<'a>> {
        self.digest(TokenType::BlockStart)?;
        let block = self.parse_block()?;
        self.digest(TokenType::BlockEnd)?;

        Ok(block)
    }

    pub(crate) fn parse_block(&mut self) -> Result<Block, ParserError<'a>> {
        let mut body = vec![];

        while let Some(token) = self.peek(None) {
            if token.token_type == TokenType::Eof {
                self.digest(TokenType::Eof)?; // consume EOF
                break;
            }
            if token.token_type == TokenType::BlockEnd {
                break;
            }

            let stmt = self.parse_statement()?;
            self.consume_statement_terminator(stmt.as_ref())?;

            body.push(build_statement_node(stmt));
        }

        Ok(body)
    }

    pub(crate) fn parse_statement_or_block(&mut self) -> Result<Block, ParserError<'a>> {
        if self.peek_type_is(TokenType::BlockStart) {
            let if_block = self.parse_block_with_delimiters()?;

            Ok(if_block)
        } else {
            let stmt = self.parse_statement()?;
            self.consume_statement_terminator(stmt.as_ref())?;
            Ok(vec![build_statement_node(stmt)])
        }
    }
}
