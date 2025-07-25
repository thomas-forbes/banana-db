use crate::{
    bql::{lexer::Lexer, parser::Parser},
    query::Engine,
    storage,
};

use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(input) => {
                rl.add_history_entry(input.as_str())?;

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
