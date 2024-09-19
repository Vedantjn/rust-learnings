use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("Hi from spawned thread {}", i);
        }
    });

    for i in 0..50 {
        println!("i from main thread {}", i);
    }

    handle.join();
}

// ---------------------------------------------------------

use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("Hi from spawned thread {}", i);
        }
    });

    handle.join();

    for i in 0..50 {
        println!("i from main thread {}", i);
    }

}

// ---------------------------------------------------------

// This code doesn't compile because 'v' goes out of scope before thread starts

use std::thread

fn main() {
    let x = 1;
    {

        let v = vec![1, 2, 3];
        thread::spawn(|| {
            println!("{:?}", v);
        });
    }
    println!("{}", x);
}

// ---------------------------------------------------------

use std::thread

fn main() {
    let x = 1;
    {

        let v = vec![1, 2, 3];
        // moving v to thread
        thread::spawn(move || {
            println!("{:?}", v);
        });
    }
    println!("{}", x);
}