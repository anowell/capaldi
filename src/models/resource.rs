use crate::models::*;
use crate::{Db, Result};
use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Resource {
    pub id: i64,
    group_id: i64,
    name: String,
    role_id: i64,
    is_fte: bool,
    // is_deleted: bool,
}

// #[rocket::get("/resources")]
// /// Lists all resources in users groups
// fn list_resources() -> Json<Vec<Resource>> {
//     todo!("implement list_resources");
// }

// #[rocket::get("/resources/<id>")]
// fn get_resource(id: i32) -> Json<Resource> {
//     todo!("implement get resource");
// }

// #[rocket::get("/resources/<id>/allocations?<start_at>")]
// fn list_resource_allocations(id: i32, start_at: Option<String>) -> Json<Vec<Allocation>> {
//     todo!("implement list_resources");
// }
