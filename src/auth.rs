use crate::models::user::User;
use crate::{Db, Result};
use log::debug;
use rocket::http::{Cookie, CookieJar};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use rocket::serde::json::{self, Json};
use rocket_db_pools::{sqlx, Connection};

// see jwt at: https://github.com/TatriX/realworld-rust-rocket/blob/master/src/auth.rs

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<User, Self::Error> {
        get_auth_user(request).or_forward(())
    }
}

fn get_auth_user<'r>(request: &'r Request<'_>) -> Option<User> {
    request.cookies().get_private("user").and_then(|cookie| {
        match json::from_str::<User>(cookie.value()) {
            Ok(u) => Some(u),
            Err(e) => {
                debug!("Error parsing user cookie: {}. {}", e, cookie.value());
                None
            }
        }
    })
}
