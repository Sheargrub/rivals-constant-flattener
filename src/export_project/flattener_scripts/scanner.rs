use crate::export_project::flattener_scripts as flt;
use flt::token::Token;
use flt::token::Token::*;

pub struct RcfScanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    error_strings: Vec<String>,
    start: usize,
    current: usize,
    line: usize,
    inited: bool,
    valid: bool,
}

impl RcfScanner {

    pub fn new(source: &str) -> RcfScanner {
        let source = source.chars().collect();
        let tokens = Vec::new();
        let error_strings = Vec::new();
        
        RcfScanner {
            source,
            tokens,
            error_strings,
            start : 0,
            current : 0,
            line : 1,
            inited : false,
            valid : true,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<String>> {
        if self.inited {
            if self.valid { Ok(self.tokens.clone()) }
            else { Err(self.error_strings.clone()) }
        }
        else {
            self.inited = true;

            while !self.is_at_end() {
                let c = self.source[self.current];
                self.current += 1;

                match c {
                    '.' => self.add_token(Dot),
                    '\n' => self.add_token(NewLine),
                    ';' => self.add_token(Semicolon),

                    '(' => self.add_token(OpenBracket(String::from("("))),
                    '{' => self.add_token(OpenBracket(String::from("{"))),
                    '[' => self.add_token(OpenBracket(String::from("["))),

                    ')' => self.add_token(CloseBracket(String::from(")"))),
                    '}' => self.add_token(CloseBracket(String::from("}"))),
                    ']' => self.add_token(CloseBracket(String::from("]"))),

                    '=' => {
                        if self.match_char('=') { self.add_token(Symbol(String::from("=="))) }
                        else                    { self.add_token(Equal) }
                    },

                    ' ' => self.process_whitespace(),
                    '\t' => self.process_whitespace(),
                    '\r' => self.process_whitespace(),
                    '/' => self.process_comment(),
                    '"' => self.process_string(),
                    '\'' => self.process_string(),
                    '0'..='9' => self.process_number(),
                    'A'..='z' => self.process_identifier(),

                    c => self.add_token(Symbol(String::from(c))),
                };

                self.start = self.current;
            }

            if self.valid { Ok(self.tokens.clone()) }
            else { Err(self.error_strings.clone()) }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    } 

    // If the next character matches c, consumes it and returns true.
    // Otherwise, returns false.
    fn match_char(&mut self, c: char) -> bool {
        if self.is_at_end() { false }
        else if self.source[self.current] != c { false }
        else {
            self.current += 1;
            true
        }
    }

    fn process_whitespace(&mut self) {
        let mut ws = String::from(self.source[self.current-1]);
        while !self.is_at_end() && is_skippable(self.source[self.current]) {
            ws.push(self.source[self.current]);
            self.current += 1;
        }
        self.add_token(Whitespace(String::from(ws)))
    }

    fn process_comment(&mut self) {
        match self.source[self.current] {
            '/' => {
                self.current += 1;
                let mut comment = String::from("//");
                let mut whitespace = String::new();
                while !self.is_at_end() && self.source[self.current] != '\n' {
                    if is_skippable(self.source[self.current]) {
                        whitespace.push(self.source[self.current]);
                    } else {
                        if whitespace != "" {
                            comment.push_str(whitespace.as_str());
                            whitespace = String::from("");
                        }
                        comment.push(self.source[self.current]);
                    }
                    self.current += 1;
                }
                self.add_token( match comment.as_str() {
                    "//#RCFBEGINIGNORE" => Token::IgnoreBegin,
                    "//#RCFENDIGNORE" => Token::IgnoreEnd,
                    "//#RCFBEGINDEFORMAT" => Token::DeformatBegin,
                    "//#RCFENDDEFORMAT" => Token::DeformatEnd,
                    c => {
                        let mut out = String::from(c);
                        out.push_str(&whitespace);
                        Token::ShortComment(String::from(out))
                    },
                } );
            },
            '*' => {
                self.current += 1;
                let mut comment = String::from("/*");
                let mut looping = true; 
                while !self.is_at_end() && looping {
                    comment.push(self.source[self.current]);
                    if self.source[self.current] == '*' {
                        let next_index = self.current + 1;
                        if next_index < self.source.len() && self.source[next_index] == '/' {
                            self.current += 1;
                            comment.push('/');
                            looping = false;
                        }
                    }
                    self.current += 1;
                }
                self.add_token(Token::LongComment(String::from(comment)));
            }
            _ => self.add_token(Symbol(String::from("/"))),
        };
    }

    fn process_string(&mut self) {
        let quote = self.source[self.current-1];
        let begin = self.current;
        let start_line = self.line;
        while !self.is_at_end() && self.source[self.current] != quote {
            self.current += 1;
            if self.source[self.current-1] == '\\' && !self.is_at_end() && self.source[self.current] == quote {
                self.current += 1; // pass over escaped quotes
            }
        };
        
        if self.is_at_end() {
            self.add_error(&format!("Unterminated string starting at line [{start_line}]."))
        }
        else {
            let source_slice = &self.source[begin-1..self.current+1];
            let input_string: String = source_slice.iter().collect();
            self.add_token(Token::Literal(input_string));
            self.current += 1;
        }
    }

    fn process_number(&mut self) {
        let begin = self.current - 1; // Since the number itself triggers this function
        while !self.is_at_end() && is_number(self.source[self.current]) {
            self.current += 1;
        };
        if !self.is_at_end() && self.source[self.current] == '.' {
            self.current += 1;
            while !self.is_at_end() && is_number(self.source[self.current]) {
                self.current += 1;
            };
        };
        let source_slice = &self.source[begin..self.current];
        let number_str: String = source_slice.iter().collect();
        self.add_token(Token::Literal(number_str));
    }

    fn process_identifier(&mut self) {
        let begin = self.current - 1; // Since the letter itself triggers this function
        while !self.is_at_end() && is_identifier_char(self.source[self.current]) {
            self.current += 1;
        };
        let source_slice = &self.source[begin..self.current];
        let input_string: String = source_slice.iter().collect();
        match input_string.as_str() {
            "true" => self.add_token(Token::Literal(String::from("true"))),
            "false" => self.add_token(Token::Literal(String::from("false"))),
            "noone" => self.add_token(Token::Literal(String::from("noone"))),
            "pi" => self.add_token(Token::Literal(String::from("pi"))),
            "null" => self.add_token(Token::Literal(String::from("null"))),
            s => self.add_token(Token::Identifier(String::from(s))),
        };
    }

    fn add_token(&mut self, t: Token) {
        self.tokens.push(t);
    }

    fn add_error(&mut self, message: &str) {
        self.error_strings.push(String::from(message));
        self.valid = false;
    }

}

fn is_number(c: char) -> bool {
    match c {
        '0'..='9' => true,
        _ => false,
    }
}

fn is_alpha(c: char) -> bool {
    match c {
        'A'..='Z'|'a'..='z' => true,
        _ => false,
    }
}

fn is_identifier_char(c: char) -> bool {
    is_alpha(c) || is_number(c) || c == '_'
}

fn is_skippable(c: char) -> bool {
        match c {
            ' ' => true,
            '\t' => true,
            '\r' => true,
            _ => false,
        }
    }