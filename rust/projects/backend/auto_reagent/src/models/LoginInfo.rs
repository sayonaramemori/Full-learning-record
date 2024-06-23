use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}