use crate::lexer::{
    self, AdditiveOperatorSubtype, OperatorType, PostfixOperatorType, Token, TokenType,
    UnaryOperatorSubtype,
};
use crate::node::{
    build_assignment_node, build_class_declaration_node, build_class_instantiation_node,
    build_conditional_node, build_function_call_node, build_function_declaration_node, build_node,
    build_postfix_node, build_program_node, build_return_node, build_statement_node,
    build_unary_node, Block, Expression,
};
use crate::parser_errors::{ParserError, ParserErrorKind};

pub struct Parser<'a> {
    pos: usize,
    tokens: Vec<lexer::Token<'a>>,
}

fn error_unexpected_token<'a>(
    token: &Token<'a>,
    expected_token_type: &TokenType,
) -> ParserError<'a> {
    ParserError {
        kind: ParserErrorKind::UnexpectedToken(expected_token_type.to_string(), token.clone()),
    }
}

fn error_unrecognized_token<'a>(token: &Token<'a>) -> ParserError<'a> {
    ParserError {
        kind: ParserErrorKind::UnrecognizedToken(token.clone()),
    }
}

fn error_eof<'a>() -> ParserError<'a> {
    ParserError {
        kind: ParserErrorKind::UnexpectedEOF,
    }
}

fn error_unexpected_empty_value<'a>() -> ParserError<'a> {
    ParserError {
        kind: ParserErrorKind::UnexpectedEmptyValue,
    }
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<lexer::Token<'a>>) -> Self {
        Parser { pos: 0, tokens }
    }

    fn peek(&self, pos: Option<usize>) -> Option<&lexer::Token<'a>> {
        self.tokens.get(pos.unwrap_or(self.pos))
    }

    fn peek_type_is(&self, expected: TokenType) -> bool {
        matches!(self.peek(None), Some(t) if t.token_type == expected)
    }

    fn digest(&mut self, expected: TokenType) -> Result<Token<'a>, ParserError<'a>> {
        let token = self.peek(None).ok_or_else(error_eof)?.clone();

        if token.token_type != expected {
            return Err(error_unexpected_token(&token, &expected));
        }

        self.pos += 1;
        Ok(token)
    }

    pub fn parse(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        Ok(build_program_node(self.parse_block()?))
    }

    fn consume_statement_terminator(&mut self, stmt: &Expression) -> Result<(), ParserError<'a>> {
        match stmt {
            Expression::IfConditional(_, _, _)
            | Expression::FunctionDeclaration(_)
            | Expression::ClassDeclaration(_) => Ok(()),
            _ => {
                self.digest(TokenType::EndOfstatement)?;
                Ok(())
            }
        }
    }

    fn parse_class_declaration(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        self.digest(TokenType::ClassDeclaration)?;
        let identifier = self.digest(TokenType::Symbol)?;
        self.digest(TokenType::BlockStart)?;

        let mut members = vec![];
        let mut methods = vec![];
        // Stuff
        while let Some(token) = self.peek(None) {
            if token.token_type == TokenType::Declaration {
                members.push(self.parse_declaration()?);
                self.digest(TokenType::EndOfstatement)?;
            } else if token.token_type == TokenType::FunctionDeclaration {
                methods.push(self.parse_function_declaration()?);
            } else {
                break;
            }
        }

        self.digest(TokenType::BlockEnd)?;

        Ok(build_class_declaration_node(
            identifier
                .value
                .ok_or_else(error_unexpected_empty_value)?
                .to_string(),
            members,
            methods,
        ))
    }

    fn parse_function_declaration(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        self.digest(TokenType::FunctionDeclaration)?;

        let identifier = self.digest(TokenType::Symbol)?;

        self.digest(TokenType::ParenthesisL)?;

        let mut args = vec![];

        while let Some(token) = self.peek(None) {
            if token.token_type == TokenType::ParenthesisR {
                break;
            }

            // Function arguments
            args.push(
                self.digest(TokenType::Symbol)?
                    .value
                    .ok_or_else(error_unexpected_empty_value)?
                    .to_string(),
            );

            // If next is not ')', expect a comma
            if let Some(next) = self.peek(None) {
                if next.token_type != TokenType::ParenthesisR {
                    self.digest(TokenType::ArgumentSeparator)?;
                }
            } else {
                return Err(error_eof());
            }
        }

        self.digest(TokenType::ParenthesisR)?;

        let block = self.parse_block_with_delimiters()?;

        let identifier = identifier
            .value
            .ok_or_else(error_unexpected_empty_value)?
            .to_string();

        Ok(build_function_declaration_node(identifier, args, block))
    }

    fn parse_block_with_delimiters(&mut self) -> Result<Block, ParserError<'a>> {
        self.digest(TokenType::BlockStart)?;
        let block = self.parse_block()?;
        self.digest(TokenType::BlockEnd)?;

        Ok(block)
    }

    fn parse_block(&mut self) -> Result<Block, ParserError<'a>> {
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
            self.consume_statement_terminator(stmt.as_ref())?;

            body.push(build_statement_node(stmt));
        }

        Ok(body)
    }

    fn parse_statement(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        let token = self.peek(None).ok_or_else(error_eof)?;

        let statement = match token.token_type {
            TokenType::NumeralLiteral(_)
            | TokenType::BooleanLiteral
            | TokenType::Operator
            | TokenType::Symbol
            | TokenType::StringLiteral
            | TokenType::ParenthesisL => Ok(self.parse_expression(0)?),
            TokenType::Declaration => Ok(self.parse_declaration()?),
            TokenType::FunctionDeclaration => Ok(self.parse_function_declaration()?),
            TokenType::ClassDeclaration => Ok(self.parse_class_declaration()?),
            TokenType::ConditionalIf => Ok(self.parse_conditional()?),
            TokenType::Return => Ok(self.parse_return()?),
            _ => Err(error_unrecognized_token(token)),
        }?;

        Ok(statement)
    }
    fn parse_declaration(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        self.digest(TokenType::Declaration)?; // consume "let"
        let symbol = self.digest(TokenType::Symbol)?;
        self.digest(TokenType::Assignment)?;
        let expr = self.parse_expression(0)?;
        Ok(build_assignment_node(
            symbol.value.ok_or_else(error_eof)?.to_string(),
            expr,
        ))
    }

    fn parse_return(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        self.digest(TokenType::Return)?;
        Ok(build_return_node(self.parse_expression(0)?))
    }

    fn parse_conditional(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
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

    fn parse_statement_or_block(&mut self) -> Result<Block, ParserError<'a>> {
        // If can be followed either by a block or by a simple statement
        if self.peek_type_is(TokenType::BlockStart) {
            let if_block = self.parse_block_with_delimiters()?;

            Ok(if_block)
        } else {
            let stmt = self.parse_statement()?;
            self.consume_statement_terminator(stmt.as_ref())?;
            Ok(vec![build_statement_node(stmt)])
        }
    }

    fn parse_expression(&mut self, precedence: i32) -> Result<Box<Expression>, ParserError<'a>> {
        let token = self.peek(None).ok_or_else(error_eof)?;

        if token.token_type == TokenType::New {
            self.parse_class_instantiation()
        } else {
            self.parse_binary_expression(precedence)
        }
    }

    fn parse_class_instantiation(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        self.digest(TokenType::New)?;
        let class_name = self.digest(TokenType::Symbol)?;
        self.digest(TokenType::ParenthesisL)?;
        let args = self.parse_method_args()?;
        self.digest(TokenType::ParenthesisR)?;

        Ok(build_class_instantiation_node(
            class_name.value.ok_or_else(error_eof)?.to_string(),
            args,
            class_name.line,
        ))
    }

    fn parse_function_call(
        &mut self,
        symbol: Token<'_>,
    ) -> Result<Box<Expression>, ParserError<'a>> {
        self.digest(TokenType::ParenthesisL)?;
        let args = self.parse_method_args()?;
        self.digest(TokenType::ParenthesisR)?;

        Ok(build_function_call_node(
            symbol.value.ok_or_else(error_eof)?.to_string(),
            args,
            symbol.line,
        ))
    }

    fn parse_method_args(&mut self) -> Result<Vec<Expression>, ParserError<'a>> {
        let mut args = vec![];

        while let Some(token) = self.peek(None) {
            if token.token_type == TokenType::ParenthesisR {
                break;
            }

            args.push(*self.parse_expression(0)?);

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

    fn parse_binary_expression(
        &mut self,
        precedence: i32,
    ) -> Result<Box<Expression>, ParserError<'a>> {
        let mut left = self.parse_term()?;

        loop {
            let op_token = match self.peek(None) {
                Some(t) if t.token_type == TokenType::Operator => t.clone(),
                _ => break,
            };

            let (op_precedence, is_right) = op_token.operator_predecende();

            if op_precedence < precedence {
                break;
            }

            self.digest(TokenType::Operator)?;

            let next_precedence = if is_right {
                op_precedence
            } else {
                op_precedence + 1
            };

            let right = self.parse_expression(next_precedence)?;
            left = build_node(&op_token, Some(left), Some(right));
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        let left = self.parse_prefix_term()?;
        self.parse_postfix_chain(left)
    }

    fn parse_prefix_term(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        let token = self.peek(None).ok_or_else(error_eof)?;

        match token.token_type {
            TokenType::Operator => match token.operator_type {
                Some(OperatorType::Additive(AdditiveOperatorSubtype::Sub)) => {
                    self.digest(TokenType::Operator)?;
                    let literal = self.parse_term()?;
                    Ok(build_unary_node(UnaryOperatorSubtype::Min, literal))
                }
                Some(OperatorType::Unary(UnaryOperatorSubtype::Not)) => {
                    self.digest(TokenType::Operator)?;
                    let literal = self.parse_term()?;
                    Ok(build_unary_node(UnaryOperatorSubtype::Not, literal))
                }
                Some(_) | None => Err(error_unrecognized_token(&token)),
            },
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        let token_type = self
            .peek(None)
            .map(|token| &token.token_type)
            .ok_or_else(error_unexpected_empty_value)?;

        match token_type {
            TokenType::Symbol => {
                let symbol = self.digest(TokenType::Symbol)?;
                if self.peek_type_is(TokenType::ParenthesisL) {
                    self.parse_function_call(symbol)
                } else {
                    Ok(build_node(&symbol, None, None))
                }
            }
            TokenType::StringLiteral
                | TokenType::BooleanLiteral
                | TokenType::NumeralLiteral(_) => {
                Ok(self.parse_literal()?)
            }
            TokenType::ParenthesisL => {
                self.digest(TokenType::ParenthesisL)?;
                let expr = self.parse_expression(0)?;
                self.digest(TokenType::ParenthesisR)?;
                Ok(expr)
            }
            _ => Err(error_unrecognized_token(
                self.peek(None).ok_or_else(error_eof)?,
            )),
        }
    }

    fn parse_literal(&mut self) -> Result<Box<Expression>, ParserError<'a>> {
        let token = self.peek(None)
            .ok_or_else(error_unexpected_empty_value)?;

        match token.token_type {
            TokenType::StringLiteral => {
                let token = self.digest(TokenType::StringLiteral)?;
                Ok(build_node(&token, None, None))
            }
            TokenType::BooleanLiteral => {
                let token = self.digest(TokenType::BooleanLiteral)?;
                Ok(build_node(&token, None, None))
            }
            TokenType::NumeralLiteral(numeral_type) => {
                let token = self.digest(TokenType::NumeralLiteral(numeral_type))?;
                Ok(build_node(&token, None, None))
            }
            _ => Err(error_unrecognized_token(token))
        }
    }

    fn parse_postfix_chain(
        &mut self,
        mut left: Box<Expression>,
    ) -> Result<Box<Expression>, ParserError<'a>> {
        while self.peek_type_is(TokenType::MemberAccess) {
            self.digest(TokenType::MemberAccess)?;

            // A member access must always be followed by a symbol
            if !self.peek_type_is(TokenType::Symbol) {
                let token = self.peek(None).ok_or_else(error_eof)?.clone();

                return Err(error_unexpected_token(&token, &TokenType::Symbol));
            }

            let right = self.parse_term()?;
            left = build_postfix_node(PostfixOperatorType::MemberAccess, left, right);
        }

        Ok(left)
    }
}
