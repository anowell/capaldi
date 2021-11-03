use crate::models::allocation::Allocation;
use crate::models::resource::{NewResource, ResourceView};
use crate::models::team::{Team, TeamView};
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
async fn list_teams(user: User, mut db: Connection<Db>) -> Result<Json<Vec<TeamView>>> {
    let teams = team::get_user_teams(&user, &mut db).await?;
    let resources = resource::get_resources(&user, &mut db).await?;

    let ref mut map = resources
        .into_iter()
        .map(|r| {
            (
                r.team_id,
                ResourceView {
                    id: r.id,
                    name: r.name,
                },
            )
        })
        .into_group_map();
    let teams_view = teams
        .into_iter()
        .map(|g| {
            let id = g.id;
            g.with_resources(map.remove(&id).unwrap_or_else(Vec::new))
        })
        .collect();

    Ok(Json(teams_view))
}

#[rocket::get("/<id>")]
async fn get_team(user: User, mut db: Connection<Db>, id: i64) -> Result<Json<Team>> {
    team::get_team(&user, &mut db, id).map_ok(Json).await
}

#[rocket::post("/<id>/resources", data = "<data>")]
async fn post_team_resource(
    user: User,
    mut db: Connection<Db>,
    id: i64,
    data: Json<NewResource>,
) -> Result<()> {
    // Ensure team is owned by user -- todo: FORWARD or 404 instead of returning an error
    let _ = team::get_team(&user, &mut db, id).await?;

    resource::create_resource(&mut db, id, data.into_inner()).await
}

#[rocket::get("/<id>/allocations?<from>")]
async fn get_team_allocations(
    user: User,
    mut db: Connection<Db>,
    id: i64,
    from: Option<NaiveDateForm>,
) -> Result<Json<Vec<Allocation>>> {
    // Ensure team is owned by user -- todo: FORWARD or 404 instead of returning an error
    let _ = team::get_team(&user, &mut db, id).await?;

    allocation::get_team_allocations(&mut db, id, from.map(|d| d.0))
        .map_ok(Json)
        .await
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![
        list_teams,
        get_team,
        get_team_allocations,
        post_team_resource
    ]
}
