#[macro_use]
extern crate rocket;

mod db;
mod models;

use db::{db_conncetion, Db};
use dotenv::dotenv;
use rocket::{Build, Rocket};

#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv().ok();
    let db_pool: Db = db_conncetion().await;
    rocket::build().manage(db_pool)
}
