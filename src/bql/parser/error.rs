use colored::Colorize;
use std::fmt::{self};

use crate::{
    bql::{
        lexer::error::LexerError,
        token::{TokenPosition, TokenType},
    },
    utils,
};

pub struct ParseError {
    input: String,
    reason: ParseErrorReason,
    position: Option<TokenPosition>,
}

impl ParseError {
    pub fn new(input: String, reason: ParseErrorReason, position: Option<TokenPosition>) -> Self {
        Self {
            input,
            reason,
            position,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.reason {
            ParseErrorReason::LexerError(e) => write!(f, "{}", e)?,
            _ => {
                write!(
                    f,
                    "{}",
                    utils::format_message(
                        &"parse error".bright_red().to_string(),
                        &self.reason.to_string()
                    )
                )?;
                if let Some(position) = &self.position {
                    write!(
                        f,
                        "{}",
                        utils::format_line_section_highlight(
                            &self.input,
                            position.start_index,
                            position.end_index
                        )
                    )?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ParseErrorReason {
    LexerError(LexerError),
    InvalidStartOfStatement(String),
    ExpectedToken((TokenType, Option<TokenType>)),
    MissingToken,
}

impl fmt::Display for ParseErrorReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorReason::LexerError(e) => write!(f, "{}", e),
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
            ParseErrorReason::MissingToken => write!(f, "Expected token but got EOF"),
        }
    }
}
