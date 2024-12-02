use crate::token::Token;
use crate::token::TokenType;
use crate::token::TokenType::*;

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralValue {
    IntValue(i64),
    FValue(f64),
    StringValue(String),
    True,
    False,
    Nil,
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValue::IntValue(integer) => integer.to_string(),
            LiteralValue::FValue(float) => float.to_string(),
            LiteralValue::StringValue(string) => string.clone(),
            LiteralValue::True => String::from("true"),
            LiteralValue::False => String::from("false"),
            LiteralValue::Nil => String::from("nil"),
        }
    }

    pub fn from_token(token: Token) -> Self {
        match token.token_type {
            NUMBER => match token.literal {
                Some(crate::token::LiteralValue::IntValue(int_value)) => Self::IntValue(int_value),
                Some(crate::token::LiteralValue::FValue(float_value)) => Self::FValue(float_value),
                _ => panic!("Could not unwrap as number"),
            },
            STRING => match token.literal {
                Some(crate::token::LiteralValue::StringValue(string_value)) => {
                    Self::StringValue(string_value)
                }
                Some(crate::token::LiteralValue::IdentifierValue(id_value)) => {
                    Self::StringValue(id_value)
                }
                _ => panic!("Could not unwrap as String"),
            },
            FALSE => Self::False,
            TRUE => Self::True,
            NIL => Self::Nil,
            _ => panic!("Could not create literal calue from {:?}", token),
        }
    }

    pub fn is_falsy(&self) -> LiteralValue {
        match self {
            Self::IntValue(x) => {
                if *x == 0 {
                    return Self::True;
                }
                Self::False
            }
            Self::FValue(x) => {
                if *x == 0.0 {
                    return Self::True;
                }
                Self::False
            }
            Self::True => Self::False,
            Self::False => Self::True,
            Self::StringValue(string) => {
                if string.len() == 0 {
                    return Self::True;
                }
                Self::False
            }
            Self::Nil => LiteralValue::True,
        }
    }
}

pub enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Grouping {
        group: Box<Expression>,
    },
    Literal {
        value: LiteralValue,
    },
}

impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                format!(
                    "({} {} {})",
                    operator.lexeme,
                    left.to_string(),
                    right.to_string()
                )
            }
            Expression::Grouping { group } => {
                format!("(group {})", group.to_string())
            }
            Expression::Literal { value } => value.to_string(),
            Expression::Unary { operator, right } => {
                format!("({} {})", operator.lexeme, right.to_string())
            }
        }
    }

    pub fn evaluate(&self) -> Result<LiteralValue, String> {
        return match self {
            Expression::Literal { value } => Ok(value.clone()),
            Expression::Grouping { group } => group.evaluate(),
            Expression::Unary { operator, right } => {
                let right = (*right).evaluate()?;

                match (&right, &operator.token_type) {
                    (LiteralValue::IntValue(value), MINUS) => Ok(LiteralValue::IntValue(-value)),
                    (LiteralValue::FValue(value), MINUS) => Ok(LiteralValue::FValue(-value)),
                    (_, MINUS) => {
                        return Err(format!("Minus not implemented for {}", right.to_string()))
                    }
                    (any, BANG) => Ok(any.is_falsy()),
                    _ => {
                        return Err(format!(
                            "Any othe non unary operator {:?} is not implemented for {}",
                            operator.token_type,
                            right.to_string(),
                        ))
                    }
                }
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left = (*left).evaluate()?;
                let right = (*right).evaluate()?;

                match &operator.token_type {
                    PLUS => self.process_plus_operator(left, right),
                    MINUS => self.process_minus_operator(left, right),
                    STAR => self.process_star_operator(left, right),
                    SLASH => self.process_slash_operator(left, right),
                    GREATER => self.process_greater_operator(left, right),
                    GREATER_EQUAL => self.process_greater_equal_operator(left, right),
                    LESS => self.process_less_operator(left, right),
                    LESS_EQUAL => self.process_less_equal_operator(left, right),
                    BANG_EQUAL => Ok(self.bool_to_literal_value_bool(left != right)),
                    EQUAL_EQUAL => Ok(self.bool_to_literal_value_bool(left == right)),
                    _ => self.not_implemented_error(&operator.token_type, &left, &right),
                }
            }
        };
    }

    fn process_plus_operator(
        &self,
        left: LiteralValue,
        right: LiteralValue,
    ) -> Result<LiteralValue, String> {
        match (&left, &right) {
            (LiteralValue::IntValue(x), LiteralValue::IntValue(y)) => {
                Ok(LiteralValue::IntValue(x + y))
            }
            (LiteralValue::FValue(x), LiteralValue::FValue(y)) => Ok(LiteralValue::FValue(x + y)),
            (LiteralValue::IntValue(x), LiteralValue::FValue(y)) => {
                Ok(LiteralValue::FValue((*x as f64) + y))
            }
            (LiteralValue::FValue(x), LiteralValue::IntValue(y)) => {
                Ok(LiteralValue::FValue(x + (*y as f64)))
            }
            (LiteralValue::StringValue(string), any) => Ok(LiteralValue::StringValue(format!(
                "{0}{1}",
                string,
                any.to_string()
            ))),
            (any, LiteralValue::StringValue(string)) => Ok(LiteralValue::StringValue(format!(
                "{0}{1}",
                any.to_string(),
                string
            ))),
            _ => self.not_implemented_error(&PLUS, &left, &right),
        }
    }

    fn process_minus_operator(
        &self,
        left: LiteralValue,
        right: LiteralValue,
    ) -> Result<LiteralValue, String> {
        match (&left, &right) {
            (LiteralValue::IntValue(x), LiteralValue::IntValue(y)) => {
                Ok(LiteralValue::IntValue(x - y))
            }
            (LiteralValue::FValue(x), LiteralValue::FValue(y)) => Ok(LiteralValue::FValue(x - y)),
            (LiteralValue::IntValue(x), LiteralValue::FValue(y)) => {
                Ok(LiteralValue::FValue((*x as f64) - y))
            }
            (LiteralValue::FValue(x), LiteralValue::IntValue(y)) => {
                Ok(LiteralValue::FValue(x - (*y as f64)))
            }
            _ => self.not_implemented_error(&MINUS, &left, &right),
        }
    }

    fn process_star_operator(
        &self,
        left: LiteralValue,
        right: LiteralValue,
    ) -> Result<LiteralValue, String> {
        match (&left, &right) {
            (LiteralValue::IntValue(x), LiteralValue::IntValue(y)) => {
                Ok(LiteralValue::IntValue(x * y))
            }
            (LiteralValue::FValue(x), LiteralValue::FValue(y)) => Ok(LiteralValue::FValue(x * y)),
            (LiteralValue::IntValue(x), LiteralValue::FValue(y)) => {
                Ok(LiteralValue::FValue((*x as f64) * y))
            }
            (LiteralValue::FValue(x), LiteralValue::IntValue(y)) => {
                Ok(LiteralValue::FValue(x * (*y as f64)))
            }
            _ => self.not_implemented_error(&STAR, &left, &right),
        }
    }

    fn process_slash_operator(
        &self,
        left: LiteralValue,
        right: LiteralValue,
    ) -> Result<LiteralValue, String> {
        match right {
            LiteralValue::IntValue(x) => {
                if x == 0 {
                    return Err(String::from("Division by 0"));
                }
            }

            LiteralValue::FValue(y) => {
                if y == 0.0 {
                    return Err(String::from("Division by 0"));
                }
            }
            _ => {}
        }

        match (&left, &right) {
            (LiteralValue::IntValue(x), LiteralValue::IntValue(y)) => {
                Ok(LiteralValue::IntValue(x / y))
            }
            (LiteralValue::FValue(x), LiteralValue::FValue(y)) => Ok(LiteralValue::FValue(x / y)),
            (LiteralValue::IntValue(x), LiteralValue::FValue(y)) => {
                Ok(LiteralValue::FValue((*x as f64) / y))
            }
            (LiteralValue::FValue(x), LiteralValue::IntValue(y)) => {
                Ok(LiteralValue::FValue(x / (*y as f64)))
            }
            _ => self.not_implemented_error(&SLASH, &left, &right),
        }
    }

    fn process_greater_operator(
        &self,
        left: LiteralValue,
        right: LiteralValue,
    ) -> Result<LiteralValue, String> {
        match (&left, &right) {
            (LiteralValue::IntValue(x), LiteralValue::IntValue(y)) => {
                Ok(self.bool_to_literal_value_bool(x > y))
            }
            (LiteralValue::FValue(x), LiteralValue::FValue(y)) => {
                Ok(self.bool_to_literal_value_bool(x > y))
            }
            (LiteralValue::IntValue(x), LiteralValue::FValue(y)) => {
                Ok(self.bool_to_literal_value_bool(*x as f64 > *y))
            }
            (LiteralValue::FValue(x), LiteralValue::IntValue(y)) => {
                Ok(self.bool_to_literal_value_bool(*x > *y as f64))
            }
            (LiteralValue::StringValue(x), LiteralValue::StringValue(y)) => {
                Ok(self.bool_to_literal_value_bool(x.len() > y.len()))
            }
            _ => self.not_implemented_error(&GREATER, &left, &right),
        }
    }

    fn process_greater_equal_operator(
        &self,
        left: LiteralValue,
        right: LiteralValue,
    ) -> Result<LiteralValue, String> {
        match (&left, &right) {
            (LiteralValue::IntValue(x), LiteralValue::IntValue(y)) => {
                Ok(self.bool_to_literal_value_bool(x >= y))
            }
            (LiteralValue::FValue(x), LiteralValue::FValue(y)) => {
                Ok(self.bool_to_literal_value_bool(x >= y))
            }
            (LiteralValue::IntValue(x), LiteralValue::FValue(y)) => {
                Ok(self.bool_to_literal_value_bool(*x as f64 >= *y))
            }
            (LiteralValue::FValue(x), LiteralValue::IntValue(y)) => {
                Ok(self.bool_to_literal_value_bool(*x >= *y as f64))
            }
            (LiteralValue::StringValue(x), LiteralValue::StringValue(y)) => {
                Ok(self.bool_to_literal_value_bool(x.len() >= y.len()))
            }
            _ => self.not_implemented_error(&GREATER_EQUAL, &left, &right),
        }
    }

    fn process_less_operator(
        &self,
        left: LiteralValue,
        right: LiteralValue,
    ) -> Result<LiteralValue, String> {
        match (&left, &right) {
            (LiteralValue::IntValue(x), LiteralValue::IntValue(y)) => {
                Ok(self.bool_to_literal_value_bool(x < y))
            }
            (LiteralValue::FValue(x), LiteralValue::FValue(y)) => {
                Ok(self.bool_to_literal_value_bool(x < y))
            }
            (LiteralValue::IntValue(x), LiteralValue::FValue(y)) => {
                Ok(self.bool_to_literal_value_bool((*x as f64) < *y))
            }
            (LiteralValue::FValue(x), LiteralValue::IntValue(y)) => {
                Ok(self.bool_to_literal_value_bool(*x < *y as f64))
            }
            (LiteralValue::StringValue(x), LiteralValue::StringValue(y)) => {
                Ok(self.bool_to_literal_value_bool(x.len() < y.len()))
            }
            _ => self.not_implemented_error(&LESS, &left, &right),
        }
    }

    fn process_less_equal_operator(
        &self,
        left: LiteralValue,
        right: LiteralValue,
    ) -> Result<LiteralValue, String> {
        match (&left, &right) {
            (LiteralValue::IntValue(x), LiteralValue::IntValue(y)) => {
                Ok(self.bool_to_literal_value_bool(x <= y))
            }
            (LiteralValue::FValue(x), LiteralValue::FValue(y)) => {
                Ok(self.bool_to_literal_value_bool(x <= y))
            }
            (LiteralValue::IntValue(x), LiteralValue::FValue(y)) => {
                Ok(self.bool_to_literal_value_bool(*x as f64 <= *y))
            }
            (LiteralValue::FValue(x), LiteralValue::IntValue(y)) => {
                Ok(self.bool_to_literal_value_bool(*x <= *y as f64))
            }
            (LiteralValue::StringValue(x), LiteralValue::StringValue(y)) => {
                Ok(self.bool_to_literal_value_bool(x.len() <= y.len()))
            }
            _ => self.not_implemented_error(&LESS_EQUAL, &left, &right),
        }
    }

    fn bool_to_literal_value_bool(&self, boolean: bool) -> LiteralValue {
        if boolean {
            return LiteralValue::True;
        }
        LiteralValue::False
    }

    fn print(&self) {
        println!("{}", self.to_string());
    }

    fn not_implemented_error(
        &self,
        token_type: &TokenType,
        left: &LiteralValue,
        right: &LiteralValue,
    ) -> Result<LiteralValue, String> {
        return Err(format!(
            "{:?} operation is not implementer for: {:?} and {:?}",
            token_type,
            left.to_string(),
            right.to_string()
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::Expression;
    use crate::expr::Expression::*;
    use crate::expr::LiteralValue::*;
    use crate::token::TokenType::*;
    use crate::Parser;
    use crate::Scanner;

    #[test]
    fn pretty_print() {
        let expression: Expression = Binary {
            left: Box::new(Unary {
                operator: Token::new(MINUS, String::from("-"), None, 1),
                right: Box::new(Literal {
                    value: IntValue(123),
                }),
            }),
            operator: Token::new(STAR, String::from("*"), None, 1),
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
        let source = "!!true";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.parse();

        assert!(expression.is_ok());
        let evaluation = expression.unwrap().evaluate();

        assert!(evaluation.is_ok());

        assert_eq!(evaluation.unwrap(), LiteralValue::True);
    }

    #[test]
    fn evaluate_bang() {
        let source = "!true";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.parse();

        assert!(expression.is_ok());
        let evaluation = expression.unwrap().evaluate();

        assert!(evaluation.is_ok());

        assert_eq!(evaluation.unwrap(), LiteralValue::False);
    }

    #[test]
    fn evaluate_minus_int() {
        let source = "-12";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.parse();

        assert!(expression.is_ok());
        let evaluation = expression.unwrap().evaluate();

        assert_eq!(evaluation.unwrap(), LiteralValue::IntValue(-12));
    }

    #[test]
    fn evaluate_minus_double() {
        let source = "-12.0";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.parse();

        assert!(expression.is_ok());
        let evaluation = expression.unwrap().evaluate();

        assert_eq!(evaluation.unwrap(), LiteralValue::FValue(-12.0));
    }

    #[test]
    fn evaluate_int_returns_sum() {
        let source = "5 + 2";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.parse();

        assert!(expression.is_ok());
        let evaluation = expression.unwrap().evaluate();

        assert_eq!(evaluation.unwrap(), LiteralValue::IntValue(7));
    }

    #[test]
    fn evaluate_string_returns_sum() {
        let source = "\"hello \" + \"world\"";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.parse();

        assert!(expression.is_ok());
        let evaluation = expression.unwrap().evaluate();

        assert_eq!(
            evaluation.unwrap(),
            LiteralValue::StringValue(String::from("hello world"))
        );
    }

    #[test]
    fn evaluate_float_and_int_returns_float_mult() {
        let source = "2 * 2.5";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.parse();

        assert!(expression.is_ok());
        let evaluation = expression.unwrap().evaluate();

        assert_eq!(evaluation.unwrap(), LiteralValue::FValue(5.0));
    }

    #[test]
    fn evaluate_string_float_returns_sum() {
        let source = "\"hello \" + 2";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.parse();

        assert!(expression.is_ok());
        let evaluation = expression.unwrap().evaluate();

        assert_eq!(
            evaluation.unwrap(),
            LiteralValue::StringValue(String::from("hello 2"))
        );
    }

    #[test]
    fn evaluate_complex_int_float_returns_result() {
        let source = "2 * 2.5 + 5 / 2";
        let mut scanner: Scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expression = parser.parse();

        assert!(expression.is_ok());
        let evaluation = expression.unwrap().evaluate();

        assert_eq!(evaluation.unwrap(), LiteralValue::FValue(7.0));
    }
}
