#[cfg(test)]
mod tests {
    use crate::scanner::Scanner;
    use crate::token::LiteralValue::*;
    use crate::token::TokenType::*;

    #[test]
    fn handler_one_char_tokens() {
        let source = "((  ))";

        let mut scanner = Scanner::new(source);

        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 5); // due to Eof token
        assert_eq!(scanner.tokens[0].token_type, LeftParen);
        assert_eq!(scanner.tokens[1].token_type, LeftParen);
        assert_eq!(scanner.tokens[2].token_type, RightParen);
        assert_eq!(scanner.tokens[3].token_type, RightParen);
        assert_eq!(scanner.tokens[4].token_type, Eof);
    }

    #[test]
    fn handler_simple_tokens() {
        let source = "! != == >= <= < > ,";

        let mut scanner = Scanner::new(source);

        let _ = scanner.scan_tokens();

        assert_eq!(scanner.tokens.len(), 9); // due to Eof token
        assert_eq!(scanner.tokens[0].token_type, Bang);
        assert_eq!(scanner.tokens[1].token_type, BangEqual);
        assert_eq!(scanner.tokens[2].token_type, EqualEqual);
        assert_eq!(scanner.tokens[3].token_type, GreaterEqual);
        assert_eq!(scanner.tokens[4].token_type, LessEqual);
        assert_eq!(scanner.tokens[5].token_type, Less);
        assert_eq!(scanner.tokens[6].token_type, Greater);
        assert_eq!(scanner.tokens[7].token_type, Comma);
        assert_eq!(scanner.tokens[8].token_type, Eof);
    }

    #[test]
    fn handler_string_literal() {
        let source = "\"ABC\"";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_ok());
        assert_eq!(scanner.tokens.len(), 2); // due to Eof token
        assert_eq!(scanner.tokens[0].token_type, String);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            StringValue(val) => assert_eq!(val, "ABC"),
            _ => panic!("Incorrect literal"),
        }

        assert_eq!(scanner.tokens[1].token_type, Eof);
    }

    #[test]
    fn handler_string_literal_not_closed_returns_error() {
        let source = "\"AB";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_err());
        assert_eq!(
            result.err(),
            Some("Unterminated string at line 1".to_string())
        );
    }

    #[test]
    fn handler_numeral_trailing_dot_returns_int() {
        let source = "123.";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_ok());
        assert_eq!(scanner.tokens[0].token_type, Number);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            IntValue(val) => assert_eq!(*val, 123),
            _ => panic!("Incorrect literal"),
        }
    }

    #[test]
    fn handler_float_numeral_returns_float_token() {
        let source = "123.15";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_ok());
        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].token_type, Number);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            FValue(val) => assert_eq!(*val, 123.15),
            _ => panic!("Incorrect literal"),
        }

        assert_eq!(scanner.tokens[1].token_type, Eof);
    }

    #[test]
    fn handler_int_numeral_returns_int_token() {
        let source = "123";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_ok());
        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].token_type, Number);
        match scanner.tokens[0].literal.as_ref().unwrap() {
            IntValue(val) => assert_eq!(*val, 123),
            _ => panic!("Incorrect literal"),
        }

        assert_eq!(scanner.tokens[1].token_type, Eof);
    }

    #[test]
    fn handler_multiline_comments_gets_cut_of() {
        let source = "/*some text\n\n\n*/";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_ok());
        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(scanner.tokens[0].token_type, Eof);
    }

    #[test]
    fn handler_multiline_comments_unclosed_comment_returns_error() {
        let source = "/*some text\n\n";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_err());
        assert_eq!(
            result.err(),
            Some("Unterminated multiline comment at line 3".to_string())
        );
        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(scanner.tokens[0].token_type, Eof);
    }

    #[test]
    fn handler_multiline_comments_extra_end_comment_returns_error() {
        let source = "*/";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_err());
        assert_eq!(
            result.err(),
            Some("Extra multiline ending comment at line 1".to_string())
        );
        assert_eq!(scanner.tokens.len(), 1);
        assert_eq!(scanner.tokens[0].token_type, Eof);
    }

    #[test]
    fn handler_keywords_returns_keyword_token() {
        let source = "class var";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_ok());
        assert_eq!(scanner.tokens.len(), 3);
        assert_eq!(scanner.tokens[0].token_type, Class);
        assert_eq!(scanner.tokens[1].token_type, Var);
        assert_eq!(scanner.tokens[2].token_type, Eof);
    }

    #[test]
    fn handler_keywords_unindentified_token_returns_identifier_token() {
        let source = "bigvar";

        let mut scanner = Scanner::new(source);

        let result = scanner.scan_tokens();

        assert!(result.is_ok());
        assert_eq!(scanner.tokens.len(), 2);
        assert_eq!(scanner.tokens[0].token_type, Identifier);
        assert_eq!(scanner.tokens[0].lexeme, "bigvar");
        assert_eq!(scanner.tokens[1].token_type, Eof);
    }

    #[test]
    fn get_keywords() {
        let source = "var this_is_a_var = 12;\nwhile true { print 3 };";
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens();

        assert!(result.is_ok());

        assert_eq!(scanner.tokens.len(), 13);

        assert_eq!(scanner.tokens[0].token_type, Var);
        assert_eq!(scanner.tokens[1].token_type, Identifier);
        assert_eq!(scanner.tokens[2].token_type, Equal);
        assert_eq!(scanner.tokens[3].token_type, Number);
        assert_eq!(scanner.tokens[4].token_type, Semicolon);
        assert_eq!(scanner.tokens[5].token_type, While);
        assert_eq!(scanner.tokens[6].token_type, True);
        assert_eq!(scanner.tokens[7].token_type, LeftBrace);
        assert_eq!(scanner.tokens[8].token_type, Print);
        assert_eq!(scanner.tokens[9].token_type, Number);
        assert_eq!(scanner.tokens[10].token_type, RightBrace);
        assert_eq!(scanner.tokens[11].token_type, Semicolon);
        assert_eq!(scanner.tokens[12].token_type, Eof);
    }

    #[test]
    fn scan_tokens_unexpected_character_returns_error() {
        let source = "&";
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens();

        assert!(result.is_err());

        assert_eq!(
            result.err(),
            Some("Unexpected character & at line 1".to_string())
        );
    }

    #[test]
    fn scan_tokens_double_star_returns_as_expected() {
        let source = "**";
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens();

        assert!(result.is_ok());

        assert_eq!(scanner.tokens.len(), 3);

        assert_eq!(scanner.tokens[0].token_type, Star);
        assert_eq!(scanner.tokens[1].token_type, Star);
        assert_eq!(scanner.tokens[2].token_type, Eof);
    }

    #[test]
    fn scan_one_line_comments_updates_line_number() {
        let source = "// comment \n // comment";
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens();

        assert!(result.is_ok());

        assert_eq!(scanner.tokens.len(), 1);

        assert_eq!(scanner.tokens[0].token_type, Eof);
    }

    #[test]
    fn scan_multi_line_string_returns_string() {
        let source = "\"hello\nworld\"";
        let mut scanner = Scanner::new(source);
        let result = scanner.scan_tokens();

        assert!(result.is_ok());

        assert_eq!(scanner.tokens.len(), 2);

        assert_eq!(scanner.tokens[0].token_type, String);
        assert_eq!(scanner.tokens[1].token_type, Eof);
    }
}
