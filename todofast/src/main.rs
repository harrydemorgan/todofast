use clap::{Parser, Subcommand};
use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Write, BufReader};


// Define the input arguments
#[derive(Parser)]
struct Cli {
    // The action to perform
    #[command(subcommand)]
    action: Option<Actions>
}

#[derive(Subcommand)]
enum Actions {
    Add { task: String },
}

fn main() {
    let args = Cli::parse();
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("todo.txt").unwrap();

    match &args.action {
        Some(Actions:: Add { task }) => { 
            let mut file_content = String::new();
            file.read_to_string(&mut file_content);
            // Add task to the file
            if !file_content.contains(task) {
                writeln!(file, "{}", task);
            } else {
                println!("Task already exists!\n")
            }
        }
        None => {
            // Print the list
            // Create an empty mutable string
            let mut file_content = String::new();

            // Copy contents of file to a mutable string
            file.read_to_string(&mut file_content);

            println!("{}", file_content);
        }
    }
}
