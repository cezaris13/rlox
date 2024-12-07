#[cfg(test)]
mod tests {
    use crate::expression::Expression::*;
    use crate::statement::Statement::{Block, Expression, Variable};
    use crate::token::TokenType::*;
    use crate::token::{LiteralValue, Token};
    use crate::Parser;
    use crate::Scanner;

    #[test]
    fn test_addition() {
        let tokens = vec![
            Token::new(Number, "1".to_string(), Some(LiteralValue::IntValue(1)), 0),
            Token::new(Plus, "+".to_string(), None, 0),
            Token::new(Number, "2".to_string(), Some(LiteralValue::IntValue(2)), 0),
            Token::new(Semicolon, ";".to_string(), None, 0),
        ];

        let mut parser = Parser::new(tokens);

        let parsed_expression = parser.expression();

        assert!(parsed_expression.is_ok());
        assert_eq!(parsed_expression.unwrap().to_string(), "(+ 1 2)");
    }

    #[test]
    fn test_comparison() {
        let source = "1 + 2 == 5 + 7";
        let mut scanner: Scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);

        let expression = parser.expression();

        assert!(expression.is_ok());
        let string_expression = expression.unwrap().to_string();

        assert_eq!(string_expression, "(== (+ 1 2) (+ 5 7))");
    }

    #[test]
    fn test_bang_operator() {
        let source = "!!2";
        let mut scanner: Scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);

        let expression = parser.expression();

        assert!(expression.is_ok());
        let string_expression = expression.unwrap().to_string();

        assert_eq!(string_expression, "(! (! 2))");
    }

    #[test]
    fn test_factor_and_term_operators() {
        let source = "12 / 2 + 3";
        let mut scanner: Scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);

        let expression = parser.expression();

        assert!(expression.is_ok());

        let string_expression = expression.unwrap().to_string();

        assert_eq!(string_expression, "(+ (/ 12 2) 3)");
    }

    #[test]
    fn test_variable_declaration_operators() {
        let source = "var some_id;";
        let mut scanner: Scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);

        let expression = parser.parse();
        let response = Variable {
            token: Token {
                token_type: Identifier,
                lexeme: std::string::String::from("some_id"),
                literal: None,
                line: 1,
            },
            initializer: Literal {
                value: crate::expression::LiteralValue::Nil,
            },
        };

        assert!(expression.is_ok());

        let string_expression = expression.unwrap();
        assert_eq!(string_expression.len(), 1);
        assert_eq!(string_expression[0], response);
    }

    #[test]
    fn test_variable_assignment_operators() {
        let source = "some_id = 2;";
        let mut scanner: Scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);

        let expression = parser.parse();
        let response = Expression {
            expression: Assign {
                name: std::string::String::from("some_id"),
                value: Box::new(Literal {
                    value: crate::expression::LiteralValue::IntValue(2),
                }),
            },
        };

        assert!(expression.is_ok());

        let string_expression = expression.unwrap();
        assert_eq!(string_expression.len(), 1);
        assert_eq!(string_expression[0], response);
    }

    #[test]
    fn test_print_operator() {
        let source = "print \"hello\";";
        let mut scanner: Scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);

        let expression = parser.parse();
        let response = crate::statement::Statement::Print {
            expression: Literal {
                value: crate::expression::LiteralValue::StringValue(std::string::String::from(
                    "hello",
                )),
            },
        };

        assert!(expression.is_ok());

        let string_expression = expression.unwrap();
        assert_eq!(string_expression.len(), 1);
        assert_eq!(string_expression[0], response);
    }

    #[test]
    fn test_blocks() {
        let source = "{a=1;}";
        let mut scanner: Scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);

        let expression = parser.parse();
        let response = Block {
            statements: vec![Expression {
                expression: Assign {
                    name: std::string::String::from("a"),
                    value: Box::new(Literal {
                        value: crate::expression::LiteralValue::IntValue(1),
                    }),
                },
            }],
        };

        assert!(expression.is_ok());

        let string_expression = expression.unwrap();
        assert_eq!(string_expression.len(), 1);
        assert_eq!(string_expression[0], response);
    }
}
