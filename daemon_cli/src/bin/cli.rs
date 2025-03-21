use clap::{Parser, Subcommand};
use crossbeam::channel::Sender;
use std::fs;
use std::process::exit;

/// Define the CLI structure
#[derive(Parser)]
#[command(name = "Task CLI")]
#[command(about = "A CLI to communicate with the daemon process")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Define the available tasks
#[derive(Subcommand)]
enum Commands {
    Compute,
    IoTask,
    Exit,
}

fn main() {
    let cli = Cli::parse();
    
    // Send command to the daemon (through a file or IPC)
    let task = match cli.command {
        Commands::Compute => "COMPUTE",
        Commands::IoTask => "IO_TASK",
        Commands::Exit => "EXIT",
    };

    fs::write("/tmp/daemon_input.txt", task).expect("Failed to write to daemon input");

    if let Commands::Exit = cli.command {
        println!("Stopping daemon...");
        exit(0);
    }
}

