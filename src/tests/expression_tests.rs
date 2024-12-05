#[cfg(test)]
mod tests {
    use crate::environment::Environment;
    use crate::expression::Expression;
    use crate::expression::Expression::*;
    use crate::expression::LiteralValue;
    use crate::expression::LiteralValue::*;
    use crate::token::Token;
    use crate::token::TokenType::*;
    use crate::Parser;
    use crate::Scanner;

    use std::string::String;

    #[test]
    fn pretty_print() {
        let expression: Expression = Binary {
            left: Box::new(Unary {
                operator: Token::new(Minus, String::from("-"), None, 1),
                right: Box::new(Literal {
                    value: IntValue(123),
                }),
            }),
            operator: Token::new(Star, String::from("*"), None, 1),
            right: Box::new(Grouping {
                group: Box::new(Literal {
                    value: FValue(45.67),
                }),
            }),
        };

        let result = expression.to_string();
        assert_eq!(result, "(* (- 123) (group 45.67))");
    }

    #[test]
    fn evaluate_bang_bang() {
        let mut environment = Environment::new();
        let source = "!!true";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.expression();

        assert!(expression.is_ok());
        let evaluation = expression.unwrap().evaluate(&mut environment);

        assert!(evaluation.is_ok());

        assert_eq!(evaluation.unwrap(), LiteralValue::True);
    }

    #[test]
    fn evaluate_bang() {
        let mut environment = Environment::new();
        let source = "!true";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.expression();

        assert!(expression.is_ok());
        let evaluation = expression.unwrap().evaluate(&mut environment);

        assert!(evaluation.is_ok());

        assert_eq!(evaluation.unwrap(), LiteralValue::False);
    }

    #[test]
    fn evaluate_minus_int() {
        let mut environment = Environment::new();
        let source = "-12";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.expression();

        assert!(expression.is_ok());
        let evaluation = expression.unwrap().evaluate(&mut environment);

        assert_eq!(evaluation.unwrap(), IntValue(-12));
    }

    #[test]
    fn evaluate_minus_double() {
        let mut environment = Environment::new();
        let source = "-12.0";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.expression();

        assert!(expression.is_ok());
        let evaluation = expression.unwrap().evaluate(&mut environment);

        assert_eq!(evaluation.unwrap(), FValue(-12.0));
    }

    #[test]
    fn evaluate_int_returns_sum() {
        let mut environment = Environment::new();
        let source = "5 + 2";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.expression();

        assert!(expression.is_ok());
        let evaluation = expression.unwrap().evaluate(&mut environment);

        assert_eq!(evaluation.unwrap(), IntValue(7));
    }

    #[test]
    fn evaluate_string_returns_sum() {
        let mut environment = Environment::new();
        let source = "\"hello \" + \"world\"";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.expression();

        assert!(expression.is_ok());
        let evaluation = expression.unwrap().evaluate(&mut environment);

        assert_eq!(
            evaluation.unwrap(),
            StringValue(String::from("hello world"))
        );
    }

    #[test]
    fn evaluate_float_and_int_returns_float_mult() {
        let mut environment = Environment::new();
        let source = "2 * 2.5";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.expression();

        assert!(expression.is_ok());
        let evaluation = expression.unwrap().evaluate(&mut environment);

        assert_eq!(evaluation.unwrap(), FValue(5.0));
    }

    #[test]
    fn evaluate_string_float_returns_sum() {
        let mut environment = Environment::new();
        let source = "\"hello \" + 2";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.expression();

        assert!(expression.is_ok());
        let evaluation = expression.unwrap().evaluate(&mut environment);

        assert_eq!(evaluation.unwrap(), StringValue(String::from("hello 2")));
    }

    #[test]
    fn evaluate_complex_int_float_returns_result() {
        let mut environment = Environment::new();
        let source = "2 * 2.5 + 5 / 2";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.expression();

        assert!(expression.is_ok());
        let evaluation = expression.unwrap().evaluate(&mut environment);

        assert_eq!(evaluation.unwrap(), FValue(7.0));
    }

    #[test]
    fn evaluate_comparison_of_strings_of_same_length() {
        let mut environment = Environment::new();
        let source = "\"ac\" < \"ab\"";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.expression();

        assert!(expression.is_ok());
        let evaluation = expression.unwrap().evaluate(&mut environment);

        assert_eq!(evaluation.unwrap(), LiteralValue::False);
    }
}
