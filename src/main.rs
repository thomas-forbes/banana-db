mod bql; // banana query language
mod query;
mod repl;
mod storage;
mod table;
mod utils;

fn main() {
    repl::start().expect("Failed to start REPL");
}
