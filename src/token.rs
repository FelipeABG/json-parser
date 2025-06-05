use std::str::from_utf8;

use crate::error::{ParseError, ParseErrorKind, Result};

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
                '"' => stream.parse_string()?,
                ' ' => {
                    stream.pointer += 1;
                    continue;
                }
                _ => return Err(ParseError::new(stream.line, ParseErrorKind::InvalidToken)),
            };
        }

        Ok(stream)
    }

    fn parse_string(&mut self) -> Result<()> {
        let start = self.pointer;

        self.pointer += 1;
        //TODO: handle unclosed strings
        while self.char_at_pointer() != '"' {
            self.pointer += 1;
        }

        let string = from_utf8(&self.source.as_bytes()[start..=self.pointer])
            .map_err(|_| ParseError::new(self.line, ParseErrorKind::InvalidStringChar))?
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
        if self.source.len() > self.pointer {
            return false;
        }
        true
    }

    fn char_at(&self, idx: usize) -> char {
        self.source.as_bytes()[idx] as char
    }

    fn char_at_pointer(&self) -> char {
        self.source.as_bytes()[self.pointer] as char
    }
}

#[cfg(test)]
mod token_test {
    use super::TokenStream;
    use crate::token::{Token, TokenKind};

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
}
