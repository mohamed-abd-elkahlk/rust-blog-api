use crate::{
    auth::{
        jwt::generate_jwt,
        password::{hash_password, verify_password},
    },
    db::Db,
    models::{
        error::ResponseError,
        user::{NewUser, User, UserCredential},
    },
};
use blog_api::timestamp_to_datetime;
use chrono::{DateTime, Utc};
use rocket::{
    http::{Cookie, CookieJar, Status},
    response::status,
    serde::json::Json,
};

#[post("/sign-in", data = "<user_credential>")]
pub async fn sign_in(
    db_pool: &rocket::State<Db>,
    cookie: &CookieJar<'_>,
    user_credential: Json<UserCredential>,
) -> Result<Json<User>, status::Custom<Json<ResponseError>>> {
    // check if user exits
    let result: bool = sqlx::query!(
        "SELECT EXISTS (SELECT 1 FROM users WHERE email = ?) AS user_exists",
        user_credential.email
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
    })?
    .user_exists
        != 0;
    // Handel error if user does not exists
    if !result {
        return Err(status::Custom(
            Status::Unauthorized,
            Json(ResponseError {
                error: "Invalid email or password".to_string(),
            }),
        ));
    }
    // get user data
    let record = sqlx::query!("SELECT * FROM users WHERE email = ?", user_credential.email)
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

    let created_at: DateTime<Utc> = timestamp_to_datetime!(record).expect("faild to parse date");
    let user = User {
        created_at: created_at,
        email: record.email,
        password: None,
        username: record.username,
        id: record.id,
        role: record.role,
    };
    if verify_password(
        &user_credential.password,
        &record.password.expect("faild to parse password"),
    )
    .is_err()
    {
        return Err(status::Custom(
            Status::Unauthorized,
            Json(ResponseError {
                error: "Invalid email or password".to_string(),
            }),
        ));
    }
    let token = generate_jwt(&record.id.to_string(), &user.role).map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            Json(ResponseError {
                error: "faild to issue jwt token".to_string(),
            }),
        )
    })?;
    cookie.add(Cookie::build(("auth_token", token)).http_only(true));
    Ok(Json(user))
}

#[post("/sign-up", data = "<user_data>")]
pub async fn sign_up(
    db_pool: &rocket::State<Db>,
    cookie: &CookieJar<'_>,
    user_data: Json<NewUser>,
) -> Result<Json<User>, status::Custom<Json<ResponseError>>> {
    let password_hash = hash_password(&user_data.password);

    let query = sqlx::query!(
        "INSERT INTO users (username,email,password) VALUES (? ,? ,?)",
        user_data.username,
        user_data.email,
        password_hash
    )
    .execute(db_pool.inner())
    .await
    .map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            Json(ResponseError {
                error: "Fiald to insert user into Database".to_string(),
            }),
        )
    })?;

    let record = sqlx::query!(
        "SELECT id, role FROM users where id = ?",
        query.last_insert_id()
    )
    .fetch_one(db_pool.inner())
    .await
    .map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            Json(ResponseError {
                error: "Fiald to insert user into Database".to_string(),
            }),
        )
    })?;
    let user = User {
        id: record.id as i32,
        username: user_data.username.clone(),
        email: user_data.email.clone(),
        created_at: Utc::now(),
        role: record.role,
        password: None,
    };
    let token = generate_jwt(&record.id.to_string(), &user.role).map_err(|_| {
        status::Custom(
            Status::InternalServerError,
            Json(ResponseError {
                error: "faild to issue jwt token".to_string(),
            }),
        )
    })?;
    cookie.add(Cookie::build(("auth_token", token)).http_only(true));
    Ok(Json(user))
}
