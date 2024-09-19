enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {

}

fn value_in_cents(coin: Coin) -> u8 {
    // match me saare cases likhne pdte hain
    // sare case handle hone chaie 
    // matches are exhaustive
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// ------------------------------------------------------

#[define(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // .....
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    println!("Value is {}", value_in_cents(coin));
}

fn value_in_cents(coin: &Coin) -> u8 {
    // match me saare cases likhne pdte hain
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(UsState: Alaska) => {
            println!("Hello from Alaska");
            25
        }
        Coin::Quarter(state) => {
            println!("Got a Quarter of value {:?}", state);
            25
        },
    }
}

// ------------------------------------------------------

#[define(Debug)]

fn main() {
    println!("Add result {}", add(50, Some(90)));
    println!("Add result {}", add(50, None));
}

fn add(num1: i32, num2: Option<i32>) -> i32 {
    match num2 {
        Some(i) => num1 + i,
        None => 0,
    }
}

// ------------------------------------------------------

// Catch All

fn main() {
    let dice_roll = 9;

    // 3 -> Fancy Hat
    // 6 -> Remove the Fancy Hat
    // Else -> Move the Dice

    match dice_roll {
        3 => println!("You got a fancy hat"),
        6 => println!("Your fancy hat is removed"),
        // other => println!("Move {} steps", other),
        _ => println!("Move {dice_roll} steps"),
        // _ => (),       // Unit value
    };
}