fn main() {
    let s1 = String::from("Vedant");
    do_something(&s1);

    println!("{}", s1);
}

fn do_something(s2: &String) {
    println!("{}", s2);
}

// -------------------------------------------------

fn main() {
    let s1 = String::from("Vedant");
    let s2 = &s1;

    println!("{}", s1);
    println!("{}", s2);
}
// -------------------------------------------------

// This is wrong since we already have one mutable reference, we can't have another reference after it 
fn main() {
    let mut s1 = String::from("Vedant");
    let s2 = &mut s1;
    let s3 = &s1;

    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
}

// -------------------------------------------------

fn main() {
    let mut s1 = String::from("Vedant");
    do_something(&mut s1);
    println!("{}", s1);
}

fn do_something(s2: &mut String) {
    s2.push_str(" Jain");
    println!("{}", s2);
}

// -------------------------------------------------