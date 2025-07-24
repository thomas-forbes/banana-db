use std::fmt;

use colored::Colorize;

use crate::{
    bql::{
        ast::*,
        lexer::Lexer,
        token::{Token, TokenPosition, TokenType},
    },
    table::{Comparison, Data},
};

pub struct ParseError {
    input: String,
    reason: ParseErrorReason,
    position: Option<TokenPosition>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!("{}: {}", "parse error".bright_red(), self.reason).bold()
        )?;
        if let Some(position) = &self.position {
            write!(f, "\n\t{}", self.input.dimmed())?;
            write!(
                f,
                "\t{}{}",
                " ".repeat(position.start_index),
                "^".repeat(position.end_index - position.start_index + 1)
                    .red()
                    .bold()
            )?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ParseErrorReason {
    InvalidStartOfStatement(String),
    ExpectedToken((TokenType, Option<TokenType>)),
    MissingToken,
}

impl fmt::Display for ParseErrorReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorReason::InvalidStartOfStatement(literal) => {
                write!(f, "`{}` is not a valid start of statement", literal)
            }
            ParseErrorReason::ExpectedToken((received, expected)) => {
                write!(f, "Received `{:?}`", received)?;
                if let Some(expected) = expected {
                    write!(f, " but expected `{:?}`", expected)?;
                }
                Ok(())
            }
            ParseErrorReason::MissingToken => write!(f, "Missing token"),
        }
    }
}

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
    fn build_error(&self, reason: ParseErrorReason, token: &Option<Token>) -> ParseError {
        ParseError {
            input: self.lexer.get_input().to_owned(),
            reason,
            position: token.as_ref().map(|t| t.position().clone()),
        }
    }
    fn get_current_token(&self) -> Result<&Token, ParseError> {
        self.current_token
            .as_ref()
            .ok_or(self.build_error(ParseErrorReason::MissingToken, &self.current_token))
    }
    fn current_token_is(&self, token_type: TokenType) -> Result<&Token, ParseError> {
        if let Some(current_token) = &self.current_token {
            if current_token.token_type() == &token_type {
                return Ok(current_token);
            } else {
                return Err(self.build_error(
                    ParseErrorReason::ExpectedToken((
                        current_token.token_type().clone(),
                        Some(token_type),
                    )),
                    &self.current_token,
                ));
            }
        } else {
            return Err(self.build_error(ParseErrorReason::MissingToken, &self.current_token));
        }
    }
    fn peek_token_is(&self, token_type: TokenType) -> Result<&Token, ParseError> {
        if let Some(peek_token) = &self.peek_token {
            if peek_token.token_type() == &token_type {
                return Ok(peek_token);
            } else {
                return Err(self.build_error(
                    ParseErrorReason::ExpectedToken((
                        peek_token.token_type().clone(),
                        Some(token_type),
                    )),
                    &self.peek_token,
                ));
            }
        } else {
            return Err(self.build_error(ParseErrorReason::MissingToken, &self.peek_token));
        }
    }
    fn expect_current(&mut self, token_type: TokenType) -> Result<Token, ParseError> {
        let current_token = self.current_token_is(token_type)?.clone();
        self.next_token();
        Ok(current_token)
    }
    fn expect_peek(&mut self, token_type: TokenType) -> Result<Token, ParseError> {
        let peek_token = self.peek_token_is(token_type)?.clone();
        self.next_token();
        Ok(peek_token)
    }

    // PARSING
    pub fn parse_query(&mut self) -> Result<Query, ParseError> {
        let current_token = self.get_current_token()?;
        match current_token.token_type() {
            TokenType::Gimme => self.parse_gimme().map(|g| Query::Gimme(g)),
            TokenType::Tables => self.parse_tables().map(|t| Query::Tables(t)),
            TokenType::New => self.parse_new_table().map(|t| Query::NewTable(t)),
            TokenType::Delete => self.parse_delete_table().map(|t| Query::DeleteTable(t)),
            TokenType::Insert => self.parse_insert().map(|i| Query::Insert(i)),
            _ => Err(self.build_error(
                ParseErrorReason::InvalidStartOfStatement(current_token.literal().clone()),
                &self.current_token,
            )),
        }
    }

    fn parse_identifier(&self) -> Result<Identifier, ParseError> {
        let token = self.get_current_token()?;

        Ok(Identifier {
            value: token.literal().clone(),
        })
    }
    fn parse_integer(&self) -> Result<i64, ParseError> {
        let token = self.get_current_token()?;

        token.literal().parse().map_err(|_| {
            self.build_error(
                ParseErrorReason::ExpectedToken((
                    token.token_type().clone(),
                    Some(TokenType::Integer),
                )),
                &self.current_token,
            )
        })
    }
    fn parse_float(&self) -> Result<f64, ParseError> {
        let token = self.get_current_token()?;

        token.literal().parse().map_err(|_| {
            self.build_error(
                ParseErrorReason::ExpectedToken((
                    token.token_type().clone(),
                    Some(TokenType::Float),
                )),
                &self.current_token,
            )
        })
    }
    fn parse_data(&self) -> Result<Data, ParseError> {
        let current_token = self.get_current_token()?;
        match current_token.token_type() {
            // data values
            TokenType::Identifier => Ok(Data::String(Some(current_token.literal().clone()))),
            TokenType::Integer => Ok(Data::Int(Some(self.parse_integer()?))),
            TokenType::Float => Ok(Data::Float(Some(self.parse_float()?))),
            TokenType::True => Ok(Data::Boolean(Some(true))),
            TokenType::False => Ok(Data::Boolean(Some(false))),
            // data types
            TokenType::IntWord => Ok(Data::Int(None)),
            TokenType::StringWord => Ok(Data::String(None)),
            TokenType::FloatWord => Ok(Data::Float(None)),
            TokenType::BooleanWord => Ok(Data::Boolean(None)),
            _ => Err(self.build_error(
                ParseErrorReason::ExpectedToken((current_token.token_type().clone(), None)),
                &self.current_token,
            )),
        }
    }
    fn parse_map(&mut self) -> Result<Map, ParseError> {
        let mut map = Vec::new();
        self.expect_current(TokenType::LeftBrace)?;

        while self.current_token_is(TokenType::RightBrace).is_ok() {
            if self.current_token_is(TokenType::Comma).is_ok() {
                self.next_token();
            }

            let key = self.parse_identifier()?;
            self.expect_peek(TokenType::Colon)?;

            self.next_token();
            let value = self.parse_data()?;

            map.push(MapItem { key, value });
            self.next_token(); // moves to , or }
        }
        Ok(map)
    }

    // GIMME
    fn parse_gimme(&mut self) -> Result<Gimme, ParseError> {
        self.expect_peek(TokenType::Identifier)?;
        let identifier = self.parse_identifier()?;

        let mut where_statement = None;
        let mut limit_statement = None;
        if self.peek_token_is(TokenType::Where).is_ok() {
            self.next_token();
            where_statement = Some(self.parse_where()?);
        }
        if self.peek_token_is(TokenType::Limit).is_ok() {
            self.next_token();
            limit_statement = Some(self.parse_limit()?);
        }
        Ok(Gimme {
            table_identifier: identifier,
            limit_statement,
            where_statement,
        })
    }
    fn parse_limit(&mut self) -> Result<Limit, ParseError> {
        self.expect_peek(TokenType::Integer)?;
        let integer = self.parse_integer()?;
        Ok(Limit {
            number: integer as usize,
        })
    }
    fn parse_where(&mut self) -> Result<Where, ParseError> {
        // identifier
        self.expect_peek(TokenType::Identifier)?;
        let identifier = self.parse_identifier()?;
        self.next_token();

        // comparison operator
        let comparison_token = self.get_current_token()?;
        let comparison_operator = match Comparison::from_token_type(comparison_token.token_type()) {
            Some(v) => v,
            None => {
                return Err(self.build_error(
                    ParseErrorReason::ExpectedToken((comparison_token.token_type().clone(), None)),
                    &self.current_token,
                ));
            }
        };
        self.next_token();

        // value
        let value = self.parse_data()?;

        Ok(Where {
            field: identifier,
            comparison: comparison_operator,
            value,
        })
    }

    // INSERT
    fn parse_insert(&mut self) -> Result<Insert, ParseError> {
        self.expect_peek(TokenType::LeftBrace)?;
        let values = self.parse_map()?;

        self.expect_peek(TokenType::Into)?;

        self.expect_peek(TokenType::Identifier)?;
        let table_identifier = self.parse_identifier()?;

        Ok(Insert {
            values,
            table_identifier,
        })
    }

    // TABLES
    fn parse_tables(&mut self) -> Result<Tables, ParseError> {
        Ok(Tables {})
    }
    fn parse_new_table(&mut self) -> Result<NewTable, ParseError> {
        self.expect_peek(TokenType::Table)?;

        self.expect_peek(TokenType::Identifier)?;
        let identifier = self.parse_identifier()?;

        self.expect_peek(TokenType::LeftBrace)?;
        let fields = self.parse_map()?;
        Ok(NewTable { identifier, fields })
    }
    fn parse_delete_table(&mut self) -> Result<DeleteTable, ParseError> {
        self.expect_peek(TokenType::Table)?;
        self.expect_peek(TokenType::Identifier)?;
        let identifier = self.parse_identifier()?;

        Ok(DeleteTable { identifier })
    }
}
