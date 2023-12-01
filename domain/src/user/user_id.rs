use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{self, write};
use serde::Deserialize;

// 独自のエラータイプを定義
#[derive(Debug)]
struct UserIdError(String);

impl fmt::Display for UserIdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UserIdError:{}", self.0)
    }
}

impl Error for UserIdError {}

// UserId構造体の定義
#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Debug)]
pub struct UserId(String);

impl TryFrom<String> for UserId {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() || value.len() > 12 {
            Err(Box::new(UserIdError("Invalid UserId".to_string())))
        } else {
            Ok(UserId(value.to_string()))
        }
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UserId({})", self.0)
    }
}

impl From<UserId> for String {
    fn from(user_id: UserId) -> Self {
        user_id.to_string()
    }
}