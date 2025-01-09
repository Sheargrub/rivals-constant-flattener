pub mod export_project;
mod rcf_cli;

use std::process;
use crate::rcf_cli::*;
use crate::export_project::*;

pub fn run_cli(args: Vec<String>) {
    
    // Take in args
    if args.len() >= 1 && args[0] == "?" {
        cli_print_help();
        process::exit(0);
    }
    if args.len() < 3 {
        cli_print_usage();
        process::exit(63);
    }
    
    // Handle flags
    let flags = match get_flags(&args) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error in flags:");
            eprintln!("    {e}");
            cli_print_usage();
            process::exit(77);
        }
    };

    if flags.dry_run { run_cli_dry(args, flags); }
    else if flags.is_silent { run_cli_silent(args, flags); } 
    else { run_cli_noisy(args, flags); }

}

fn run_cli_noisy(args: Vec<String>, flags: Flags) {

    // Ensure validity of source
    if let Err(e) = cli_check_source_valid(&args[1]) {
        eprintln!("Error with source directory:");
        eprintln!("    {e}");
        process::exit(65);
    }

    // See if there's an output directory
    if let Err(e) = cli_check_dest_valid(&args[2], &flags) {
        eprintln!("Error with destination directory:");
        eprintln!("    {e}");
        process::exit(66);
    }

    // Perform export
    if let Err(e) = export_project(&args[1], &args[2], flags.user_event, flags.strip_whitespace, flags.strip_comments, flags.inert_run) {
        eprintln!("Unexpected error while exporting project:");
        eprintln!("    {e}");
        process::exit(70);
    }

    // Apply export_config file
    match export_config(&args[1], &args[2], flags.inert_run) {
        Ok(true) => println!("Created new config_export.ini file in source folder. This will be used in lieu of config.ini for future exports."),
        Ok(false) => (),
        Err(e) => {
            eprintln!("Unexpected error while exporting config file:");
            eprintln!("    {e}");
            eprintln!("Due to the nature of this error, it is likely that the project was otherwise exported successfully.");
            process::exit(71);
        }
    }

    println!("Export completed successfully.");
}

fn run_cli_silent(args: Vec<String>, flags: Flags) {

    // Ensure validity of input
    if let Err(_) = cli_check_source_valid(&args[1]) {
        process::exit(65);
    }

    // See if there's an output directory
    if let Err(_) = cli_check_dest_valid(&args[2], &flags) {
        process::exit(66);
    }

    // Perform export
    if let Err(_) = export_project(&args[1], &args[2], flags.user_event, flags.strip_whitespace, flags.strip_comments, flags.inert_run) {
        process::exit(70);
    }

    // Apply export_config file
    match export_config(&args[1], &args[2], flags.inert_run) {
        Ok(_) => (),
        Err(_) => process::exit(71),
    }
}

fn run_cli_dry(args: Vec<String>, flags: Flags) {
    // Ensure validity of input
    if let Err(e) = cli_check_source_valid(&args[1]) {
        if !flags.is_silent {
            eprintln!("Error with source directory:");
            eprintln!("    {e}");
        }
        process::exit(65);
    }

    // Dry-export project
    if let Err(e) = export_project_dry(&args[1]) {
        if !flags.is_silent {
            eprintln!("Unexpected error while dry-exporting project:");
            eprintln!("    {e}");
        }
        process::exit(70);
    }
}