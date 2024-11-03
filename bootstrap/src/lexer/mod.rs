use std::cell::RefCell;

use crate::core::position::{LineColumn, LineNumber, Position, SourceIndex};
use crate::core::token::Token;
use crate::lexer::Error::UnexpectedEndOfFile;

mod separator;
mod operator;
mod keyword;
mod literal;

#[derive(Debug)]
pub enum Error {
    OutOfBounds,
    UnexpectedEndOfFile,
    UnknownKeyword(String),
    UnknownOperator(String),
    UnknownSeparator(String),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Clone)]
pub struct Reader<'a> {
    data: &'a str,
    pos: RefCell<usize>,
}

impl<'a> Reader<'a> {
    pub(crate) fn new(data: &'a str) -> Self {
        Reader {
            data,
            pos: RefCell::new(0),
        }
    }

    pub(crate) fn length(&self) -> usize {
        self.data.len()
    }

    pub(crate) fn pos(&self) -> usize {
        *self.pos.borrow()
    }

    pub(crate) fn consume_next(&self) -> Result<char> {
        let mut pos = self.pos.borrow_mut();
        if *pos >= self.data.len() {
            return Err(UnexpectedEndOfFile);
        }

        let next_char = self.data.chars().nth(*pos).ok_or(UnexpectedEndOfFile)?;
        *pos += 1;
        Ok(next_char)
    }

    pub(crate) fn at_the_end(&self) -> bool {
        *self.pos.borrow() >= self.data.len()
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
        if pos >= self.data.len() {
            return None;
        }

        let result: String = self.data.chars().skip(pos).take(sequence.len()).collect();
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
        if pos >= self.data.len() {
            return None;
        }

        self.data.chars().nth(pos).map(|c| Some(c)).unwrap_or(None)
    }

    pub fn peek_many(&self, window: usize) -> Option<String> {
        let pos = *self.pos.borrow();
        if pos >= self.data.len() {
            return None;
        }

        let chars: Vec<char> = self.data.chars().skip(pos).take(window).collect();
        if chars.is_empty() {
            return None;
        }

        Some(chars.into_iter().collect())
    }

    pub fn peek_if(&self, sequence: &str) -> Option<String> {
        let pos = *self.pos.borrow();
        if pos >= self.data.len() {
            return None;
        }

        let chars: String = self.data.chars().skip(pos).take(sequence.len()).collect();
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

pub struct Lexer<'a> {
    reader: Reader<'a>,
    current_line: RefCell<LineNumber>,
    current_column: RefCell<LineColumn>,
}

impl<'a> Lexer<'a> {
    pub fn new(str: &'a str) -> Self {
        Self {
            reader: Reader::new(str),
            current_line: RefCell::new(LineNumber(1)),
            current_column: RefCell::new(LineColumn(1)),
        }
    }

    pub fn advance(&self) -> Result<Token> {
        if let Some(next) = self.reader.peek_next() {
            match next {
                _ if self.is_whitespace(next) => self.consume_whitespace(),
                _ if self.is_operator(next) => self.consume_operator(),
                _ if self.is_separator(next) => self.consume_separator(),
                _ if self.is_keyword(next) => self.consume_keyword(),
                _ if self.is_string(next) => self.consume_string(),
                _ if self.is_number(next) => self.consume_number(),
                _ if self.is_bool(next) => self.consume_bool(),
                _ => unimplemented!()
            }
        } else {
            return Err(UnexpectedEndOfFile);
        }
    }

    pub(crate) fn position(&self) -> Position {
        Position {
            line: self.current_line.borrow().clone(),
            column: self.current_column.borrow().clone(),
            index: SourceIndex(*self.reader.pos.borrow()),
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
}
