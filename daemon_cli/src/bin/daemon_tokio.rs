use tokio::net::UnixListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::fs;
use tokio::spawn;

const SOCKET_PATH: &str = "/tmp/daemon_socket";

#[tokio::main]
async fn main() {
    println!("🚀 Daemon started...");

    // Remove old socket if it exists
    let _ = fs::remove_file(SOCKET_PATH);

    // Create an async Unix socket server
    let listener = UnixListener::bind(SOCKET_PATH).expect("❌ Failed to bind socket");

    loop {
        match listener.accept().await {
            Ok((mut stream, _)) => {
                spawn(async move {
                    let mut task = String::new();
                    if stream.read_to_string(&mut task).await.is_ok() {
                        let task = task.trim();
                        println!("📥 Received task: {}", task);

                        let response = match task {
                            "COMPUTE" => {
                                println!("⚙️ Processing Compute Task...");
                                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                                "✅ Compute Task Done!".to_string()
                            }
                            "IO_TASK" => {
                                println!("🗂️ Processing IO Task...");
                                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                                "✅ IO Task Done!".to_string()
                            }
                            "EXIT" => {
                                println!("🛑 Daemon exiting...");
                                std::process::exit(0);
                            }
                            _ => "❌ Unknown Task".to_string(),
                        };

                        // Send response back to CLI
                        stream.write_all(response.as_bytes()).await.unwrap();
                        stream.flush().await.unwrap();
                    }
                });
            }
            Err(e) => eprintln!("❌ Connection failed: {}", e),
        }
    }
}

