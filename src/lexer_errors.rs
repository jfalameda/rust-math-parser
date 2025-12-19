use std::fmt;

#[derive(Debug, Clone)]
pub enum LexerInvalidTokenKind {
    MalformedNumberLiteral(String),
    UnexpectedToken(String),
}

#[derive(Debug, Clone)]
pub struct LexerInvalidTokenError {
    pub kind: LexerInvalidTokenKind,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for LexerInvalidTokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerInvalidTokenKind::MalformedNumberLiteral(lit) => {
                write!(f, "Malformed number literal: '{}'", lit)
            }
            LexerInvalidTokenKind::UnexpectedToken(c) => {
                write!(f, "Syntax error: unexpected token '{}'", c)
            }
        }
    }
}

impl fmt::Display for LexerInvalidTokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lexer error at line {}, column {}: {}",
            self.line, self.column, self.kind
        )
    }
}

impl std::error::Error for LexerInvalidTokenError {}
