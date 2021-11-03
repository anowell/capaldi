use crate::models::*;
use crate::models::{allocation::Allocation, allocation::NewResourceAllocation, team::Team, user::User};
use crate::util::NaiveDateForm;
use crate::{Db, Result};
use futures::TryFutureExt;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};

// #[rocket::get("/<id>")]
// async fn get_resource(user: User, mut db: Connection<Db>) -> Result<Json<Resource>> {
//     team::get_user_teams(&user, &mut db).map_ok(Json).await
// }

#[rocket::put("/<id>/allocations/<date>", data = "<data>")]
async fn put_allocations(user: User, mut db: Connection<Db>, id: i64, date:NaiveDateForm, data: Json<Vec<NewResourceAllocation>>) -> Result<()> {
    // Ensure resource is owned by user -- todo: FORWARD or 404 instead of returning an error
    let _ = resource::get_resource(&user, &mut db, id).await?;

    allocation::put_resource_allocations(&mut db, id, date.0, data.into_inner()).await
}


pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![put_allocations]
}
