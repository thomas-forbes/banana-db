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

tables;
new table Users {id: Int, name: String};
delete table Users;

insert {id: 5, name: "Thomas"} into Users;
*/
