mod token;
mod scanner;
mod reader;
mod flattener;

use crate::export_project::flattener_scripts as flt;
use flt::scanner::*;
use flt::flattener::*;
use std::collections::HashMap;

pub fn get_constants_map(src: &str) -> Result<HashMap<String, String>, String> {
    let mut s = RcfScanner::new(src);
    let tokens = s.scan_tokens().expect("Scan failed"); // TODO: proper error handling
    let map = reader::get_constants_map(&tokens).expect("Mapping failed"); // TODO: proper error handling
    Ok(map)
} 

pub fn flatten_file(src: &str, map: &HashMap<String, String>, user_event: u8, skip_whitespace: bool, skip_comments: bool) -> Result<String, String> {
    let mut s = RcfScanner::new(src);
    let tokens = s.scan_tokens().expect("Scan failed"); // TODO: proper error handling
    let mut f = Flattener::new(user_event, skip_whitespace, skip_comments);
    Ok(f.flatten_program(&tokens, map).expect("Flattening failed")) // TODO: proper error handling
}