// <json> ::= <primitive> | <container>
//
// <primitive> ::= <number> | <string> | <boolean>
// ; Where:
// ; <number> is a valid real number expressed in one of a number of given formats
// ; <string> is a string of valid characters enclosed in quotes
// ; <boolean> is one of the literal strings 'true', 'false', or 'null' (unquoted)
//
// <container> ::= <object> | <array>
// <array> ::= '[' [ <json> *(', ' <json>) ] ']' ; A sequence of JSON values separated by commas
// <object> ::= '{' [ <member> *(', ' <member>) ] '}' ; A sequence of 'members'
// <member> ::= <string> ': ' <json> ; A pair consisting of a name, and a JSON value

use crate::ast::Primitive;
use crate::error::ParseError;
use crate::error::ParseErrorKind;
use crate::error::Result;
use crate::token::{TokenKind, TokenStream};

struct Parser<'a> {
    ts: TokenStream<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(ts: TokenStream<'a>) -> Self {
        Self { ts }
    }

    pub fn parse(&mut self) -> Result<Vec<Primitive>> {
        let mut result = Vec::new();
        while !self.ts.end_of_stream() {
            result.push(self.parse_primitive()?)
        }

        Ok(result)
    }

    fn parse_primitive(&mut self) -> Result<Primitive> {
        let token = self.ts.next()?;

        match token.kind {
            TokenKind::String(s) => Ok(Primitive::String(s)),
            TokenKind::Number(n) => Ok(Primitive::Number(n)),
            TokenKind::True => Ok(Primitive::Boolean(true)),
            TokenKind::False => Ok(Primitive::Boolean(false)),
            TokenKind::Null => Ok(Primitive::Null),
            _ => Err(ParseError::new(token.line, ParseErrorKind::NotAPrimitive)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::Primitive, token::TokenStream};

    use super::Parser;

    #[test]
    fn test_parse_primitive() {
        let mut parser = Parser::new(TokenStream::new("35 \"meudeus\" false null"));
        let ast = parser.parse().unwrap();
        let mut iter = ast.iter();

        assert_eq!(iter.next().unwrap(), &Primitive::Number(35.0));
        assert_eq!(
            iter.next().unwrap(),
            &Primitive::String("\"meudeus\"".to_string())
        );
        assert_eq!(iter.next().unwrap(), &Primitive::Boolean(false));
        assert_eq!(iter.next().unwrap(), &Primitive::Null);
    }
}
