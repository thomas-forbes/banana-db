use crate::{
    bql::{lexer::Lexer, parser::Parser},
    database::query::Engine,
};

pub mod data;
mod query;
mod storage;
mod table;

pub struct Database {
    file: storage::File,
}

impl Database {
    pub fn new(file_path: &str) -> Database {
        let file = storage::File::open(file_path);
        Database { file }
    }
    pub fn handle_query(&mut self, query: &str) -> Result<String, String> {
        let lexer = Lexer::new(&query);

        let mut parser = Parser::new(lexer).map_err(|err| err.to_string())?;
        let query = parser.parse_query().map_err(|err| err.to_string())?;

        let mut engine = Engine::new(&mut self.file);
        engine.handle_query(query).map_err(|err| err.to_string())
    }
    pub fn delete(&mut self) -> Result<(), std::io::Error> {
        self.file.delete()
    }
}
