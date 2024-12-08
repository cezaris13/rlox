#[cfg(test)]
mod tests {
    use crate::expression_literal_value::LiteralValue;
    use crate::token::{Token, TokenType};

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
}
