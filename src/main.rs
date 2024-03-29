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
    Swap { index1: usize, index2: usize }
}

fn main() {
    // Get arguments
    let args = Cli::parse();
    let home_dir = dirs_next::home_dir();
    // Convert to path string
    let mut dir: String = match home_dir {
        Some(path) => path.to_string_lossy().into_owned(),
        None => String::from("/"),
    };
    dir.push_str("/Documents/ToDoFast/");
    // Create directory to store text file
    let _ = fs::create_dir(&dir);
    // Create text file
    dir.push_str("todo.txt");
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(&dir).unwrap();

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
                writeln!(file, "{}", task).expect("Failed to write to file");
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
                    .open(&dir).unwrap();
                for (line_num, line) in lines.iter().enumerate().take(lines.len()-1) {
                    if line_num != index-1 {
                        writeln!(file, "{}", line).expect("Failed to write to file");
                    }
                }
            }
        }
        Some(Actions:: Swap { index1, index2 }) => {
            let mut file_content = String::new();
            let _ = file.read_to_string(&mut file_content);
            let lines: Vec<&str> = file_content.split('\n').collect();
            let swap_one = lines[*index1-1];
            let swap_two = lines[*index2-1];
            if file_content.lines().count() > index1-1 && file_content.lines().count() > index2-1  {
                let mut file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .truncate(true)
                    .open(&dir).unwrap();
                for (line_num, line) in lines.iter().enumerate().take(lines.len()-1) {
                    if line_num == index1-1 {
                        writeln!(file, "{}", swap_two).expect("Failed to write to file"); 
                    } else if line_num == index2-1 {
                        writeln!(file, "{}", swap_one).expect("Failed to write to file"); 
                    } else {
                        writeln!(file, "{}", line).expect("Failed to write to file");
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
