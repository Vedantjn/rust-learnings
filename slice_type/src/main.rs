fn main() {
    let mut s = String::from("Hello World");
    let res = find_first_word(&s);

    // s.clear();

    // println!("For string {s}, the result is {res}");
    
    
    let hello = &s[0..5];
    // let hello = &s[..5];
    let world = &s[6..11];
    // let world = &s[6..];
    // let world = &s[..];
    
    // println!("Hello = {hello} and World = {world}");
    

    println!("For string {s}, the result is {res}");
}

fn find_first_word(input: &String) -> &str {
    let s = input.as_bytes();

    for (i, &item) in s.iter().enumerate() {
        if item == b' ' {
            return &input[..i];
        }
    }

    &input[..]
}