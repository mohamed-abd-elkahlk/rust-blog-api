use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: i64,
    pub post_id: i64,
    pub author_id: i64,
    pub body: String,
    pub created_at: DateTime<Utc>,
}
#[derive(Deserialize)]
pub struct CommentBody {
    pub id: i64,
    pub body: String,
}
