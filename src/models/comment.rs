use blog_api::date_time_format;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Comment {
    id: i32,
    post_id: i32,
    author_id: i32,
    body: String,
    #[serde(with = "date_time_format")]
    created_at: NaiveDateTime,
}
