use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Deserialize, Serialize)]

pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: Option<String>,
    pub created_at: DateTime<Utc>,
    pub role: String,
}

#[derive(Deserialize)]
pub struct UserCredential {
    pub email: String,
    pub password: String,
}
