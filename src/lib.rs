pub mod flatten;
pub mod fetch_project;

use std::fs;

pub fn do_test_flatten() {
    let test_src_path = r"C:\Users\Shear\Documents\Workshop Files\commando-rcf-src/scripts/user_event2.gml";
    let test_src = fs::read_to_string(test_src_path).expect("Read failed");
    let map = flatten::get_constants_map(&test_src).expect("Mapping failed");
    //println!("{:?}", map);

    let test_tgt_path = r"C:\Users\Shear\Documents\Workshop Files\commando-rcf-src/scripts/init.gml";
    let test_tgt = fs::read_to_string(test_tgt_path).expect("Read failed");
    println!("\n{}", flatten::flatten(&test_tgt, &map, 2, true, true).expect("Flattening failed"));
}

pub fn do_test_fetch() {
    let src = r"C:\Users\Shear\Documents\Workshop Files\commando-rcf-src";
    let project_type = fetch_project::get_project_type(src).expect("Fetch failed");
    let include = fetch_project::get_include(src, project_type).expect("Include failed");

    println!("{:?}\n", project_type);
    println!("{}\n", include);
}