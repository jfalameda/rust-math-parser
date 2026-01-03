use std::fmt;

use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum ParserErrorKind<'a> {
    UnrecognizedToken(Token<'a>),
    UnexpectedToken(String, Token<'a>),
    UnexpectedEOF,
    UnexpectedEmptyValue,
}

#[derive(Debug, Clone)]
pub struct ParserError<'a> {
    pub kind: ParserErrorKind<'a>,
}

impl fmt::Display for ParserErrorKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserErrorKind::UnrecognizedToken(token) => {
                let found = token.value.unwrap_or_default();
                write!(
                    f,
                    "Syntax error: Unrecognized token {} at line {} and column {}",
                    found, token.line, token.column
                )
            }
            ParserErrorKind::UnexpectedToken(expected, token) => {
                let found = token.value.unwrap_or_default();
                write!(
                    f,
                    "Syntax error: Expected token {} at line {} and column {}, instead found {}",
                    expected, token.line, token.column, found
                )
            }
            ParserErrorKind::UnexpectedEOF => {
                write!(f, "Parser error: Unexpected error, no more tokens to parse")
            }
            ParserErrorKind::UnexpectedEmptyValue => {
                write!(f, "Parser error: Unexpected empty value")
            }
        }
    }
}

impl fmt::Display for ParserError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parsing error: {}", self.kind)
    }
}

impl<'a> std::error::Error for ParserError<'a> {}
