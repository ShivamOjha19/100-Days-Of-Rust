mod task;
mod commands;
mod storage;

use clap::{Parser, Subcommand};
use commands::*;

#[derive(Parser)]
#[command(name = "Task Manager")]
#[command(about = "A simple CLI To-Do app", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { description: String },
    List,
    Done { id: usize },
    Remove { id: usize },
}

fn main() {
    let cli = Cli::parse();
    let mut tasks = storage::load_tasks();

    match cli.command {
        Commands::Add { description } => add_task(&mut tasks, description),
        Commands::List => list_tasks(&tasks),
        Commands::Done { id } => mark_done(&mut tasks, id),
        Commands::Remove { id } => remove_task(&mut tasks, id),
    }

    storage::save_tasks(&tasks);
}