mod token;
mod scanner;
mod reader;
mod flattener;
mod compressor;

use crate::export_project::flattener_scripts as flt;
use flt::scanner::*;
use flt::flattener::*;
use std::collections::HashMap;

pub fn get_constants_map(src: &str) -> Result<HashMap<String, String>, String> {
    let mut s = RcfScanner::new(src);
    let tokens = devectorize_errors(s.scan_tokens())?;
    reader::get_constants_map(&tokens)
} 

pub fn flatten_file(src: &str, map: &HashMap<String, String>, user_event: Option<u8>, skip_whitespace: bool, skip_comments: bool) -> Result<String, String> {
    let mut s = RcfScanner::new(src);
    let tokens = devectorize_errors(s.scan_tokens())?;
    let mut f = Flattener::new(user_event, skip_whitespace, skip_comments);
    f.flatten_program(&tokens, map)
}

fn devectorize_errors<T>(r: Result<T, Vec<String>>) -> Result<T, String> {
    match r {
        Ok(t) => Ok(t),
        Err(v) => {
            let mut out = String::new();
            for s in v.iter() {
                out.push_str(s);
                out.push('\n');
            }
            Err(out)
        }
    }
}