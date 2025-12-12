use crate::lexer::{self, Token, TokenType};
use crate::node::{
    Expression, build_node, build_unary_node, build_program_node, build_statement_node,
    build_method_call_node, build_assignment_node,
};
use crate::parser_errors::{ParserError, ParserErrorKind};

pub struct Parser {
    pos: usize,
    tokens: Vec<lexer::Token>,
}

fn error_unexpected_token(token: &Token, expected_token_type: &TokenType) -> ParserError {
    ParserError {
        kind: ParserErrorKind::UnexpectedToken(expected_token_type.to_string(), token.clone()),
    }
}

fn error_unrecognized_token(token: &Token) -> ParserError {
    ParserError {
        kind: ParserErrorKind::UnrecognizedToken(token.clone()),
    }
}

fn error_eof() -> ParserError {
    ParserError {
        kind: ParserErrorKind::UnexpectedEOF,
    }
}

impl Parser {
    pub fn new(tokens: Vec<lexer::Token>) -> Self {
        Parser { pos: 0, tokens }
    }

    fn peek(&self, pos: Option<usize>) -> Option<&lexer::Token> {
        self.tokens.get(pos.unwrap_or(self.pos))
    }

    fn digest(&mut self, expected: Option<TokenType>) -> Result<Token, ParserError> {
        let token = self
            .peek(None)
            .ok_or_else(error_eof)?
            .clone();

        if let Some(expected_type) = expected {
            if token.token_type != expected_type {
                return Err(error_unexpected_token(&token, &expected_type));
            }
        }

        self.pos += 1;
        Ok(token)
    }

    fn get_current_operator_precedence(&self) -> i32 {
        match self.peek(None) {
            Some(op) if op.token_type == TokenType::Operator => op.clone().operator_predecende(),
            _ => 0,
        }
    }

    pub fn parse(&mut self) -> Result<Box<Expression>, ParserError> {
        Ok(build_program_node(self.parse_program()?))
    }

    fn parse_program(&mut self) -> Result<Vec<Box<Expression>>, ParserError> {
        let mut body = vec![];

        while let Some(token) = self.peek(None) {
            if token.token_type == TokenType::Eof {
                self.digest(None)?; // consume EOF
                break;
            }

            let stmt = self.parse_statement()?;
            self.digest(Some(TokenType::EndOfstatement))?;
            body.push(build_statement_node(stmt));
        }

        Ok(body)
    }

    fn parse_statement(&mut self) -> Result<Box<Expression>, ParserError> {
        let token = self.peek(None).ok_or_else(error_eof)?;

        match token.token_type {
            TokenType::NumeralLiteral(_)
            | TokenType::Operator
            | TokenType::Symbol
            | TokenType::StringLiteral => Ok(self.parse_expression(0)?),

            TokenType::Declaration => {
                self.digest(Some(TokenType::Declaration))?; // consume "let"
                let symbol = self.digest(Some(TokenType::Symbol))?;
                self.digest(Some(TokenType::Assignment))?;
                let expr = self.parse_expression(0)?;
                Ok(build_assignment_node(symbol.value.ok_or_else(error_eof)?, expr))
            }

            _ => Err(error_unrecognized_token(token)),
        }
    }

    fn parse_expression(&mut self, precedence: i32) -> Result<Box<Expression>, ParserError> {
        let token = self.peek(None).ok_or_else(error_eof)?;
        let next = self.peek(Some(self.pos + 1));

        if token.token_type == TokenType::Symbol
            && matches!(next.map(|t| t.token_type.clone()), Some(TokenType::ParenthesisL))
        {
            self.parse_method_call()
        } else {
            self.parse_binary_expression(precedence)
        }
    }

    fn parse_method_call(&mut self) -> Result<Box<Expression>, ParserError> {
        let method_name = self.digest(Some(TokenType::Symbol))?;
        self.digest(Some(TokenType::ParenthesisL))?;
        let args = self.parse_method_args()?;
        self.digest(Some(TokenType::ParenthesisR))?;

        Ok(build_method_call_node(method_name.value.ok_or_else(error_eof)?, args))
    }

    fn parse_method_args(&mut self) -> Result<Vec<Box<Expression>>, ParserError> {
        let mut args = vec![];

        while let Some(token) = self.peek(None) {
            if token.token_type == TokenType::ParenthesisR {
                break;
            }

            args.push(self.parse_expression(0)?);

            // If next is not ')', expect a comma
            if let Some(next) = self.peek(None) {
                if next.token_type != TokenType::ParenthesisR {
                    self.digest(Some(TokenType::ArgumentSeparator))?;
                }
            } else {
                return Err(error_eof());
            }
        }

        Ok(args)
    }

    fn parse_binary_expression(&mut self, precedence: i32) -> Result<Box<Expression>, ParserError> {
        let mut left = self.parse_term()?;

        while precedence < self.get_current_operator_precedence() {
            let token = self.peek(None).ok_or_else(error_eof)?.clone();
            let op_precedence = self.get_current_operator_precedence();

            self.digest(None)?; // consume operator
            let right = self.parse_expression(op_precedence)?;

            left = build_node(&token, Some(left), Some(right));
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Box<Expression>, ParserError> {
        let token = self.peek(None).ok_or_else(error_eof)?.clone();

        match token.token_type {
            TokenType::Operator => {
                match token.value.as_deref() {
                    Some("-") => {
                        self.digest(None)?; // consume '-'
                        let literal = self.parse_term()?;
                        Ok(build_unary_node(&token, literal))
                    }
                    _ => Err(error_unrecognized_token(&token)),
                }
            }

            TokenType::Symbol
            | TokenType::StringLiteral
            | TokenType::NumeralLiteral(_) => {
                self.digest(None)?; // consume literal
                Ok(build_node(&token, None, None))
            }

            TokenType::ParenthesisL => {
                self.digest(None)?; // consume '('
                let expr = self.parse_expression(0)?;
                self.digest(Some(TokenType::ParenthesisR))?;
                Ok(expr)
            }

            _ => Err(error_unrecognized_token(&token)),
        }
    }
}
