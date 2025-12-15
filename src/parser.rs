use crate::lexer::{self, AdditiveOperatorSubtype, OperatorType, Token, TokenType, UnaryOperatorSubtype};
use crate::node::{
    Block, Expression, build_assignment_node, build_conditional_node, build_method_call_node, build_node, build_program_node, build_statement_node, build_unary_node
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

    fn peek_type_is(&self, expected: TokenType) -> bool {
        matches!(self.peek(None), Some(t) if t.token_type == expected)
    }

    fn digest(&mut self, expected: TokenType) -> Result<Token, ParserError> {
        let token = self
            .peek(None)
            .ok_or_else(error_eof)?
            .clone();

        if token.token_type != expected {
            return Err(error_unexpected_token(&token, &expected));
        }

        self.pos += 1;
        Ok(token)
    }

    pub fn parse(&mut self) -> Result<Box<Expression>, ParserError> {
        Ok(build_program_node(self.parse_block()?))
    }

    fn consume_statement_terminator(&mut self, stmt: &Box<Expression>) -> Result<(), ParserError> {
        match stmt.as_ref() {
            Expression::IfConditional(_, _, _) => Ok(()),
            _ => {
                self.digest(TokenType::EndOfstatement)?;
                Ok(())
            },
        }
}

    fn parse_block(&mut self) -> Result<Block, ParserError> {
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
            self.consume_statement_terminator(&stmt)?;

            body.push(build_statement_node(stmt));
        }

        Ok(body)
    }

    fn parse_statement(&mut self) -> Result<Box<Expression>, ParserError> {
        let token = self.peek(None).ok_or_else(error_eof)?;

        let statement = match token.token_type {
            TokenType::NumeralLiteral(_)
            | TokenType::BooleanLiteral
            | TokenType::Operator
            | TokenType::Symbol
            | TokenType::StringLiteral => Ok(self.parse_expression(0)?),
            TokenType::Declaration     => Ok(self.parse_declaration()?),
            TokenType::ConditionalIf   => Ok(self.parse_conditional()?),
            _ => Err(error_unrecognized_token(token)),
        }?;

        Ok(statement)
    }
    fn parse_declaration(&mut self) -> Result<Box<Expression>, ParserError> {
        self.digest(TokenType::Declaration)?; // consume "let"
        let symbol = self.digest(TokenType::Symbol)?;
        self.digest(TokenType::Assignment)?;
        let expr = self.parse_expression(0)?;
        Ok(build_assignment_node(symbol.value.ok_or_else(error_eof)?, expr))
    }

    fn parse_conditional(&mut self) -> Result<Box<Expression>, ParserError> {
        self.digest(TokenType::ConditionalIf)?;
        self.digest(TokenType::ParenthesisL)?;
        let expr = self.parse_expression(0)?;
        self.digest(TokenType::ParenthesisR)?;
        
        let if_block = self.parse_statement_or_block()?;

        let else_block = if self.peek_type_is(TokenType::ConditionalElse) {
            self.digest(TokenType::ConditionalElse)?;
            Some(self.parse_statement_or_block()?)
        } else {
            None
        };

        Ok(build_conditional_node(expr, if_block, else_block))
    }

    fn parse_statement_or_block(&mut self)  -> Result<Block, ParserError> {
        // If can be followed either by a block or by a simple statement
        if self.peek_type_is(TokenType::BlockStart) {
            self.digest(TokenType::BlockStart)?;
            let if_block = self.parse_block()?;
            self.digest(TokenType::BlockEnd)?;
            Ok(if_block)
        }
        else {
            let stmt = self.parse_statement()?;
            self.consume_statement_terminator(&stmt)?;
            Ok(vec![stmt])
        }
    }


    fn parse_expression(&mut self, precedence: i32) -> Result<Box<Expression>, ParserError> {
        let token = self.peek(None).ok_or_else(error_eof)?;
        let next = self.peek(Some(self.pos + 1));

        let expr = if token.token_type == TokenType::Symbol
            && matches!(next.map(|t| t.token_type.clone()), Some(TokenType::ParenthesisL))
        {
            self.parse_method_call()
        } else {
            self.parse_binary_expression(precedence)
        };

        return expr;
    }

    fn parse_method_call(&mut self) -> Result<Box<Expression>, ParserError> {
        let method_name = self.digest(TokenType::Symbol)?;
        self.digest(TokenType::ParenthesisL)?;
        let args = self.parse_method_args()?;
        self.digest(TokenType::ParenthesisR)?;

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
                    self.digest(TokenType::ArgumentSeparator)?;
                }
            } else {
                return Err(error_eof());
            }
        }

        Ok(args)
    }

    fn parse_binary_expression(&mut self, precedence: i32) -> Result<Box<Expression>, ParserError> {
        let mut left = self.parse_term()?;

        loop {
            let op_token = match self.peek(None) {
                Some(t) if t.token_type == TokenType::Operator => t.clone(),
                _ => break,
            };

            let (op_precedence, is_right) = op_token.clone().operator_predecende();

            if op_precedence < precedence {
                break;
            }

            self.digest(TokenType::Operator)?;

            let next_precedence = if is_right { op_precedence } else { op_precedence + 1 };

            let right = self.parse_expression(next_precedence)?;
            left = build_node(&op_token, Some(left), Some(right));
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Box<Expression>, ParserError> {
        let token = self.peek(None).ok_or_else(error_eof)?.clone();

        match token.token_type {
            TokenType::Operator => {
                match token.operator_type {
                    Some(OperatorType::Additive(AdditiveOperatorSubtype::Sub)) => {
                        self.digest(TokenType::Operator)?; // consume '-'
                        let literal = self.parse_term()?;
                        Ok(build_unary_node(UnaryOperatorSubtype::Min, literal))
                    },
                    Some(OperatorType::Unary(UnaryOperatorSubtype::Not)) => {
                        self.digest(TokenType::Operator)?;
                        let literal = self.parse_term()?;
                        Ok(build_unary_node(UnaryOperatorSubtype::Not, literal))
                    }
                    Some(_) | None => Err(error_unrecognized_token(&token))
                }
            }

            TokenType::Symbol
            | TokenType::StringLiteral
            | TokenType::BooleanLiteral
            | TokenType::NumeralLiteral(_) => {
                self.digest(token.token_type.clone())?; // consume literal
                Ok(build_node(&token, None, None))
            }

            TokenType::ParenthesisL => {
                self.digest(TokenType::ParenthesisL)?; // consume '('
                let expr = self.parse_expression(0)?;
                self.digest(TokenType::ParenthesisR)?;
                Ok(expr)
            }

            _ => Err(error_unrecognized_token(&token)),
        }
    }
}
