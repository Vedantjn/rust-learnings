fn main() {
    // let s = "Hello World!";

    // {
    //     let x = "Hey from x!"; // x scope starts
    // } // x scope ends

    // println!("x = {x}");
    // println!("s = {s}");

    // let mut s = String::from("Hello");
    // s.push_str(" World");
    // println!("S = {s}");


    // let mut x = 5;
    // let y = x; // copy

    // x = 10;

    // println!("X is {x} and Y is {y}");


    // Move
    // let s1 = String::from("I am x");
    // let s2 = s1; // now s1 is invalid
    // // let s2 = s1.clone(); 
    // println!("s1 is {s1}");
    // println!("s2 is {s2}");


    let num = 10;
    let result = add(num);

    let name = String::from("Vedant Jain");
    let s = gives_ownership();
    let s = takes_and_gives_back(s);
    takes_ownership(name);

    println!("Num is {num} and result is {result}");
    // println!("Value of name is : {name}");
    println!("s = {s}");



    let s3 = String::from("vedant jain");
    let (s4, l) = calculae_length(s3);
    println!("The length of {s4} is {l}");

    
}

fn add(x: i32) -> i32 {
    x+10
}

fn takes_ownership(s: String) {
    println!("Inside ownership {s}");
}

fn gives_ownership() -> String {
    let s = String::from("This is a string from gives_ownership function");
    s
}

fn takes_and_gives_back(s: String) -> String {
    println!("s in takes_and_gives_back {s}");
}

fn calculae_length(s: String) -> (String, usize) {
    let result = s.len();
    (s, result)
}