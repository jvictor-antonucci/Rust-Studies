use crate::{class::Class, token_type::TokenType};

#[derive(Clone, Debug)]
pub struct Token {
    pub class: Option<Class>,
    pub lexeme: Option<String>,
    pub token_type: Option<TokenType>,
}

impl Token {
    pub fn new(
        class: Option<Class>,
        lexeme: Option<String>,
        token_type: Option<TokenType>,
    ) -> Token {
        Token {
            class,
            lexeme,
            token_type,
        }
    }

    pub fn new_from_lexeme(lexeme: &str) -> Token {
        Token {
            class: Class::from_lexeme(lexeme),
            lexeme: Some(String::from(lexeme)),
            token_type: TokenType::from_lexeme(lexeme),
        }
    }

    pub fn new_from_ref(token: &Token) -> Token {
        token.clone()
    }
}
