use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use rocket::http::{Cookie, CookieJar};
use crate::models::User;
use crate::{Result, Db};
use rocket::serde::json::{self, Json};
use log::debug;
use rocket_db_pools::{sqlx, Connection};

// see jwt at: https://github.com/TatriX/realworld-rust-rocket/blob/master/src/auth.rs

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<User, Self::Error> {
        request.cookies()
            .get_private("user")
            .and_then(|cookie| {
                match json::from_str::<User>(cookie.value()) {
                    Ok(u) => Some(u),
                    Err(e) => {
                        debug!("Error parsing user cookie: {}. {}", e, cookie.value());
                        None
                    }
                }
            })
            .or_forward(())
    }
}