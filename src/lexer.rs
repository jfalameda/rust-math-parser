use crate::lexer_errors::{LexerInvalidTokenError, LexerInvalidTokenKind};

#[derive(PartialEq, Clone)]
pub enum TokenType {
    Operator,
    NumeralLiteral,
    ParenthesisL,
    ParenthesisR,
    Declaration,
    Symbol,
    Assignment,
    EndOfstatement,
    ArgumentSeparator,
    StringLiteral,
    Eof
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            TokenType::Assignment => "=",
            TokenType::Declaration => "let",
            TokenType::EndOfstatement => ";",
            TokenType::Eof => "EOF",
            TokenType::NumeralLiteral => "literal",
            TokenType::Operator => "operator",
            TokenType::ParenthesisL => "(",
            TokenType::ParenthesisR => ")",
            TokenType::Symbol => "symbol",
            TokenType::ArgumentSeparator => ",",
            TokenType::StringLiteral => "\"",
        }.to_string()
    }
}

#[derive(PartialEq, Clone)]
pub enum OperatorType {
    Additive,
    Factorial,
    Exponential
}

pub struct Token {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub token_type: TokenType,
    pub operator_type: Option<OperatorType>,
    pub value: Option<String>
}

impl Token {
    pub fn operator_predecende(self) -> i32 {
        match self.value.unwrap().as_str() {
            "^" => 3,
            "*" | "/" => 2,
            "+" | "-" => 1,
            _ => 0
        }
    }
    pub fn clone_token(&self) -> Token {
        return Token {
            start: self.start.clone(),
            end: self.end.clone(),
            line: self.line.clone(),
            token_type: self.token_type.clone(),
            value: Some(self.value.as_ref().unwrap_or(&String::from("")).clone()),
            operator_type: self.operator_type.clone()
        }
    }
}

pub struct TokenParser {
    pos : usize,
    column : usize,
    line : usize,
    program : Vec<char>
}

impl TokenParser {
    pub fn new(program: String) -> Self {
        TokenParser {
            pos: 0,
            column: 1,
            line: 1,
            program: program.chars().collect()
        }
    }

    fn digest(&mut self) -> char {
        self.pos += 1;
        let ch = self.program[self.pos-1];
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        }
        else {
            self.column += 1;
        }
        return ch;
    }
    fn digest_n(&mut self, number_of_tokens: u32) -> String {
        (0..number_of_tokens).map(|_| {
            self.digest()
        }).collect()
    }
    fn peek(&self) -> Option<char> {
        if self.pos < self.program.len() {
            return Some(self.program[self.pos]);
        }
        return None;
    }
    fn peek_with_offset(&self, offset: usize) -> Option<char> {
        let pos = self.pos+offset;
        if pos < self.program.len() {
            return Some(self.program[pos]);
        }
        return None;
    }
    pub fn parse(&mut self) -> Result<Vec<Token>, LexerInvalidTokenError> {
        let mut tokens = vec![];

        while let Some(c) = self.peek() {
            if c == ' ' || c == '\n' {
                self.digest();
            }
            else if c == '/' && self.peek_with_offset(1).unwrap_or(' ') == '/' {
                let next = self.peek_with_offset(1).unwrap();
                if next == '/' {
                    self.digest_n(2);
                    while let Some(next) = self.peek() {
                        if next == '\n' {
                            self.digest();
                            break;
                        }
                        self.digest();
                    }
                }
            }
            else if c == ';' {
                self.digest();
                tokens.push(Token {
                    start: self.column-1,
                    end: self.column,
                    line: self.line,
                    token_type: TokenType::EndOfstatement,
                    value: Some(";".to_string()),
                    operator_type: None
                })
            }
            else if c == '=' {
                self.digest();
                tokens.push(Token {
                    start: self.column-1,
                    end: self.column,
                    line: self.line,
                    token_type: TokenType::Assignment,
                    value: Some("=".to_string()),
                    operator_type: None
                })
            }
            else if c == 'l' {
                let pos = self.column;
                let d = self.peek_with_offset(1).unwrap();
                let e = self.peek_with_offset(2).unwrap();
                if d == 'e' && e == 't' {
                    self.digest_n(3);
                    tokens.push(Token {
                        start: pos,
                        end: self.column,
                        line: self.line,
                        token_type: TokenType::Declaration,
                        value: Some("let".to_string()),
                        operator_type: None
                    })
                }
            }
            else if c == '"' {
                let pos = self.column;
                self.digest();
                let mut string : String = String::from("");
                while let Some(d) = self.peek() {
                    if d == '"' {
                        self.digest();
                        tokens.push(Token {
                            start: pos,
                            end: self.column,
                            line: self.line,
                            token_type: TokenType::StringLiteral,
                            value: Some(string),
                            operator_type: None
                        });
                        break;
                    }
                    string.push(self.digest());
                }
                               
            }
            else if c.is_ascii_alphabetic() {
                let pos = self.column;
                let mut symbol = format!("{}", self.digest());
                while let Some(d) = self.peek() {
                    if !d.is_ascii_alphanumeric() && !(d == '_') {
                        break;
                    }
                    self.digest();
                    symbol.push(d);
                }
                tokens.push(Token {
                    start: pos,
                    end: self.column,
                    line: self.line,
                    token_type: TokenType::Symbol,
                    value: Some(symbol),
                    operator_type: None
                })                
            }
            else if c.is_numeric() {
                let pos = self.column;
                let mut number = format!("{}", self.digest());
                let mut is_floating: bool = false;
                while self.peek().unwrap_or_default().is_numeric() || self.peek().unwrap_or_default() == '.' {
                    if self.peek().unwrap_or_default() == '.' {
                        if is_floating {
                            number.push(self.digest());

                            return Err(LexerInvalidTokenError {
                                kind: LexerInvalidTokenKind::MalformedNumberLiteral(number),
                                line: self.line,
                                column: self.column,
                            });
                        }
                        is_floating = true;
                    }
                    number.push(self.digest());
                }
                tokens.push(Token {
                    start: pos,
                    end: self.column,
                    line: self.line,
                    token_type: TokenType::NumeralLiteral,
                    value: Some(number),
                    operator_type: None
                })
            }
            else if c == '+' || c == '-' || c == '*' || c == '/' || c == '^' {
                tokens.push(Token {
                    start: self.column-1,
                    end: self.column,
                    line: self.line,
                    token_type: TokenType::Operator,
                    operator_type: match c {
                        '+' | '-' => Some(OperatorType::Additive),
                        '*' | '/' => Some(OperatorType::Factorial),
                        '^'  => Some(OperatorType::Exponential),
                        _ => None
                    },
                    value: Some(format!("{}", c))
                });
                self.digest();
            }
            else if c == '(' {
                tokens.push(Token {
                    start: self.column-1,
                    end: self.column,
                    line: self.line,
                    token_type: TokenType::ParenthesisL,
                    value: Some(format!("{}", c)),
                    operator_type: None
                });
                self.digest();
            }
            else if c == ')' {
                tokens.push(Token {
                    start: self.column-1,
                    end: self.column,
                    line: self.line,
                    token_type: TokenType::ParenthesisR,
                    value: Some(format!("{}", c)),
                    operator_type: None
                });
                self.digest();
            }
            else if c == ',' {
                tokens.push(Token {
                    start: self.column-1,
                    end: self.column,
                    line: self.line,
                    token_type: TokenType::ArgumentSeparator,
                    value: Some(format!("{}", c)),
                    operator_type: None
                });
                self.digest();
            }
            else {
                return Err(LexerInvalidTokenError {
                    kind: LexerInvalidTokenKind::UnexpectedToken(c),
                    line: self.line,
                    column: self.column,
                });
            }
        }
        tokens.push(Token {
            start: self.column,
            end: self.column,
            line: self.line,
            token_type: TokenType::Eof,
            value: None,
            operator_type: None
        });
        return Ok(tokens);
    }
}