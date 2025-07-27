use clap::Parser;

mod bql; // banana query language
mod database;
mod repl;
mod utils;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, default_value = "db.bin")]
    file_name: String,
    #[arg(short, long)]
    run_file: Option<String>,
}

fn main() {
    let args = Args::parse();

    match args.run_file {
        Some(file) => bql::run_file(&args.file_name, &file).expect("Failed to run file"),
        None => repl::start(&args.file_name).expect("Failed to start REPL"),
    }
}
