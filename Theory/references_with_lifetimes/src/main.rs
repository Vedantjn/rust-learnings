// 3 Rules of lifetimes

// 1. A function with one parameter gets one lifetime parameter and a function with multiple parameters gets separate lifetime parameters for each parameter
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters

// -------------------------------------------------------------------------------

fn main() {
    // 'x' is declared without a value, its lifetime starts here
    let x;

    {
        // 'y' is created with a value, its lifetime starts here
        let y = 10;
        // 'x' borrows a reference to 'y', tying its lifetime to 'y'
        x = &y;
    } // 'y' goes out of scope here, ending its lifetime

    // Attempting to use 'x' here would be an error, as its lifetime ended with 'y'
    println!("{}", x);
}

// -------------------------------------------------------------------------------

fn main() {
    // 's1' is created, its lifetime starts here and extends to the end of main
    let s1 = String::from("abcd");
    
    {
        // 's2' is created, its lifetime starts here and ends at the end of this block
        let s2 = String::from("xyz");
        // 'result' borrows from both 's1' and 's2', its lifetime is limited by the shorter of the two
        let result = longest(s1.as_str(), s2.as_str());
        // 'result' can be used here, as both 's1' and 's2' are still alive
    } // 's2' and 'result' go out of scope here, ending their lifetimes

    // Using 'result' here would be an error, as its lifetime has ended
    println!("The longest string is {}", result);
} // 's1' goes out of scope here, ending its lifetime

// 'a is a generic lifetime parameter
// This function's return value will live at least as long as both input parameters
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// -------------------------------------------------------------------------------

// Example of static lifetime

// A static variable with 'static lifetime
static HELLO: &str = "Hello, world!";

fn main() {
    // Using the static variable
    println!("Static string: {}", HELLO);

    // A function that returns a reference with 'static lifetime
    fn returns_static_str() -> &'static str {
        "I have a static lifetime"
    }

    let static_str = returns_static_str();
    println!("String with static lifetime: {}", static_str);

    // A reference to a string literal also has 'static lifetime
    let another_static: &'static str = "I'm also static";
    println!("Another static string: {}", another_static);
}
