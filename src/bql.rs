use crate::{
    bql::{lexer::Lexer, parser::Parser},
    query::Engine,
    storage,
};

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;

pub fn run_file(db_file_path: &str, bql_file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    todo!()
}
