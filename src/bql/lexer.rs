use crate::bql::token::{self, Token, TokenType};
use std::{iter::Peekable, str::CharIndices};

type CurrentChar = (usize, char);
pub struct Lexer<'a> {
    input: String,
    chars: Peekable<CharIndices<'a>>,
    current_char: Option<CurrentChar>,
}

impl Lexer<'_> {
    pub fn new<'a>(input: &'a str) -> Lexer<'a> {
        let mut lexer: Lexer<'a> = Lexer {
            input: input.to_owned(),
            chars: input.char_indices().peekable(),
            current_char: None,
        };
        lexer.read_next_char();
        return lexer;
    }

    pub fn get_input(&self) -> &str {
        &self.input
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let (current_index, current_char) = match self.current_char {
            Some(c) => c,
            None => return None,
        };

        let start_index = current_index;
        let next_token: Result<Token, String> = match current_char {
            '=' => {
                if let Some(next_c) = self.peek()
                    && next_c == '='
                {
                    self.read_next_char();
                    Ok(Token::new(
                        TokenType::Equals,
                        "==".to_owned(),
                        start_index,
                        start_index + 1,
                    ))
                } else {
                    Err("Expected '=' after '='".to_owned())
                }
            }
            '!' => {
                if let Some(next_c) = self.peek()
                    && next_c == '='
                {
                    self.read_next_char();
                    Ok(Token::new(
                        TokenType::NotEquals,
                        "!=".to_owned(),
                        start_index,
                        start_index + 1,
                    ))
                } else {
                    Err("Expected '=' after '!'".to_owned())
                }
            }
            '<' => {
                if let Some(next_c) = self.peek()
                    && next_c == '='
                {
                    self.read_next_char();
                    Ok(Token::new(
                        TokenType::LessEquals,
                        "<=".to_owned(),
                        start_index,
                        start_index + 1,
                    ))
                } else {
                    Ok(Token::new(
                        TokenType::Less,
                        "<".to_owned(),
                        start_index,
                        start_index,
                    ))
                }
            }
            '>' => {
                if let Some(next_c) = self.peek()
                    && next_c == '='
                {
                    self.read_next_char();
                    Ok(Token::new(
                        TokenType::GreaterEquals,
                        "<=".to_owned(),
                        start_index,
                        start_index + 1,
                    ))
                } else {
                    Ok(Token::new(
                        TokenType::Greater,
                        ">".to_owned(),
                        start_index,
                        start_index,
                    ))
                }
            }
            ',' => Ok(Token::new(
                TokenType::Comma,
                ",".to_owned(),
                start_index,
                start_index,
            )),
            '{' => Ok(Token::new(
                TokenType::LeftBrace,
                "{".to_owned(),
                start_index,
                start_index,
            )),
            '}' => Ok(Token::new(
                TokenType::RightBrace,
                "}".to_owned(),
                start_index,
                start_index,
            )),
            ':' => Ok(Token::new(
                TokenType::Colon,
                ":".to_owned(),
                start_index,
                start_index,
            )),
            ';' => Ok(Token::new(
                TokenType::Semicolon,
                ";".to_owned(),
                start_index,
                start_index,
            )),
            c => {
                if c.is_ascii_alphabetic() {
                    let literal = self.read_while_condition(|c| c.is_ascii_alphabetic());
                    let offset = literal.len() - 1;
                    let token_type = token::lookup_identifier(&literal);
                    Ok(Token::new(
                        token_type,
                        literal,
                        start_index,
                        start_index + offset,
                    ))
                } else if c.is_ascii_digit() {
                    let literal = self.read_while_condition(|c| c.is_ascii_digit() || c == '.');
                    let offset = literal.len() - 1;

                    let token_type = if literal.contains('.') {
                        TokenType::Float
                    } else {
                        TokenType::Integer
                    };
                    Ok(Token::new(token_type, literal, start_index, offset))
                } else {
                    Err(format!("Invalid character '{c}'").to_owned())
                }
            }
        };

        self.read_next_char();

        return match next_token {
            Ok(token) => Some(token),
            Err(error) => {
                eprintln!("{}", error);
                None
            }
        };
    }

    fn read_while_condition(&mut self, condition: impl Fn(char) -> bool) -> String {
        let mut out = String::new();
        match self.current_char {
            Some((_, c)) => out.push(c),
            None => {}
        };
        while let Some(next_c) = self.peek()
            && condition(next_c)
        {
            self.read_next_char();
            out.push(next_c);
        }
        return out;
    }

    fn skip_whitespace(&mut self) {
        let whitespace_chars = vec![' ', '\n', '\t', '\r'];
        loop {
            let current_char = match self.current_char {
                Some((_, c)) => c,
                None => break,
            };
            if !whitespace_chars.contains(&current_char) {
                break;
            } else {
                self.read_next_char();
            }
        }
    }

    fn read_next_char(&mut self) -> Option<CurrentChar> {
        if let Some(current_char) = self.chars.next() {
            self.current_char = Some(current_char);
        } else {
            self.current_char = None;
        }
        return self.current_char;
    }

    fn peek(&mut self) -> Option<char> {
        match self.chars.peek() {
            Some((_, peek_char)) => Some(*peek_char),
            None => None,
        }
    }
}
