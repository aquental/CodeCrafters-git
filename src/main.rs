use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    // Check if there are enough arguments
    if args.len() < 2 {
        eprintln!("Error: No command provided. Usage: {} <command>", args[0]);
        process::exit(1);
    }

    if args[1] == "init" {
        if fs::metadata(".git").is_ok() {
            eprintln!("Error: .git directory already exists");
            process::exit(1);
        }
        // Create .git directory structure with proper error handling
        if let Err(e) = fs::create_dir(".git") {
            eprintln!("Error creating .git directory: {}", e);
            process::exit(1);
        }
        if let Err(e) = fs::create_dir(".git/objects") {
            eprintln!("Error creating .git/objects directory: {}", e);
            process::exit(1);
        }
        if let Err(e) = fs::create_dir(".git/refs") {
            eprintln!("Error creating .git/refs directory: {}", e);
            process::exit(1);
        }
        if let Err(e) = fs::write(".git/HEAD", "ref: refs/heads/master\n") {
            eprintln!("Error writing .git/HEAD file: {}", e);
            process::exit(1);
        }
        println!("Initialized git repository");
    } else {
        eprintln!("Unknown command: {}", args[1]);
        process::exit(1);
    }
}
