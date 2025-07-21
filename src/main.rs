use crate::{
    bql::{lexer::Lexer, parser::Parser},
    query::Engine,
    table::Table,
};

mod bql; // banana query language
mod query;
mod repl;
mod storage;
mod table;

fn main() {
    repl::start();
    // let input = "gimme users where id==1 limit 2;".to_owned();
    // let lexer = Lexer::new(&input);
    // let mut parser = Parser::new(lexer);

    // let query = parser.parse_query().unwrap();
    // println!("{:?}", query);

    // let file: storage::File<Table> = storage::File::open("db.bin".to_string());
    // let engine = Engine::new(file);

    // engine.handle_query(query);
}
