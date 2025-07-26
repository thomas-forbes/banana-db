use std::fmt;

use colored::Colorize;

use crate::{bql::token::TokenPosition, utils};

#[derive(Debug, Clone)]
pub enum LexerErrorReason {
    ExpectedChar((char, Option<char>)),
    InvalidCharacter(char),
    UnexpectedEOF,
}

impl fmt::Display for LexerErrorReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerErrorReason::ExpectedChar((received, expected)) => {
                write!(f, "Received `{}`", received)?;
                if let Some(expected) = expected {
                    write!(f, " but expected `{}`", expected)?;
                }
                Ok(())
            }
            LexerErrorReason::InvalidCharacter(c) => write!(f, "Invalid character `{}`", c),
            LexerErrorReason::UnexpectedEOF => write!(f, "Unexpected end of input"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LexerError {
    pub reason: LexerErrorReason,
    input: String,
    position: Option<TokenPosition>,
}

impl LexerError {
    pub fn new(input: String, reason: LexerErrorReason, position: Option<TokenPosition>) -> Self {
        Self {
            input,
            reason,
            position,
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            utils::format_message(
                &"lexer error".bright_red().to_string(),
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
        Ok(())
    }
}
