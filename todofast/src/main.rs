use clap::{Parser, Subcommand};

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

    let mut list: Vec<String> = Vec::new();

    match &args.action {
        Some(Actions:: Add { task }) => {
            println!("'add' was used, task is: {:?}", task);
            list.push(task.to_string());
            for (index, task) in list.iter().enumerate() {
                println!("{} {}", index+1, task)
            }
        }
        None => {
            println!("'default subcommand'");
        }
    }
}
