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

impl Token {

    pub fn get_string(&self) -> String {
        match self {
            Token::Dot => String::from("."),
            Token::Equal => String::from("="),
            Token::NewLine => String::from("\n"),
            Token::Semicolon => String::from(";"),
            Token::IgnoreBegin => String::from("//#RCFBEGINIGNORE"),
            Token::IgnoreEnd => String::from("//#RCFENDIGNORE"),
            Token::DeformatBegin => String::from("//#RCFBEGINDEFORMAT"),
            Token::DeformatEnd => String::from("//#RCFENDDEFORMAT"),

            Token::Symbol(s) => String::from(s),
            Token::Identifier(s) => String::from(s),
            Token::Literal(s) => String::from(s),
            Token::ShortComment(s) => String::from(s),
            Token::LongComment(s) => String::from(s),
            Token::Whitespace(s) => String::from(s),
            Token::OpenBracket(s) => String::from(s),
            Token::CloseBracket(s) => String::from(s),
        }
    }
}