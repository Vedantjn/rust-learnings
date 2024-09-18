fn longest(a: String, b: String) -> String {
    if a.len() > b.len() {
        a
    }
    else {
        b
    }
}

fn main() {
    let longest_string;
    let a = String::from("small");
    let b = String::from("larger");
    longest_string = longest(a, b);
    println!("{}", longest_string);
}

// ---------------------------------------------------

fn longest(a: String, b: String) -> String {
    if a.len() > b.len() {
        a
    }
    else {
        b
    }
}

fn main() {
    let longest_string;
    let a = String::from("small");
    {
        let b = String::from("larger");
        longest_string = longest(a, b);
    }
    println!("{}", longest_string);
}

// ---------------------------------------------------

// Now, lifetimes come into picture, the above part was nothing to do with lifetimes

// This will give error
fn longest(a: &str, b: &str) -> &str {
    if a.len() > b.len() {
        a
    }
    else {
        b
    }
}

fn main() {
    let longest_string;
    let a = String::from("small");
    {
        let b = String::from("larger");
        longest_string = longest(&a, &b); // the life of 'longest_string' variable will be the intersection of a and b
    }
    // longest_string will be a dangling pointer
    // b goes out of scope here

    println!("{}", longest_string);
}

// ---------------------------------------------------

// Generic Lifetime Annotation

// return type ka lifetime -> intersection of lifetime of str1 and str2
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str { 
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn main() {
    let str1 = String::from("small");
    let longest_string;
    {
        let str2 = String::from("larger");
        longest_string = longest(&str1, &str2); 
    }  // 'str2' is dropped here

    // `longest_string` might refer to `str2`, which is already dropped.
    println!("{}", longest_string); // Compilation error
}

// Now, this code will fail with a better error message