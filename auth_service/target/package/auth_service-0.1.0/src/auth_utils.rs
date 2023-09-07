pub fn login(creds: models::Credentials) {
    // authenticate user
    crate::database::get_user();
}

fn logout() {
    // logout user
}

pub mod models;
