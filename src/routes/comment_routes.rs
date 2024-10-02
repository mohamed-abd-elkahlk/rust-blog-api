use rocket::Route;

use crate::handlers::comments_handler::{
    create_comment, delete_comment, get_comment, update_comment,
};

pub fn comment_routes() -> Vec<Route> {
    routes![create_comment, delete_comment, get_comment, update_comment]
}
