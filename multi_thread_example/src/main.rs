/*
Multi-threading example for multiple producers single consumer
*/

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
	// Creating an mpsc channel here
	// (sender, receiver) is the return parameters
	let (tx, rx) = mpsc::channel();

	// Clone the sender to create multiple sender objects
	let tx1 = tx.clone();

	let i = 1;

	// Sender/Producer 0
	thread::spawn(move || {
		for i in 1..=3 {
			println!("Sender 1: sending");
			tx.send(format!("Sender 1: Message ")).unwrap();
			tx.send(i.to_string()).unwrap();
			thread::sleep(Duration::from_millis(500));
		}
	});

	// Sender/Producer 1
	thread::spawn(move || {
		for i in 100..=110 {
			println!("Sender 2: sending");
			tx1.send(format!("Sender 2: Messahe ")).unwrap();
			tx1.send(i.to_string()).unwrap();
			thread::sleep(Duration::from_millis(500));
		}
	});

	// One receiver
	for received in rx {
		println!("Parent thread, i = {}", i);
		println!("Received: {}", received);
	}
}
