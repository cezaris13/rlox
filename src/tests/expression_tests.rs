#[cfg(test)]
mod tests {
    use crate::environment::Environment;
    use crate::expression::Expression;
    use crate::expression::Expression::*;
    use crate::expression::LiteralValue;
    use crate::expression::LiteralValue::*;
    use crate::token::Token;
    use crate::token::TokenType;
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
    fn test_bang_operator() {
        let sources = vec!["!0", "!0.0", "!\"hello\"", "!nil", "!!true", "!true"];

        let responses: Vec<Result<LiteralValue, String>> = vec![
            Ok(LiteralValue::True),
            Ok(LiteralValue::True),
            Ok(LiteralValue::False),
            Ok(LiteralValue::True),
            Ok(LiteralValue::True),
            Ok(LiteralValue::False),
        ];

        let evaluated_expressions = evaluate_list_of_sources(&sources);

        assert_eq!(evaluated_expressions, responses);
    }

    #[test]
    fn test_minus_unary_operator() {
        let sources = vec!["-1", "-1.0", "--12", "-true"];

        let responses: Vec<Result<LiteralValue, String>> = vec![
            Ok(IntValue(-1)),
            Ok(FValue(-1.0)),
            Ok(IntValue(12)),
            Err(String::from("Minus not implemented for Bool")),
        ];

        let evaluated_expressions = evaluate_list_of_sources(&sources);

        assert_eq!(evaluated_expressions, responses);
    }

    #[test]
    fn evaluate_group() {
        let mut environment = Environment::new();
        let source = "( 1 + 2 )";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.expression();

        assert!(expression.is_ok());
        let evaluation = expression.unwrap().evaluate(&mut environment);

        assert_eq!(evaluation.unwrap(), IntValue(3));
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
    fn literal_value_to_string() {
        let literals = vec![
            LiteralValue::Nil,
            LiteralValue::False,
            LiteralValue::True,
            LiteralValue::IntValue(12),
            LiteralValue::StringValue(String::from("Hello")),
            LiteralValue::FValue(1.1),
        ];

        let responses = vec!["nil", "false", "true", "12", "Hello", "1.1"]
            .iter()
            .map(|response| String::from(*response))
            .collect::<Vec<String>>();

        let result = literals
            .iter()
            .map(|literal| literal.to_string())
            .collect::<Vec<String>>();

        assert_eq!(result, responses);
    }

    #[test]
    fn literal_value_to_type() {
        let literals = vec![
            LiteralValue::Nil,
            LiteralValue::False,
            LiteralValue::True,
            LiteralValue::IntValue(12),
            LiteralValue::StringValue(String::from("Hello")),
            LiteralValue::FValue(1.1),
        ];

        let responses = vec!["Nil", "Bool", "Bool", "Int", "String", "Float"];

        let result = literals
            .iter()
            .map(|literal| literal.to_type())
            .collect::<Vec<&str>>();

        assert_eq!(result, responses);
    }

    #[test]
    fn test_plus_operator() {
        let sources = vec![
            "5+5",
            "5+5.5",
            "5.5+5",
            "5.5+5.5",
            "\"a\" + \"a\"",
            "\"a\" + 5",
            "\"a\" + 5.5",
            "5.5 + \"a\"",
            "true + false",
        ];

        let responses: Vec<Result<LiteralValue, String>> = vec![
            Ok(IntValue(10)),
            Ok(FValue(10.5)),
            Ok(FValue(10.5)),
            Ok(FValue(11.0)),
            Ok(StringValue(String::from("aa"))),
            Ok(StringValue(String::from("a5"))),
            Ok(StringValue(String::from("a5.5"))),
            Ok(StringValue(String::from("5.5a"))),
            Err(String::from(
                "Plus operation is not implemented for: \"true\" and \"false\"",
            )),
        ];

        let evaluated_expressions = evaluate_list_of_sources(&sources);

        assert_eq!(evaluated_expressions, responses);
    }

    #[test]
    fn test_minus_operator() {
        let sources = vec!["5-5", "5-5.5", "5.5-5", "6.0-5.5", "\"a\" - \"a\""];

        let responses: Vec<Result<LiteralValue, String>> = vec![
            Ok(IntValue(0)),
            Ok(FValue(-0.5)),
            Ok(FValue(0.5)),
            Ok(FValue(0.5)),
            Err(String::from(
                "Minus operation is not implemented for: \"a\" and \"a\"",
            )),
        ];

        let evaluated_expressions = evaluate_list_of_sources(&sources);

        assert_eq!(evaluated_expressions, responses);
    }

    #[test]
    fn test_star_operator() {
        let sources = vec!["5*5", "5*5.5", "5.5*5", "6.0*5.5", "\"a\" * \"a\""];

        let responses: Vec<Result<LiteralValue, String>> = vec![
            Ok(IntValue(25)),
            Ok(FValue(27.5)),
            Ok(FValue(27.5)),
            Ok(FValue(33.0)),
            Err(String::from(
                "Star operation is not implemented for: \"a\" and \"a\"",
            )),
        ];

        let evaluated_expressions = evaluate_list_of_sources(&sources);

        assert_eq!(evaluated_expressions, responses);
    }

    #[test]
    fn test_slash_operator() {
        let sources = vec![
            "5/5",
            "9/1.5",
            "5.5/5",
            "27.5/5.5",
            "\"a\" / \"a\"",
            "5/0",
            "5/0.0",
        ];

        let responses: Vec<Result<LiteralValue, String>> = vec![
            Ok(IntValue(1)),
            Ok(FValue(6.0)),
            Ok(FValue(1.1)),
            Ok(FValue(5.0)),
            Err(String::from(
                "Slash operation is not implemented for: \"a\" and \"a\"",
            )),
            Err(String::from("Division by 0")),
            Err(String::from("Division by 0")),
        ];

        let evaluated_expressions = evaluate_list_of_sources(&sources);

        assert_eq!(evaluated_expressions, responses);
    }

    #[test]
    fn test_less_operator() {
        let sources = vec![
            "5<5",
            "5<5.5",
            "5.5<5",
            "6.0<5.5",
            "\"a\" < \"a\"",
            "false < true",
        ];

        let responses: Vec<Result<LiteralValue, String>> = vec![
            Ok(LiteralValue::False),
            Ok(LiteralValue::True),
            Ok(LiteralValue::False),
            Ok(LiteralValue::False),
            Ok(LiteralValue::False),
            Err(String::from(
                "Less operation is not implemented for: \"false\" and \"true\"",
            )),
        ];

        let evaluated_expressions = evaluate_list_of_sources(&sources);

        assert_eq!(evaluated_expressions, responses);
    }

    #[test]
    fn test_less_equal_operator() {
        let sources = vec![
            "5<=5",
            "5<=5.5",
            "5.5<=5",
            "6.0<=5.5",
            "\"a\" <= \"a\"",
            "false <= true",
        ];

        let responses: Vec<Result<LiteralValue, String>> = vec![
            Ok(LiteralValue::True),
            Ok(LiteralValue::True),
            Ok(LiteralValue::False),
            Ok(LiteralValue::False),
            Ok(LiteralValue::True),
            Err(String::from(
                "LessEqual operation is not implemented for: \"false\" and \"true\"",
            )),
        ];

        let evaluated_expressions = evaluate_list_of_sources(&sources);

        assert_eq!(evaluated_expressions, responses);
    }

    #[test]
    fn test_greater_operator() {
        let sources = vec![
            "5>5",
            "5>5.5",
            "5.5>5",
            "6.0>5.5",
            "\"a\" > \"a\"",
            "false > true",
        ];

        let responses: Vec<Result<LiteralValue, String>> = vec![
            Ok(LiteralValue::False),
            Ok(LiteralValue::False),
            Ok(LiteralValue::True),
            Ok(LiteralValue::True),
            Ok(LiteralValue::False),
            Err(String::from(
                "Greater operation is not implemented for: \"false\" and \"true\"",
            )),
        ];

        let evaluated_expressions = evaluate_list_of_sources(&sources);

        assert_eq!(evaluated_expressions, responses);
    }

    #[test]
    fn test_greater_equal_operator() {
        let sources = vec![
            "5>=5",
            "5>=5.5",
            "5.5>=5",
            "6.0>=5.5",
            "\"a\" >= \"a\"",
            "false >= true",
        ];

        let responses: Vec<Result<LiteralValue, String>> = vec![
            Ok(LiteralValue::True),
            Ok(LiteralValue::False),
            Ok(LiteralValue::True),
            Ok(LiteralValue::True),
            Ok(LiteralValue::True),
            Err(String::from(
                "GreaterEqual operation is not implemented for: \"false\" and \"true\"",
            )),
        ];

        let evaluated_expressions = evaluate_list_of_sources(&sources);

        assert_eq!(evaluated_expressions, responses);
    }

    #[test]
    fn test_equal_operator() {
        let sources = vec!["5==5", "5!=5.5", "\"a\" ==\"a\""];

        let responses: Vec<Result<LiteralValue, String>> = vec![
            Ok(LiteralValue::True),
            Ok(LiteralValue::True),
            Ok(LiteralValue::True),
        ];

        let evaluated_expressions = evaluate_list_of_sources(&sources);

        assert_eq!(evaluated_expressions, responses);
    }

    #[test]
    fn test_from_token() {
        let tokens = vec![
            Token::new(TokenType::False, "".to_string(), None, 0),
            Token::new(TokenType::True, "".to_string(), None, 0),
            Token::new(TokenType::Nil, "".to_string(), None, 0),
            Token::new(
                TokenType::Number,
                "12".to_string(),
                Some(crate::token::LiteralValue::IntValue(12)),
                0,
            ),
            Token::new(
                TokenType::String,
                "hello".to_string(),
                Some(crate::token::LiteralValue::StringValue(String::from(
                    "hello",
                ))),
                0,
            ),
            Token::new(
                TokenType::String,
                "hello".to_string(),
                Some(crate::token::LiteralValue::IdentifierValue(String::from(
                    "hello",
                ))),
                0,
            ),
        ];

        let responses: Vec<LiteralValue> = vec![
            LiteralValue::False,
            LiteralValue::True,
            LiteralValue::Nil,
            LiteralValue::IntValue(12),
            LiteralValue::StringValue(String::from("hello")),
            LiteralValue::StringValue(String::from("hello")),
        ];

        let evaluated_expressions = tokens
            .iter()
            .map(|token| LiteralValue::from_token(token.clone()))
            .collect::<Vec<LiteralValue>>();

        assert_eq!(evaluated_expressions, responses);
    }

    fn evaluate_list_of_sources(sources: &Vec<&str>) -> Vec<Result<LiteralValue, String>> {
        sources
            .iter()
            .map(|source| {
                let mut scanner: Scanner = Scanner::new(source);
                let tokens = scanner.scan_tokens().unwrap();
                let mut parser = Parser::new(tokens);
                let expression = parser.expression().unwrap();
                let mut environment = Environment::new();
                expression.evaluate(&mut environment)
            })
            .collect::<Vec<Result<LiteralValue, String>>>()
    }
}
