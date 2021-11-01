use crate::models::allocation::Allocation;
use crate::models::group::{Group, GroupView};
use crate::models::resource::{ResourceView, NewResource};
use crate::models::user::User;
use crate::models::*;
use crate::util::NaiveDateForm;
use crate::{Db, Result};
use futures::TryFutureExt;
use itertools::Itertools;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};

#[rocket::get("/")]
async fn list_groups(user: User, mut db: Connection<Db>) -> Result<Json<Vec<GroupView>>> {
    let groups = group::get_user_groups(&user, &mut db).await?;
    let resources = resource::get_resources(&user, &mut db).await?;

    let ref mut map = resources
        .into_iter()
        .map(|r| {
            (
                r.group_id,
                ResourceView {
                    id: r.id,
                    name: r.name,
                },
            )
        })
        .into_group_map();
    let groups_view = groups
        .into_iter()
        .map(|g| { let id = g.id; g.with_resources(map.remove(&id).unwrap_or_else(Vec::new))})
        .collect();

    Ok(Json(groups_view))
}

#[rocket::get("/<id>")]
async fn get_group(user: User, mut db: Connection<Db>, id: i64) -> Result<Json<Group>> {
    group::get_group(&user, &mut db, id).map_ok(Json).await
}

#[rocket::post("/<id>/resources", data = "<data>")]
async fn post_group_resource(user: User, mut db: Connection<Db>, id: i64, data: Json<NewResource>) -> Result<()> {
    // Ensure group is owned by user -- todo: FORWARD or 404 instead of returning an error
    let _ = group::get_group(&user, &mut db, id).await?;

    resource::create_resource(&mut db, id, data.into_inner()).await
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
    rocket::routes![list_groups, get_group, get_group_allocations, post_group_resource]
}
