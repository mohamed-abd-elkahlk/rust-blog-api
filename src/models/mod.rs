use serde::Serialize;

pub mod comment;
pub mod error;
pub mod post;
pub mod user;
#[derive(Serialize)]
pub struct PagedResponse<T> {
    pub data: Vec<T>,
    pub total_pages: i64,
    pub total_items: i64,
    pub current_page: i64,
    pub page_size: i64,
}
