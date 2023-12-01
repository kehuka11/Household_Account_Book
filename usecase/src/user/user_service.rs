use anyhow::Result;
use std::convert::TryFrom;
use domain::user::user_id::UserId;
use domain::user::user_repository::UserRepository;
use domain::user::dto::user_dto::UserDto;

/// DI
pub struct UserService<T>
where
   T: UserRepository,
   {
    user_repository: T,
   }

/// アプリケーションサービスの振る舞いを定義
impl<T: UserRepository> UserService<T> {
    /// コンストラクタ
    pub fn new(user_repository: T) -> Self {
        Self {user_repository}
    }

    pub fn handle(&self, user_id: String) -> Result<UserDto> {
        let user_id = UserId::try_from(user_id).unwrap();
        match self.user_repository.find_by_user_id(&user_id) {
            Ok(value) => Ok(UserDto::new(value)),
            Err(_) => Err(anyhow::anyhow!(
                "ユーザが存在しません: UserId{:?}",
                user_id
            )),
        }
    }

}