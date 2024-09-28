use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize)]

pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize)]

pub struct UpdatedPost {
    pub title: Option<String>,
    pub body: Option<String>,
}

#[derive(FromRow, Serialize, Deserialize)]

pub struct Post {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
}
