use std::io;
use std::fs;
use crate::export_project::*;

pub struct Flags {
    pub user_event: Option<u8>,
    pub strip_comments: bool,
    pub strip_whitespace: bool,
    pub is_silent: bool,
    pub do_overwrite: bool,
    pub block_overwrite: bool,
    pub dry_run: bool,
    pub inert_run: bool,
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            user_event: None,
            strip_comments: false,
            strip_whitespace: false,
            is_silent: false,
            do_overwrite: false,
            block_overwrite: false,
            dry_run: false,
            inert_run: false,
        }
    }
}

pub fn get_flags(args: &Vec<String>) -> Result<Flags, String> {
    let mut f = Flags::new();
    let mut save_ue = false;
    let mut passed_over = 0; // Used to ensure source and dest arguments are not flags
    for (idx, arg) in args.iter().enumerate() {
        if save_ue {
            if let Ok(num) = arg.parse::<u8>() {
                f.user_event = Some(num);
                save_ue = false;
            } else {
                return Err(String::from("Provided user_event number is invalid"));
            }
        }
        else { match arg.as_str() {
            "-ue" => save_ue = true,
            "-c" => f.strip_comments = true,
            "-w" => f.strip_whitespace = true,
            "-s" => f.is_silent = true,
            "-o" => f.do_overwrite = true,
            "-safe" => f.block_overwrite = true,
            "-init" => f.dry_run = true,
            "-inert" => f.inert_run = true,
            _ => if idx <= 2 {
                passed_over += 1;
            },
        } };
    }

    if save_ue {
        Err(String::from("Flag -ue must be followed by a user_event number"))
    }
    else if passed_over < 2 || (passed_over == 2 && !f.dry_run) {
        Err(String::from("Source and destination arguments must not be flags"))
    }
    else if f.do_overwrite && f.block_overwrite {
        Err(String::from("Flags -o and -safe are mutually exclusive"))
    }
    else if f.is_silent && !f.block_overwrite && !f.do_overwrite {
        Err(String::from("Flag -s must be used alongside flag -o or -safe"))
    }
    else if f.dry_run && f.inert_run {
        Err(String::from("Flags -init and -inert are mutually exclusive"))
    }
    else {
        Ok(f)
    }
}

// TODO: add flags
pub fn cli_print_help() {
    println!("Usage: rcf.exe [source directory] [destination directory] [flags]");
    println!("-ue [#]: Sets the user_event used as the constant source");
    println!("-c: Strip comments on export");
    println!("-w: Strip whitespace on export");
    println!("-s: Silent mode (mutes output; requires -o or -safe)");
    println!("-o: Force overwrite of destination directory (may result in data loss)");
    println!("-safe: Disable overwrite of destination directory (incompatible with -o)");
    println!("-init: Initializes RCF files in the source, but does not export");
    println!("-inert: Exports as normal, but does not initialize RCF files in the source (incompatible with -init)");
}

pub fn cli_print_usage() {
    eprintln!("Usage: rcf.exe [source directory] [destination directory] [flags]");
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