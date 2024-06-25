use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Deserialize,sqlx::FromRow,Debug)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize,Serialize,Default,Clone)]
pub struct Identity{
    pub username: String,
    pub privilege: String,
}