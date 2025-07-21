use std::io::Write;

use crate::{
    bql::{lexer::Lexer, parser::Parser},
    query::Engine,
    storage,
    table::Table,
};

pub fn start() {
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let query = match parser.parse_query() {
            Some(q) => q,
            None => {
                eprintln!("INVALID QUERY");
                continue;
            }
        };
        let file: storage::File<Table> = storage::File::open("db.bin".to_string());
        let engine = Engine::new(file);

        engine.handle_query(query);
    }
}
