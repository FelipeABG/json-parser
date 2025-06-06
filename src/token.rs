use crate::error::{ParseError, ParseErrorKind, Result};
use std::str::from_utf8;

#[derive(Debug)]
pub struct TokenStream<'a> {
    tokens: Vec<Token>,
    line: usize,
    pointer: usize,
    source: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    line: usize,
    lexeme: String,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    LeftCurlyBracket,
    RightCurlyBracket,
    LeftSquareBracket,
    RightSquareBracket,
    Comma,
    Colon,

    String(String),
    Number(f64),

    True,
    False,
    Null,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, lexeme: String) -> Self {
        Self { kind, line, lexeme }
    }
}

impl<'a> TokenStream<'a> {
    pub fn build(source: &'a str) -> Result<Self> {
        let mut stream = Self {
            tokens: Vec::new(),
            line: 1,
            pointer: 0,
            source,
        };

        while !stream.end_of_stream() {
            match stream.char_at_pointer() {
                '{' => stream.single_token(TokenKind::LeftCurlyBracket),
                '}' => stream.single_token(TokenKind::RightCurlyBracket),
                '[' => stream.single_token(TokenKind::LeftSquareBracket),
                ']' => stream.single_token(TokenKind::RightSquareBracket),
                ',' => stream.single_token(TokenKind::Comma),
                ':' => stream.single_token(TokenKind::Colon),
                '"' => stream.tokenize_string()?,
                ' ' => stream.pointer += 1,
                char if char.is_ascii_digit() => stream.tokenize_number()?,
                char if char.is_ascii_alphabetic() => stream.tokenize_literal()?,
                '\n' | '\r' | '\t' => {
                    stream.pointer += 1;
                    stream.line += 1
                }
                _ => return Err(ParseError::new(stream.line, ParseErrorKind::InvalidToken)),
            };
        }

        Ok(stream)
    }

    fn tokenize_literal(&mut self) -> Result<()> {
        let start = self.pointer;

        while !self.end_of_stream() && self.char_at_pointer().is_ascii_alphabetic() {
            self.pointer += 1;
        }

        let bool_lexeme = from_utf8(&self.source.as_bytes()[start..self.pointer])
            .map_err(|_| ParseError::new(self.line, ParseErrorKind::InvalidString))?;

        match bool_lexeme {
            "true" => self.tokens.push(Token::new(
                TokenKind::True,
                self.line,
                bool_lexeme.to_string(),
            )),
            "false" => self.tokens.push(Token::new(
                TokenKind::False,
                self.line,
                bool_lexeme.to_string(),
            )),
            "null" => self.tokens.push(Token::new(
                TokenKind::Null,
                self.line,
                bool_lexeme.to_string(),
            )),
            _ => return Err(ParseError::new(self.line, ParseErrorKind::InvalidValue)),
        }

        Ok(())
    }

    fn tokenize_number(&mut self) -> Result<()> {
        let start = self.pointer;

        //TODO: handle float numbers
        while !self.end_of_stream() && self.char_at_pointer().is_ascii_digit() {
            self.pointer += 1;
        }

        let number_lexeme = from_utf8(&self.source.as_bytes()[start..self.pointer])
            .map_err(|_| ParseError::new(self.line, ParseErrorKind::InvalidString))?
            .to_string();

        self.tokens.push(Token::new(
            TokenKind::Number(
                number_lexeme
                    .parse()
                    .map_err(|_| ParseError::new(self.line, ParseErrorKind::InvalidNumber))?,
            ),
            self.line,
            number_lexeme,
        ));

        Ok(())
    }

    fn tokenize_string(&mut self) -> Result<()> {
        let start = self.pointer;

        self.pointer += 1;

        while !self.end_of_stream() && self.char_at_pointer() != '"' {
            if self.char_at_pointer() == '\n' {
                return Err(ParseError::new(
                    self.line,
                    ParseErrorKind::UnterminatedString,
                ));
            }
            self.pointer += 1;
        }

        if self.end_of_stream() {
            return Err(ParseError::new(
                self.line,
                ParseErrorKind::UnterminatedString,
            ));
        }

        let string = from_utf8(&self.source.as_bytes()[start..=self.pointer])
            .map_err(|_| ParseError::new(self.line, ParseErrorKind::InvalidString))?
            .to_string();

        self.tokens.push(Token::new(
            TokenKind::String(string.clone()),
            self.line,
            string,
        ));

        self.pointer += 1;

        Ok(())
    }
    fn single_token(&mut self, kind: TokenKind) {
        self.tokens.push(Token::new(
            kind,
            self.line,
            self.char_at_pointer().to_string(),
        ));
        self.pointer += 1;
    }

    fn end_of_stream(&self) -> bool {
        !(self.source.len() > self.pointer)
    }

    fn char_at_pointer(&self) -> char {
        self.source.as_bytes()[self.pointer] as char
    }
}

#[cfg(test)]
mod token_test {
    use std::fs;

    use super::TokenStream;
    use crate::{
        error::ParseError,
        token::{Token, TokenKind},
    };

    #[test]
    fn test_single_token() {
        let ts = TokenStream::build(",:{}[]").unwrap();
        assert_eq!(ts.tokens.len(), 6);
        assert_eq!(
            ts.tokens[0],
            Token::new(TokenKind::Comma, 1, ",".to_string())
        )
    }

    #[test]
    fn test_string() {
        let ts = TokenStream::build("\"Cleitonrasta\"").unwrap();
        assert_eq!(ts.tokens.len(), 1);
        assert_eq!(
            ts.tokens[0],
            Token::new(
                TokenKind::String(String::from("\"Cleitonrasta\"")),
                1,
                "\"Cleitonrasta\"".to_string()
            )
        )
    }

    #[test]
    fn test_number() {
        let ts = TokenStream::build("64").unwrap();
        assert_eq!(ts.tokens.len(), 1);
        assert_eq!(
            ts.tokens[0],
            Token::new(TokenKind::Number(64.0), 1, "64".to_string())
        )
    }

    #[test]
    fn test_literals() {
        let ts = TokenStream::build("true false null").unwrap();
        assert_eq!(ts.tokens.len(), 3);
        assert_eq!(
            ts.tokens[0],
            Token::new(TokenKind::True, 1, "true".to_string())
        )
    }

    #[test]
    fn test_full_syntax() {
        let string = fs::read_to_string("test.json").unwrap();
        TokenStream::build(&string).unwrap();
    }

    #[test]
    fn test_errors() {
        assert!(TokenStream::build("\"Cleitonrasta").is_err())
    }
}
