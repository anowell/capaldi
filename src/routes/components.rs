use crate::models::{component::Component, *};
use crate::{Db, Result};
use futures::TryFutureExt;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;

#[rocket::get("/")]
async fn list_components(
    mut db: Connection<Db>,
) -> Result<Json<Vec<Component>>> {
    component::get_components(&mut db)
        .map_ok(Json)
        .await
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![list_components]
}
