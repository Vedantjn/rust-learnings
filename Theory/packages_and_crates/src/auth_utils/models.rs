pub fn login(cred: models::Credentials) {
    // try to login the user
    // super::database::get_user();
    crate::database::get_user();

}

pub mod models {
    pub struct Credentials {
        pub username: String,
        pub password: String,
    }
}