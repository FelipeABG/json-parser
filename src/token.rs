pub struct TokenStream<'a> {
    tokens: Vec<Token>,
    line: usize,
    pointer: usize,
    source: &'a String,
}

pub struct Token {
    kind: TokenKind,
    line: usize,
}

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
    pub fn new(kind: TokenKind, line: usize) -> Self {
        Self { kind, line }
    }
}

impl<'a> TokenStream<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            tokens: Vec::new(),
            line: 1,
            pointer: 0,
            source,
        }
    }
}
