use crate::user::user::User;
use getset::Getters;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;


#[derive(Serialize, Deserialize, Clone, Getters, PartialEq, Eq, Debug)]
pub struct UserDto {
    #[getset(get = "pub with_prefix")]
    user_id: String,
    #[getset(get = "pub with_prefix")]
    last_name: String,
    #[getset(get = "pub with_prefix")]
    first_name: String,
}

impl UserDto {
    pub fn new(source: User) -> Self {
        Self {
            user_id: source.user_id.try_into().unwrap(),
            last_name: source.last_name,
            first_name: source.first_name,
        }
    }
}