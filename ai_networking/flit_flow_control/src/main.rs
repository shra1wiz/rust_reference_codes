//use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crossbeam::channel::{bounded, Receiver, Sender};

/// Maximum buffer size at the receiver
const BUFFER_SIZE: usize = 4;

/// Struct to represent a FLIT (Flow Control Unit)
#[derive(Debug, Clone)]
struct Flit {
    id: u32,
}

/// Struct to represent Credits
#[derive(Debug)]
struct Credit {
	credit: u32,
}

/// Receiver that processes FLITs and returns credits
fn receiver(flit_rx: Receiver<Flit>, credit_tx: Sender<Credit>) {
    let mut buffer: Vec<Flit> = Vec::new();

    while let Ok(flit) = flit_rx.recv() {
        println!("Receiver: Processing FLIT {:?}", flit);
        buffer.push(flit);

        // Simulate processing delay
        thread::sleep(Duration::from_millis(500));

        // Remove a FLIT from the buffer (processed)
        if let Some(removed_flit) = buffer.pop() {
            println!("Receiver: FLIT {:?} processed, returning credit.", removed_flit);
			let credit = Credit {credit: 1};
            credit_tx.send(credit).unwrap(); // Return a credit
        }
    }
}

/// Sender that transmits FLITs based on available credits
fn sender(flit_tx: Sender<Flit>, credit_rx: Receiver<Credit>) {
    let mut credits = BUFFER_SIZE; // Start with full credits
    let mut flit_id = 1;

    loop {
        if credits > 0 {
            let flit = Flit { id: flit_id };
            flit_id += 1;
            println!("Sender: Sending FLIT {:?}", flit);
            flit_tx.send(flit).unwrap();
            credits -= 1; // Consume a credit
        } else {
            println!("Sender: Waiting for credits...");
        }

        // Receive credits when available
        if let Ok(credit) = credit_rx.try_recv() {
            credits += credit.credit as usize;
            println!("Sender: Received {:#?} credit(s), total available: {}", credit, credits);
        }

        // Simulate network transmission delay
        thread::sleep(Duration::from_millis(200));
    }
}

fn main() {
    // Create channels for FLIT transmission and credit return
    let (flit_tx, flit_rx) = bounded::<Flit>(BUFFER_SIZE); // Buffer size = 4
    let (credit_tx, credit_rx) = bounded::<Credit>(BUFFER_SIZE);

    // Spawn sender and receiver threads
    let sender_thread = thread::spawn(move || sender(flit_tx, credit_rx));
    let receiver_thread = thread::spawn(move || receiver(flit_rx, credit_tx));

    // Wait for threads to complete (infinite loop in this case)
    sender_thread.join().unwrap();
    receiver_thread.join().unwrap();
}

