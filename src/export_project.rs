pub mod include_list;
pub mod fetch_project;
pub mod flatten_file;
pub mod write_project;

use crate::export_project as rcf;
use rcf::fetch_project::*;

pub fn export_project(src: &str, dest: &str, user_event: u8) -> Result<(), String> {

    let (files, ue_file) = fetch_project(src, user_event)?;



    Ok(())

}


// \/ ---- DELETE LATER ---- \/ \\

use std::fs;

pub fn do_test_flatten() {
    let test_src_path = r"C:\Users\Shear\Documents\Workshop Files\commando-rcf-src/scripts/user_event2.gml";
    let test_src = fs::read_to_string(test_src_path).expect("Read failed");
    let map = flatten_file::get_constants_map(&test_src).expect("Mapping failed");
    //println!("{:?}", map);

    let test_tgt_path = r"C:\Users\Shear\Documents\Workshop Files\commando-rcf-src/scripts/init.gml";
    let test_tgt = fs::read_to_string(test_tgt_path).expect("Read failed");
    println!("\n{}", flatten_file::flatten_file(&test_tgt, &map, 2, true, true).expect("Flattening failed"));
}

pub fn do_test_include() {
    let src_path = "./test_inputs/rcf_include.txt";
    let src = fs::read_to_string(src_path).expect("Read failed");
    println!("{:?}", include_list::IncludeList::construct(&src))
}

pub fn do_test_fetch() {
    let src_path = r"C:\Users\Shear\Documents\Workshop Files\commando-rcf-src\";
    let result = fetch_project::fetch_project(src_path, 2);
    println!("{:?}", result);
}