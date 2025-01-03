pub mod flatten;

use std::fs;

pub fn do_test_flatten() {
    let test_src_path = "C:/Users/Shear/AppData/Local/RivalsofAether/workshop/commando-G/scripts/user_event2.gml";
    let test_src = fs::read_to_string(test_src_path).expect("Read failed");
    let map = flatten::get_constants_map(&test_src).expect("Mapping failed");
    //println!("{:?}", map);

    let test_tgt_path = "C:/Users/Shear/AppData/Local/RivalsofAether/workshop/commando-G/scripts/init.gml";
    let test_tgt = fs::read_to_string(test_tgt_path).expect("Read failed");
    println!("\n{}", flatten::flatten(&test_tgt, &map, 2, true, true).expect("Flattening failed"));
}