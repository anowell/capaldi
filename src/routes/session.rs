use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use rocket::http::{Cookie, CookieJar};
use crate::models::user::User;
use crate::{Result, Db};
use rocket::serde::json::{self, Json};
use log::debug;
use rocket_db_pools::{sqlx, Connection};



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

#[rocket::post("/login")]
async fn post_login(jar: &CookieJar<'_>, mut db: Connection<Db>) -> Result<Json<User>> {
    // if login.email == "anowell@gmail.com" && login.password == "password" {
        let user = sqlx::query_as!(User, "SELECT id, email FROM users WHERE id = ?", 1)
            .fetch_one(&mut *db)
            .await?;
        jar.add_private(Cookie::new("user", serde_json::to_string(&user).expect("failed to serialize user")));
        Ok(Json(user))
    // } else {
    //     Err(Flash::error(Redirect::to(uri!(login_page)), "Invalid username/password."))
    // }
}

#[rocket::post("/logout")]
fn logout(jar: &CookieJar<'_>) {
    jar.remove_private(Cookie::named("user"));
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![post_login, logout]
}