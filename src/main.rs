pub mod token;
pub mod scanner;
pub mod reader;

use scanner::*;
use reader::*;
use std::fs;

fn main() {
    let test_path = "./test_inputs/user_event2.gml";
    let test_program = fs::read_to_string(test_path).expect("Read failed");
    let mut s = RcfScanner::new(&test_program);
    let tokens = s.scan_tokens().expect("Scan failed");
    println!("{:?}", get_constants_map(&tokens));
}
