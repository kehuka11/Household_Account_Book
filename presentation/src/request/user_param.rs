use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserParam {
   pub user_id: String,
}