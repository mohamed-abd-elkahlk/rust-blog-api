#[macro_use]
extern crate rocket;

mod auth;
mod db;
mod guards;
mod handlers;
mod models;
mod routes;
use db::{db_conncetion, Db};
use dotenv::dotenv;
use rocket::{Build, Rocket};

#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv().ok();
    let db_pool: Db = db_conncetion().await;
    rocket::build()
        .manage(db_pool)
        .mount("/auth", routes::auth_routes::get_auth_routes())
}
