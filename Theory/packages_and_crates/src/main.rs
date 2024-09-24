// src/main.rs -> Root Binary Crate 
// src/lib.rs -> Root Library Crate 

// use packages_and_crates::Credentials;
// use packages_and_crates::authenticate;
// use packages_and_crates::{authenticate, Credentials};
use packages_and_crates::auth_utils::models::Credentials;

fn main() {
    let cred = Credentials {
        username: String::from("Vedant Jain"),
        password: String::from("admin123"),
    };

    // packages_and_crates::authenticate(cred);
    authenticate(cred);
}
