use crate::token::Token;
use crate::token::Token::*;
use std::collections::HashMap;

pub struct Flattener {
    user_event : u8,
    skip_whitespace : bool,
    skip_comments : bool,
    deformat_active : bool,
    needs_space : bool,
    escape_handled : bool,
    ignoring: bool,
}

impl Flattener {

    pub fn new(user_event: u8, skip_whitespace: bool, skip_comments: bool) -> Flattener {
        let deformat_active = false;
        let needs_space = false;
        let ignoring = false;
        let escape_handled = false;
        Flattener{ user_event, skip_whitespace, skip_comments, deformat_active, needs_space, ignoring, escape_handled }
    }

    pub fn flatten_program(&mut self, ts: &Vec<Token>, map: &HashMap<String, String>) -> Result<String, String> {
        let mut output = String::new();
        let mut stack: Vec<&Token> = Vec::new();
        for t in ts {
            if self.ignoring { match t {
                IgnoreEnd => self.ignoring = false,
                _ => (),
            } } else { match t {
                IgnoreBegin => self.ignoring = true,
                IgnoreEnd => return Err(String::from("Reached unpaired end-ignore declaration")),

                DeformatBegin => self.deformat_active = true,
                DeformatEnd => self.deformat_active = false,

                NewLine => {
                    if !self.skips_whitespace() {
                        output.push('\n');
                    } else if !self.escape_handled {
                        output.push('\n');
                        self.escape_handled = true;
                    }
                }
                Semicolon => {
                    if !self.skips_whitespace() {
                        output.push(';');
                    } else if !self.escape_handled {
                        output.push(';');
                        self.escape_handled = true;
                    }
                }
                Whitespace(s) => {
                    if !self.skips_whitespace() {
                        output.push_str(&s);
                    }
                },

                LongComment(s) => {
                    if !self.skips_comments() {
                        output.push_str(&s);
                        self.escape_handled = false;
                    }
                },
                ShortComment(s) => {
                    if !self.skips_comments() {
                        output.push_str(&s);
                        self.escape_handled = false;
                    }
                },
                
                _ => {
                    output.push_str(&t.get_string());
                    self.escape_handled = false;
                },
            } };
        }
        Ok(output)
    }

    fn skips_whitespace(&self) -> bool {
        self.skip_whitespace || self.deformat_active
    }

    fn skips_comments(&self) -> bool {
        self.skip_comments || self.deformat_active
    }
}