#![allow(unused_assignments)]

use rocket::fairing::{self, AdHoc};
use rocket::serde::{json::Json, Serialize};
use rocket::response::{Debug, status::Created};
use rocket::serde::json::{json, Value};
use rocket_db_pools::{sqlx, Database, sqlx::SqlitePool};

#[derive(Database)]
#[database("sqlite_capaldi")]
pub struct Db(SqlitePool);


pub mod models;
pub mod routes;

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;


#[derive(Serialize)]
struct Health {
    status: String,
}

#[rocket::get("/health")]
fn health() -> Json<Health> {
    Json(Health {
        status: "pass".to_string(),
   })
}

#[rocket::catch(404)]
fn not_found() -> Value {
    json!({
        "error": {
            "code": 404,
            "message": "Resource not found." ,
        }
    })
}

#[rocket::catch(500)]
fn internal_error() -> Value {
    json!({
        "error": {
            "code": 500,
            "message": "Internal server error." ,
        }
    })
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", rocket::routes![health])
        .mount("/session", routes::session::routes())
        .mount("/groups", routes::groups::routes())
        .register("/", rocket::catchers![not_found, internal_error])
}
