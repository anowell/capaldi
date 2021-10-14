use crate::models::*;
use crate::{Result, Db};
use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection, sqlx::SqlitePool};
use serde::{Serialize, Deserialize};
use crate::models::user::User;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Group {
    pub id: i64,
    pub owner_id: i64,
    pub name: String,
}

pub async fn query_user_groups(user: &User, db: &mut Connection<Db>) -> Result<Vec<Group>> {
    let groups = sqlx::query_as!(Group, "SELECT * FROM groups WHERE owner_id = ?", user.id)
        .fetch_all(&mut **db)
        .await?;
    Ok(groups)
}

