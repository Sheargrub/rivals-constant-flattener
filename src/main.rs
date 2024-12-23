pub mod token;
pub mod scanner;
pub mod reader;
pub mod flattener;

use scanner::*;
use reader::*;
use flattener::*;
use std::fs;

fn main() {
    let test_src_path = "./test_inputs/user_event2.gml";
    let test_src = fs::read_to_string(test_src_path).expect("Read failed");
    let mut s = RcfScanner::new(&test_src);
    let tokens = s.scan_tokens().expect("Scan failed");
    let map = get_constants_map(&tokens).expect("Mapping failed");
    println!("{:?}", map);


    let test_tgt_path = "./test_inputs/init.gml";
    let test_tgt = fs::read_to_string(test_tgt_path).expect("Read failed");
    let mut s = RcfScanner::new(&test_tgt);
    let tokens = s.scan_tokens().expect("Scan failed");
    let mut f = Flattener::new(2, true, true);
    println!("\n{}", f.flatten_program(&tokens, &map).expect("Flattening failed"));
}
