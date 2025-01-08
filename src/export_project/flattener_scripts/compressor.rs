use crate::export_project::flattener_scripts as flt;
use flt::scanner::is_skippable;
use flt::scanner::is_identifier_char;

enum CompressorState {
    Default,
    InString(char),
    InSingleComment,
    InMultiComment,
}

use flt::compressor::CompressorState::*;

// Removes whitespace characters based on the characters on either side.
// Since this is operates on slightly naive windows, it's not suitable for handling semicolons.
pub fn compress_whitespace(s: &str) -> String {
    if s.len() == 0 { return String::new() };

    let mut iter = s.chars().peekable();
    let mut out = String::new();
    let mut prev = ' ';
    let mut state = Default;

    // Ensure the first character taken in is not whitespace
    let mut done = false;
    while !done {
        if let Some(cur) = iter.next() {
            if !is_skippable(cur) && cur != '\n' {
                prev = cur;
                out.push(prev);
                done = true;
            }
        }
        else { done = true; }
    }

    let mut multiline_exited = false;
    while let Some(cur) = iter.next() {
        match state {
            Default => {
                match cur {
                    '\r' => (),
                    ' ' | '\t' => {
                        if let Some(next) = iter.peek() {
                            if is_identifier_char(prev) && is_identifier_char(*next) {
                                prev = cur;
                                out.push(prev);
                            }
                        }
                    },
                    '\n' => {
                        if prev != ';' && prev != '\n' && prev != '{' && prev != '}' && prev != ',' {
                            if let Some(next) = iter.peek() {
                                //TODO: can valid GML statements start with anything other than an identifier?
                                //If so, this next heuristic breaks
                                if is_identifier_char(*next) {
                                    prev = cur;
                                    out.push(prev);
                                }
                            }
                        }
                    }
                    '"' | '\'' | '`' => {
                        state = InString(cur);
                        prev = cur;
                        out.push(prev);
                    }
                    '/' => {
                        if prev == '/' && !multiline_exited {
                            state = InSingleComment;
                        }
                        prev = cur;
                        out.push(prev);
                    }
                    '*' => {
                        if prev == '/' { state = InMultiComment };
                        prev = cur;
                        out.push(prev);
                    }
                    _ => {
                        prev = cur;
                        out.push(prev);
                    }
                }
                multiline_exited = false;
            },
            InString(quote) => {
                if cur == quote && prev != '\\' { state = Default };
                prev = cur;
                out.push(prev);
            },
            InSingleComment => {
                if cur == '\n' { state = Default };
                prev = cur;
                out.push(prev);
            },
            InMultiComment => {
                if cur == '/' && prev == '*' {
                    state = Default;
                    multiline_exited = true;
                }
                prev = cur;
                out.push(prev);
            },
        }
    }

    out
}