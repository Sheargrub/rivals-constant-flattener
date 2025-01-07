#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum Token {
    Dot,
    Equal,
    NewLine, // Since newlines act as terminators
    Semicolon, // Also terminators, but more opinionated
    IgnoreBegin,
    IgnoreEnd,
    DeformatBegin,
    DeformatEnd,
    Symbol(String),
    OpenBracket(String),
    CloseBracket(String),
    Literal(String),
    Identifier(String),
    ShortComment(String), // Distinguished by its reliance on newline characters
    LongComment(String),
    Whitespace(String), // Since this is meant to preserve formatting
}