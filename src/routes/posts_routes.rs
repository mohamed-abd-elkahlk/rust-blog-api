use rocket::Route;

use crate::handlers::post_handlers::{create_post, delete_post, get_post, get_posts, update_post};

pub fn posts_routes() -> Vec<Route> {
    routes![create_post, delete_post, get_post, update_post, get_posts]
}
