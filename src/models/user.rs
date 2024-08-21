// user.rs

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Debug, Deserialize, Serialize)]
pub struct User {
    pub uuid: String,
    pub name: String,
}

impl User {
    pub fn new(uuid: String, name: String) -> User {
        User { uuid, name }
    }
}
