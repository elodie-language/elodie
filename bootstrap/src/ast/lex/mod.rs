use std::cell::RefCell;

use crate::ast::lex::Error::UnexpectedEndOfFile;
use crate::ast::lex::token::{Column, Index, Position, Row, TextSpan, Token, TokenKind};
use crate::ast::lex::token::TokenKind::EOF;

mod comment;
mod separator;
mod operator;
mod keyword;
mod literal;
mod identifier;
pub mod token;

#[derive(Debug)]
pub enum Error {
    UnexpectedEndOfFile,
    UnknownKeyword(String),
    UnknownOperator(String),
    UnknownSeparator(String),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Clone)]
pub struct Reader<'a> {
    content: &'a str,
    pos: RefCell<usize>,
}

impl<'a> Reader<'a> {
    pub(crate) fn new(content: &'a str) -> Self {
        Reader {
            content,
            pos: RefCell::new(0),
        }
    }

    pub(crate) fn consume_next(&self) -> Result<char> {
        let mut pos = self.pos.borrow_mut();
        if *pos >= self.content.len() {
            return Err(UnexpectedEndOfFile);
        }

        let next_char = self.content.chars().nth(*pos).ok_or(UnexpectedEndOfFile)?;
        *pos += 1;
        Ok(next_char)
    }

    pub(crate) fn at_the_end(&self) -> bool {
        *self.pos.borrow() >= self.content.len()
    }

    pub(crate) fn consume_while(&self, test: impl Fn(char) -> bool) -> Result<String> {
        let mut result = String::new();
        loop {
            if self.at_the_end() {
                return Ok(result);
            }
            if let Some(next) = self.peek_next() {
                if test(next) {
                    result.push(self.consume_next()?)
                } else {
                    return Ok(result);
                }
            } else {
                return Ok(result);
            }
        }
    }

    pub fn consume_if(&self, sequence: &str) -> Option<String> {
        let pos = *self.pos.borrow();
        if pos >= self.content.len() {
            return None;
        }

        let result: String = self.content.chars().skip(pos).take(sequence.len()).collect();
        if result.is_empty() {
            return None;
        }

        if result == sequence {
            let mut pos = self.pos.borrow_mut();
            *pos += result.len();
            Some(result)
        } else {
            None
        }
    }

    pub fn peek_next(&self) -> Option<char> {
        let pos = *self.pos.borrow();
        if pos >= self.content.len() {
            return None;
        }

        self.content.chars().nth(pos).map(|c| Some(c)).unwrap_or(None)
    }

    pub fn peek_many(&self, window: usize) -> Option<String> {
        let pos = *self.pos.borrow();
        if pos >= self.content.len() {
            return None;
        }

        let chars: Vec<char> = self.content.chars().skip(pos).take(window).collect();
        if chars.is_empty() {
            return None;
        }

        Some(chars.into_iter().collect())
    }

    pub fn peek_if(&self, sequence: &str) -> Option<String> {
        let pos = *self.pos.borrow();
        if pos >= self.content.len() {
            return None;
        }

        let chars: String = self.content.chars().skip(pos).take(sequence.len()).collect();
        if chars.is_empty() {
            return None;
        }

        if chars == sequence {
            Some(chars)
        } else {
            None
        }
    }

    pub(crate) fn peek_while(&self, test: impl Fn(char) -> bool) -> Result<String> {
        let mut result = String::new();
        let temp_reader = self.clone();
        temp_reader.consume_while(test)
    }
}

pub fn lex(str: &str) -> Result<Vec<Token>> {
    let lexer = Lexer::new(str);
    lexer.all()
}

pub(crate) struct Lexer<'a> {
    reader: Reader<'a>,
    current_line: RefCell<Row>,
    current_column: RefCell<Column>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(str: &'a str) -> Self {
        Self {
            reader: Reader::new(str),
            current_line: RefCell::new(Row(1)),
            current_column: RefCell::new(Column(1)),
        }
    }

    pub(crate) fn all(&self) -> Result<Vec<Token>> {
        let mut result = vec![];
        loop {
            let token = self.advance()?;
            if token.kind == EOF {
                result.push(token);
                break;
            } else {
                result.push(token);
            }
        }

        Ok(result)
    }

    pub fn advance(&self) -> Result<Token> {
        if self.reader.at_the_end() {
            return Ok(Token {
                kind: TokenKind::EOF,
                span: TextSpan {
                    start: self.position(),
                    end: self.position(),
                    value: "".to_string(),
                },
            });
        }

        loop {
            if let Some(next) = self.reader.peek_next() {
                if self.is_whitespace(next) {
                    self.consume_whitespace()?;
                } else if self.is_comment(next) {
                    self.consume_comment()?;
                } else {
                    break;
                }
            } else {
                return Ok(Token {
                    kind: TokenKind::EOF,
                    span: TextSpan {
                        start: self.position(),
                        end: self.position(),
                        value: "".to_string(),
                    },
                });
            }
        }

        if let Some(next) = self.reader.peek_next() {
            match next {
                _ if self.is_operator(next) => self.consume_operator(),
                _ if self.is_separator(next) => self.consume_separator(),
                _ if self.is_keyword(next) => self.consume_keyword(),
                _ if self.is_string(next) => self.consume_string(),
                _ if self.is_number(next) => self.consume_number(),
                _ if self.is_bool(next) => self.consume_bool(),
                _ => self.consume_identifier()
            }
        } else {
            return Err(UnexpectedEndOfFile);
        }
    }

    pub(crate) fn position(&self) -> Position {
        Position {
            row: self.current_line.borrow().clone(),
            column: self.current_column.borrow().clone(),
            index: Index(*self.reader.pos.borrow()),
        }
    }

    pub(crate) fn peek_next(&self) -> Option<char> {
        self.reader.peek_next()
    }

    pub(crate) fn peek_many(&self, window: usize) -> Option<String> {
        self.reader.peek_many(window)
    }

    pub(crate) fn peek_if(&self, sequence: &str) -> Option<String> {
        self.reader.peek_if(sequence)
    }

    pub(crate) fn peek_while(&self, test: impl Fn(char) -> bool) -> Result<String> {
        let result = self.reader.peek_while(test)?;
        Ok(result)
    }

    pub(crate) fn consume_next(&self) -> Result<char> {
        let result = self.reader.consume_next()?;
        self.current_column.borrow_mut().0 += 1;
        Ok(result)
    }

    pub(crate) fn consume_while(&self, test: impl Fn(char) -> bool) -> Result<String> {
        let result = self.reader.consume_while(test)?;
        self.current_column.borrow_mut().0 += result.len();
        Ok(result)
    }

    pub(crate) fn consume_if(&self, sequence: &str) -> Option<String> {
        if let Some(result) = self.reader.consume_if(sequence) {
            self.current_column.borrow_mut().0 += result.len();
            return Some(result);
        }
        None
    }

    pub(crate) fn look_ahead(&self) -> Result<String> {
        self.peek_while(|c| c.is_ascii_alphanumeric() || c == '_')
    }
}