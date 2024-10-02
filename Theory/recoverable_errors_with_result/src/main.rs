// Function to divide two integers
fn divide(x: i32, y: i32) -> Result<i32, String> {
    // Check if the divisor is zero
    if y == 0 {
        // Return an error if division by zero is attempted
        return Err(String::from("Cannot divide by zero"));
    }
    // Perform the division and return the result
    Ok(x / y)
}

fn main() {
    // Attempt to divide 10 by 2
    let r = match divide(10, 2) {
        Ok(num) => num,  // If successful, assign the result to r
        Err(err) => {
            // If an error occurs, print the error message
            println!("{}", err);
            -1  // Return -1 as a default value in case of error
        } 
    };

    // Print the result of the division
    println!("{}", r);
}

// --------- File Operations Example ---------

use std::fs::File;
use std::io::{self, Read};

// Function to read username from a file
fn read_username_from_file() -> Result<String, io::Error> { 
    // Open the file "username.txt"
    let mut username_file = File::open("username.txt")?;

    // Create a new empty String to store the username
    let mut username = String::new();

    // Read the contents of the file into the username string
    username_file.read_to_string(&mut username)?;

    // Return the username wrapped in Ok
    Ok(username)
}

fn main() {
    // Call the read_username_from_file function and handle the result
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),  // If successful, print the username
        Err(error) => println!("Error reading username: {:?}", error),  // If an error occurred, print the error
    }
}

// --------- Using expect() for Error Handling ---------

use std::fs::File;
use std::io::{self, Read};

// Function to read username from a file using expect() for error handling
fn read_username_from_file() -> Result<String, io::Error> { 
    // Open the file "username.txt" and handle potential errors
    let mut username_file = File::open("username.txt").expect("Failed to open username.txt");

    // Create a new empty String to store the username
    let mut username = String::new();

    // Read the contents of the file into the username string and handle potential errors
    username_file.read_to_string(&mut username)
        .expect("Failed to read username from file");

    // Return the username wrapped in Ok
    Ok(username)
}

fn main() {
    // Call the read_username_from_file function and handle the result
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),  // If successful, print the username
        Err(error) => println!("Error reading username: {:?}", error),  // If an error occurred, print the error
    }
}

// The `expect` method is used on `Result` types to handle errors.
// If the `Result` is `Ok`, it returns the value inside.
// If it's an `Err`, it calls `panic!` with the provided message.
// 
// In this example:
// 1. `expect` is used when opening the file. If it fails, it will panic with the message "Failed to open username.txt".
// 2. `expect` is also used when reading from the file. If it fails, it will panic with the message "Failed to read username from file".
// 
// Using `expect` is more informative than using `unwrap` because it allows you to specify an error message.
// However, it still causes the program to panic on error, so it should be used when you're certain that a failure is unrecoverable.
