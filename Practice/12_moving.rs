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