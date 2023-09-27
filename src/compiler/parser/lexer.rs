use logos::Logos;

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    ParseIntegerNumberError,
    ParseRealNumberError,
    #[default]
    Error,
}

impl From<std::num::ParseIntError> for LexingError {
    fn from(_: std::num::ParseIntError) -> Self {
        LexingError::ParseIntegerNumberError
    }
}

impl From<std::num::ParseFloatError> for LexingError {
    fn from(_: std::num::ParseFloatError) -> Self {
        LexingError::ParseRealNumberError
    }
}

#[derive(Default)]
pub struct TokenExtras {
    pub line: usize,
}

#[derive(logos::Logos, PartialEq, Copy, Clone)]
#[logos(error = LexingError)]
#[logos(extras = TokenExtras)]
#[logos(skip r"[ \t\f]+")]
pub enum TokenKind {
    #[token("\n", |lex| {
        lex.extras.line += 1;
        logos::Skip
    })]
    LineBreak,

    #[regex("//[^\n]*\n", |lex| {
        lex.extras.line += 1;
        logos::Skip
    })]
    Comment,
    #[regex("/\\*([^*]|\\*[^/])*\\*/", |_| {
        logos::Skip
    })]
    MultiLineComment,

    #[token("&")]
    Amp, // &
    #[token("&&")]
    Amp2, // &&
    #[token("!")]
    Bang, // !
    #[token("^")]
    Caret, // ^
    #[token(":")]
    Colon, // :
    #[token(",")]
    Comma, // ,
    #[token(".")]
    Dot, // .
    #[token("=")]
    Eq, // =
    #[token("==")]
    Eq2, // ==
    #[token("<")]
    LAngle, // <
    #[token("<=")]
    LtEq, // <=
    #[token("<<")]
    ShiftLeft, // <<
    #[token(">")]
    RAngle, // >
    #[token(">=")]
    GtEq, // >=
    #[token(">>")]
    ShiftRight, // >>
    #[token("[")]
    LBrack, // [
    #[token("]")]
    RBrack, // ]
    #[token("{")]
    LCurly, // {
    #[token("}")]
    RCurly, // }
    #[token("(")]
    LParen, // (
    #[token(")")]
    RParen, // )
    #[token("-")]
    Minus, // -
    #[token("!=")]
    NEq, // !=
    #[token("%")]
    Percent, // %
    #[token("|")]
    Pipe, // |
    #[token("||")]
    Pipe2, // ||
    #[token("+")]
    Plus, // +
    #[token("?")]
    Question, // ?
    #[token(";")]
    Semicolon, // ;
    #[token("/")]
    Slash, // /
    #[token("*")]
    Star, // *
    #[token("~")]
    Tilde, // ~

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Integer(i64),
    #[regex("[0-9]+.[0-9]+", |lex| lex.slice().parse())]
    Real(f64),
    #[regex("\"([^\"\\\\]|\\.)*\"")]
    Literal,

    // Keywords
    #[token("null")]
    NullKeyword,
    #[token("true")]
    TrueKeyword,
    #[token("false")]
    FalseKeyword,
    #[token("struct")]
    StructKeyword,
    #[token("variant")]
    VariantKeyword,
    #[token("var")]
    VarKeyword,
    #[token("opt")]
    OptKeyword,

    NoToken,
}

impl<'a> std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::LineBreak => f.write_str("line break"),
            TokenKind::Comment => f.write_str("comment"),
            TokenKind::MultiLineComment => f.write_str("multiline comment"),
            TokenKind::Amp => f.write_str("&"),
            TokenKind::Amp2 => f.write_str("&&"),
            TokenKind::Bang => f.write_str("!"),
            TokenKind::Caret => f.write_str("^"),
            TokenKind::Colon => f.write_str(":"),
            TokenKind::Comma => f.write_str(","),
            TokenKind::Dot => f.write_str("."),
            TokenKind::Eq => f.write_str("="),
            TokenKind::Eq2 => f.write_str("=="),
            TokenKind::GtEq => f.write_str(">="),
            TokenKind::LAngle => f.write_str("<"),
            TokenKind::LBrack => f.write_str("["),
            TokenKind::LCurly => f.write_str("{"),
            TokenKind::LParen => f.write_str("("),
            TokenKind::LtEq => f.write_str("<="),
            TokenKind::Minus => f.write_str("-"),
            TokenKind::NEq => f.write_str("!="),
            TokenKind::Percent => f.write_str("%"),
            TokenKind::Pipe => f.write_str("|"),
            TokenKind::Pipe2 => f.write_str("||"),
            TokenKind::Plus => f.write_str("+"),
            TokenKind::Question => f.write_str("?"),
            TokenKind::RAngle => f.write_str(">"),
            TokenKind::RBrack => f.write_str("]"),
            TokenKind::RCurly => f.write_str("}"),
            TokenKind::RParen => f.write_str(")"),
            TokenKind::Semicolon => f.write_str(";"),
            TokenKind::ShiftLeft => f.write_str("<<"),
            TokenKind::ShiftRight => f.write_str(">>"),
            TokenKind::Slash => f.write_str("/"),
            TokenKind::Star => f.write_str("*"),
            TokenKind::Tilde => f.write_str("~"),

            TokenKind::Identifier => f.write_str("identifier"),
            TokenKind::Integer(_) => f.write_str("integer"),
            TokenKind::Real(_) => f.write_str("real"),
            TokenKind::Literal => f.write_str("literal"),

            // Keywords
            TokenKind::NullKeyword => f.write_str("null keyword"),
            TokenKind::TrueKeyword => f.write_str("true keyword"),
            TokenKind::FalseKeyword => f.write_str("false keyword"),
            TokenKind::StructKeyword => f.write_str("struct keyword"),
            TokenKind::VariantKeyword => f.write_str("variant keyword"),
            TokenKind::VarKeyword => f.write_str("var keyword"),
            TokenKind::OptKeyword => f.write_str("opt keyword"),

            TokenKind::NoToken => f.write_str("no token"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Token<'a> {
    pub line: usize,

    pub span: (usize, usize),

    pub kind: TokenKind,
    pub slice: &'a str,
}

pub struct Lexer<'a> {
    lexer: logos::Lexer<'a, TokenKind>,
    current: Token<'a>,
    peeked: Option<Token<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            lexer: TokenKind::lexer(input),
            current: Token {
                line: 0,
                span: (0, 0),
                kind: TokenKind::NoToken,
                slice: "",
            },
            peeked: None,
        };

        lexer.current = lexer.raw_next();

        lexer
    }

    pub fn current(&self) -> Token<'a> {
        self.current
    }

    pub fn next(&mut self) -> Token<'a> {
        self.current = match self.peeked.take() {
            Some(token) => token,
            None => self.raw_next(),
        };

        self.current
    }

    pub fn peek(&mut self) -> Token<'a> {
        match self.peeked {
            Some(token) => token,
            None => {
                let token = self.raw_next();
                self.peeked = Some(token);
                token
            }
        }
    }

    pub fn consume(&mut self) {
        self.current = match self.peeked.take() {
            Some(token) => token,
            None => self.raw_next(),
        };
    }

    fn raw_next(&mut self) -> Token<'a> {
        match self.lexer.next() {
            Some(Ok(kind)) => {
                let span = self.lexer.span();
                Token {
                    line: self.lexer.extras.line,
                    span: (span.start, span.end),
                    kind: kind,
                    slice: self.lexer.slice(),
                }
            }
            _ => Token {
                line: self.lexer.extras.line,
                span: (0, 0),
                kind: TokenKind::NoToken,
                slice: self.lexer.slice(),
            },
        }
    }
}
