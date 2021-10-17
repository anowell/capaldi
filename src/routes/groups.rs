use crate::models::*;
use crate::models::{allocation::Allocation, group::Group, user::User};
use crate::util::NaiveDateForm;
use crate::{Db, Result};
use futures::TryFutureExt;
use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection};
use serde::{Deserialize, Serialize};

#[rocket::get("/")]
async fn list_groups(user: User, mut db: Connection<Db>) -> Result<Json<Vec<Group>>> {
    group::get_user_groups(&user, &mut db).map_ok(Json).await
}

#[rocket::get("/<id>")]
async fn get_group(user: User, mut db: Connection<Db>, id: i64) -> Result<Json<Group>> {
    group::get_group(&user, &mut db, id).map_ok(Json).await
}

#[rocket::get("/<id>/allocations?<from>")]
async fn get_group_allocations(
    user: User,
    mut db: Connection<Db>,
    id: i64,
    from: Option<NaiveDateForm>,
) -> Result<Json<Vec<Allocation>>> {
    // Ensure group is owned by user -- todo: FORWARD or 404 instead of returning an error
    let _ = group::get_group(&user, &mut db, id).await?;

    allocation::get_group_allocations(&mut db, id, from.map(|d| d.0))
        .map_ok(Json)
        .await
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![list_groups, get_group, get_group_allocations]
}
