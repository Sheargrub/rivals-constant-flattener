use crate::token::Token;
use crate::token::Token::*;
use std::collections::HashMap;

pub struct Flattener {
    user_event : u8,
    skip_whitespace : bool,
    skip_comments : bool,
    deformat_active : bool,
    needs_space : bool,
}

impl Flattener {

    pub fn new(user_event: u8, skip_whitespace: bool, skip_comments: bool) -> Flattener {
        let deformat_active = false;
        let needs_space = false;
        Flattener{ user_event, skip_whitespace, skip_comments, deformat_active, needs_space }
    }

    pub fn flatten_program(&mut self, ts: &Vec<Token>, map: &HashMap<String, String>) -> Result<String, String> {
        let mut output = String::new();
        let mut stack = String::new();
        let mut ignoring = false;
        let mut escape_handled = false;
        let mut needs_space = true;
        for t in ts {
            if ignoring { match t {
                IgnoreEnd => ignoring = false,
                _ => (),
            } } else { match t {
                IgnoreBegin => {
                    if stack != "" {
                        if needs_space && self.skips_whitespace() { output.push(' ') };
                        output.push_str(&stack);
                        stack = String::new();
                        needs_space = true;
                    }
                    ignoring = true;
                }
                IgnoreEnd => {
                    if stack != "" {
                        if needs_space && self.skips_whitespace() { output.push(' ') };
                        output.push_str(&stack);
                        stack = String::new();
                        needs_space = true;
                    }
                    return Err(String::from("Reached unpaired end-ignore declaration"));
                }

                DeformatBegin => {
                    if stack != "" {
                        if needs_space && self.skips_whitespace() { output.push(' ') };
                        output.push_str(&stack);
                        stack = String::new();
                        needs_space = true;
                    }
                    self.deformat_active = true;
                }
                DeformatEnd => {
                    if stack != "" {
                        if needs_space && self.skips_whitespace() { output.push(' ') };
                        output.push_str(&stack);
                        stack = String::new();
                        needs_space = true;
                    }
                    self.deformat_active = false;
                }

                NewLine => {
                    if stack != "" {
                        if needs_space && self.skips_whitespace() { output.push(' ') };
                        output.push_str(&stack);
                        stack = String::new();
                        needs_space = true;
                    }
                    if !self.skips_whitespace() {
                        output.push('\n');
                        needs_space = false;
                    } else if !escape_handled {
                        output.push('\n');
                        escape_handled = true;
                        needs_space = false;
                    }
                }
                Semicolon => {
                    if stack != "" {
                        if needs_space && self.skips_whitespace() { output.push(' ') };
                        output.push_str(&stack);
                        stack = String::new();
                    }
                    needs_space = false;
                    if !self.skips_whitespace() {
                        output.push(';');
                    } else if !escape_handled {
                        output.push(';');
                        escape_handled = true;
                    }
                }
                Whitespace(s) => {
                    if stack != "" {
                        if needs_space && self.skips_whitespace() { output.push(' ') };
                        output.push_str(&stack);
                        stack = String::new();
                        needs_space = true;
                    }
                    if !self.skips_whitespace() {
                        output.push_str(&s);
                    }
                },

                LongComment(s) => {
                    if stack != "" {
                        if needs_space && self.skips_whitespace() { output.push(' ') };
                        output.push_str(&stack);
                        stack = String::new();
                        needs_space = self.skips_comments();
                    }
                    if !self.skips_comments() {
                        output.push_str(&s);
                        escape_handled = false;
                    }
                },
                ShortComment(s) => {
                    if stack != "" {
                        if needs_space && self.skips_whitespace() { output.push(' ') };
                        output.push_str(&stack);
                        stack = String::new();
                        needs_space = self.skips_comments();
                    }
                    if !self.skips_comments() {
                        output.push_str(&s);
                        escape_handled = false;
                    }
                },

                Identifier(s) => {
                    let query = map.get(s);
                    if let Some(val) = query {
                        // Match should correspond to a constant,
                        // so toss out the reference stack.
                        if needs_space && self.skips_whitespace() { output.push(' ') };
                        output.push_str(&stack);
                        output.push_str(&val);
                        stack = String::new();
                    }
                    else {
                        stack.push_str(&s);
                    }
                    escape_handled = false;
                },

                Dot => {
                    if stack != "" {
                        stack.push('.');
                    } else {
                        output.push('.');
                    }
                    escape_handled = false;
                    needs_space = true;
                }
                
                Literal(s) => {
                    if stack != "" {
                        if needs_space && self.skips_whitespace() { output.push(' ') };
                        output.push_str(&stack);
                        stack = String::new();
                        needs_space = true;
                    }
                    if needs_space && self.skips_whitespace() { output.push(' ') };
                    output.push_str(&s);
                    escape_handled = false;
                    needs_space = true;
                },

                Symbol(s) | OpenBracket(s) | CloseBracket(s) => {
                    if stack != "" {
                        if needs_space && self.skips_whitespace() { output.push(' ') };
                        output.push_str(&stack);
                        stack = String::new();
                    }
                    output.push_str(&s);
                    escape_handled = false;
                    needs_space = false;
                },

                Equal => {
                    if stack != "" {
                        if needs_space && self.skips_whitespace() { output.push(' ') };
                        output.push_str(&stack);
                        stack = String::new();
                    }
                    output.push('=');
                    escape_handled = false;
                    needs_space = false;
                },

            } };
        }
        output.push_str(&stack);
        Ok(output)
    }

    fn skips_whitespace(&self) -> bool {
        self.skip_whitespace || self.deformat_active
    }

    fn skips_comments(&self) -> bool {
        self.skip_comments || self.deformat_active
    }
}