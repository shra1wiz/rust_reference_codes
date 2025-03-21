use std::os::unix::net::UnixListener;
use std::io::{Read, Write};
use std::fs;
use std::thread;

const SOCKET_PATH: &str = "/tmp/daemon_socket";

fn main() {
    println!("🚀 Daemon started...");

    // Remove old socket if it exists
    let _ = fs::remove_file(SOCKET_PATH);

    // Create a Unix socket server
    let listener = UnixListener::bind(SOCKET_PATH).expect("❌ Failed to bind socket");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || {
                    let mut task = String::new();
                    stream.read_to_string(&mut task).unwrap();
                    let task = task.trim();

                    println!("📥 Received task: {}", task);
                    let response = match task {
                        "COMPUTE" => {
                            println!("⚙️ Processing Compute Task...");
                            "✅ Compute Task Done!".to_string()
                        }
                        "IO_TASK" => {
                            println!("🗂️ Processing IO Task...");
                            "✅ IO Task Done!".to_string()
                        }
                        "EXIT" => {
                            println!("🛑 Daemon exiting...");
                            std::process::exit(0);
                        }
                        _ => "❌ Unknown Task".to_string(),
                    };

                    // Send response back to CLI
                    stream.write_all(response.as_bytes()).unwrap();
                    stream.flush().unwrap();
                });
            }
            Err(e) => eprintln!("❌ Connection failed: {}", e),
        }
    }
}

