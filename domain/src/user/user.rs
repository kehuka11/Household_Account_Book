use serde::Deserialize;
use crate::user::user_id::UserId;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct User{
    pub user_id: UserId,
    pub last_name: String,
    pub first_name: String,
}

impl User {
    pub fn new(user_id: UserId, last_name: String, first_name: String) -> Self {

        Self { 
            user_id, 
            last_name, 
            first_name, 
        }
    }
}

