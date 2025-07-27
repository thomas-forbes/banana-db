use colored::Colorize;

use crate::database::Database;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;

pub fn run_file(db_file_path: &str, bql_file_path: &str) -> Result<(), String> {
    let bql_file = std::fs::read_to_string(bql_file_path).map_err(|err| err.to_string())?;
    let mut db = Database::new(db_file_path);

    for line in bql_file.lines() {
        println!("> {}", line.dimmed());
        match db.handle_query(line) {
            Ok(out) => println!("{}", out),
            Err(err) => eprintln!("{}", err),
        }
    }
    Ok(())
}
