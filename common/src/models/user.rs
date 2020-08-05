use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use super::Time;

#[derive(Default, FromRow, Serialize, Deserialize, Clone)]
#[serde(rename_all="camelCase")]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub email: String,
    pub username: String,
    pub password: String,
    #[serde(default = "Time::now")]
    pub created_at: i32,
}
