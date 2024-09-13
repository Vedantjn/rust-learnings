const X: u8 = 100;

fn main() {
    println!("Hello, world!");
    let age = 21;
    println!("Value of age is {age}");
    // age = 22;
    println!("Value of age is {age}");

    const PI: u8 = 10;

    const AGE: u32 = 32;

    // const THREE_HOUR_IN_SECONDS: u32 = 3*60*60 + age;
    const THREE_HOUR_IN_SECONDS: u32 = 3*60*60 + AGE;

    // Shadowing
    let apples = 10;
    println!("apples {apples}");
    let apples = true;
    println!("apples {apples}");


    let x = 5;
    let x = x + 1;
    {
        let x = x*2;
        println!("The value of x in inner scope is {x}");
    }
    
    println!("The value of x in inner scope is {x}");


    
}