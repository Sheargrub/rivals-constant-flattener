use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug)]
enum AllowType {
    Folder(Box<IncludeList>),
    Type(String),
    File(String),
}

impl AllowType {
    // Panics if called on a folder!
    pub fn unsafe_to_str(&self) -> &str {
        match self {
            Folder(_) => panic!("Cannot call on folders."),
            Type(s) => &s,
            File(s) => &s,
        }
    }
}

use crate::include_list::AllowType::*;

#[derive(Debug)]
pub struct IncludeList {
    contents: HashMap<String, AllowType>,
}

impl IncludeList {

    pub fn new() -> IncludeList {
        IncludeList{ contents: HashMap::new() }
    }

    pub fn construct(raw_contents: &str) -> Result<IncludeList, String> {
        let mut incl = IncludeList{ contents: HashMap::new() };
        incl.add_entries(raw_contents)?;
        Ok(incl)
    }

    pub fn add_entries(&mut self, raw_contents: &str) -> Result<(), String> {
        let mut path = Vec::new();
        let mut entry = String::new();
        let mut is_wildcard = false;
        let mut wildcard_valid = false;

        for c in raw_contents.chars() {
            match c {
                '/' | '\\' => {
                    if is_wildcard {
                        return Err(String::from("Encountered '*' in folder name"));
                    }
                    path.push(entry);
                    entry = String::new();
                }
                '\n' => {
                    if entry != "" {
                        if is_wildcard {
                            if wildcard_valid {
                                self.add_entry_inner(&path, Type(entry));
                            } else {
                                return Err(String::from("Encountered improper use of '*'"));
                            }
                        } else {
                            self.add_entry_inner(&path, File(entry));
                        }
                        path = Vec::new();
                        entry = String::new();
                        is_wildcard = false;
                        wildcard_valid = false;
                    }
                }
                '\r' => (), // omit carriage returns
                '*' => {
                    if entry != "" || is_wildcard { 
                        return Err(String::from("Encountered improper use of '*'"));
                    }
                    is_wildcard = true;
                }
                '.' => {
                    if entry != "" && is_wildcard { 
                        return Err(String::from("Encountered improper use of '*'"));
                    }
                    wildcard_valid = is_wildcard;
                    entry.push(c);
                }
                _ => {
                    entry.push(c);
                }
            }
        }
        
        Ok(())
    }

    fn add_entry_inner(&mut self, path: &[String], entry: AllowType) {
        let len = path.len();
        match len {
            0 => {
                // No Folder-type objects should ever be saved to entry.
                // As such, safety should be guaranteed by internal conditions.
                self.contents.insert(
                    String::from(entry.unsafe_to_str()),
                    entry,
                );
            },
            _ => {
                let cur = &path[0];
                if let Some(Folder(f)) = self.contents.get_mut(cur) {
                    f.add_entry_inner(&path[1..path.len()], entry);
                } else {
                    let mut f = IncludeList::new();
                    f.add_entry_inner(&path[1..path.len()], entry);
                    self.contents.insert(
                        String::from(cur),
                        Folder(Box::new(f)),
                    );
                }
            },
        };
    }

}