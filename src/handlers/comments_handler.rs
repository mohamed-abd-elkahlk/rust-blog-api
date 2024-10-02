use crate::{
    db::Db,
    guards::jwt_guard::JwtAuth,
    models::{
        comment::{Comment, CommentBody},
        error::ResponseError,
    },
};
use blog_api::timestamp_to_datetime;
use chrono::{DateTime, Utc};
use rocket::{http::Status, response::status, serde::json::Json};

#[get("/comment/<post_id>")]
pub async fn get_comment(
    db_pool: &rocket::State<Db>,
    post_id: i64,
) -> Result<Json<Vec<Comment>>, status::Custom<Json<ResponseError>>> {
    let query = sqlx::query!("SELECT * FROM comments WHERE post_id = ?", post_id)
        .fetch_all(db_pool.inner())
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => status::Custom(
                Status::NotFound,
                Json(ResponseError {
                    error: "Comment not found".to_string(),
                }),
            ),
            _ => status::Custom(
                Status::InternalServerError,
                Json(ResponseError {
                    error: "Database Error".to_string(),
                }),
            ),
        })?;

    let comments: Vec<Comment> = query
        .iter()
        .map(|row| {
            let created_at = timestamp_to_datetime!(row).expect("faild to parse datatime");
            Comment {
                id: row.id as i64,
                author_id: row.author_id as i64,
                post_id: row.post_id as i64,
                body: row.body.clone(),
                created_at,
            }
        })
        .collect();

    Ok(Json(comments))
}

#[post("/comment/<post_id>", data = "<comment_body>")]
pub async fn create_comment(
    db_pool: &rocket::State<Db>,
    user: JwtAuth,
    post_id: i64,
    comment_body: Json<CommentBody>,
) -> Result<Json<Comment>, status::Custom<Json<ResponseError>>> {
    sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM posts WHERE id = ?)", post_id)
        .fetch_one(db_pool.inner())
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => status::Custom(
                Status::NotFound,
                Json(ResponseError {
                    error: "Post not found".to_string(),
                }),
            ),
            _ => status::Custom(
                Status::InternalServerError,
                Json(ResponseError {
                    error: "Database Error".to_string(),
                }),
            ),
        })?;

    let author_id = user.claims.sub.parse::<i64>().unwrap();
    // Insert the new comment if the post exists
    let result = sqlx::query!(
        "INSERT INTO comments (post_id, author_id, body) VALUES (?, ?, ?)",
        post_id,
        author_id,
        comment_body.body
    )
    .execute(db_pool.inner())
    .await
    .map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            Json(ResponseError {
                error: "Failed to create comment".to_string(),
            }),
        )
    })?;
    let comment = Comment {
        id: result.last_insert_id() as i64,
        author_id,
        post_id,
        body: comment_body.body.clone(),
        created_at: Utc::now(),
    };
    // Return a success message
    Ok(Json(comment))
}
#[put("/post/<post_id>/comment/<comment_id>", data = "<comment>")]
pub async fn update_comment(
    db_pool: &rocket::State<Db>,
    user: JwtAuth,
    post_id: i64,
    comment_id: i64,
    comment: Json<CommentBody>,
) -> Result<Json<String>, status::Custom<Json<ResponseError>>> {
    // Check if the post exists
    let post_exists =
        sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM posts WHERE id = ?)", post_id)
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

    // Convert the result to a boolean
    if post_exists == 0 {
        return Err(status::Custom(
            Status::NotFound,
            Json(ResponseError {
                error: "Post not found".to_string(),
            }),
        ));
    }

    // Extract author ID from JWT claims
    let author_id = user.claims.sub.parse::<i64>().unwrap();

    // Update the comment for the given comment_id and author_id
    let result = sqlx::query!(
        "UPDATE comments SET body = ? WHERE id = ? AND author_id = ?",
        comment.body,
        comment_id,
        author_id
    )
    .execute(db_pool.inner())
    .await
    .map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            Json(ResponseError {
                error: "Failed to update comment".to_string(),
            }),
        )
    })?;

    // Check if the update affected any rows (i.e., if the comment existed and was updated)
    if result.rows_affected() == 0 {
        return Err(status::Custom(
            Status::NotFound,
            Json(ResponseError {
                error: "Comment not found or you're not authorized to update it.".to_string(),
            }),
        ));
    }

    Ok(Json("Comment updated.".to_string()))
}

#[delete("/comment/<comment_id>")]
pub async fn delete_comment(
    db_pool: &rocket::State<Db>,
    user: JwtAuth,
    comment_id: i64,
) -> Result<status::Custom<()>, status::Custom<Json<ResponseError>>> {
    let author_id = user.claims.sub.parse::<i64>().unwrap();
    let result = sqlx::query!(
        "DELETE FROM comments WHERE id = ? AND author_id = ? ",
        comment_id,
        author_id
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
    if result.rows_affected() == 0 {
        return Err(status::Custom(
            Status::NotFound,
            Json(ResponseError {
                error: "Comment not found or you're not authorized to delete it.".to_string(),
            }),
        ));
    }

    Ok(status::Custom(Status::NoContent, ()))
}

// struct Comment {
//     id: i32,
//     post_id: i32,
//     author_id: i32,
//     body: String,
//     pub created_at: DateTime<Utc>,
// }
