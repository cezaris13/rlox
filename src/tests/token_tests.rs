#[cfg(test)]
mod tests {
    use crate::token::Token;
    use crate::token::TokenType::*;

    use std::string::String;

    #[test]
    fn to_string_test() {
        let token = Token::new(Plus, String::from("+"), None, 2);

        let result = token.to_string();

        assert_eq!(result, String::from("Plus \"+\" None"));
    }
}
