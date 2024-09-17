use std::env;

use sqlx::{MySql, Pool};

pub type Db = Pool<MySql>;

pub async fn db_conncetion() -> Db {
    let database_url = env::var("DATABASE_URL").expect("no Database url provided!");

    let db_pool = sqlx::MySqlPool::connect(&database_url).await.unwrap();

    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS posts (
           id INT  PRIMARY KEY AUTO_INCREMENT,
            author_id INT NOT NULL,
            title TEXT NOT NULL,
            body TEXT NOT NULL,
            created_at DATETIME NOT NULL
        );
        CREATE TABLE IF NOT EXISTS users (
           id INT AUTO_INCREMENT PRIMARY KEY,
            email VARCHAR(255) NOT NULL UNIQUE,
            username VARCHAR(255) NOT NULL,
            password_hash TEXT NOT NULL,
            created_at DATETIME NOT NULL
        );
        CREATE TABLE IF NOT EXISTS comments (
            id INT PRIMARY KEY AUTO_INCREMENT,
            post_id INT NOT NULL,
            author_id INt NOT NULL,
            body TEXT NOT NULL,
            created_at DATETIME NOT NULL,
            FOREIGN KEY(post_id) REFERENCES posts(id) ON DELETE CASCADE,
            FOREIGN KEY(author_id) REFERENCES users(id) ON DELETE CASCADE
        );
        ",
    )
    .execute(&db_pool)
    .await
    .unwrap();
    db_pool
}
