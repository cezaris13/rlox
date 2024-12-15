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
        let variable_count = interpreter.environment.borrow().values.len();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());
        assert_eq!(
            variable_count,
            interpreter.environment.borrow().values.len()
        );
    }

    #[test]
    fn expression_initializer_with_provided_value_test() {
        let source = "var a = 12;";

        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut interpreter: Interpreter = Interpreter::new();
        let variable_count = interpreter.environment.borrow().values.len();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());
        assert_eq!(
            interpreter.environment.borrow().values.len(),
            variable_count + 1
        );
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
        let variable_count = interpreter.environment.borrow().values.len();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());

        assert_eq!(
            interpreter.environment.borrow().values.len(),
            variable_count + 1
        );
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
        let variable_count = interpreter.environment.borrow().values.len();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());

        assert_eq!(
            interpreter.environment.borrow().values.len(),
            variable_count + 3
        );
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
        let variable_count = interpreter.environment.borrow().values.len();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());

        assert_eq!(
            interpreter.environment.borrow().values.len(),
            variable_count + 3
        );
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
        let variable_count = interpreter.environment.borrow().values.len();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());
        assert_eq!(
            interpreter.environment.borrow().values.len(),
            variable_count + 1
        );
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
        let variable_count = interpreter.environment.borrow().values.len();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());

        assert_eq!(
            interpreter.environment.borrow().values.len(),
            variable_count + 1
        );
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
        let variable_count = interpreter.environment.borrow().values.len();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());

        assert_eq!(
            interpreter.environment.borrow().values.len(),
            variable_count + 1
        );
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
        let variable_count = interpreter.environment.borrow().values.len();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());

        assert_eq!(
            interpreter.environment.borrow().values.len(),
            variable_count + 1
        );
        assert_eq!(
            interpreter.environment.borrow().get("a"),
            Ok(LiteralValue::IntValue(20))
        );
    }

    #[test]
    fn expression_with_function_statement_test() {
        let source = "
            var a=0;

            fun addOne(a) {
                a = a + 1;
            }

            var b = addOne(a);
            var c = clock();
          ";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut interpreter: Interpreter = Interpreter::new();
        let variable_count = interpreter.environment.borrow().values.len();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());

        assert_eq!(
            interpreter.environment.borrow().values.len(),
            variable_count + 4
        );
        assert_eq!(
            interpreter.environment.borrow().get("a"),
            Ok(LiteralValue::IntValue(0))
        );
        assert_eq!(
            interpreter.environment.borrow().get("b"),
            Ok(LiteralValue::Nil)
        );
        assert_ne!(
            interpreter.environment.borrow().get("c"),
            Ok(LiteralValue::Nil)
        );
    }

    #[test]
    fn test_function_with_return() {
        let source = "
            var a=0;

            fun addOne(a) {
                return a + 1;
            }

            var b = addOne(a);
          ";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut interpreter: Interpreter = Interpreter::new();
        let variable_count = interpreter.environment.borrow().values.len();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());

        assert_eq!(
            interpreter.environment.borrow().values.len(),
            variable_count + 3
        );
        assert_eq!(
            interpreter.environment.borrow().get("a"),
            Ok(LiteralValue::IntValue(0))
        );
        assert_eq!(
            interpreter.environment.borrow().get("b"),
            Ok(LiteralValue::IntValue(1))
        );
    }

    #[test]
    fn test_function_with_empty_return() {
        let source = "
            var a=0;

            fun printA(a) {
                print a;
                return;
            }

            var b = printA(a);
          ";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut interpreter: Interpreter = Interpreter::new();
        let variable_count = interpreter.environment.borrow().values.len();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());

        assert_eq!(
            interpreter.environment.borrow().values.len(),
            variable_count + 3
        );
        assert_eq!(
            interpreter.environment.borrow().get("a"),
            Ok(LiteralValue::IntValue(0))
        );
        assert_eq!(
            interpreter.environment.borrow().get("b"),
            Ok(LiteralValue::Nil)
        );
    }

    #[test]
    fn test_function_with_conditional_return() {
        let source = "
            fun condreturn(a) {
                if (a <= 0)
                {
                    return 0;
                }

                return a - 1;
            }

            var a = condreturn(4);
            var b = condreturn(-1);
          ";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut interpreter: Interpreter = Interpreter::new();
        let variable_count = interpreter.environment.borrow().values.len();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());

        assert_eq!(
            interpreter.environment.borrow().values.len(),
            variable_count + 3
        );
        assert_eq!(
            interpreter.environment.borrow().get("a"),
            Ok(LiteralValue::IntValue(3))
        );
        assert_eq!(
            interpreter.environment.borrow().get("b"),
            Ok(LiteralValue::IntValue(0))
        );
    }

    #[test]
    fn test_function_with_nested_blocks() {
        let source = "
            fun nested(a) {
                if (a < 3) {
                    if (a > 1) {
                        return a;
                    }
                }
                {
                    a = a + 2;
                    return a;
                }
                return -1;
            }

            var a = nested(2);
            var b = nested(1);
          ";
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();

        let mut parser = Parser::new(tokens);
        let statements = parser.parse().unwrap();

        let mut interpreter: Interpreter = Interpreter::new();
        let variable_count = interpreter.environment.borrow().values.len();
        let result = interpreter.interpret_statements(statements);

        assert!(result.is_ok());

        assert_eq!(
            interpreter.environment.borrow().values.len(),
            variable_count + 3
        );
        assert_eq!(
            interpreter.environment.borrow().get("a"),
            Ok(LiteralValue::IntValue(2))
        );
        assert_eq!(
            interpreter.environment.borrow().get("b"),
            Ok(LiteralValue::IntValue(3))
        );
    }
}
