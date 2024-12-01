use crate::expr::Expression;
use crate::expr::Expression::*;
use crate::expr::LiteralValue;
use crate::token::Token;
use crate::token::TokenType;
use crate::token::TokenType::*;

struct Parser {
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

    pub fn expression(self: &mut Self) -> Expression {
        self.equality()
    }

    fn equality(self: &mut Self) -> Expression {
        let mut expression: Expression = self.comparison();

        while self.match_tokens(vec![BANG_EQUAL, EQUAL_EQUAL]) {
            let operator = self.previous();

            let right: Expression = self.comparison();

            expression = Binary {
                left: Box::new(expression),
                operator: operator,
                right: Box::new(right),
            }
        }

        expression
    }

    fn comparison(self: &mut Self) -> Expression {
        let mut expression: Expression = self.term();

        while self.match_tokens(vec![GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let operator = self.previous();

            let right = self.term();

            expression = Binary {
                left: Box::new(expression),
                operator: operator,
                right: Box::new(right),
            }
        }

        expression
    }

    fn term(self: &mut Self) -> Expression {
        let mut expression = self.factor();

        while self.match_tokens(vec![MINUS, PLUS]) {
            let operator = self.previous();

            let right = self.factor();

            expression = Binary {
                left: Box::new(expression),
                operator: operator,
                right: Box::new(right),
            }
        }

        expression
    }

    fn factor(self: &mut Self) -> Expression {
        let mut expression = self.unary();

        while self.match_tokens(vec![SLASH, STAR]) {
            let operator = self.previous();
            let right = self.unary();

            expression = Binary {
                left: Box::new(expression),
                operator: operator,
                right: Box::new(right),
            }
        }

        expression
    }

    fn unary(self: &mut Self) -> Expression {
        if self.match_tokens(vec![BANG, MINUS]) {
            let operator = self.previous();
            let right = self.unary();

            return Unary {
                operator: operator,
                right: Box::new(right),
            };
        }

        return self.primary();
    }

    fn primary(self: &mut Self) -> Expression {
        if self.match_tokens(vec![FALSE]) {
            return Literal {
                value: LiteralValue::False,
            };
        }

        if self.match_tokens(vec![TRUE]) {
            return Literal {
                value: LiteralValue::True,
            };
        }
        if self.match_tokens(vec![NIL]) {
            return Literal {
                value: LiteralValue::Nil,
            };
        }

        if self.match_tokens(vec![LEFT_PAREN]) {
            let expression = self.expression();

            self.consume(RIGHT_PAREN, "Expected )");

            return Grouping {
                group: Box::new(expression),
            };
        }

        if self.match_tokens(vec![STRING, NUMBER]) {
            let token: Token = self.previous();
            return Literal {
                value: LiteralValue::from_token(token),
            };
        }

        // fix this
        return Literal {
            value: LiteralValue::Nil,
        };
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
        self.peek().token_type == EOF
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

    fn consume(self: &mut Self, token_type: TokenType, message: &str) {
        let token = self.peek();

        if token.token_type == token_type {
            self.advance();
        } else {
            panic!("{}", message);
        }
    }

    // endregion
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::Scanner;
    use crate::token::LiteralValue;

    #[test]
    fn test_addition() {
        let tokens = vec![
            Token::new(NUMBER, "1".to_string(), Some(LiteralValue::IntValue(1)), 0),
            Token::new(PLUS, "+".to_string(), None, 0),
            Token::new(NUMBER, "2".to_string(), Some(LiteralValue::IntValue(2)), 0),
            Token::new(SEMICOLON, ";".to_string(), None, 0),
        ];

        let mut parser = Parser::new(tokens);

        let parsed_expression = parser.expression();

        assert_eq!(parsed_expression.to_string(), "(+ 1 2)");
    }

    #[test]
    fn test_comparison() {
        let source = "1 + 2 == 5 + 7";
        let mut scanner: Scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);

        let expression = parser.expression();

        let string_expression = expression.to_string();

        assert_eq!(string_expression, "(== (+ 1 2) (+ 5 7))");
    }

    #[test]
    fn test_bang_operator() {
        let source = "!!2";
        let mut scanner: Scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);

        let expression = parser.expression();

        let string_expression = expression.to_string();

        assert_eq!(string_expression, "(! (! 2))");
    }

    #[test]
    fn test_factor_and_term_operators() {
        let source = "12 / 2 + 3";
        let mut scanner: Scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);

        let expression = parser.expression();

        let string_expression = expression.to_string();

        assert_eq!(string_expression, "(+ (/ 12 2) 3)");
    }
}
