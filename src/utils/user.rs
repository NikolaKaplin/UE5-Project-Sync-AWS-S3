use serde::{Deserialize, Serialize};
use whoami::username;
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
}

impl User {
    pub fn get_user_name() -> String {
        username().unwrap().to_string()
    }
}