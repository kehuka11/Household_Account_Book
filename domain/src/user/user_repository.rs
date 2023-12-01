use anyhow::Result;
use crate::user::user_id::UserId;
use crate::user::user::User;

pub trait UserRepository {

    /// ユーザIDからユーザを検索する
    fn find_by_user_id(&self, id: &UserId) -> Result<User>;
}