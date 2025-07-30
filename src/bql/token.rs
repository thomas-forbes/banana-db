#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    // keywords

    // gimme
    Gimme,
    Where,
    Limit,
    // insert
    Insert,
    Into,
    // tables
    Tables,
    Table,
    New,
    Delete,
    // data types
    IntWord,
    StringWord,
    FloatWord,
    BooleanWord,
    // boolean
    True,
    False,

    // comparison
    Equals,
    NotEquals,
    Less,
    LessEquals,
    Greater,
    GreaterEquals,

    // data
    Identifier,
    Integer,
    Float,

    // delimiters
    LeftBrace,
    RightBrace,
    Semicolon,
    Colon,
    Comma,

    // other
    At,
}

#[derive(Clone, Debug)]
pub struct TokenPosition {
    pub start_index: usize,
    pub end_index: usize,
}

#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    literal: String,
    position: TokenPosition,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        literal: String,
        start_index: usize,
        end_index: usize,
    ) -> Self {
        return Token {
            token_type,
            literal,
            position: TokenPosition {
                start_index,
                end_index,
            },
        };
    }
    pub fn literal(&self) -> &String {
        &self.literal
    }
    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }
    pub fn position(&self) -> &TokenPosition {
        &self.position
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
        // gimme
        "gimme" => Some(TokenType::Gimme),
        "where" => Some(TokenType::Where),
        "limit" => Some(TokenType::Limit),
        // insert
        "insert" => Some(TokenType::Insert),
        "into" => Some(TokenType::Into),
        // tables
        "tables" => Some(TokenType::Tables),
        "table" => Some(TokenType::Table),
        "new" => Some(TokenType::New),
        "delete" => Some(TokenType::Delete),
        // data types
        "Int" => Some(TokenType::IntWord),
        "Float" => Some(TokenType::FloatWord),
        "String" => Some(TokenType::StringWord),
        "Boolean" => Some(TokenType::BooleanWord),
        // boolean
        "true" => Some(TokenType::True),
        "false" => Some(TokenType::False),
        _ => None,
    }
}
