use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();
    // let mut users: HashMap<String, u32> = HashMap::new();

    users.insert(String::from("Vedant"), 22);
    users.insert(String::from("Samyak"), 34);


    // Get the age of the user "Vedant"
    // if let Some(&age) = users.get("Vedant") {
    //     println!("{}", age);
    // } else {
    //     println!("User not found");

    let first_user_age = users.get("Vedant");
    
    match first_user_age{
        Some(age) => println!("{}", age),
        None => println!("User not found"),
    }
}

// --------------------------------------------------------------

use std::collections::HashMap;

fn main() {
    let input_vec = vec![(String::from("Vedant"), 22), (String::from("Samyak"), 34)];
    let hm = group_values_by_keys(input_vec);

    println!("{:?}", hm);
}

fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();

    for (key, value) in vec {
        hm.insert(key, value);
    }

    hm
}
