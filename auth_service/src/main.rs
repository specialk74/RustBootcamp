use auth_service::Credentials;
fn main () {
    let creds = Credentials {
        username: String::from("username"),
        password: String::from("password"),
    };

    auth_service::authenticate(creds);
}