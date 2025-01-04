mod include_list;
mod fetch_project;
mod flattener_scripts;
mod write_project;

use std::vec::Vec;
use std::fs;
use std::path::Path;

use crate::export_project as rcf;
use rcf::fetch_project::*;
use rcf::flattener_scripts::*;

pub fn export_project(src: &str, dest: &str, user_event: u8, skip_whitespace: bool, skip_comments: bool) -> Result<(), String> {

    let _ = fs::create_dir_all(dest); // not especially worried about errors on this one

    let (files, ue_file) = fetch_project(src, user_event)?;
    let src = apply_trailing_slash(src);
    let dest = apply_trailing_slash(dest);

    let mut ue_path = src.clone();
    ue_path.push_str(&ue_file);
    let ue_script = fs::read_to_string(&ue_path).expect(&format!("Failed to read file {}", ue_path));
    let constants_map = get_constants_map(&ue_script)?;

    for f in files.iter() {
        let mut src_path = src.clone();
        src_path.push_str(f);
        let src_path = Path::new(&src_path);

        let mut dest_path = dest.clone();
        dest_path.push_str(f);
        let dest_path = Path::new(&dest_path);

        let extension = src_path.extension();
        let mut ancestors = dest_path.ancestors();
        let _ = ancestors.next(); // Skip once, as the first iteration step is just the original path
        if let Some(p) = ancestors.next() {
            let _ = fs::create_dir_all(p); // not especially worried about errors on this one
        }

        let err1 = format!("Failed to read file {:?}", src_path);
        let err2 = format!("Failed to write to file {:?}", dest_path);
        let err3 = format!("Failed to copy from file {:?} to {:?}", src_path, dest_path);

        if let Some(e) = extension {
            if let Some("gml") = e.to_str() {
                let src_script = fs::read_to_string(&src_path).expect(&err1);
                let dest_script = flatten_file(&src_script, &constants_map, user_event, skip_whitespace, skip_comments)?;
                fs::write(dest_path, dest_script).expect(&err2);
            } else {
                fs::copy(src_path, dest_path).expect(&err3);
            }
        } else {
            fs::copy(src_path, dest_path).expect(&err3);
        }
    }

    Ok(())

}

fn apply_trailing_slash(s: &str) -> String {
    let mut s = Vec::from_iter(s.chars());
    let last = s[s.len()-1];
    if last != '/' && last != '\\' {
        s.push('/');
    }
    s.iter().collect::<String>()
}


// \/ ---- DELETE LATER ---- \/ \\

pub fn do_test_flatten() {
    let test_src_path = r"C:\Users\Shear\Documents\Workshop Files\commando-rcf-src/scripts/user_event2.gml";
    let test_src = fs::read_to_string(test_src_path).expect("Read failed");
    let map = flattener_scripts::get_constants_map(&test_src).expect("Mapping failed");
    //println!("{:?}", map);

    let test_tgt_path = r"C:\Users\Shear\Documents\Programming\Rust\Rivals Constant Flattener\rivals-constant-flattener\test_inputs\user_event4.gml";
    let test_tgt = fs::read_to_string(test_tgt_path).expect("Read failed");
    println!("\n{}", flattener_scripts::flatten_file(&test_tgt, &map, 2, true, true).expect("Flattening failed"));
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

pub fn do_test_export() {
    let src_path = r"C:\Users\Shear\Documents\Workshop Files\commando-rcf-src\";
    let dest_path = r"C:\Users\Shear\AppData\Local\RivalsofAether\workshop\commando-rcf-dest";
    println!("{:?}", export_project(src_path, dest_path, 2, false, false));
}