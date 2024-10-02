use crate::auth::jwt::{verify_jwt, Claims};
use rocket::http::{CookieJar, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

pub struct JwtAuth {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtAuth {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies: &CookieJar<'_> = request.cookies();

        // Retrieve the JWT token from the "token" cookie
        if let Some(cookie) = cookies.get("auth_token") {
            let token = cookie.value();

            // Verify the JWT and extract claims
            if let Ok(decoded) = verify_jwt(token) {
                return Outcome::Success(JwtAuth {
                    claims: decoded.claims,
                });
            }
        }

        // If token is missing or invalid, return Unauthorized
        Outcome::Error((Status::Unauthorized, ()))
    }
}
