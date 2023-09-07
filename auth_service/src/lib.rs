mod auth_utils;
mod database;

pub use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    } else {
        println!("Could not connect to database");
    }
}
