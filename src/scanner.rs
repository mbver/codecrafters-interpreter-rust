use std::str::Chars;
use std::{fmt::Display, iter::Peekable};

use crate::token::{Literal, Token, TokenType, keyword_token_type};

pub struct Scanner<'a> {
    reader: CharReader<'a>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    has_error: bool,
}

struct CharReader<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
}

impl<'a> CharReader<'a> {
    fn peek_next(&self) -> Option<char> {
        let mut chars = self.chars.clone();
        chars.next()?;
        chars.next()
    }
}

#[derive(Debug)]
pub enum ScanError {
    UnexpectedCharacter { line: usize, character: char },
    UnterminatedString { line: usize },
}

impl Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanError::UnexpectedCharacter { line, character } => {
                write!(f, "[line {}] Unexpected character: {}", line, character)
            }
            ScanError::UnterminatedString { line } => {
                write!(f, "[line {}] Unterminated string", line)
            }
        }
    }
}
impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            reader: CharReader {
                source: source,
                chars: source.chars().peekable(),
            },
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
            has_error: false,
        }
    }
    pub fn scan_tokens(&mut self) {
        loop {
            match self.scan_token() {
                Ok(true) => {}
                Ok(false) => break,
                Err(e) => {
                    eprintln!("{e}");
                    self.has_error = true;
                }
            }
            self.start = self.current;
        }
        self.tokens.push(Token::new(
            TokenType::EOF,
            "".to_string(),
            Literal::null,
            self.line,
        ));
    }
    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
    fn advance(&mut self) -> Option<char> {
        let c = self.reader.chars.next()?;
        self.current += c.len_utf8();
        Some(c)
    }
    fn match_char(&mut self, expected: char) -> bool {
        if self.reader.chars.peek() == Some(&expected) {
            self.advance();
            return true;
        }
        false
    }
    fn match_and_add(&mut self, expected: char, matched: TokenType, unmatched: TokenType) {
        let token_type = if self.match_char(expected) {
            matched
        } else {
            unmatched
        };
        self.add_simple_token(token_type);
    }
    fn read_string(&mut self) -> Result<(), ScanError> {
        loop {
            match self.reader.chars.peek() {
                Some('"') => break,
                Some(&ch) => {
                    if ch == '\n' {
                        self.line += 1;
                    }
                    self.advance();
                }
                None => return Err(ScanError::UnterminatedString { line: self.line }),
            }
        }
        self.advance(); // the closing '"'
        let value = self.reader.source[self.start + 1..self.current - 1].to_string(); // " is 1 byte in utf8 encoding
        self.add_token(TokenType::String, Literal::String(value));
        Ok(())
    }
    fn has_next_digit(&mut self) -> bool {
        self.reader
            .chars
            .peek()
            .is_some_and(|ch| ch.is_ascii_digit())
    }
    fn has_next_alphanumeric(&mut self) -> bool {
        self.reader
            .chars
            .peek()
            .is_some_and(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
    }
    fn read_number(&mut self) {
        while self.has_next_digit() {
            self.advance();
        }
        if self.reader.chars.peek() == Some(&'.')
            && self
                .reader
                .peek_next()
                .is_some_and(|ch| ch.is_ascii_digit())
        {
            self.advance();
            while self.has_next_digit() {
                self.advance();
            }
        }

        let value = self.reader.source[self.start..self.current]
            .parse::<f64>()
            .unwrap(); // can't be invalide number?
        self.add_token(TokenType::Number, Literal::Number(value));
    }
    fn read_identifier(&mut self) {
        while self.has_next_alphanumeric() {
            self.advance();
        }
        let word = &self.reader.source[self.start..self.current];
        let token_type = keyword_token_type(word).unwrap_or(TokenType::Identifier);
        self.add_simple_token(token_type);
    }
    fn add_token(&mut self, token_type: TokenType, literal: Literal) {
        let lexeme = self.reader.source[self.start..self.current].to_string();
        self.tokens
            .push(Token::new(token_type, lexeme, literal, self.line));
    }
    fn add_simple_token(&mut self, token_type: TokenType) {
        self.add_token(token_type, Literal::null);
    }
    // return false at EOF
    pub fn scan_token(&mut self) -> Result<bool, ScanError> {
        let Some(ch) = self.advance() else {
            return Ok(false);
        };
        match ch {
            '(' => self.add_simple_token(TokenType::LEFT_PAREN),
            ')' => self.add_simple_token(TokenType::RIGHT_PAREN),
            '{' => self.add_simple_token(TokenType::LEFT_BRACE),
            '}' => self.add_simple_token(TokenType::RIGHT_BRACE),
            ',' => self.add_simple_token(TokenType::COMMA),
            '.' => self.add_simple_token(TokenType::DOT),
            '-' => self.add_simple_token(TokenType::MINUS),
            '+' => self.add_simple_token(TokenType::PLUS),
            ';' => self.add_simple_token(TokenType::SEMICOLON),
            '*' => self.add_simple_token(TokenType::STAR),
            '!' => self.match_and_add('=', TokenType::BangEqual, TokenType::BANG),
            '=' => self.match_and_add('=', TokenType::EqualEqual, TokenType::EQUAL),
            '<' => self.match_and_add('=', TokenType::LessEqual, TokenType::LESS),
            '>' => self.match_and_add('=', TokenType::GreaterEqual, TokenType::GREATER),
            '/' => {
                if self.match_char('/') {
                    while let Some(&ch) = self.reader.chars.peek()
                        && ch != '\n'
                    {
                        self.advance();
                    }
                } else {
                    self.add_simple_token(TokenType::SLASH);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => self.read_string()?,
            ch if ch.is_ascii_digit() => {
                self.read_number();
            }
            ch if ch.is_ascii_alphabetic() || ch == '_' => self.read_identifier(),
            ch => {
                return Err(ScanError::UnexpectedCharacter {
                    line: self.line,
                    character: ch,
                });
            }
        };
        Ok(true) // continue
    }
    pub fn has_error(&self) -> bool {
        self.has_error
    }
}
