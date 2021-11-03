#![allow(unused_assignments, unused_imports)]

use rocket::fs::{relative, FileServer};
use rocket::serde::json::{json, Value};
use rocket::serde::{json::Json, Serialize};
use rocket_db_pools::{sqlx, sqlx::SqlitePool, Database};
use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

#[derive(Database)]
#[database("sqlite_capaldi")]
pub struct Db(SqlitePool);

pub mod auth;
pub mod models;
pub mod routes;
pub mod util;

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

#[rocket::get("/<_file..>", rank = 11)]
async fn spa_fallback(_file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(relative!("frontend/public/index.html"))).await.ok()
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", rocket::routes![health])
        .mount("/api/session", routes::session::routes())
        .mount("/api/teams", routes::teams::routes())
        .mount("/api/allocations", routes::allocations::routes())
        .mount("/api/projects", routes::projects::routes())
        .mount("/api/resources", routes::resources::routes())
        .mount("/api/components", routes::components::routes())
        .mount("/", FileServer::from(relative!("frontend/public")))
        .mount("/", rocket::routes![spa_fallback])
        .register("/", rocket::catchers![not_found, internal_error])
}
