use crossbeam::channel::{unbounded, Receiver};
use std::{fs, thread, time::Duration};

fn main() {
    println!("Daemon started...");

    let (tx, rx) = unbounded(); // Channel for sending tasks to workers

    // Spawn a background thread for listening to tasks
    thread::spawn(move || {
        loop {
            if let Ok(task) = fs::read_to_string("/tmp/daemon_input.txt") {
                fs::write("/tmp/daemon_input.txt", "").unwrap(); // Clear the file after reading
                if task.trim() == "EXIT" {
                    println!("Daemon exiting...");
                    break;
                }
                tx.send(task.trim().to_string()).unwrap();
            }
            thread::sleep(Duration::from_secs(1)); // Avoid excessive polling
        }
    });

    // Worker loop: Process tasks as they arrive
    while let Ok(task) = rx.recv() {
        println!("Received task: {}", task);
        thread::spawn(move || {
            match task.as_str() {
                "COMPUTE" => {
                    println!("Starting Compute Task...");
                    thread::sleep(Duration::from_secs(10)); // Simulate work
                    println!("Compute Task Done!");
                }
                "IO_TASK" => {
                    println!("Starting IO Task...");
                    thread::sleep(Duration::from_secs(20)); // Simulate file I/O
                    println!("IO Task Done!");
                }
                _ => println!("Unknown task"),
            }
        });
    }
}

