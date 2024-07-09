use crate::compiler::Token;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Default)]
pub(in crate::compiler) enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String,
    InterpolatedString,
    Number,

    And,
    Class,
    Else,
    False,
    For,
    Fn,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,

    #[default]
    Eof,
}

impl Token {
    pub(in crate::compiler) fn new(
        token_type: TokenType,
        token: String,
        start: usize,
        line: u32,
    ) -> Token {
        Token {
            token_type,
            token,
            start,
            line,
        }
    }
}
