use crate::models::*;
use crate::models::user::User;
use crate::{Db, Result};
use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Resource {
    pub id: i64,
    group_id: i64,
    name: String,
}

pub async fn get_resource(user: &User, db: &mut Connection<Db>, id: i64) -> Result<Resource> {
    let resource = sqlx::query_as!(
        Resource,
        "SELECT id, group_id, name
        FROM resources WHERE id = ? AND group_id
        IN (SELECT id FROM groups WHERE owner_id = ?)",
        id,
        user.id
    )
    .fetch_one(&mut **db)
    .await?;
    Ok(resource)
}

