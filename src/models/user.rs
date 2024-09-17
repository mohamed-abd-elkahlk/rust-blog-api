use blog_api::date_time_format;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    id: i32,
    username: String,
    email: String,
    password_hash: String,
    #[serde(with = "date_time_format")]
    created_at: NaiveDateTime,
}
