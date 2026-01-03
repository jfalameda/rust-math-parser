mod arguments;
mod block;
mod class;
mod errors;
mod expressions;
mod functions;
mod statements;

pub use errors::{ParserError, ParserErrorKind};

use crate::lexer::{self, Token, TokenType};
use crate::ast::{build_program_node, Expression};

pub struct Parser<'a> {
    pos: usize,
    tokens: Vec<lexer::Token<'a>>,
}

pub(super) fn error_unexpected_token<'a>(
    token: &Token<'a>,
    expected_token_type: &TokenType,
) -> ParserError<'a> {
    ParserError {
        kind: ParserErrorKind::UnexpectedToken(expected_token_type.to_string(), token.clone()),
    }
}

pub(super) fn error_unrecognized_token<'a>(token: &Token<'a>) -> ParserError<'a> {
    ParserError {
        kind: ParserErrorKind::UnrecognizedToken(token.clone()),
    }
}

pub(super) fn error_eof<'a>() -> ParserError<'a> {
    ParserError {
        kind: ParserErrorKind::UnexpectedEOF,
    }
}

pub(super) fn error_unexpected_empty_value<'a>() -> ParserError<'a> {
    ParserError {
        kind: ParserErrorKind::UnexpectedEmptyValue,
    }
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<lexer::Token<'a>>) -> Self {
        Parser { pos: 0, tokens }
    }

    pub fn parse(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        Ok(build_program_node(self.parse_block()?))
    }

    pub(crate) fn peek(&self, pos: Option<usize>) -> Option<&lexer::Token<'a>> {
        self.tokens.get(pos.unwrap_or(self.pos))
    }

    pub(crate) fn peek_type_is(&self, expected: TokenType) -> bool {
        matches!(self.peek(None), Some(t) if t.token_type == expected)
    }

    pub(crate) fn digest(&mut self, expected: TokenType) -> Result<&Token<'a>, ParserError<'a>> {
        let token = self.tokens.get(self.pos).ok_or_else(error_eof)?;

        if token.token_type != expected {
            return Err(error_unexpected_token(token, &expected));
        }

        self.pos += 1;
        Ok(token)
    }
}
