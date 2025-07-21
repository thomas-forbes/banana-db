use std::{process::id, thread::panicking};

use crate::bql::{
    ast::*,
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    peek_token: Option<Token>,
    current_token: Option<Token>,
}

impl Parser<'_> {
    pub fn new<'a>(lexer: Lexer<'a>) -> Parser<'a> {
        let mut parser: Parser<'a> = Parser {
            lexer,
            peek_token: None,
            current_token: None,
        };

        // fills peek and current token
        parser.next_token();
        parser.next_token();
        return parser;
    }

    // HELPERS
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }
    fn current_token_is(&self, token_type: TokenType) -> Option<&Token> {
        if let Some(current_token) = &self.current_token
            && current_token.token_type() == &token_type
        {
            return Some(current_token);
        } else {
            return None;
        }
    }
    fn peek_token_is(&self, token_type: TokenType) -> Option<&Token> {
        if let Some(peek_token) = &self.peek_token
            && peek_token.token_type() == &token_type
        {
            return Some(peek_token);
        } else {
            return None;
        }
    }
    fn expect_peek(&mut self, token_type: TokenType) -> Option<Token> {
        if let Some(peek_token) = self.peek_token_is(token_type).cloned() {
            self.next_token();
            return Some(peek_token);
        } else {
            return None;
        }
    }

    // PARSING
    pub fn parse_query(&mut self) -> Option<Query> {
        match &self.current_token {
            Some(token) => match token.token_type() {
                TokenType::Gimme => Some(Query::Gimme(self.parse_gimme())),
                _ => panic!("Invalid start of statement. Must be `gimme` or `insert`"),
            },
            None => None,
        }
    }
    fn parse_identifier(&self) -> Identifier {
        Identifier {
            value: self
                .current_token
                .as_ref()
                .expect("Parsing should only happen on validated tokens")
                .literal()
                .clone(),
        }
    }
    fn parse_integer(&self) -> i64 {
        self.current_token
            .as_ref()
            .expect("Parsing should only happen on validated tokens")
            .literal()
            .parse()
            .expect("`parse_integer` should only be called on `TokenType::Integer`")
    }
    fn parse_gimme(&mut self) -> Gimme {
        if let Some(_) = self.expect_peek(TokenType::Identifier) {
            let identifier = self.parse_identifier();

            let mut where_statement = None;
            let mut limit_statement = None;
            if self.peek_token_is(TokenType::Where).is_some() {
                self.next_token();
                where_statement = Some(self.parse_where());
            }
            if self.peek_token_is(TokenType::Limit).is_some() {
                self.next_token();
                limit_statement = Some(self.parse_limit());
            }
            return Gimme {
                table_identifier: identifier,
                limit_statement,
                where_statement,
            };
        } else {
            panic!("Expected identifier after `gimme`");
        }
    }
    fn parse_limit(&mut self) -> Limit {
        if let Some(_) = self.expect_peek(TokenType::Integer) {
            let integer = self.parse_integer();
            return Limit {
                number: integer as usize,
            };
        } else {
            panic!("Integer must follow `limit` statement");
        }
    }
    fn parse_where(&mut self) -> Where {
        // identifier
        if self.expect_peek(TokenType::Identifier).is_none() {
            panic!("Field required as first part of `where` comparison");
        }
        let identifier = self.parse_identifier();

        // comparison operator
        let comparison_operator = match COMPARISON_OPERATORS
            .iter()
            .find(|&operator| self.expect_peek(operator.clone()).is_some())
        {
            None => panic!("Invalid comparison operation"),
            Some(_) => self.current_token.clone().unwrap(),
        };

        // value
        if self.expect_peek(TokenType::Integer).is_none() {
            panic!("Integer is only supported datatype for comparison");
        }
        let value = self.parse_integer();

        return Where {
            field: identifier,
            comparison_operator,
            value,
        };
    }
    fn parse_condition() {}
}
