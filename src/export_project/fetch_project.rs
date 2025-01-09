use std::fs;
use std::fs::File;
use std::vec::Vec;

use crate::export_project as rcf;
use rcf::include_list::IncludeList;

pub fn get_project_type(root: &str) -> Option<u8> {
    let mut config_path = String::from(root);
    let len = config_path.len();
    let end = &root[len-1..len];
    if end != "/" && end != "\\" { config_path.push('/'); }
    config_path.push_str("config.ini");
    
    // Searching for substring of form 'type="i"'. Whitespace is acceptable.
    let mut index = 0;
    let versus = Vec::from_iter(str::chars("type=\"\""));
    let mut contents = String::new();
    if let Ok(config) = fs::read_to_string(config_path) {
        for c in config.chars() {
            match index {
                0 | 1 | 2 | 3 => { // "type"
                    if c == versus[index] {
                        index = index + 1;
                    } else {
                        index = 0;
                    }
                },
                4 | 5 => { // = or open "
                    if c == versus[index] {
                        index = index + 1;
                    } else if c != ' ' {
                        index = 0;
                    }
                },
                6 => { // Contents of quote, searching for close "
                    if c == versus[index] {
                        if let Ok(num) = contents.parse::<u8>() {
                            return Some(num);
                        } else {
                            return None;
                        }
                    } else if c != ' ' {
                        contents.push(c);
                    }
                }
                _ => panic!("State machine reached unknown condition"),
            }
        }
        None
    } else {
        None
    }
}

pub fn get_include(root: &str, project_type: u8) -> Result<IncludeList, String> {
    if 3 < project_type { return Err(format!("Invalid project type for project in directory {}", root)) };

    let mut include_path = String::from(root);
    let len = include_path.len();
    let end = &root[len-1..len];
    if end != "/" && end != "\\" { include_path.push('/'); }
    include_path.push_str("rcf_include.txt");

    // If include file does not exist, create it
    if let Err(_) = File::open(&include_path) {
        let f = fs::write(&include_path, make_raw_include(project_type));
        if let Err(_) = f {
            return Err(format!("Could not write include file to project in directory {}", root));
        }
    }

    // Now, go ahead and read it in
    if let Ok(s) = fs::read_to_string(&include_path) {
        Ok(IncludeList::construct(&s)?)
    } else {
        Err(format!("Could not read include file from project in directory {}", root))
    }

}

pub fn fetch_project(root: &str, user_event: u8) -> Result<(Vec<String>, String), String> {
    // Ensure that valid project is being fetched
    let project_type = get_project_type(root);
    if let None = project_type {
        return Err(format!("Could not find a valid project at directory {}", root));
    }
    let project_type = project_type.unwrap();

    // Get include list
    let incl = Box::new(get_include(root, project_type)?);

    // Get all files
    let ue_name = format!("user_event{}.gml", user_event);
    match visit_folder(root, root, &ue_name, &incl) {
        Ok((file_paths, Some(ue_path))) => Ok((file_paths, ue_path)),
        Ok((_, None)) => {
            Err(format!("Could not locate {}", ue_name))
        },
        Err(_) => Err(format!("Unknown error with project at directory {}", root)),
    }

}

fn visit_folder(root: &str, cur: &str, user_event: &str, incl: &Box<IncludeList>) -> Result<(Vec<String>, Option<String>), String> {
    let src_dir = fs::read_dir(cur).expect(&format!("Could not open source directory {}", cur));
    let mut file_paths = Vec::new();
    let mut ue_path = None;

    for entry in src_dir {
        let dir = entry.expect("Reached invalid directory entry");
        let path = dir.path();
        let path_str = path.to_str().expect("Unexpected error when getting file path");
        let name = dir.file_name();
        let name = name.to_str().expect("Unexpected error when getting file name");

        if path.is_dir() {
            if let Some(sub_incl) = incl.get_folder(name) {
                match visit_folder(root, path_str, user_event, sub_incl) {
                    Ok((mut sub_paths, Some(sub_ue))) => {
                        file_paths.append(&mut sub_paths);
                        ue_path = Some(sub_ue);
                    },
                    Ok((mut sub_paths, None)) => {
                        file_paths.append(&mut sub_paths);
                    },
                    Err(_) => {
                        return Err(format!("Unknown error with project at directory {}", path_str));
                    },
                }
            }
        }
        else if path.is_file() {
            let mut extension = String::from(name);
            if let Some(e) = path.extension() {
                let e_str = e.to_str().expect("Unexpected error when getting file extension");
                extension = String::from(".");
                extension.push_str(e_str);
            }

            if dir.file_name().to_str() == Some(user_event) {
                ue_path = Some(crop_file_name(root, path_str)?);
            }
            else if incl.file_included(name) || incl.type_included(&extension) {
                file_paths.push(crop_file_name(root, path_str)?);
            }
        }
    }

    Ok((file_paths, ue_path))
}

fn crop_file_name(root: &str, f: &str) -> Result<String, String> {
    let file_chars = Vec::from_iter(f.chars());
    let root_chars = Vec::from_iter(root.chars());
    let (file_len, root_len) = (file_chars.len(), root_chars.len());
    if file_len <= root_len || &file_chars[0..root_len] != &root_chars {
        return Err(format!("File {} is not a child of the root directory", f));
    }

    let mut file_cropped = Vec::new();
    if file_chars[root_len] == '/' || file_chars[root_len] == '\\' {
        file_chars[root_len+1..file_len].clone_into(&mut file_cropped);
    } else {
        file_chars[root_len..file_len].clone_into(&mut file_cropped);
    }
    let file_cropped = file_cropped.iter().collect::<String>();

    Ok(file_cropped)
}

fn make_raw_include(project_type: u8) -> &'static str {
    match project_type {
        0 => {
"fonts/*.ini
scripts/*.gml
scripts/attacks/*.gml
sounds/*.ogg
sprites/*.png
config.ini
charselect.png
hud.png
hurt.png
icon.png
offscreen.png
portrait.png
preview.png
result_small.png
charselect.ogg"
        },
        1 => panic!("Buddies not yet supported"), // TODO
        2 => panic!("Stages not yet supported"),
        3 => panic!("Character skins not yet supported"),
        _ => panic!("Unexpected input to make_include()"),
    }
}