use crate::auth::jwt::Claims;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

use super::jwt_guard::JwtAuth;

pub struct RoleAuth {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RoleAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Use the JwtAuth guard to first validate the JWT
        if let Outcome::Success(jwt_auth) = JwtAuth::from_request(request).await {
            // Check if the user has the "admin" role
            if jwt_auth.claims.role == "admin" {
                return Outcome::Success(RoleAuth {
                    claims: jwt_auth.claims,
                });
            }
        }

        // If the role is not "admin", return Forbidden
        Outcome::Error((Status::Forbidden, ()))
    }
}
