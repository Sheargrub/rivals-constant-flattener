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
    if args.len() < 4 {
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

    if flags.is_silent { run_cli_silent(args, flags); } 
    else { run_cli_noisy(args, flags); }

}

fn run_cli_noisy(args: Vec<String>, flags: Flags) {

    // Get user event #
    let user_event = match cli_get_ue(&args[3]) {
        Ok(ue) => ue,
        Err(e) => {
            eprintln!("Error in user_event:");
            eprintln!("    {e}");
            process::exit(64);
        }
    };

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
    if let Err(e) = export_project(&args[1], &args[2], user_event, flags.strip_whitespace, flags.strip_comments) {
        eprintln!("Unexpected error while exporting project:");
        eprintln!("    {e}");
        process::exit(70);
    }

    // Apply export_config file
    match export_config(&args[1], &args[2]) {
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

    // Get user event #
    let user_event = match cli_get_ue(&args[3]) {
        Ok(ue) => ue,
        Err(_) => process::exit(64),
    };

    // Ensure validity of input
    if let Err(_) = cli_check_source_valid(&args[1]) {
        process::exit(65);
    }

    // See if there's an output directory
    if let Err(_) = cli_check_dest_valid(&args[2], &flags) {
        process::exit(66);
    }

    // Perform export
    if let Err(_) = export_project(&args[1], &args[2], user_event, flags.strip_whitespace, flags.strip_comments) {
        process::exit(70);
    }

    // Apply export_config file
    match export_config(&args[1], &args[2]) {
        Ok(_) => (),
        Err(_) => process::exit(71),
    }
}