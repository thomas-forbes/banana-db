use crate::bql::token::{Token, TokenType};

#[derive(Debug)]
pub enum Query {
    Gimme(Gimme),
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub value: String,
}

#[derive(Debug)]
pub struct Gimme {
    pub table_identifier: Identifier,
    pub limit_statement: Option<Limit>,
    pub where_statement: Option<Where>,
}

#[derive(Debug)]
pub struct Limit {
    pub number: usize,
}

pub const COMPARISON_OPERATORS: [TokenType; 2] = [TokenType::Equals, TokenType::NotEquals];
#[derive(Debug, Clone)]
pub struct Where {
    pub field: Identifier,
    pub value: i64, // TODO: many datatype parsing
    pub comparison_operator: Token,
}

// struct Insert {}
