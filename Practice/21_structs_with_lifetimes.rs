struct User {
    name: &str,
}

fn main() {
    let username = String::from("Vedant");
    let user = User {
        name: &username
    };

    println!("Name -> {}", user.name);
}

// -------------------------------------------------

// Lifetime in struct

struct User<'a> {
    name: &'a str
}

fn main() {
    let username = String::from("Vedant");
    let user = User {
        name: &username
    };

    println!("Name -> {}", user.name);
}

// -------------------------------------------------

