use std::env;
use rivals_constant_flattener::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    run_cli(args);
}
