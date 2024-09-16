fn main() {
    create_string();
}

fn create_string() {
    let s1 = String::from("Vedant");
    // let s2 = s1.clone();
    let s2 = s1;

    println!("{}", s1);
    println!("{}", s2);
}

// -------------------------------------------------------------

fn main() {
    let s1 = String::from("Vedant");
    do_something(s1.clone());
    do_something(s1); // Pass ownership of s1 to do_something

    // println!("{}", s1); // This will cause a compile-time error because s1 is moved
}

fn do_something(s2: String) {
    println!("{}", s2);
}

// -------------------------------------------------------------


fn main() {
    let mut s1 = String::from("Vedant");
    s1 = do_something(s1);

    println!("{}", s1); 
}

fn do_something(s2: String) -> String{
    println!("{}", s2);
    return s2;
}
