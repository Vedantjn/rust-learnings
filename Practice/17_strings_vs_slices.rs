// String Slices -> We want a 'view' into the original string and not copy it over

fn main() {
    let word = String::from("Hello World");
    // let word2 = &word;
    let word2 = &word[0..5];
    // you can have multiple immutable references, if you have a mutable reference, 
    // then you cannot have other mutable/immutable references

    // word.clear(); 

    println!("{}", word2);
}

// --------------------------------------------------------------------------------

fn main() {
    let mut word = String::from("Hello World");
    let word2 = find_first_word(&word);
    println!("{}", word);
    println!("{}", word2);
}

fn find_first_word(word: &String) -> &str {
    let mut index = 0;
    for(_, i) in word.chars().enumerate() {
        if i == ' ' {
            break;
        }
        index = index + 1;
    }
    
    return &word[0..index];
}

// --------------------------------------------------------------------------------

// String Literal
// stored in the final binary
fn main() {
    let word = "Hello";
    
}

// --------------------------------------------------------------------------------

fn main() {
    let arr = [1, 2, 3];
    println!("{}", arr);
    
    let arr_slice = &arr[0..1];
    println!("{}", arr_slice);
}