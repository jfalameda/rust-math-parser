use crate::lexer_errors::{LexerInvalidTokenError, LexerInvalidTokenKind};

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum NumeralType {
    Integer,
    Float,
}

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    Operator,
    UnaryOperator,
    NumeralLiteral(NumeralType),
    BooleanLiteral,
    ParenthesisL,
    ParenthesisR,
    Declaration,
    Symbol,
    Assignment,
    EndOfstatement,
    ArgumentSeparator,
    StringLiteral,
    ConditionalIf,
    ConditionalElse,
    BlockStart,
    BlockEnd,
    Eof,
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            TokenType::Operator => "Operator",
            TokenType::UnaryOperator => "UnaryOperator",
            TokenType::NumeralLiteral(_) => "NumeralLiteral",
            TokenType::BooleanLiteral => "BooleanLiteral",
            TokenType::ParenthesisL => "ParenthesisL",
            TokenType::ParenthesisR => "ParenthesisR",
            TokenType::Declaration => "Declaration",
            TokenType::Symbol => "Symbol",
            TokenType::Assignment => "Assignment",
            TokenType::EndOfstatement => "EndOfStatement",
            TokenType::ArgumentSeparator => "ArgumentSeparator",
            TokenType::StringLiteral => "StringLiteral",
            TokenType::ConditionalIf => "ConditionalIf",
            TokenType::ConditionalElse => "ConditionalElse",
            TokenType::BlockStart => "BlockStart",
            TokenType::BlockEnd => "BlockEnd",
            TokenType::Eof => "Eof",
        }
        .to_string()
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum OperatorType {
    Additive,
    Multiplicative,
    Exponential,
    Eq,
    Neq,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub token_type: TokenType,
    pub operator_type: Option<OperatorType>,
    pub value: Option<String>,
}

impl Token {
    pub fn operator_predecende(self) -> (i32, bool) {
        match self.operator_type {
            Some(OperatorType::Additive) => (1, false),
            Some(OperatorType::Multiplicative) => (2, false),
            Some(OperatorType::Exponential) => (3, true),
            Some(OperatorType::Eq | OperatorType::Neq) => (4, false),
            None => (1, false),
        }
    }
}

pub struct TokenParser {
    pos: usize,    // byte offset
    column: usize,
    line: usize,
    program: String,
}

impl TokenParser {
    pub fn new(program: String) -> Self {
        Self {
            pos: 0,
            column: 1,
            line: 1,
            program,
        }
    }

    fn peek(&self) -> Option<char> {
        self.program[self.pos..].chars().next()
    }

    fn peek_with_offset(&self, n: usize) -> Option<char> {
        self.program[self.pos..].chars().nth(n)
    }

    fn digest(&mut self) -> char {
        let c = self.peek().unwrap();
        self.pos += c.len_utf8();

        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        c
    }

    fn slice_to_string(&self, start: usize) -> String {
        self.program[start..self.pos].to_string()
    }

    pub fn parse(&mut self) -> Result<Vec<Token>, LexerInvalidTokenError> {
        let mut tokens = Vec::with_capacity(self.program.len() / 2);

        while let Some(c) = self.peek() {
            match c {
                ' ' | '\n' => {
                    self.digest();
                }

                '/' if self.peek_with_offset(1) == Some('/') => {
                    self.digest();
                    self.digest();
                    while let Some(ch) = self.peek() {
                        self.digest();
                        if ch == '\n' {
                            break;
                        }
                    }
                }

                ';' => {
                    let start = self.pos;
                    self.digest();
                    tokens.push(Token {
                        start,
                        end: self.pos,
                        line: self.line,
                        token_type: TokenType::EndOfstatement,
                        operator_type: None,
                        value: Some(";".to_string()),
                    });
                }

                '!' if self.peek_with_offset(1) == Some('=') => {
                    let start = self.pos;
                    self.digest();
                    self.digest();
                    tokens.push(Token {
                        start,
                        end: self.pos,
                        line: self.line,
                        token_type: TokenType::Operator,
                        operator_type: Some(OperatorType::Neq),
                        value: Some("!=".to_string()),
                    });
                }

                '=' => {
                    let start = self.pos;
                    self.digest();
                    if self.peek() == Some('=') {
                        self.digest();
                        tokens.push(Token {
                            start,
                            end: self.pos,
                            line: self.line,
                            token_type: TokenType::Operator,
                            operator_type: Some(OperatorType::Eq),
                            value: Some("==".to_string()),
                        });
                    } else {
                        tokens.push(Token {
                            start,
                            end: self.pos,
                            line: self.line,
                            token_type: TokenType::Assignment,
                            operator_type: None,
                            value: Some("=".to_string()),
                        });
                    }
                }

                '"' => {
                    let start = self.pos;
                    self.digest();
                    while let Some(ch) = self.peek() {
                        self.digest();
                        if ch == '"' {
                            break;
                        }
                    }
                    let value = self.program[start + 1..self.pos - 1].to_string();
                    tokens.push(Token {
                        start,
                        end: self.pos,
                        line: self.line,
                        token_type: TokenType::StringLiteral,
                        operator_type: None,
                        value: Some(value),
                    });
                }

                'a'..='z' | 'A'..='Z' | '_' => {
                    let start = self.pos;
                    self.digest();
                    while let Some(ch) = self.peek() {
                        if !ch.is_ascii_alphanumeric() && ch != '_' {
                            break;
                        }
                        self.digest();
                    }

                    let text = &self.program[start..self.pos];
                    let token_type = match text {
                        "if" => TokenType::ConditionalIf,
                        "else" => TokenType::ConditionalElse,
                        "let" => TokenType::Declaration,
                        "true" | "false" => TokenType::BooleanLiteral,
                        _ => TokenType::Symbol,
                    };

                    tokens.push(Token {
                        start,
                        end: self.pos,
                        line: self.line,
                        token_type,
                        operator_type: None,
                        value: Some(text.to_string()),
                    });
                }

                '0'..='9' => {
                    let start = self.pos;
                    let mut is_float = false;
                    self.digest();

                    while let Some(ch) = self.peek() {
                        match ch {
                            '0'..='9' => {
                                self.digest();
                            }
                            '.' if !is_float => {
                                is_float = true;
                                self.digest();
                            }
                            '.' => {
                                return Err(LexerInvalidTokenError {
                                    kind: LexerInvalidTokenKind::MalformedNumberLiteral(
                                        self.slice_to_string(start),
                                    ),
                                    line: self.line,
                                    column: self.column,
                                });
                            }
                            _ => break,
                        }
                    }

                    tokens.push(Token {
                        start,
                        end: self.pos,
                        line: self.line,
                        token_type: TokenType::NumeralLiteral(if is_float {
                            NumeralType::Float
                        } else {
                            NumeralType::Integer
                        }),
                        operator_type: None,
                        value: Some(self.slice_to_string(start)),
                    });
                }

                '+' | '-' | '*' | '/' | '^' => {
                    let start = self.pos;
                    let op = self.digest();
                    let operator_type = match op {
                        '+' | '-' => Some(OperatorType::Additive),
                        '*' | '/' => Some(OperatorType::Multiplicative),
                        '^' => Some(OperatorType::Exponential),
                        _ => None,
                    };

                    tokens.push(Token {
                        start,
                        end: self.pos,
                        line: self.line,
                        token_type: TokenType::Operator,
                        operator_type,
                        value: Some(self.slice_to_string(start)),
                    });
                }

                '!' => {
                    let start = self.pos;
                    self.digest();
                    tokens.push(Token {
                        start,
                        end: self.pos,
                        line: self.line,
                        token_type: TokenType::UnaryOperator,
                        operator_type: None,
                        value: Some("!".to_string()),
                    });
                }

                '(' | ')' | '{' | '}' | ',' => {
                    let start = self.pos;
                    let ch = self.digest();
                    let token_type = match ch {
                        '(' => TokenType::ParenthesisL,
                        ')' => TokenType::ParenthesisR,
                        '{' => TokenType::BlockStart,
                        '}' => TokenType::BlockEnd,
                        ',' => TokenType::ArgumentSeparator,
                        _ => unreachable!(),
                    };

                    tokens.push(Token {
                        start,
                        end: self.pos,
                        line: self.line,
                        token_type,
                        operator_type: None,
                        value: Some(self.slice_to_string(start)),
                    });
                }

                _ => {
                    return Err(LexerInvalidTokenError {
                        kind: LexerInvalidTokenKind::UnexpectedToken(c.to_string()),
                        line: self.line,
                        column: self.column,
                    });
                }
            }
        }

        tokens.push(Token {
            start: self.pos,
            end: self.pos,
            line: self.line,
            token_type: TokenType::Eof,
            operator_type: None,
            value: None,
        });

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    fn parse_program(program: String) -> Result<Vec<Token>, LexerInvalidTokenError> {
        let mut parser = TokenParser::new(program.to_string());
        parser.parse()
    }

    #[test]
    fn parses_numerical_values() -> Result<(), Box<dyn Error>>{

        let numbers = ["1", "100", "200", "123", "12340345", "0.1", "1.001", "100.12"];

        for &number in numbers.iter() {
            let result = parse_program(number.to_string());
            let token = result?.first().ok_or("List was empty")?.token_type.clone();
            assert!(
                matches!(token, TokenType::NumeralLiteral(_)),
                "The token must be a NumeralLiteral, {} was found",
                token.to_string()
            );
        }

        Ok(())
    }

    #[test]
    fn malformed_numerical_values_should_not_pass() -> Result<(), Box<dyn Error>>{
        let result: Result<Vec<Token>, LexerInvalidTokenError> = parse_program(String::from("10..1"));

        if let Err(LexerInvalidTokenError {
            kind: LexerInvalidTokenKind::MalformedNumberLiteral(ref literal),
        ..
        }) = result {
            assert_eq!(literal, "10.", "Lexer ingested invalid tokens");
        }

        Ok(())
    }

    #[test]
    fn expressions_are_properly_parsed() -> Result<(), Box<dyn Error>>{
        let test_cases  = [
            (
                "1+2+3",
                vec![
                    TokenType::NumeralLiteral(NumeralType::Integer),
                    TokenType::Operator,
                    TokenType::NumeralLiteral(NumeralType::Integer),
                    TokenType::Operator,
                    TokenType::NumeralLiteral(NumeralType::Integer),
                    TokenType::Eof
                ]
            ),
            (
                "if (1 == 1) { 2 }",
                vec![
                    TokenType::ConditionalIf,
                    TokenType::ParenthesisL,
                    TokenType::NumeralLiteral(NumeralType::Integer),
                    TokenType::Operator,
                    TokenType::NumeralLiteral(NumeralType::Integer),
                    TokenType::ParenthesisR,
                    TokenType::BlockStart,
                    TokenType::NumeralLiteral(NumeralType::Integer),
                    TokenType::BlockEnd,
                    TokenType::Eof
                ]
            ),
            (
                "if (1 != 1) { 2 }",
                vec![
                    TokenType::ConditionalIf,
                    TokenType::ParenthesisL,
                    TokenType::NumeralLiteral(NumeralType::Integer),
                    TokenType::Operator,
                    TokenType::NumeralLiteral(NumeralType::Integer),
                    TokenType::ParenthesisR,
                    TokenType::BlockStart,
                    TokenType::NumeralLiteral(NumeralType::Integer),
                    TokenType::BlockEnd,
                    TokenType::Eof
                ]
            ),
            (
                "if (true != false) { 2 }",
                vec![
                    TokenType::ConditionalIf,
                    TokenType::ParenthesisL,
                    TokenType::BooleanLiteral,
                    TokenType::Operator,
                    TokenType::BooleanLiteral,
                    TokenType::ParenthesisR,
                    TokenType::BlockStart,
                    TokenType::NumeralLiteral(NumeralType::Integer),
                    TokenType::BlockEnd,
                    TokenType::Eof
                ]
            )
        ];

        for (program, expected_tokens) in test_cases.iter() {
            let tokens = parse_program(program.to_string())?;

            let actual_token_types: Vec<TokenType> = tokens.iter().map(|t| t.token_type.clone()).collect();

            assert_eq!(
                actual_token_types,
                *expected_tokens,
                "Token mismatch for input: {}",
                program
            );
        }

        Ok(())
    }
}