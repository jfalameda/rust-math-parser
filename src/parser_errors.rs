use std::fmt;

use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum ParserErrorKind {
    UnrecognizedToken(Token),
    UnexpectedToken(String, Token),
    UnexpectedEOF,
    UnexpectedEmptyValue
}

#[derive(Debug, Clone)]
pub struct ParserError {
    pub kind: ParserErrorKind
}

impl fmt::Display for ParserErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserErrorKind::UnrecognizedToken(token) => {
                let found = token.value.clone().unwrap_or_default();
                write!(f, "Syntax error: Unrecognized token {} at line {} and character {}", found, token.line, token.start)
            }
            ParserErrorKind::UnexpectedToken(expected, token) => {
                let found = token.value.clone().unwrap_or_default();
                write!(f, "Syntax error: Expected token {} at line {} and character {}, instead found {}", expected, token.line, token.start, found)
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

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parsing error: {}",
            self.kind
        )
    }
}

impl std::error::Error for ParserError {}