use clap::{Parser, Subcommand};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

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

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let task = match cli.command {
        Commands::Compute => "COMPUTE",
        Commands::IoTask => "IO_TASK",
        Commands::Exit => "EXIT",
    };

    // Connect to the daemon using an async Unix socket
    match UnixStream::connect(SOCKET_PATH).await {
        Ok(mut stream) => {
            stream.write_all(task.as_bytes()).await.unwrap();
            stream.flush().await.unwrap();

            // Wait for response
            let mut response = String::new();
            stream.read_to_string(&mut response).await.unwrap();
            println!("📩 Response from daemon: {}", response);
        }
        Err(e) => {
            eprintln!("❌ Failed to connect to daemon: {}", e);
        }
    }
}

