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
    Add { task: Option<String> },
}

fn main() {
    let args = Cli::parse();

    match &args.action {
        Some(Actions:: Add { task }) => {
            println!("'add' was used, task is: {:?}", task);
        }
        None => {
            println!("'default subcommand'");
        }
    }
}
