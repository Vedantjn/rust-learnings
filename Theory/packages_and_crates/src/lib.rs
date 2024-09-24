// Auth Service:

#![allow(dead_code, unused_variables)]

mod database;

pub mod auth_utils;

use auth_utils::login;
use database::{connect_to_database, Status};


fn pub authenticate(cred: auth_utils::models::Credentials) {
    if let Status::Connected = connect_to_database() {
        auth_utils::login(cred);
    }
}

pub mod util;
// src/util.rs (More preferred)
// src/util/mod.rs