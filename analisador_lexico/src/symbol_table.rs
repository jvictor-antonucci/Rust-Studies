use std::collections::HashMap;

use crate::token::Token;


pub struct SymbolTable {
    pub hash_map: HashMap<String, Token>,
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        let hash_map = HashMap::new();

        let mut symbol_table = SymbolTable { hash_map };
        symbol_table.init_reserved_words();

        symbol_table
    }

    pub fn get(&self, lexeme: String) -> Option<Token> {
        if let Some(token) = self.hash_map.get(lexeme.as_str()) {
            return Some(Token::new_from_ref(token));
        }

        None
    }

    pub fn insert(&mut self, lexeme: String, token: Token) {
        self.hash_map.insert(lexeme, token);
    }

    pub fn update(&mut self, lexeme: String, token: Token) {
        if self.hash_map.contains_key(lexeme.as_str()) {
            self.insert(lexeme, token);
        }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, Token> {
        self.hash_map.iter()
    }


    fn init_reserved_words(&mut self) {
        let reserved_words: [&str; 14] = [
            "inicio",
            "varinicio",
            "varfim",
            "escreva",
            "leia",
            "se",
            "entao",
            "fimse",
            "repita",
            "fimrepita",
            "fim",
            "inteiro",
            "literal",
            "real",
        ];

        for lexeme in reserved_words {
            self.hash_map.insert(String::from(lexeme), Token::new_from_lexeme(lexeme));
        }
    }
}
