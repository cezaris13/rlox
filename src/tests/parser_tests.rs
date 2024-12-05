#[cfg(test)]
mod tests {
    use crate::parser::Parser;
    use crate::scanner::Scanner;
    use crate::token::TokenType::*;
    use crate::token::{LiteralValue, Token};

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
}
