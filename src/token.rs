use crate::error::{ParseError, ParseErrorKind, Result};
use std::str::from_utf8;

#[derive(Debug)]
pub struct TokenStream<'a> {
    line: usize,
    pointer: usize,
    source: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    line: usize,
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
    pub fn new(kind: TokenKind, line: usize) -> Self {
        Self { kind, line }
    }
}

impl<'a> TokenStream<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            line: 1,
            pointer: 0,
            source,
        }
    }

    pub fn next(&mut self) -> Result<Token> {
        if self.end_of_stream() {
            return Err(ParseError::new(self.line, ParseErrorKind::EndOfStream));
        };

        match self.char_at_pointer() {
            '{' => self.single_token(TokenKind::LeftCurlyBracket),
            '}' => self.single_token(TokenKind::RightCurlyBracket),
            '[' => self.single_token(TokenKind::LeftSquareBracket),
            ']' => self.single_token(TokenKind::RightSquareBracket),
            ',' => self.single_token(TokenKind::Comma),
            ':' => self.single_token(TokenKind::Colon),
            '"' => self.tokenize_string(),
            ' ' => {
                self.pointer += 1;
                self.next()
            }
            char if char.is_ascii_digit() => self.tokenize_number(),
            char if char.is_ascii_alphabetic() => self.tokenize_literal(),
            '\n' | '\r' | '\t' => {
                self.pointer += 1;
                self.line += 1;
                self.next()
            }
            _ => return Err(ParseError::new(self.line, ParseErrorKind::InvalidToken)),
        }
    }

    pub fn peek(&mut self) -> Result<Token> {
        let previous_pointer = self.pointer;
        let previous_line = self.line;

        let result = self.next();

        self.pointer = previous_pointer;
        self.line = previous_line;
        result
    }

    fn tokenize_literal(&mut self) -> Result<Token> {
        let start = self.pointer;

        while !self.end_of_stream() && self.char_at_pointer().is_ascii_alphabetic() {
            self.pointer += 1;
        }

        let bool_lexeme = from_utf8(&self.source.as_bytes()[start..self.pointer])
            .map_err(|_| ParseError::new(self.line, ParseErrorKind::InvalidString))?;

        match bool_lexeme {
            "true" => Ok(Token::new(TokenKind::True, self.line)),
            "false" => Ok(Token::new(TokenKind::False, self.line)),
            "null" => Ok(Token::new(TokenKind::Null, self.line)),
            _ => return Err(ParseError::new(self.line, ParseErrorKind::InvalidValue)),
        }
    }

    fn tokenize_number(&mut self) -> Result<Token> {
        let start = self.pointer;

        //TODO: handle float numbers
        while !self.end_of_stream() && self.char_at_pointer().is_ascii_digit() {
            self.pointer += 1;
        }

        let number_lexeme = from_utf8(&self.source.as_bytes()[start..self.pointer])
            .map_err(|_| ParseError::new(self.line, ParseErrorKind::InvalidString))?
            .to_string();

        Ok(Token::new(
            TokenKind::Number(
                number_lexeme
                    .parse()
                    .map_err(|_| ParseError::new(self.line, ParseErrorKind::InvalidNumber))?,
            ),
            self.line,
        ))
    }

    fn tokenize_string(&mut self) -> Result<Token> {
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

        let result = Token::new(TokenKind::String(string.clone()), self.line);

        self.pointer += 1;

        Ok(result)
    }
    fn single_token(&mut self, kind: TokenKind) -> Result<Token> {
        let result = Token::new(kind, self.line);
        self.pointer += 1;
        Ok(result)
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
    use crate::token::{Token, TokenKind};

    #[test]
    fn test_single_token() {
        let mut ts = TokenStream::new(",:{}[]");
        assert_eq!(ts.next().unwrap(), Token::new(TokenKind::Comma, 1))
    }

    #[test]
    fn test_string() {
        let mut ts = TokenStream::new("\"Cleitonrasta\"");
        assert_eq!(
            ts.next().unwrap(),
            Token::new(TokenKind::String(String::from("\"Cleitonrasta\"")), 1,)
        )
    }

    #[test]
    fn test_number() {
        let mut ts = TokenStream::new("64");
        assert_eq!(ts.next().unwrap(), Token::new(TokenKind::Number(64.0), 1))
    }

    #[test]
    fn test_literals() {
        let mut ts = TokenStream::new("true false null");
        assert_eq!(ts.next().unwrap(), Token::new(TokenKind::True, 1))
    }

    #[test]
    fn test_full_syntax() {
        let string = fs::read_to_string("test.json").unwrap();
        let mut ts = TokenStream::new(&string);

        let expected_tokens = vec![
            TokenKind::LeftCurlyBracket,
            TokenKind::String("\"name\"".to_string()),
            TokenKind::Colon,
            TokenKind::String("\"Alice\"".to_string()),
            TokenKind::Comma,
            TokenKind::String("\"age\"".to_string()),
            TokenKind::Colon,
            TokenKind::Number(30.0),
            TokenKind::Comma,
            TokenKind::String("\"isStudent\"".to_string()),
            TokenKind::Colon,
            TokenKind::False,
            TokenKind::Comma,
            TokenKind::String("\"skills\"".to_string()),
            TokenKind::Colon,
            TokenKind::LeftSquareBracket,
            TokenKind::String("\"Rust\"".to_string()),
            TokenKind::Comma,
            TokenKind::String("\"JavaScript\"".to_string()),
            TokenKind::Comma,
            TokenKind::String("\"Python\"".to_string()),
            TokenKind::RightSquareBracket,
            TokenKind::Comma,
            TokenKind::String("\"address\"".to_string()),
            TokenKind::Colon,
            TokenKind::LeftCurlyBracket,
            TokenKind::String("\"street\"".to_string()),
            TokenKind::Colon,
            TokenKind::String("\"123 Maple Street\"".to_string()),
            TokenKind::Comma,
            TokenKind::String("\"city\"".to_string()),
            TokenKind::Colon,
            TokenKind::String("\"Wonderland\"".to_string()),
            TokenKind::Comma,
            TokenKind::String("\"zip\"".to_string()),
            TokenKind::Colon,
            TokenKind::String("\"12345\"".to_string()),
            TokenKind::RightCurlyBracket,
            TokenKind::RightCurlyBracket,
        ];

        for expected_kind in expected_tokens {
            let token = ts.next().unwrap();
            assert_eq!(token.kind, expected_kind);
        }

        assert!(ts.next().is_err())
    }

    #[test]
    fn test_errors() {
        assert!(TokenStream::new("\"Cleitonrasta").next().is_err())
    }

    #[test]
    fn test_peek_does_not_advance() {
        let mut ts = TokenStream::new("true false");

        let peeked = ts.peek().unwrap();
        let next = ts.next().unwrap();

        assert_eq!(peeked, next);

        let peeked = ts.peek().unwrap();
        let next = ts.next().unwrap();

        assert_eq!(peeked, next);
    }
}
