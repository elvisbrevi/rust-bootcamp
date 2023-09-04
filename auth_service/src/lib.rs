struct Credentials {
    username: String,
    password: String,
}

enum Status {
    Connected,
    Interrupted,
}

fn authenticate(creds: Credentials) {
    if let Status::Connected = connect_to_database() {
        login(creds);
    } else {
        println!("Could not connect to database");
    }
}

fn connect_to_database() -> Status {
    return Status::Connected;
}

fn login(creds: Credentials) {
    // authenticate user
    get_user(creds);
}

fn logout() {
    // logout user
}

fn get_user(creds: Credentials) {
    // get user from database
}
