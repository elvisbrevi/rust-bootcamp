use example_brevi_auth_service::Credentials;

fn main() {
    let creds = Credentials {
        username: String::from("username"),
        password: String::from("password"),
    };

    example_brevi_auth_service::authenticate(creds);
}
