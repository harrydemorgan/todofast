use clap::{Parser, Subcommand};
use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Write};

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
            println!("'add' was used, task is: {:?}", task);
            // Add task to the file
            file.write_all(task.as_bytes()).expect("failed to write file");
            file.write_all("\n".as_bytes());
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
