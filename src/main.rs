use code_crafters_git::init_git_repo;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();

    // Check if there are enough arguments
    if args.len() < 2 {
        eprintln!("Error: No command provided. Usage: {} <command>", args[0]);
        process::exit(1);
    }

    let command = &args[1];
    match command.as_str() {
        "init" => {
            if let Err(e) = init_git_repo() {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
        "cat-file" => cat_file_cmd(args),
        "add" => add_cmd(args),
        "commit" => {
            // TODO: Implement commit logic (e.g., create commit object)
            println!(
                "Commit message: {}",
                if args.len() > 2 {
                    &args[2]
                } else {
                    "Unnamed commit"
                }
            );
        }
        "status" => {
            // TODO: Implement status logic (e.g., check staged/unstaged files)
            println!("Repository status: Clean");
        }
        _ => {
            eprintln!("Error: Unknown command: {}", command);
            process::exit(1);
        }
    }
}


fn cat_file_cmd(args: Vec<String>) {
    if args.len() < 3 {
        eprintln!(
            "Error: No object hash provided. Usage: {} cat-file <object_hash>",
            args[0]
        );
        process::exit(1);
    }
    let object_hash = &args[2];
    // TODO: Implement cat-file logic (e.g., read object from disk and print its contents)
    println!("Object hash: {}", object_hash);
}

fn add_cmd(args: Vec<String>) {
    // TODO: Implement add logic (e.g., stage files)
    if args.len() < 3 {
        eprintln!("Error: No file provided. Usage: {} add <file>", args[0]);
        process::exit(1);
    }
    println!("Added file: {}", args[2]);
}
