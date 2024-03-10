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
    Remove { index: usize },
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
            let mut task_exists = false;
            // Add task to the file
            for line in file_content.lines() {
                if line == (task) {
                    task_exists = true;
                }
            }
            if task_exists {
                println!("Task already exists!")
            } else {
                writeln!(file, "{}", task);
            }
        }
        Some(Actions:: Remove { index }) => {
            let mut file_content = String::new();
            file.read_to_string(&mut file_content);
            let lines: Vec<&str> = file_content.split('\n').collect();
            if file_content.lines().count() > index-1 {
                let mut file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .truncate(true)
                    .open("todo.txt").unwrap();
                for (line_num, line) in lines.iter().enumerate().take(lines.len()-1) {
                    if line_num != index-1 {
                        writeln!(file, "{}", line);
                    }
                }
            }
        }
        None => {
            // Print the list
            // Create an empty mutable string
            let mut file_content = String::new();
            
            // Copy contents of file to a mutable string
            file.read_to_string(&mut file_content);

            let lines: Vec<&str> = file_content.split('\n').collect();
            // Print lines with line numbers
            for (line_num, line) in lines.iter().enumerate().take(lines.len()-1) {
                println!("{: >3} {}", line_num + 1, line);
            }
        }
    }
}
