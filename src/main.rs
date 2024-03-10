use clap::{Parser, Subcommand};
use std::fs;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use dirs_next;

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
    let home_dir = dirs_next::home_dir();
    let home_d = dirs_next::home_dir();
    let mut dir: String = match home_dir {
        Some(path) => path.to_string_lossy().into_owned(),
        None => String::from("/"),
    };
    let mut filename: String = match home_d {
        Some(path) => path.to_string_lossy().into_owned(),
        None => String::from("/"),
    };
    dir.push_str("/Documents/ToDoFast/");
    filename.push_str("/Documents/ToDoFast/todo.txt");
    let filename_copy = filename.clone();
    let _ = fs::create_dir(dir);
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(filename).unwrap();

    match &args.action {
        Some(Actions:: Add { task }) => { 
            let mut file_content = String::new();
            let _ = file.read_to_string(&mut file_content);
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
            let _ = file.read_to_string(&mut file_content);
            let lines: Vec<&str> = file_content.split('\n').collect();
            if file_content.lines().count() > index-1 {
                let mut file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .truncate(true)
                    .open(filename_copy).unwrap();
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
            let _ = file.read_to_string(&mut file_content);

            if file_content.is_empty() {
                println!("Nothing to do");
            }

            let lines: Vec<&str> = file_content.split('\n').collect();
            // Print lines with line numbers
            for (line_num, line) in lines.iter().enumerate().take(lines.len()-1) {
                println!("{: >3} {}", line_num + 1, line);
            }
        }
    }
}
