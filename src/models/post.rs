use blog_api::date_time_format;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
struct Post {
    id: i32,
    author_id: i32,
    title: String,
    body: String,
    #[serde(with = "date_time_format")]
    created_at: NaiveDateTime,
}
