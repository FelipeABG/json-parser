pub type Result<T> = std::result::Result<T, ParseError>;

pub struct ParseError {
    line: usize,
    kind: ParseErrorKind,
}

pub enum ParseErrorKind {}

impl ParseError {
    pub fn new(line: usize, kind: ParseErrorKind) -> Self {
        Self { line, kind }
    }
}
