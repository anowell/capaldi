use crate::models::user::User;
use crate::{Db, Result};
use log::warn;
use rocket::http::{Cookie, CookieJar};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use rocket::serde::json::{self, Json};
use rocket_db_pools::{sqlx, Connection};

#[rocket::post("/")]
async fn post_login(jar: &CookieJar<'_>, mut db: Connection<Db>) -> Result<Json<User>> {
    // if login.email == "anowell@gmail.com" && login.password == "password" {
    let user = sqlx::query_as!(User, "SELECT id, email FROM users WHERE id = ?", 1)
        .fetch_one(&mut *db)
        .await?;
    jar.add_private(Cookie::new(
        "user",
        serde_json::to_string(&user).expect("failed to serialize user"),
    ));
    Ok(Json(user))
    // } else {
    //     Err(Flash::error(Redirect::to(uri!(login_page)), "Invalid username/password."))
    // }
}

#[rocket::delete("/")]
fn logout(jar: &CookieJar<'_>) {
    jar.remove_private(Cookie::named("user"));
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![post_login, logout]
}
