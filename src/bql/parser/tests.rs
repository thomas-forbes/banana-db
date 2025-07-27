use crate::{
    bql::{
        ast::{Identifier, MapItem, Query},
        lexer::Lexer,
        parser::Parser,
    },
    database::data::{Comparison, Data},
};

#[test]
fn parse_gimme_simple() {
    let input = "gimme users;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let query = parser.parse_query().unwrap();
    match query {
        Query::Gimme(gimme) => {
            assert_eq!(gimme.table_identifier.value, "users");
            assert!(gimme.limit_statement.is_none());
            assert!(gimme.where_statement.is_none());
        }
        _ => panic!("Expected Gimme query"),
    }
}

#[test]
fn parse_gimme_limit() {
    let input = "gimme users limit 10;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let query = parser.parse_query().unwrap();
    match query {
        Query::Gimme(gimme) => {
            assert_eq!(gimme.table_identifier.value, "users");
            let limit_stmt = gimme.limit_statement.expect("Expected limit statement");
            assert_eq!(limit_stmt.number, 10);
        }
        _ => panic!("Expected Gimme query"),
    }
}

#[test]
fn parse_gimme_where_limit() {
    let input = "gimme users where age >= 18 limit 10;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let query = parser.parse_query().unwrap();
    match query {
        Query::Gimme(gimme) => {
            assert_eq!(gimme.table_identifier.value, "users");
            let where_stmt = gimme.where_statement.expect("Expected where statement");
            assert_eq!(where_stmt.field.value, "age");
            assert_eq!(where_stmt.comparison, Comparison::GreaterEquals);
            assert_eq!(where_stmt.value, Data::Int(Some(18)));
            let limit_stmt = gimme.limit_statement.expect("Expected limit statement");
            assert_eq!(limit_stmt.number, 10);
        }
        _ => panic!("Expected Gimme query"),
    }
}

#[test]
fn parse_insert_simple() {
    let input = "insert {id: 1, name: John, wealth: 1.5, dead: false} into users;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let query = parser.parse_query().unwrap();
    match query {
        Query::Insert(insert) => {
            assert_eq!(insert.table_identifier.value, "users");
            assert_eq!(insert.values.len(), 4);
            assert_eq!(insert.values[0].key.value, "id");
            assert_eq!(insert.values[0].value, Data::Int(Some(1)));
            assert_eq!(insert.values[1].key.value, "name");
            assert_eq!(
                insert.values[1].value,
                Data::String(Some("John".to_string()))
            );
            assert_eq!(insert.values[2].key.value, "wealth");
            assert_eq!(insert.values[2].value, Data::Float(Some(1.5)));
            assert_eq!(insert.values[3].key.value, "dead");
            assert_eq!(insert.values[3].value, Data::Boolean(Some(false)));
        }
        _ => panic!("Expected Insert query"),
    }
}

#[test]
fn parse_invalid_start() {
    let input = "foobar";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let err = parser.parse_query();
    assert!(err.is_err());
}

#[test]
fn parse_value_map() {
    let input = "{id: 1, name: John, wealth: 1.5, dead: false};";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let map = parser.parse_map().unwrap();
    let expected_map = vec![
        MapItem {
            key: Identifier {
                value: "id".to_string(),
            },
            value: Data::Int(Some(1)),
        },
        MapItem {
            key: Identifier {
                value: "name".to_string(),
            },
            value: Data::String(Some("John".to_string())),
        },
        MapItem {
            key: Identifier {
                value: "wealth".to_string(),
            },
            value: Data::Float(Some(1.5)),
        },
        MapItem {
            key: Identifier {
                value: "dead".to_string(),
            },
            value: Data::Boolean(Some(false)),
        },
    ];
    assert_eq!(map, expected_map);
}

#[test]
fn parse_type_map() {
    let input = "{id: Int, name: String, wealth: Float, dead: Boolean};";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let map = parser.parse_map().unwrap();
    let expected_map = vec![
        MapItem {
            key: Identifier {
                value: "id".to_string(),
            },
            value: Data::Int(None),
        },
        MapItem {
            key: Identifier {
                value: "name".to_string(),
            },
            value: Data::String(None),
        },
        MapItem {
            key: Identifier {
                value: "wealth".to_string(),
            },
            value: Data::Float(None),
        },
        MapItem {
            key: Identifier {
                value: "dead".to_string(),
            },
            value: Data::Boolean(None),
        },
    ];
    assert_eq!(map, expected_map);
}

#[test]
fn semicolon_required() {
    let inputs = vec![
        "gimme users",
        "insert {id: 1, name: John, wealth: 1.5, dead: false} into users",
        "tables",
        "new table users {id: Int, name: String, wealth: Float, dead: Boolean}",
        "delete table users",
    ];
    for input in inputs {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer).unwrap();
        let err = parser.parse_query();
        assert!(err.is_err());
    }
}
