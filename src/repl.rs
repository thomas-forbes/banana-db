use crate::{
    bql::{lexer::Lexer, parser::Parser},
    query::Engine,
    storage,
};

use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

fn handle_query_string(db_file_name: &str, input: &str) -> Result<String, String> {
    let file = storage::File::open(&db_file_name);
    let mut engine = Engine::new(file);

    let lexer = Lexer::new(&input);

    let mut parser = Parser::new(lexer).map_err(|err| format!("{}", err))?;
    let query = parser.parse_query().map_err(|err| format!("{}", err))?;

    engine
        .handle_query(query)
        .map_err(|error| error.to_string())
}

pub fn start(db_file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(input) => {
                rl.add_history_entry(&input)?;
                match handle_query_string(db_file_name, &input) {
                    Ok(out) => println!("{}", out),
                    Err(err) => eprintln!("{}", err),
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                eprintln!("{:?}", err);
                break;
            }
        }
    }
    Ok(())
}
