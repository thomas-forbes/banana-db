use std::io::Write;

use crate::{
    bql::{lexer::Lexer, parser::Parser},
    query::Engine,
    storage,
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
            Ok(q) => q,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        let file = storage::File::open("db.bin".to_string());
        let mut engine = Engine::new(file);

        match engine.handle_query(query) {
            Ok(out) => println!("{}", out),
            Err(error) => eprintln!("{}", error),
        }
    }
}
