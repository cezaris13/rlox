#[cfg(test)]
mod tests {
    use crate::expression_literal_value::LiteralValue;
    use crate::Interpreter;
    use crate::Parser;
    use crate::Scanner;

    #[test]
    fn expression_test() {
        let source = "12+5;";

        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut interpreter: Interpreter = Interpreter::new();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());
        assert!(interpreter.environment.borrow().values.is_empty());
    }

    #[test]
    fn expression_initializer_with_provided_value_test() {
        let source = "var a = 12;";

        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut interpreter: Interpreter = Interpreter::new();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());
        assert_eq!(interpreter.environment.borrow().values.is_empty(), false);
        assert_eq!(interpreter.environment.borrow().values.len(), 1);
        assert_eq!(
            interpreter.environment.borrow().get("a"),
            Ok(LiteralValue::IntValue(12))
        );
    }

    #[test]
    fn expression_test_no_initializer_value() {
        let source = "var a;";

        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut interpreter: Interpreter = Interpreter::new();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());

        assert_eq!(interpreter.environment.borrow().values.is_empty(), false);
        assert_eq!(interpreter.environment.borrow().values.len(), 1);
        assert_eq!(
            interpreter.environment.borrow().get("a"),
            Ok(LiteralValue::Nil)
        );
    }

    #[test]
    fn expression_with_blocks_test() {
        let source = "
            var a = \"global a\";
            var b = \"global b\";
            var c = \"global c\";
            {
                var a = \"outer a\";
                var b = \"outer b\";
                {
                    var a = \"inner a\";
                    print a;
                    print b;
                    print c;
                }
                print a;
                print b;
                print c;
            }
            print a;
            print b;
            print c;";

        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut interpreter: Interpreter = Interpreter::new();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());

        assert_eq!(interpreter.environment.borrow().values.is_empty(), false);
        assert_eq!(interpreter.environment.borrow().values.len(), 3);
        assert_eq!(
            interpreter.environment.borrow().get("a"),
            Ok(LiteralValue::StringValue(String::from("global a")))
        );
        assert_eq!(
            interpreter.environment.borrow().get("b"),
            Ok(LiteralValue::StringValue(String::from("global b")))
        );
        assert_eq!(
            interpreter.environment.borrow().get("c"),
            Ok(LiteralValue::StringValue(String::from("global c")))
        );
    }

    #[test]
    fn expression_with_if_statement_test() {
        let source = "
            var a = 5;
            var b = 6;
            var c = 12;

            if (a < 5) {
                b = 12;
            } else {
                b = 13;
            }

            if (b == 13) {
                c = \"hello\";
            }
          ";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut interpreter: Interpreter = Interpreter::new();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());

        assert_eq!(interpreter.environment.borrow().values.is_empty(), false);
        assert_eq!(interpreter.environment.borrow().values.len(), 3);
        assert_eq!(
            interpreter.environment.borrow().get("a"),
            Ok(LiteralValue::IntValue(5))
        );
        assert_eq!(
            interpreter.environment.borrow().get("b"),
            Ok(LiteralValue::IntValue(13))
        );
        assert_eq!(
            interpreter.environment.borrow().get("c"),
            Ok(LiteralValue::StringValue(String::from("hello")))
        );
    }

    #[test]
    fn expression_with_while_statement_test() {
        let source = "
            var a = 5;
            while (a < 12) {
                a = a + 1;
            }
          ";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut interpreter: Interpreter = Interpreter::new();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());

        assert_eq!(interpreter.environment.borrow().values.is_empty(), false);
        assert_eq!(interpreter.environment.borrow().values.len(), 1);
        assert_eq!(
            interpreter.environment.borrow().get("a"),
            Ok(LiteralValue::IntValue(12))
        );
    }

    #[test]
    fn expression_with_for_statement_test() {
        let source = "
            var i;
            for (i = 0; i < 10; i = i + 1) {
                print i;
            }
          ";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut interpreter: Interpreter = Interpreter::new();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());

        assert_eq!(interpreter.environment.borrow().values.is_empty(), false);
        assert_eq!(interpreter.environment.borrow().values.len(), 1);
        assert_eq!(
            interpreter.environment.borrow().get("i"),
            Ok(LiteralValue::IntValue(10))
        );
    }

    #[test]
    fn expression_with_for_statement_no_initializer_test() {
        let source = "
            var i=0;
            for (;i < 10; i = i + 1) {
                print i;
            }
          ";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut interpreter: Interpreter = Interpreter::new();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());

        assert_eq!(interpreter.environment.borrow().values.is_empty(), false);
        assert_eq!(interpreter.environment.borrow().values.len(), 1);
        assert_eq!(
            interpreter.environment.borrow().get("i"),
            Ok(LiteralValue::IntValue(10))
        );
    }

    #[test]
    fn expression_with_for_statement_initializer_test() {
        let source = "
            var a=0;
            for (var i = 0;i < 10; i = i + 1) {
                a = a + 2;
            }
          ";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut interpreter: Interpreter = Interpreter::new();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());

        assert_eq!(interpreter.environment.borrow().values.is_empty(), false);
        assert_eq!(interpreter.environment.borrow().values.len(), 1);
        assert_eq!(
            interpreter.environment.borrow().get("a"),
            Ok(LiteralValue::IntValue(20))
        );
    }
}
