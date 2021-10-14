use crate::models::*;
use crate::{Result, Db};
use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection};
use serde::{Serialize, Deserialize};
use crate::models::{user::User, group::Group, allocation::Allocation};
use futures::TryFutureExt;

#[rocket::get("/groups")]
async fn list_groups(user: User, mut db: Connection<Db>) -> Result<Json<Vec<Group>>> {
    group::query_user_groups(&user, &mut db).map_ok(Json).await
}

// #[rocket::get("/groups")]
// async fn list_groups(user: User, mut db: Connection<Db>) -> Result<Json<Vec<Group>>> {
//     let groups = sqlx::query_as!(Group, "SELECT * FROM groups WHERE owner_id = ?", user.id)
//         .fetch_all(&mut *db)
//         .await?;
//     Ok(Json(groups))
// }

#[rocket::get("/groups/<id>")]
async fn get_group(user: User, mut db: Connection<Db>, id: i64) -> Result<Json<Group>> {
    let group = sqlx::query_as!(Group, "SELECT * from groups where id = ? AND owner_id = ?", id, user.id)
        .fetch_one(&mut *db)
        .await?;
    Ok(Json(group))
}

#[rocket::get("/groups/<id>/allocations?<start_at>")]
async fn get_group_allocations(user: User, mut db: Connection<Db>, id: i64, start_at:Option<String>) -> Result<Json<Vec<Allocation>>> {
    // consider implementing FromForm on a NaiveDateForm
    // e.g. https://stackoverflow.com/questions/55029850/how-to-use-a-date-in-the-url-with-rocket-rs

    // let group = get_group(user, db, id).await?;
    
    todo!("implement get_group_allocations");
}


pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![list_groups, get_group, get_group_allocations]
}