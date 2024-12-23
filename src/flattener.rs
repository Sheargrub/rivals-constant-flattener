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
        let mut stack_pushable = false;
        let mut ignoring = false;
        let mut escape_handled = false;
        for t in ts {
            if ignoring { match t {
                IgnoreEnd => ignoring = false,
                _ => (),
            } } else { match t {
                IgnoreBegin => {
                    if stack != "" {
                        output.push_str(&stack);
                        stack = String::new();
                    }
                    ignoring = true;
                }
                IgnoreEnd => {
                    if stack != "" {
                        output.push_str(&stack);
                        stack = String::new();
                    }
                    return Err(String::from("Reached unpaired end-ignore declaration"));
                }

                DeformatBegin => {
                    if stack != "" {
                        output.push_str(&stack);
                        stack = String::new();
                    }
                    self.deformat_active = true;
                }
                DeformatEnd => {
                    if stack != "" {
                        output.push_str(&stack);
                        stack = String::new();
                    }
                    self.deformat_active = false;
                }

                NewLine => {
                    if stack != "" {
                        output.push_str(&stack);
                        stack = String::new();
                    }
                    if !self.skips_whitespace() {
                        output.push('\n');
                    } else if !escape_handled {
                        output.push('\n');
                        escape_handled = true;
                    }
                }
                Semicolon => {
                    if stack != "" {
                        output.push_str(&stack);
                        stack = String::new();
                    }
                    if !self.skips_whitespace() {
                        output.push(';');
                    } else if !escape_handled {
                        output.push(';');
                        escape_handled = true;
                    }
                }
                Whitespace(s) => {
                    if stack != "" {
                        output.push_str(&stack);
                        stack = String::new();
                    }
                    if !self.skips_whitespace() {
                        output.push_str(&s);
                    }
                },

                LongComment(s) => {
                    if stack != "" {
                        output.push_str(&stack);
                        stack = String::new();
                    }
                    if !self.skips_comments() {
                        output.push_str(&s);
                        escape_handled = false;
                    }
                },
                ShortComment(s) => {
                    if stack != "" {
                        output.push_str(&stack);
                        stack = String::new();
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
                }
                
                _ => {
                    if stack != "" {
                        output.push_str(&stack);
                        stack = String::new();
                    }
                    output.push_str(&t.get_string());
                    escape_handled = false;
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