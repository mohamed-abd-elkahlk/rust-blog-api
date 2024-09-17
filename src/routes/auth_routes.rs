use rocket::Route;

use crate::handlers::auth_handlers::{sign_in, sign_up};

pub fn get_auth_routes() -> Vec<Route> {
    routes![sign_in, sign_up]
}
