use std::io;
use std::fs;
use crate::export_project::*;

pub struct Flags {
    pub strip_comments: bool,
    pub strip_whitespace: bool,
    pub is_silent: bool,
    pub do_overwrite: bool,
    pub block_overwrite: bool,
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            strip_comments: false,
            strip_whitespace: false,
            is_silent: false,
            do_overwrite: false,
            block_overwrite: false,
        }
    }
}

pub fn get_flags(args: &Vec<String>) -> Result<Flags, String> {
    let mut f = Flags::new();
    for arg in args.iter() {
        match arg.as_str() {
            "-c" => f.strip_comments = true,
            "-w" => f.strip_whitespace = true,
            "-s" => f.is_silent = true,
            "-o" => f.do_overwrite = true,
            "-safe" => f.block_overwrite = true,
            _ => (),
        }
    }
    if f.do_overwrite && f.block_overwrite {
        Err(String::from("Flags -o and -safe are mutually exclusive"))
    }
    else if f.is_silent && !f.block_overwrite && !f.do_overwrite {
        Err(String::from("Flag -s must be used alongside flag -o or -safe"))
    }
    else {
        Ok(f)
    }
}

pub fn cli_print_help() {
    println!("Usage: rcf.exe [source directory] [destination directory] [user_event #] [flags]");
    println!("-c: Strip comments on export");
    println!("-w: Strip whitespace on export");
    println!("-s: Silent mode (mutes output; requires -o or -safe)");
    println!("-o: Force overwrite of destination directory (may result in data loss)");
    println!("-safe: Disable overwrite of destination directory (incompatible with -o)");
}

pub fn cli_print_usage() {
    eprintln!("Usage: rcf.exe [source directory] [destination directory] [user_event #] [flags]");
    eprintln!("For list of flags: rcf.exe ?");
}

pub fn cli_check_source_valid(src: &str) -> Result<(), String> {
    match get_export_type(src) {
        Some(0) => Ok(()),
        Some(1 | 2 | 3) => Err(String::from("Project type is not yet supported")),
        _ => Err(String::from("Could not find valid project file at source")),
    }
}

pub fn cli_check_dest_valid(dest: &str, flags: &Flags) -> Result<(), String> {
    match flags {
        Flags{ block_overwrite: true, .. } => cli_dest_empty(dest),
        Flags{ do_overwrite: true, .. } => cli_dest_overwrite(dest),
        _ => cli_dest_noisy(dest),
    }
}

// This can only be called in noisy mode, so it can use inline print statements
fn cli_dest_noisy(dest: &str) -> Result<(), String> {
    let dest_copy = dest;
    match get_export_type(dest_copy) {
        Some(_) => {
            loop {
                println!("There appears to be an existing project at: {dest}");
                println!("Overwrite it? (Y/N)");
                let mut input = String::new();
                io::stdin().read_line(&mut input)
                    .expect("Failed to read line");
                let input = input.to_lowercase();
                
                match input.as_str() {
                    "y\r\n" | "yes\r\n" | "y\n" | "yes\n" => {
                        println!("Overwrite confirmed, continuing...");
                        return cli_dest_overwrite(dest);
                    }
                    "n\r\n" | "no\r\n" | "n\n" | "no\n" => {
                        return Err(String::from("Overwrite canceled."));
                    }
                    _ => eprintln!("Unrecognized input."),
                }
            }
        }
        _ => cli_dest_empty(dest),
    }
}

fn cli_dest_empty(dest: &str) -> Result<(), String> {
    if let Ok(false) = fs::exists(dest) { return Ok(()) };
    let dir = match fs::read_dir(dest) {
        Ok(d) => d,
        Err(e) => return Err(e.to_string()),
    };
    if dir.count() != 0 {
        Err(String::from("Destination folder is non-empty (use -o to enable overwriting)"))
    } else {
        Ok(())
    }
}

// THIS FUNCTION DELETES ALL CONTENTS AT THE DESTINATION!
// Use with care!
fn cli_dest_overwrite(dest: &str) -> Result<(), String> {
    _ = fs::remove_dir_all(dest);
    Ok(())
}

pub fn cli_get_ue(ue_str: &str) -> Result<u8, String> {
    if let Ok(num) = ue_str.parse::<u8>() {
        Ok(num)
    } else {
        Err(String::from("Provided user_event # is invalid"))
    }
}