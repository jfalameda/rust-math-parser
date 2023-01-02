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
    line_pos : usize,
    line : usize,
    program : Vec<char>
}

impl TokenParser {
    pub fn new(program: String) -> Self {
        TokenParser {
            pos: 0,
            line_pos: 1,
            line: 1,
            program: program.chars().collect()
        }
    }

    fn token_parse_error(&self, message: String) -> ! {
        let c = self.peek().unwrap();
        eprintln!("{} '{}' at line {} and character {}", message, c, self.line, self.line_pos);
        std::process::exit(1)
    }

    fn digest(&mut self) -> char {
        self.pos += 1;
        let ch = self.program[self.pos-1];
        if ch == '\n' {
            self.line += 1;
            self.line_pos = 1;
        }
        else {
            self.line_pos += 1;
        }
        return ch;
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
    pub fn parse(&mut self) -> Vec<Token>{
        let mut tokens = vec![];

        while let Some(c) = self.peek() {
            if c == ' ' || c == '\n' {
                self.digest();
            }
            else if c == '/' && self.peek_with_offset(1).unwrap_or(' ') == '/' {
                let next = self.peek_with_offset(1).unwrap();
                if next == '/' {
                    self.digest();
                    self.digest();
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
                    start: self.line_pos-1,
                    end: self.line_pos,
                    line: self.line,
                    token_type: TokenType::EndOfstatement,
                    value: Some(";".to_string()),
                    operator_type: None
                })
            }
            else if c == '=' {
                self.digest();
                tokens.push(Token {
                    start: self.line_pos-1,
                    end: self.line_pos,
                    line: self.line,
                    token_type: TokenType::Assignment,
                    value: Some("=".to_string()),
                    operator_type: None
                })
            }
            else if c == 'l' {
                let pos = self.line_pos;
                let d = self.peek_with_offset(1).unwrap();
                let e = self.peek_with_offset(2).unwrap();
                if d == 'e' && e == 't' {
                    self.digest();
                    self.digest();
                    self.digest();
                    tokens.push(Token {
                        start: pos,
                        end: self.line_pos,
                        line: self.line,
                        token_type: TokenType::Declaration,
                        value: Some("let".to_string()),
                        operator_type: None
                    })
                }
            }
            else if c == '"' {
                let pos = self.line_pos;
                self.digest();
                let mut string : String = String::from("");
                while let Some(d) = self.peek() {
                    if d == '"' {
                        self.digest();
                        tokens.push(Token {
                            start: pos,
                            end: self.line_pos,
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
                let pos = self.line_pos;
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
                    end: self.line_pos,
                    line: self.line,
                    token_type: TokenType::Symbol,
                    value: Some(symbol),
                    operator_type: None
                })                
            }
            else if c.is_numeric() {
                let pos = self.line_pos;
                let mut number = format!("{}", self.digest());
                let mut is_floating: bool = false;
                while self.peek().unwrap_or_default().is_numeric() || self.peek().unwrap_or_default() == '.' {
                    if self.peek().unwrap_or_default() == '.' {
                        if is_floating {
                            self.token_parse_error("Malformed number literal, found".to_string());
                        }
                        is_floating = true;
                    }
                    number.push(self.digest());
                }
                tokens.push(Token {
                    start: pos,
                    end: self.line_pos,
                    line: self.line,
                    token_type: TokenType::NumeralLiteral,
                    value: Some(number),
                    operator_type: None
                })
            }
            else if c == '+' || c == '-' || c == '*' || c == '/' || c == '^' {
                tokens.push(Token {
                    start: self.line_pos-1,
                    end: self.line_pos,
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
                    start: self.line_pos-1,
                    end: self.line_pos,
                    line: self.line,
                    token_type: TokenType::ParenthesisL,
                    value: Some(format!("{}", c)),
                    operator_type: None
                });
                self.digest();
            }
            else if c == ')' {
                tokens.push(Token {
                    start: self.line_pos-1,
                    end: self.line_pos,
                    line: self.line,
                    token_type: TokenType::ParenthesisR,
                    value: Some(format!("{}", c)),
                    operator_type: None
                });
                self.digest();
            }
            else if c == ',' {
                tokens.push(Token {
                    start: self.line_pos-1,
                    end: self.line_pos,
                    line: self.line,
                    token_type: TokenType::ArgumentSeparator,
                    value: Some(format!("{}", c)),
                    operator_type: None
                });
                self.digest();
            }
            else {
                self.token_parse_error("Syntax error: unrecognized character".to_string());
            }
        }
        tokens.push(Token {
            start: self.line_pos,
            end: self.line_pos,
            line: self.line,
            token_type: TokenType::Eof,
            value: None,
            operator_type: None
        });
        return tokens;
    }
}