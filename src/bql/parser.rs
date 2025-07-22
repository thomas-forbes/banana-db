use std::cmp;

use crate::{
    bql::{
        ast::*,
        lexer::Lexer,
        token::{Token, TokenType},
    },
    table::{Comparison, Data},
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
                TokenType::Tables => Some(Query::Tables(self.parse_tables())),
                TokenType::New => Some(Query::NewTable(self.parse_new_table())),
                TokenType::Delete => Some(Query::DeleteTable(self.parse_delete_table())),
                TokenType::Insert => Some(Query::Insert(self.parse_insert())),
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
    fn parse_float(&self) -> f64 {
        self.current_token
            .as_ref()
            .expect("Parsing should only happen on validated tokens")
            .literal()
            .parse()
            .expect("`parse_float` should only be called on `TokenType::Float`")
    }
    fn parse_data(&self) -> Data {
        match &self.current_token {
            Some(token) => match token.token_type() {
                // data values
                TokenType::Identifier => Data::String(Some(token.literal().clone())),
                TokenType::Integer => Data::Int(Some(self.parse_integer())),
                TokenType::Float => Data::Float(Some(self.parse_float())),
                TokenType::True => Data::Boolean(Some(true)),
                TokenType::False => Data::Boolean(Some(false)),
                // data types
                TokenType::IntWord => Data::Int(None),
                TokenType::StringWord => Data::String(None),
                TokenType::FloatWord => Data::Float(None),
                TokenType::BooleanWord => Data::Boolean(None),
                _ => panic!("Invalid value in map"),
            },
            None => panic!("Expected value after identifier in map"),
        }
    }
    fn parse_map(&mut self) -> Map {
        let mut map = Vec::new();
        if self.current_token_is(TokenType::LeftBrace).is_some() {
            self.next_token();
        } else {
            panic!("Called `parse_map` on invalid token");
        }

        while self.current_token_is(TokenType::RightBrace).is_none() {
            if self.current_token_is(TokenType::Comma).is_some() {
                self.next_token();
            }

            let key = self.parse_identifier();
            if self.expect_peek(TokenType::Colon).is_none() {
                panic!("Colon must follow key identifier");
            }

            self.next_token();
            let value = self.parse_data();

            map.push(MapItem { key, value });
            self.next_token(); // move to , or }
        }
        return map;
    }

    // GIMME
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
        self.next_token();

        // comparison operator
        let comparison_operator = match &self.current_token {
            Some(t) => match Comparison::from_token_type(t.token_type()) {
                Some(v) => v,
                None => panic!("Invalid token for comparison"),
            },
            None => panic!("Missing comparison operator"),
        };
        self.next_token();

        // value
        let value = self.parse_data();

        return Where {
            field: identifier,
            comparison: comparison_operator,
            value,
        };
    }

    // INSERT
    fn parse_insert(&mut self) -> Insert {
        if self.expect_peek(TokenType::LeftBrace).is_none() {
            panic!("`{{` expected after `insert`");
        }
        let values = self.parse_map();
        if self.expect_peek(TokenType::Into).is_none() {
            panic!("`into` expected after `insert`");
        }
        if self.expect_peek(TokenType::Identifier).is_none() {
            panic!("Identifier expected after `into`");
        }
        let table_identifier = self.parse_identifier();

        Insert {
            values,
            table_identifier,
        }
    }

    // TABLES
    fn parse_tables(&mut self) -> Tables {
        Tables {}
    }
    fn parse_new_table(&mut self) -> NewTable {
        if self.expect_peek(TokenType::Table).is_none() {
            panic!("`table` expected after `delete`");
        }

        if self.expect_peek(TokenType::Identifier).is_none() {
            panic!("Identifier expected after `new table`");
        }
        let identifier = self.parse_identifier();

        if self.expect_peek(TokenType::LeftBrace).is_none() {
            panic!("Field map expected during table creation");
        }
        let fields = self.parse_map();
        NewTable { identifier, fields }
    }
    fn parse_delete_table(&mut self) -> DeleteTable {
        if self.expect_peek(TokenType::Table).is_none() {
            panic!("`table` expected after `delete`");
        }
        if self.expect_peek(TokenType::Identifier).is_none() {
            panic!("Identifier expected after `delete table`");
        }
        let identifier = self.parse_identifier();

        DeleteTable { identifier }
    }
}
