use crate::export_project::flattener_scripts as flt;
use flt::token::Token;
use flt::token::Token::*;
use std::collections::HashMap;

pub struct Flattener {
    user_event : String,
    skip_whitespace : bool,
    skip_comments : bool,
    deformat_active : bool,
    needs_space : bool,
    stack : String,
    output : String,
}

impl Flattener {

    pub fn new(user_event: u8, skip_whitespace: bool, skip_comments: bool) -> Flattener {
        let user_event = user_event.to_string();
        let deformat_active = false;
        let needs_space = false;
        let stack = String::new();
        let output = String::new();
        Flattener{ user_event, skip_whitespace, skip_comments, deformat_active, needs_space, stack, output }
    }

    pub fn flatten_program(&mut self, ts: &Vec<Token>, map: &HashMap<String, String>) -> Result<String, String> {
        self.output = String::new();

        // For ignore tags
        let mut ignoring = false;

        // For safe whitespace stripping
        let mut escape_handled = false;

        // For stripping RCF user event
        let mut closing_user_event = false;
        let mut eating_semicolon = false;

        for t in ts {
            if ignoring { match t {
                IgnoreEnd => ignoring = false,
                _ => (),
            } }
            else { match t {
                IgnoreBegin => {
                    self.flush_stack(true);
                    ignoring = true;
                    closing_user_event = false;
                    eating_semicolon = false;
                }
                IgnoreEnd => {
                    return Err(String::from("Reached unpaired end-ignore declaration"));
                }

                DeformatBegin => {
                    self.flush_stack(true);
                    self.deformat_active = true;
                    closing_user_event = false;
                    eating_semicolon = false;
                }
                DeformatEnd => {
                    self.flush_stack(true);
                    self.deformat_active = false;
                    closing_user_event = false;
                    eating_semicolon = false;
                }

                NewLine => {
                    self.flush_stack(true);
                    if !self.skips_whitespace() {
                        self.output.push('\n');
                        self.needs_space = false;
                    } else if !escape_handled {
                        self.output.push('\n');
                        escape_handled = true;
                        self.needs_space = false;
                    }
                    closing_user_event = false;
                    eating_semicolon = false;
                }
                Semicolon => {
                    self.flush_stack(false);
                    self.needs_space = false;
                    if !eating_semicolon && (!self.skips_whitespace() || !escape_handled) {
                        self.output.push(';');
                    }
                    escape_handled = true;
                    closing_user_event = false;
                    eating_semicolon = false;
                }
                Whitespace(s) => {
                    self.flush_stack(true);
                    if !self.skips_whitespace() {
                        self.output.push_str(&s);
                    } else if self.needs_space {
                        self.output.push(' ');
                        self.needs_space = false;
                    }
                    closing_user_event = false;
                },

                LongComment(s) => {
                    self.flush_stack(false);
                    self.needs_space = self.skips_comments();
                    if !self.skips_comments() {
                        self.output.push_str(&s);
                        escape_handled = false;
                    }
                    closing_user_event = false;
                },
                ShortComment(s) => {
                    self.flush_stack(false);
                    self.needs_space = self.skips_comments();
                    if !self.skips_comments() {
                        self.output.push_str(&s);
                        escape_handled = false;
                    }
                    closing_user_event = false;
                },

                Identifier(s) => {
                    if let Some(val) = map.get(s) {
                        // Match should correspond to a constant,
                        // so insert that and toss out the contents of self.stack.
                        if self.needs_space && self.skips_whitespace() { self.output.push(' ') };
                        self.output.push_str(&val);
                        self.stack = String::new();
                        self.needs_space = true;
                    }
                    else { // This implicitly catches user_event calls, too
                        self.stack.push_str(&s);
                    }
                    escape_handled = false;
                    closing_user_event = false;
                    eating_semicolon = false;
                },

                Dot => {
                    if self.stack != "" {
                        self.stack.push('.');
                    } else {
                        self.output.push('.');
                        self.needs_space = false;
                    }
                    escape_handled = false;
                    closing_user_event = false;
                    eating_semicolon = false;
                }
                
                Literal(s) => {
                    if self.stack == "user_event(" && *s == self.user_event {
                        self.stack.push_str(&s);
                        closing_user_event = true;
                    } else {
                        self.flush_stack(true);
                        if self.needs_space && self.skips_whitespace() { self.output.push(' ') };
                        self.output.push_str(&s);
                        closing_user_event = false;
                    }
                    
                    self.needs_space = true;
                    escape_handled = false;
                    eating_semicolon = false;
                },

                Symbol(s) => {
                    self.flush_stack(false);
                    self.output.push_str(&s);

                    escape_handled = s == ","; 
                    self.needs_space = false;
                    closing_user_event = false;
                    eating_semicolon = false;
                },

                OpenBracket(s) => {
                    if self.stack == "user_event" && s == "(" {
                        self.stack.push_str(&s);
                    } else {
                        self.flush_stack(false);
                        self.output.push_str(&s);
                    }

                    escape_handled = true;
                    self.needs_space = false;
                    closing_user_event = false;
                    eating_semicolon = false;
                },

                CloseBracket(s) => {
                    if closing_user_event && s == ")" {
                        // matching user_event found -> discard
                        self.stack = String::new();
                        eating_semicolon = true;
                    } else {
                        self.flush_stack(false);
                        self.output.push_str(&s);
                        eating_semicolon = false;
                    }

                    escape_handled = s == "}"; // need to test if ) and ] behave nicely
                    self.needs_space = false;
                    closing_user_event = false;
                },

                Equal => {
                    self.flush_stack(false);
                    self.output.push('=');
                    escape_handled = false;
                    self.needs_space = false;
                    closing_user_event = false;
                    eating_semicolon = false;
                },

            } };
        }
        self.output.push_str(&self.stack);
        Ok(self.output.clone())
    }

    fn skips_whitespace(&self) -> bool {
        self.skip_whitespace || self.deformat_active
    }

    fn skips_comments(&self) -> bool {
        self.skip_comments || self.deformat_active
    }

    fn flush_stack(&mut self, set_needs_space: bool) {
        if self.stack != "" {
            if self.needs_space && self.skips_whitespace() { self.output.push(' ') };
            self.output.push_str(&self.stack);
            self.stack = String::new();
            self.needs_space = true;
        }
    }
}