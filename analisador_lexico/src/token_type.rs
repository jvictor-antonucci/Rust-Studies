#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Inicio,
    Varinicio,
    Varfim,
    Escreva,
    Leia,
    Se,
    Entao,
    Fimse,
    Repita,
    Fimrepita,
    Fim,
    Inteiro,
    Literal,
    Real,
}

impl TokenType {
    pub fn from_lexeme(lexeme: &str) -> Option<TokenType> {
        match lexeme {
            "inicio" => Some(TokenType::Inicio),
            "varinicio" => Some(TokenType::Varinicio),
            "varfim" => Some(TokenType::Varfim),
            "escreva" => Some(TokenType::Escreva),
            "leia" => Some(TokenType::Leia),
            "se" => Some(TokenType::Se),
            "entao" => Some(TokenType::Entao),
            "fimse" => Some(TokenType::Fimse),
            "repita" => Some(TokenType::Repita),
            "fimrepita" => Some(TokenType::Fimrepita),
            "fim" => Some(TokenType::Fim),
            "inteiro" => Some(TokenType::Inteiro),
            "literal" => Some(TokenType::Literal),
            "real" => Some(TokenType::Real),
            _ => None,
        }
    }

    pub fn to_str(token_type: Option<TokenType>) -> String {
        match token_type {
            Some(TokenType::Inicio) => String::from("inicio"),
            Some(TokenType::Varinicio) => String::from("varinicio"),
            Some(TokenType::Varfim) => String::from("varfim"),
            Some(TokenType::Escreva) => String::from("escreva"),
            Some(TokenType::Leia) => String::from("leia"),
            Some(TokenType::Se) => String::from("se"),
            Some(TokenType::Entao) => String::from("entao"),
            Some(TokenType::Fimse) => String::from("fimse"),
            Some(TokenType::Repita) => String::from("repita"),
            Some(TokenType::Fimrepita) => String::from("fimrepita"),
            Some(TokenType::Fim) => String::from("fim"),
            Some(TokenType::Inteiro) => String::from("inteiro"),
            Some(TokenType::Literal) => String::from("literal"),
            Some(TokenType::Real) => String::from("real"),
            None => String::from("Null"),
        }
    }
}