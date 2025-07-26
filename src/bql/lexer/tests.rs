use super::*;
use crate::bql::token::TokenType;

#[test]
fn identifier() {
    let mut lexer = Lexer::new("foo");
    let token = lexer.next_token().unwrap();
    assert_eq!(*token.token_type(), TokenType::Identifier);
    assert_eq!(token.literal(), "foo");
}

#[test]
fn keyword() {
    let mut lexer = Lexer::new("gimme");
    let token = lexer.next_token().unwrap();
    assert_eq!(*token.token_type(), TokenType::Gimme);
    assert_eq!(token.literal(), "gimme");
}

#[test]
fn integer() {
    let mut lexer = Lexer::new("123");
    let token = lexer.next_token().unwrap();
    assert_eq!(*token.token_type(), TokenType::Integer);
    assert_eq!(token.literal(), "123");
}

#[test]
fn float() {
    let mut lexer = Lexer::new("3.14");
    let token = lexer.next_token().unwrap();
    assert_eq!(*token.token_type(), TokenType::Float);
    assert_eq!(token.literal(), "3.14");
}

#[test]
fn comparison() {
    let mut lexer = Lexer::new("== != <= < > >=");
    let tokens = vec![
        TokenType::Equals,
        TokenType::NotEquals,
        TokenType::LessEquals,
        TokenType::Less,
        TokenType::Greater,
        TokenType::GreaterEquals,
    ];
    for expected_token in tokens {
        let token = lexer.next_token().unwrap();
        assert_eq!(*token.token_type(), expected_token);
    }
}

#[test]
fn invalid_character() {
    let mut lexer = Lexer::new("$");
    let err = lexer.next_token();
    assert!(err.is_err());
    match err {
        Err(e) => assert!(matches!(e.reason, LexerErrorReason::InvalidCharacter('$'))),
        _ => panic!("Expected error"),
    }
}

#[test]
fn gimme_statement() {
    let mut lexer = Lexer::new("gimme users where age >= 18 limit 10;");
    let tokens = vec![
        Token::new(TokenType::Gimme, "gimme".to_string(), 0, 0),
        Token::new(TokenType::Identifier, "users".to_string(), 0, 0),
        Token::new(TokenType::Where, "where".to_string(), 0, 0),
        Token::new(TokenType::Identifier, "age".to_string(), 0, 0),
        Token::new(TokenType::GreaterEquals, ">=".to_string(), 0, 0),
        Token::new(TokenType::Integer, "18".to_string(), 0, 0),
        Token::new(TokenType::Limit, "limit".to_string(), 0, 0),
        Token::new(TokenType::Integer, "10".to_string(), 0, 0),
        Token::new(TokenType::Semicolon, ";".to_string(), 0, 0),
    ];
    for expected_token in tokens {
        let token = lexer.next_token().unwrap();
        assert_eq!(*token.token_type(), *expected_token.token_type());
        assert_eq!(token.literal(), expected_token.literal());
    }
}

#[test]
fn insert_statement() {
    let mut lexer = Lexer::new("insert {id: 0, name: John, wealth: 1.12, dead: false} into users;");
    let tokens = vec![
        Token::new(TokenType::Insert, "insert".to_string(), 0, 0),
        Token::new(TokenType::LeftBrace, "{".to_string(), 0, 0),
        Token::new(TokenType::Identifier, "id".to_string(), 0, 0),
        Token::new(TokenType::Colon, ":".to_string(), 0, 0),
        Token::new(TokenType::Integer, "0".to_string(), 0, 0),
        Token::new(TokenType::Comma, ",".to_string(), 0, 0),
        Token::new(TokenType::Identifier, "name".to_string(), 0, 0),
        Token::new(TokenType::Colon, ":".to_string(), 0, 0),
        Token::new(TokenType::Identifier, "John".to_string(), 0, 0),
        Token::new(TokenType::Comma, ",".to_string(), 0, 0),
        Token::new(TokenType::Identifier, "wealth".to_string(), 0, 0),
        Token::new(TokenType::Colon, ":".to_string(), 0, 0),
        Token::new(TokenType::Float, "1.12".to_string(), 0, 0),
        Token::new(TokenType::Comma, ",".to_string(), 0, 0),
        Token::new(TokenType::Identifier, "dead".to_string(), 0, 0),
        Token::new(TokenType::Colon, ":".to_string(), 0, 0),
        Token::new(TokenType::False, "false".to_string(), 0, 0),
        Token::new(TokenType::RightBrace, "}".to_string(), 0, 0),
        Token::new(TokenType::Into, "into".to_string(), 0, 0),
        Token::new(TokenType::Identifier, "users".to_string(), 0, 0),
        Token::new(TokenType::Semicolon, ";".to_string(), 0, 0),
    ];
    for expected_token in tokens {
        let token = lexer.next_token().unwrap();
        assert_eq!(*token.token_type(), *expected_token.token_type());
        assert_eq!(token.literal(), expected_token.literal());
    }
}
