use std::env;

use sqlx::{MySql, Pool};

pub type Db = Pool<MySql>;

pub async fn db_conncetion() -> Db {
    let database_url = env::var("DATABASE_URL").expect("no Database url provided!");
    let db_pool = sqlx::MySqlPool::connect(&database_url).await.unwrap();
    db_pool
}
