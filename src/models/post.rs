use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
struct Post {
    id: i32,
    author_id: i32,
    title: String,
    body: String,
    pub created_at: DateTime<Utc>,
}
