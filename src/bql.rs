use crate::bql::lexer::Lexer;
use crate::bql::parser::Parser;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;

/*
bql examples:

gimme Users;
gimme Users limit 2;
gimme Users where id==5;
gimme Users where id==5 limit 2;
insert {id: 5} into Users;
*/
