use clap::{Parser, Subcommand};
use std::os::unix::net::UnixStream;
use std::io::{Write, Read};

const SOCKET_PATH: &str = "/tmp/daemon_socket";

/// CLI structure
#[derive(Parser)]
#[command(name = "Task CLI")]
#[command(about = "Send tasks to the daemon and wait for a response")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Available commands
#[derive(Subcommand)]
enum Commands {
    Compute,
    IoTask,
    Exit,
}

fn main() {
    let cli = Cli::parse();

    let task = match cli.command {
        Commands::Compute => "COMPUTE",
        Commands::IoTask => "IO_TASK",
        Commands::Exit => "EXIT",
    };

    // Connect to the daemon via Unix socket
    match UnixStream::connect(SOCKET_PATH) {
        Ok(mut stream) => {
            stream.write_all(task.as_bytes()).unwrap();
            stream.flush().unwrap();

            // Wait for response
            let mut response = String::new();
            stream.read_to_string(&mut response).unwrap();
            println!("📩 Response from daemon: {}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to connect to daemon: {}", e);
        }
    }
}

