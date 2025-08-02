pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
pub struct ParseError {
    line: usize,
    kind: ParseErrorKind,
}

#[derive(Debug)]
pub enum ParseErrorKind {
    InvalidToken,
    InvalidString,
    InvalidNumber,
    InvalidValue,
    UnterminatedString,
    EndOfStream,
    NotAPrimitive,
}

impl ParseError {
    pub fn new(line: usize, kind: ParseErrorKind) -> Self {
        Self { line, kind }
    }
}
