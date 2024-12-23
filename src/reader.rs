use crate::token::Token;
use crate::token::Token::*;
use std::collections::HashMap;

pub fn get_constants_map(source: &Vec<Token>) -> Result<HashMap<String, String>, String> {

    let mut map = HashMap::new();
    let iter = source.iter();

    // State 0: seeking binding or ignore declaration
    // State 1: seeking assignment operator
    // State 2: taking in value for binding
    let mut mapper_state = 0;
    let mut ignoring = false;
    let mut bracket_depth = 0;
    let mut key = String::new();
    let mut value = String::new();
    let mut needs_space = false;
    let mut can_return = false;

    for t in iter {
        if ignoring { match t {
            IgnoreEnd => ignoring = false,
            _ => (),
        } }
        else { match t {
            NewLine => {
                if mapper_state == 2 && can_return {
                    map.insert(key.clone(), value.clone()); // value is guaranteed to be non-empty
                    mapper_state = 0;
                }
            },

            Semicolon => {
                match mapper_state {
                    0 => return Err(String::from("Reached semicolon with no associated statement")),
                    1 => return Err(format!("Unexpected semicolon in assignment for {}", key)),
                    2 => {
                        if can_return {
                            map.insert(key.clone(), value.clone()); // value is guaranteed to be non-empty
                            mapper_state = 0;
                        } else {
                            return Err(format!("Unexpected semicolon in assignment for {}", key));
                        }
                    },
                    _ => panic!("Invalid state reached"),
                };
            },

            IgnoreBegin => {
                match mapper_state {
                    0 => ignoring = true,
                    1 => return Err(format!("Unexpected begin-ignore declaration in assignment for {}", key)),
                    2 => {
                        if can_return {
                            map.insert(key.clone(), value.clone()); // value is guaranteed to be non-empty
                            mapper_state = 0;
                        } else {
                            return Err(format!("Unexpected begin-ignore declaration in assignment for {}", key));
                        }
                    }
                    _ => panic!("Invalid state reached"),
                };
            },

            IgnoreEnd => return Err(String::from("Reached unpaired end-ignore declaration")),
            DeformatBegin |
            DeformatEnd => return Err(String::from("Deformatting is not supported in the RCF user_event")),

            Whitespace(_) => (),
            LongComment(_) => (),
            ShortComment(_) => (),

            Identifier(s) => {
                match mapper_state {
                    0 => {
                        key = String::from(s);
                        mapper_state = 1;
                    }
                    1 => return Err(format!("Expected '=', got identifier '{}'", s)),
                    2 => {
                        let result = map.get(s);
                        if let Some(r) = result {
                            if needs_space { value.push(' ') };
                            value.push_str(&r);
                            needs_space = true;
                            can_return = bracket_depth == 0;
                        } else {
                            return Err(format!("Tried to assign to non-constant identifier {}", s));
                        }
                    } 
                    _ => panic!("Invalid state reached"),
                };
            }

            Equal => {
                match mapper_state {
                    1 => {
                        mapper_state = 2;
                        value = String::new();
                        needs_space = false;
                        can_return = false;
                        bracket_depth = 0;
                    }
                    _ => return Err(String::from("Reached unexpected '='")),
                };
            }

            Literal(s) => {
                match mapper_state {
                    0 => return Err(format!("Expected start statement, got literal '{}'", s)),
                    1 => return Err(format!("Expected '=', got literal {}", s)),
                    2 => {
                        if needs_space { value.push(' ') };
                        value.push_str(&s);
                        needs_space = true;
                        can_return = bracket_depth == 0;
                    },
                    _ => panic!("Invalid state reached"),
                };
            }

            Symbol(s) => {
                match mapper_state {
                    0 => return Err(format!("Expected start statement, got symbol '{}'", s)),
                    1 => return Err(format!("Expected '=', got symbol '{}'", s)),
                    2 => {
                        value.push_str(&s);
                        needs_space = false;
                        can_return = false; // this may not be an accurate heuristic - need to check
                    },
                    _ => panic!("Invalid state reached"),
                };
            }

            Dot => {
                match mapper_state {
                    0 => return Err(String::from("Expected start statement, got '.'")),
                    1 => return Err(String::from("Expected '=', got '.'")),
                    2 => {
                        value.push_str(".");
                        needs_space = false;
                        can_return = false; // this may not be an accurate heuristic - need to check
                    },
                    _ => panic!("Invalid state reached"),
                };
            }

            OpenBracket(s) => {
                match mapper_state {
                    0 => return Err(format!("Expected start statement, got open bracket '{}'", s)),
                    1 => return Err(format!("Expected '=', got open bracket '{}'", s)),
                    2 => {
                        value.push_str(&s);
                        needs_space = false;
                        can_return = false;
                        bracket_depth += 1;
                    },
                    _ => panic!("Invalid state reached"),
                };
            }
            
            // RCF is trusting the user to be inputting valid code, so brackets
            // are only being matched to make sure that statements are being broken
            // up correctly. As such, no need to check what kinds of brackets they are.
            CloseBracket(s) => {
                match mapper_state {
                    0 => return Err(format!("Expected start statement, got close bracket '{}'", s)),
                    1 => return Err(format!("Expected '=', got close bracket '{}'", s)),
                    2 => {
                        if bracket_depth <= 0 { return Err(format!("Reached unpaired close bracket {}", s)) }
                        bracket_depth -= 1;
                        value.push_str(&s);
                        needs_space = false;
                        can_return = bracket_depth == 0;
                    },
                    _ => panic!("Invalid state reached"),
                };
            }
            
        } };
    }

    // Loop done; wrap up loose ends
    match mapper_state {
        0 => (),
        1 => return Err(format!("Unexpected end-of-file in assignment for {}", key)),
        2 => {
            if can_return {
                map.insert(key.clone(), value.clone()); // value is guaranteed to be non-empty
            } else {
                return Err(format!("Unexpected end-of-file in assignment for {}", key));
            }
        },
        _ => panic!("Invalid state reached"),
    };

    Ok(map)

}