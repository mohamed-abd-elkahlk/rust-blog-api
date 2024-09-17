use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Comment {
    id: i32,
    post_id: i32,
    author_id: i32,
    body: String,
    pub created_at: DateTime<Utc>,
}
