use crate::expression::Expression::*;
use crate::expression::{Expression, LiteralValue};
use crate::statement::Statement;
use crate::token::TokenType::*;
use crate::token::{Token, TokenType};

use std::string::String;

#[cfg(test)]
#[path = "./tests/parser_tests.rs"]
mod tests;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
        }
    }

    // region grammar components

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = vec![];
        let mut errors = vec![];

        while !self.is_at_end() {
            match self.declaration() {
                Ok(statement) => statements.push(statement),
                Err(message) => errors.push(message),
            }
        }

        if errors.len() != 0 {
            // question, shoulf we just print it here and return the remaining ones?
            return Err(errors.join("\n"));
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Statement, String> {
        if self.match_tokens(vec![Var]) {
            return match self.variable_declaration() {
                Ok(statement) => Ok(statement),
                Err(message) => {
                    self.synchronize();
                    Err(message)
                }
            };
        }

        self.statement()
    }

    fn variable_declaration(&mut self) -> Result<Statement, String> {
        let token_name = self.consume(Identifier, "Expect variable name")?;

        let mut initializer: Expression = Literal {
            value: LiteralValue::Nil,
        };

        if self.match_tokens(vec![Equal]) {
            initializer = self.expression()?;
        }

        self.consume(Semicolon, "Expected ';' after a variable declaration")?;

        Ok(Statement::Variable {
            token: token_name,
            initializer: initializer,
        })
    }

    fn statement(&mut self) -> Result<Statement, String> {
        if self.match_tokens(vec![LeftBrace]) {
            let blocks = self.blocks()?;
            return Ok(Statement::Block { statements: blocks });
        }
        if self.match_tokens(vec![Print]) {
            return self.print_statement();
        }
        return self.expression_statement();
    }

    fn blocks(&mut self) -> Result<Vec<Statement>, String> {
        let mut statements = vec![];

        while !self.check(RightBrace) && !self.is_at_end() {
            let declaration = self.declaration()?;
            statements.push(declaration);
        }

        self.consume(RightBrace, "Expect '}' after block")?;

        Ok(statements)
    }

    fn print_statement(&mut self) -> Result<Statement, String> {
        let expression = self.expression()?;
        self.consume(Semicolon, "Expected ';' after the value.")?;

        Ok(Statement::Print {
            expression: expression,
        })
    }

    fn expression_statement(&mut self) -> Result<Statement, String> {
        let expression = self.expression()?;
        self.consume(Semicolon, "Exprected ';' after the value.")?;

        Ok(Statement::Expression {
            expression: expression,
        })
    }

    pub fn expression(self: &mut Self) -> Result<Expression, String> {
        self.assignment()
    }

    pub fn assignment(&mut self) -> Result<Expression, String> {
        let expression = self.equality()?;

        if self.match_tokens(vec![Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            return match expression {
                Variable { token } => Ok(Assign {
                    name: token.lexeme,
                    value: Box::new(value),
                }),
                _ => Err(format!("Invalid assignment target {}", equals.lexeme)),
            };
        }
        Ok(expression)
    }

    fn equality(self: &mut Self) -> Result<Expression, String> {
        let mut expression: Expression = self.comparison()?;

        while self.match_tokens(vec![BangEqual, EqualEqual]) {
            let operator = self.previous();

            let right: Expression = self.comparison()?;

            expression = Binary {
                left: Box::new(expression),
                operator: operator,
                right: Box::new(right),
            }
        }

        Ok(expression)
    }

    fn comparison(self: &mut Self) -> Result<Expression, String> {
        let mut expression: Expression = self.term()?;

        while self.match_tokens(vec![Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous();

            let right = self.term()?;

            expression = Binary {
                left: Box::new(expression),
                operator: operator,
                right: Box::new(right),
            }
        }

        Ok(expression)
    }

    fn term(self: &mut Self) -> Result<Expression, String> {
        let mut expression = self.factor()?;

        while self.match_tokens(vec![Minus, Plus]) {
            let operator = self.previous();

            let right = self.factor()?;

            expression = Binary {
                left: Box::new(expression),
                operator: operator,
                right: Box::new(right),
            }
        }

        Ok(expression)
    }

    fn factor(self: &mut Self) -> Result<Expression, String> {
        let mut expression = self.unary()?;

        while self.match_tokens(vec![Slash, Star]) {
            let operator = self.previous();
            let right = self.unary()?;

            expression = Binary {
                left: Box::new(expression),
                operator: operator,
                right: Box::new(right),
            }
        }

        Ok(expression)
    }

    fn unary(self: &mut Self) -> Result<Expression, String> {
        if self.match_tokens(vec![Bang, Minus]) {
            let operator = self.previous();
            let right = self.unary()?;

            return Ok(Unary {
                operator: operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(self: &mut Self) -> Result<Expression, String> {
        if self.match_tokens(vec![False]) {
            return Ok(Literal {
                value: LiteralValue::False,
            });
        }

        if self.match_tokens(vec![True]) {
            return Ok(Literal {
                value: LiteralValue::True,
            });
        }
        if self.match_tokens(vec![Nil]) {
            return Ok(Literal {
                value: LiteralValue::Nil,
            });
        }

        if self.match_tokens(vec![Identifier]) {
            return Ok(Variable {
                token: self.previous(),
            });
        }

        if self.match_tokens(vec![LeftParen]) {
            let expression = self.expression()?;

            self.consume(RightParen, "Expected )")?;

            return Ok(Grouping {
                group: Box::new(expression),
            });
        }

        if self.match_tokens(vec![String, Number]) {
            let token: Token = self.previous();
            return Ok(Literal {
                value: LiteralValue::from_token(token),
            });
        }

        Err(String::from(format!(
            "Expected expression at line: {}, literal: {1}",
            self.peek().line,
            self.peek().lexeme
        )))
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == Semicolon {
                return;
            }
            match self.peek().token_type {
                Class | Fun | Var | For | If | While | Print | Return => return,
                _ => {}
            }

            self.advance();
        }
    }

    // endregion

    // region helper functions

    fn match_tokens(self: &mut Self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == Eof
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn advance(self: &mut Self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn consume(self: &mut Self, token_type: TokenType, message: &str) -> Result<Token, String> {
        let token = self.peek();

        if token.token_type != token_type {
            return Err(message.to_string());
        }

        self.advance();
        Ok(token)
    }

    // endregion
}
