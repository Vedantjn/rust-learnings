use std:: {
    sync::mpsc,
    thread::{self, spawn},
};

fn main() {
    // mpsc -> multiple producer single consumer
    let (tx, rx) = mpsc::channel();
    spawn(move || {
        tx.send(String::from("Hello World")).unwrap();
    });

    let value = rx.recv().unwrap();
    println!("{}", value);
}

// -------------------------------------------------------------

// Q). Write the code that finds the sum from 1 - 10^8
// Use threads to make sure you use all cores of your machine
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let producer = tx.clone();

        thread::spawn(move || {
            let mut sum: u64 = 0;
            for j in (i * 1_000_000)..(i + 1) * 1_000_000 { 
                sum += j;
            }
            producer.send(sum).unwrap();
        });
    }

    drop(tx); // Close the sending end of the channel

    let mut final_sum: u64 = 0;

    // Use a loop to receive all values from the receiver
    for val in rx.iter() { 
        println!("Received value from thread");
        final_sum += val; 
    }

    println!("{}", final_sum);
}
