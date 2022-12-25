#[derive(PartialEq, Clone)]
pub enum TokenType {
    Operator,
    NumeralLiteral,
    ParenthesisL,
    ParenthesisR,
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
            token_type: self.token_type.clone(),
            value: Some(self.value.as_ref().unwrap_or(&String::from("")).clone()),
            operator_type: self.operator_type.clone()
        }
    }
}

pub struct TokenParser {
    pos : usize,
    program : Vec<char>
}

impl TokenParser {
    pub fn new(program: String) -> Self {
        TokenParser {
            pos: 0,
            program: program.chars().collect()
        }
    }

    fn digest(&mut self) -> char {
        self.pos += 1;
        return self.program[self.pos-1];
    }
    fn peek(&self) -> Option<char> {
        if self.pos < self.program.len() {
            return Some(self.program[self.pos]);
        }
        return None;
    }
    pub fn parse(&mut self) -> Vec<Token>{
        let mut tokens = vec![];

        while let Some(c) = self.peek() {
            if c == ' ' {
                self.digest();
            }
            else if c.is_numeric() {
                let pos = self.pos;
                let mut number = format!("{}", self.digest());
                while self.peek().unwrap_or_default().is_numeric() || self.peek().unwrap_or_default() == '.' {
                    number.push(self.digest());
                }
                tokens.push(Token {
                    start: pos,
                    end: self.pos,
                    token_type: TokenType::NumeralLiteral,
                    value: Some(number),
                    operator_type: None
                })
            }
            else if c == '+' || c == '-' || c == '*' || c == '/' || c == '^' {
                tokens.push(Token {
                    start: self.pos,
                    end: self.pos,
                    token_type: TokenType::Operator,
                    operator_type: Some(match c {
                        '+' | '-' => OperatorType::Additive,
                        '*' | '/' => OperatorType::Factorial,
                        '^'  => OperatorType::Exponential,
                        _ => {
                            eprintln!("Syntax error: unrecognized character '{}' at {}", c, self.pos);
                            std::process::exit(1)
                        }
                    }),
                    value: Some(format!("{}", c))
                });
                self.digest();
            }
            else if c == '(' {
                tokens.push(Token {
                    start: self.pos,
                    end: self.pos+1,
                    token_type: TokenType::ParenthesisL,
                    value: Some(format!("{}", c)),
                    operator_type: None
                });
                self.digest();
            }
            else if c == ')' {
                tokens.push(Token {
                    start: self.pos,
                    end: self.pos+1,
                    token_type: TokenType::ParenthesisR,
                    value: Some(format!("{}", c)),
                    operator_type: None
                });
                self.digest();
            }
            else {
                eprintln!("Syntax error: unrecognized character '{}' at {}", c, self.pos);
                std::process::exit(1)
            }
        }
        tokens.push(Token {
            start: self.pos,
            end: self.pos,
            token_type: TokenType::Eof,
            value: None,
            operator_type: None
        });
        return tokens;
    }
}