use colored::Colorize;

use crate::{
    bql::{lexer::Lexer, parser::Parser},
    query::Engine,
    storage,
};

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;

fn handle_query(line: &str, mut db_file: &mut storage::File) -> Result<String, String> {
    let lexer = Lexer::new(line);

    let mut parser = Parser::new(lexer).map_err(|err| err.to_string())?;
    let query = parser.parse_query().map_err(|err| err.to_string())?;

    let mut engine = Engine::new(&mut db_file);
    return engine.handle_query(query).map_err(|err| err.to_string());
}

pub fn run_file(db_file_path: &str, bql_file_path: &str) -> Result<(), String> {
    let bql_file = std::fs::read_to_string(bql_file_path).map_err(|err| err.to_string())?;
    let mut db_file = storage::File::open(&db_file_path);

    for line in bql_file.lines() {
        println!("> {}", line.dimmed());
        match handle_query(line, &mut db_file) {
            Ok(out) => println!("{}", out),
            Err(err) => eprintln!("{}", err),
        }
    }
    Ok(())
}
