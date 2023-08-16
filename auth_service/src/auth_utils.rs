pub fn login(credentials: models::Credentials) {
    crate::database::get_user();
}

fn logout() {
    // log user out...
}

pub mod models;