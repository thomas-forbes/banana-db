use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

use crate::database::Database;

pub fn start(db_file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(input) => {
                rl.add_history_entry(&input)?;
                let mut db = Database::new(db_file_name);
                match db.handle_query(&input) {
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
