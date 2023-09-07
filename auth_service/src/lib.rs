use rand::prelude::*;

mod auth_utils;
mod database;

pub use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials) {
    let timeout = rand::thread_rng().gen_range(100..500);
    println!("Timeout is: {}", timeout);
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    } else {
        println!("Could not connect to database");
    }
}
