#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    // keywords
    Gimme,
    Where,
    Limit,
    Insert,
    Into,

    // comparison
    Equals,
    NotEquals,
    // Less,
    // Greater,

    // identifiers
    Identifier,
    Integer,

    // delimiters
    Semicolon,
    LeftBrace,
    RightBrace,

    // object stuff
    Colon,
    Period,
}

#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String) -> Self {
        return Token {
            token_type,
            literal,
        };
    }
    pub fn literal(&self) -> &String {
        &self.literal
    }
    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }
}

pub fn lookup_identifier(identifier: &str) -> TokenType {
    match keyword_to_token(identifier) {
        Some(keyword) => keyword,
        None => TokenType::Identifier,
    }
}

fn keyword_to_token(keyword: &str) -> Option<TokenType> {
    match keyword {
        "gimme" => Some(TokenType::Gimme),
        "where" => Some(TokenType::Where),
        "insert" => Some(TokenType::Insert),
        "limit" => Some(TokenType::Limit),
        "into" => Some(TokenType::Into),
        _ => None,
    }
}
