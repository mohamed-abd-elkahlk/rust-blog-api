use blog_api::timestamp_to_datetime;
use chrono::{DateTime, Utc};
use rocket::{http::Status, response::status, serde::json::Json};

use crate::{
    db::Db,
    guards::jwt_guard::JwtAuth,
    models::{
        error::ResponseError,
        post::{NewPost, Post, UpdatedPost},
    },
};

// create post
#[post("/post", data = "<new_post>")]
pub async fn create_post(
    db_pool: &rocket::State<Db>,
    user: JwtAuth,
    new_post: Json<NewPost>,
) -> Result<Json<Post>, status::Custom<Json<ResponseError>>> {
    let query = sqlx::query!(
        "INSERT INTO posts (author_id, title, body) VALUES (?, ? ,?)",
        user.claims.sub.parse::<i64>().unwrap(),
        new_post.title,
        new_post.body
    )
    .execute(db_pool.inner())
    .await
    .map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            Json(ResponseError {
                error: "Database Error".to_string(),
            }),
        )
    })?;

    let post = Post {
        id: query.last_insert_id() as i32,
        author_id: user.claims.sub.parse().expect("fiald to parse author id"),
        body: new_post.body.clone(),
        title: new_post.title.clone(),
        created_at: Utc::now(),
    };
    Ok(Json(post))
}

// read post
#[get("/post/<id>")]
pub async fn get_post(
    db_pool: &rocket::State<Db>,
    user: JwtAuth,
    id: i64,
) -> Result<Json<Post>, status::Custom<Json<ResponseError>>> {
    let record = sqlx::query!(
        "SELECT * FROM posts WHERE id = ? and author_id = ?",
        id,
        user.claims.sub.parse::<i64>().unwrap()
    )
    .fetch_one(db_pool.inner())
    .await
    .map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            Json(ResponseError {
                error: "Database Error".to_string(),
            }),
        )
    })?;
    let created_at = timestamp_to_datetime!(record).unwrap();
    let post = Post {
        id: record.id as i32,
        author_id: record.author_id,
        body: record.body,
        title: record.title,
        created_at,
    };
    Ok(Json(post))
}

// update post
#[put("/post/<id>", data = "<post_data>")]
pub async fn update_post(
    db_pool: &rocket::State<Db>,
    user: JwtAuth,
    id: i64,
    post_data: Json<UpdatedPost>, // Post data may contain None for optional fields
) -> Result<Json<Post>, status::Custom<Json<ResponseError>>> {
    // Fetch the post to ensure it exists and belongs to the current user
    let record = sqlx::query!(
        "SELECT * FROM posts WHERE id = ? AND author_id = ?",
        id,
        user.claims.sub.parse::<i64>().unwrap()
    )
    .fetch_one(db_pool.inner())
    .await
    .map_err(|_| {
        status::Custom(
            Status::NotFound,
            Json(ResponseError {
                error: "Post not found or you're not authorized to update it.".to_string(),
            }),
        )
    })?;

    // Prepare dynamic update query depending on which fields are present
    let mut title = record.title;
    let mut body = record.body;

    // Update only if the new values are provided
    if let Some(ref new_title) = post_data.title {
        title = new_title.clone();
    }
    if let Some(ref new_body) = post_data.body {
        body = new_body.clone();
    }

    // Perform the update query
    sqlx::query!(
        "UPDATE posts SET title = ?, body = ? WHERE id = ? AND author_id = ?",
        title,
        body,
        id,
        user.claims.sub.parse::<i64>().unwrap()
    )
    .execute(db_pool.inner())
    .await
    .map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            Json(ResponseError {
                error: "Failed to update post".to_string(),
            }),
        )
    })?;
    let created_at = timestamp_to_datetime!(record).unwrap();
    // Return the updated post
    let updated_post = Post {
        id: record.id as i32,
        author_id: record.author_id,
        title,
        body,
        created_at,
    };

    Ok(Json(updated_post))
}

#[delete("/post/<id>")]
pub async fn delete_post(
    db_pool: &rocket::State<Db>,
    user: JwtAuth,
    id: i64,
) -> Result<Json<String>, status::Custom<Json<ResponseError>>> {
    // First, let's check if the user is authorized to delete the post
    let post_owner_id = sqlx::query_scalar!("SELECT author_id FROM posts WHERE id = ?", id)
        .fetch_optional(&**db_pool)
        .await
        .map_err(|_| {
            status::Custom(
                Status::InternalServerError,
                Json(ResponseError {
                    error: "Error checking post owner".to_string(),
                }),
            )
        })?;

    // If the post doesn't exist, return a 404 error
    if post_owner_id.is_none() {
        return Err(status::Custom(
            Status::NotFound,
            Json(ResponseError {
                error: "Post not found".to_string(),
            }),
        ));
    }

    let post_owner_id = post_owner_id.unwrap();

    // Check if the user is either an admin or the owner of the post
    if user
        .claims
        .sub
        .parse::<i32>()
        .expect("faild to parse user id")
        != post_owner_id
        && user.claims.role == "admin"
    {
        return Err(status::Custom(
            Status::Forbidden,
            Json(ResponseError {
                error: "You do not have permission to delete this post".to_string(),
            }),
        ));
    }

    // Proceed with deleting the post
    sqlx::query!("DELETE FROM posts WHERE id = ?", id)
        .execute(&**db_pool)
        .await
        .map_err(|_| {
            status::Custom(
                Status::InternalServerError,
                Json(ResponseError {
                    error: "Error deleting the post".to_string(),
                }),
            )
        })?;

    // Return success message
    Ok(Json("Post successfully deleted".to_string()))
}
