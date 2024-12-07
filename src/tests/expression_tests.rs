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
    fn pretty_print_logical() {
        let expression: Expression = Logical {
            left: Box::new(Unary {
                operator: Token::new(Minus, String::from("-"), None, 1),
                right: Box::new(Literal {
                    value: IntValue(123),
                }),
            }),
            operator: Token::new(Or, String::from("or"), None, 1),
            right: Box::new(Grouping {
                group: Box::new(Literal {
                    value: FValue(45.67),
                }),
            }),
        };

        let result = expression.to_string();
        assert_eq!(result, "(or (- 123) (group 45.67))");
    }

    #[test]
    fn pretty_print_variable() {
        let expression: Expression = Variable {
            token: Token {
                token_type: Identifier,
                lexeme: String::from("a"),
                literal: None,
                line: 1,
            },
        };

        let result = expression.to_string();
        assert_eq!(result, "(defvar a)");
    }

    #[test]
    fn pretty_print_variable_with_value() {
        let expression: Expression = Variable {
            token: Token {
                token_type: String,
                lexeme: String::from("a"),
                literal: Some(crate::token::LiteralValue::StringValue(String::from(
                    "hello",
                ))),
                line: 1,
            },
        };

        let result = expression.to_string();
        assert_eq!(result, "(defvar a \"hello\")");
    }

    #[test]
    fn pretty_print_assignment() {
        let expression: Expression = Assign {
            name: String::from("a"),
            value: Box::new(Literal {
                value: IntValue(12),
            }),
        };

        let result = expression.to_string();
        assert_eq!(result, "(= a 12)");
    }

    #[test]
    fn test_bang_operator() {
        let test_cases: Vec<(&str, Result<LiteralValue, String>)> = vec![
            ("!0", Ok(LiteralValue::True)),
            ("!0.0", Ok(LiteralValue::True)),
            ("!\"hello\"", Ok(LiteralValue::False)),
            ("!nil", Ok(LiteralValue::True)),
            ("!!true", Ok(LiteralValue::True)),
            ("!true", Ok(LiteralValue::False)),
        ];

        let inputs = get_inputs(&test_cases);
        let expected_results = get_expected_results(&test_cases);

        let results = evaluate_list_of_sources(&inputs);

        assert_eq!(results, expected_results);
    }

    #[test]
    fn test_minus_unary_operator() {
        let test_cases: Vec<(&str, Result<LiteralValue, String>)> = vec![
            ("-1", Ok(IntValue(-1))),
            ("-1.0", Ok(FValue(-1.0))),
            ("--12", Ok(IntValue(12))),
            ("-true", Err(String::from("Minus not implemented for Bool"))),
        ];

        let inputs = get_inputs(&test_cases);
        let expected_results = get_expected_results(&test_cases);

        let results = evaluate_list_of_sources(&inputs);

        assert_eq!(results, expected_results);
    }

    #[test]
    fn evaluate_group() {
        let source = "( 1 + 2 )";

        let mut environment = Environment::new();
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
        let test_cases: Vec<(&str, Result<LiteralValue, String>)> = vec![
            ("5+5", Ok(IntValue(10))),
            ("5+5.5", Ok(FValue(10.5))),
            ("5.5+5", Ok(FValue(10.5))),
            ("5.5+5.5", Ok(FValue(11.0))),
            ("\"a\" + \"a\"", Ok(StringValue(String::from("aa")))),
            ("\"a\" + 5", Ok(StringValue(String::from("a5")))),
            ("\"a\" + 5.5", Ok(StringValue(String::from("a5.5")))),
            ("5.5 + \"a\"", Ok(StringValue(String::from("5.5a")))),
            (
                "true + false",
                Err(String::from(
                    "Plus operation is not implemented for: \"true\" and \"false\"",
                )),
            ),
        ];

        let inputs = get_inputs(&test_cases);
        let expected_results = get_expected_results(&test_cases);

        let results = evaluate_list_of_sources(&inputs);

        assert_eq!(results, expected_results);
    }

    #[test]
    fn test_minus_operator() {
        let test_cases: Vec<(&str, Result<LiteralValue, String>)> = vec![
            ("5-5", Ok(IntValue(0))),
            ("5-5.5", Ok(FValue(-0.5))),
            ("5.5-5", Ok(FValue(0.5))),
            ("6.0-5.5", Ok(FValue(0.5))),
            (
                "\"a\" - \"a\"",
                Err(String::from(
                    "Minus operation is not implemented for: \"a\" and \"a\"",
                )),
            ),
        ];

        let inputs = get_inputs(&test_cases);
        let expected_results = get_expected_results(&test_cases);

        let results = evaluate_list_of_sources(&inputs);

        assert_eq!(results, expected_results);
    }

    #[test]
    fn test_star_operator() {
        let test_cases: Vec<(&str, Result<LiteralValue, String>)> = vec![
            ("5*5", Ok(IntValue(25))),
            ("5*5.5", Ok(FValue(27.5))),
            ("5.5*5", Ok(FValue(27.5))),
            ("6.0*5.5", Ok(FValue(33.0))),
            (
                "\"a\" * \"a\"",
                Err(String::from(
                    "Star operation is not implemented for: \"a\" and \"a\"",
                )),
            ),
        ];

        let inputs = get_inputs(&test_cases);
        let expected_results = get_expected_results(&test_cases);

        let results = evaluate_list_of_sources(&inputs);

        assert_eq!(results, expected_results);
    }

    #[test]
    fn test_slash_operator() {
        let test_cases: Vec<(&str, Result<LiteralValue, String>)> = vec![
            ("5/5", Ok(IntValue(1))),
            ("9/1.5", Ok(FValue(6.0))),
            ("5.5/5", Ok(FValue(1.1))),
            ("27.5/5.5", Ok(FValue(5.0))),
            (
                "\"a\" / \"a\"",
                Err(String::from(
                    "Slash operation is not implemented for: \"a\" and \"a\"",
                )),
            ),
            ("5/0", Err(String::from("Division by 0"))),
            ("5/0.0", Err(String::from("Division by 0"))),
        ];

        let inputs = get_inputs(&test_cases);
        let expected_results = get_expected_results(&test_cases);

        let results = evaluate_list_of_sources(&inputs);

        assert_eq!(results, expected_results);
    }

    #[test]
    fn test_less_operator() {
        let test_cases: Vec<(&str, Result<LiteralValue, String>)> = vec![
            ("5<5", Ok(LiteralValue::False)),
            ("5<5.5", Ok(LiteralValue::True)),
            ("5.5<5", Ok(LiteralValue::False)),
            ("6.0<5.5", Ok(LiteralValue::False)),
            ("\"a\" < \"a\"", Ok(LiteralValue::False)),
            (
                "false < true",
                Err(String::from(
                    "Less operation is not implemented for: \"false\" and \"true\"",
                )),
            ),
        ];

        let inputs = get_inputs(&test_cases);
        let expected_results = get_expected_results(&test_cases);

        let results = evaluate_list_of_sources(&inputs);

        assert_eq!(results, expected_results);
    }

    #[test]
    fn test_less_equal_operator() {
        let test_cases: Vec<(&str, Result<LiteralValue, String>)> = vec![
            ("5<=5", Ok(LiteralValue::True)),
            ("5<=5.5", Ok(LiteralValue::True)),
            ("5.5<=5", Ok(LiteralValue::False)),
            ("6.0<=5.5", Ok(LiteralValue::False)),
            ("\"a\" <= \"a\"", Ok(LiteralValue::True)),
            (
                "false <= true",
                Err(String::from(
                    "LessEqual operation is not implemented for: \"false\" and \"true\"",
                )),
            ),
        ];

        let inputs = get_inputs(&test_cases);
        let expected_results = get_expected_results(&test_cases);

        let results = evaluate_list_of_sources(&inputs);

        assert_eq!(results, expected_results);
    }

    #[test]
    fn test_greater_operator() {
        let test_cases: Vec<(&str, Result<LiteralValue, String>)> = vec![
            ("5>5", Ok(LiteralValue::False)),
            ("5>5.5", Ok(LiteralValue::False)),
            ("5.5>5", Ok(LiteralValue::True)),
            ("6.0>5.5", Ok(LiteralValue::True)),
            ("\"a\" > \"a\"", Ok(LiteralValue::False)),
            (
                "false > true",
                Err(String::from(
                    "Greater operation is not implemented for: \"false\" and \"true\"",
                )),
            ),
        ];

        let inputs = get_inputs(&test_cases);
        let expected_results = get_expected_results(&test_cases);

        let results = evaluate_list_of_sources(&inputs);

        assert_eq!(results, expected_results);
    }

    #[test]
    fn test_greater_equal_operator() {
        let test_cases: Vec<(&str, Result<LiteralValue, String>)> = vec![
            ("5>=5", Ok(LiteralValue::True)),
            ("5>=5.5", Ok(LiteralValue::False)),
            ("5.5>=5", Ok(LiteralValue::True)),
            ("6.0>=5.5", Ok(LiteralValue::True)),
            ("\"a\" >= \"a\"", Ok(LiteralValue::True)),
            (
                "false >= true",
                Err(String::from(
                    "GreaterEqual operation is not implemented for: \"false\" and \"true\"",
                )),
            ),
        ];

        let inputs = get_inputs(&test_cases);
        let expected_results = get_expected_results(&test_cases);

        let results = evaluate_list_of_sources(&inputs);

        assert_eq!(results, expected_results);
    }

    #[test]
    fn test_equal_operator() {
        let test_cases: Vec<(&str, Result<LiteralValue, String>)> = vec![
            ("5==5", Ok(LiteralValue::True)),
            ("5!=5.5", Ok(LiteralValue::True)),
            ("\"a\" ==\"a\"", Ok(LiteralValue::True)),
        ];

        let inputs = get_inputs(&test_cases);
        let expected_results = get_expected_results(&test_cases);

        let results = evaluate_list_of_sources(&inputs);

        assert_eq!(results, expected_results);
    }

    #[test]
    fn test_logical_operators() {
        let test_cases: Vec<(&str, Result<LiteralValue, String>)> = vec![
            ("\"hi\" or 2", Ok(StringValue(String::from("hi")))),
            ("nil or \"yes\"", Ok(StringValue(String::from("yes")))),
            ("5.5 and 5", Ok(IntValue(5))),
            ("0 and 5", Ok(IntValue(0))),
        ];

        let inputs = get_inputs(&test_cases);
        let expected_results = get_expected_results(&test_cases);

        let results = evaluate_list_of_sources(&inputs);

        assert_eq!(results, expected_results);
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
        ];

        let responses: Vec<LiteralValue> = vec![
            LiteralValue::False,
            LiteralValue::True,
            LiteralValue::Nil,
            LiteralValue::IntValue(12),
            LiteralValue::StringValue(String::from("hello")),
        ];

        let evaluated_expressions = tokens
            .iter()
            .map(|token| LiteralValue::from(token.clone()))
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

    fn get_inputs<'a>(
        test_cases: &'a Vec<(&'a str, Result<LiteralValue, String>)>,
    ) -> Vec<&'a str> {
        test_cases
            .iter()
            .map(|(input, _)| *input)
            .collect::<Vec<&str>>()
    }

    fn get_expected_results<'a>(
        test_cases: &'a Vec<(&'a str, Result<LiteralValue, String>)>,
    ) -> Vec<Result<LiteralValue, String>> {
        test_cases
            .iter()
            .map(|(_, output)| output.clone())
            .collect::<Vec<Result<LiteralValue, String>>>()
    }
}
