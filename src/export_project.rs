mod include_list;
mod fetch_project;
mod flattener_scripts;

use std::vec::Vec;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::export_project as rcf;
use rcf::fetch_project::*;
use rcf::flattener_scripts::*;

pub fn get_export_type(src: &str) -> Option<u8> {
    get_project_type(src)
}

pub fn export_project(src: &str, dest: &str, user_event: Option<u8>, skip_whitespace: bool, skip_comments: bool, inert_run: bool) -> Result<(), String> {

    let _ = fs::create_dir_all(dest); // not especially worried about errors on this one

    let (files, ue_file) = fetch_project(src, user_event, inert_run)?;
    let src = apply_trailing_slash(src);
    let dest = apply_trailing_slash(dest);
    
    let constants_map = {
        if let Some(ue_file) = ue_file {
            let mut ue_path = src.clone();
            ue_path.push_str(&ue_file);
            let ue_script = fs::read_to_string(&ue_path).expect(&format!("Failed to read file {}", ue_path));
            get_constants_map(&ue_script)?
        } else {
            HashMap::new()
        }
    };

    for f in files.iter() {
        let mut src_path = src.clone();
        src_path.push_str(f);
        let src_path = Path::new(&src_path);

        let mut dest_path = dest.clone();
        dest_path.push_str(f);
        let dest_path = Path::new(&dest_path);

        if src_path == dest_path {
            return Err(String::from("Cannot use source directory as destination"));
        }

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
                if dest_script != "" {
                    fs::write(&dest_path, &dest_script).expect(&err2);
                }
            } else {
                fs::copy(&src_path, &dest_path).expect(&err3);
            }
        } else {
            fs::copy(&src_path, &dest_path).expect(&err3);
        }
    }

    Ok(())

}

// Boolean output denotes whether a new config_export.ini file was initialized
pub fn export_config(src: &str, dest: &str, inert_run: bool) -> Result<bool, String> {
    let src_path = apply_trailing_slash(src);
    let mut src_conf_path = src_path.clone();
    src_conf_path.push_str("config_export.ini");
    let mut src_origconf_path = src_path.clone();
    src_origconf_path.push_str("config.ini");
    let mut dest_conf_path = apply_trailing_slash(dest);
    dest_conf_path.push_str("config.ini");

    if let Ok(true) = fs::exists(&src_conf_path) {
        match fs::copy(&src_conf_path, &dest_conf_path) {
            Ok(_) => Ok(false),
            Err(e) => Err(e.to_string()),
        }
    } else if !inert_run {
        match fs::copy(&src_origconf_path, &src_conf_path) {
            Ok(_) => Ok(true),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Ok(false)
    }
}

pub fn export_project_dry(src: &str) ->  Result<(), String> {
    fetch_project(src, None, false)?;
    init_config_dry(src)?;
    Ok(())
}

pub fn init_config_dry(src: &str) -> Result<(), String> {
    let src_path = apply_trailing_slash(src);
    let mut src_conf_path = src_path.clone();
    src_conf_path.push_str("config_export.ini");
    let mut src_origconf_path = src_path.clone();
    src_origconf_path.push_str("config.ini");

    if let Ok(true) = fs::exists(&src_conf_path) {
        Ok(())
    } else {
        match fs::copy(&src_origconf_path, &src_conf_path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}

fn apply_trailing_slash(s: &str) -> String {
    let mut s = Vec::from_iter(s.chars());
    let last = s[s.len()-1];
    if last != '/' && last != '\\' {
        s.push('/');
    }
    s.iter().collect::<String>()
}