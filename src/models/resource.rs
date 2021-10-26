use crate::models::*;
use crate::models::user::User;
use crate::{Db, Result};
use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Resource {
    pub id: i64,
    pub group_id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewResource {
    pub name: String,
    pub role_id: i64,
    pub is_fte: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceView {
    pub id: i64,
    pub name: String,
    // role: String
    // is_fte: bool,
}

pub async fn create_resource(db: &mut Connection<Db>, group_id: i64, resource: NewResource) -> Result<()> {
    let _ = sqlx::query!(
        "INSERT INTO resources (group_id, name, role_id, is_fte)
        VALUES(?, ?, ?, ?)",
        group_id,
        resource.name,
        resource.role_id,
        resource.is_fte,
    )
    .execute(&mut **db)
    .await?;
    Ok(())
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

pub async fn get_resources(user: &User, db: &mut Connection<Db>) -> Result<Vec<Resource>> {
    // TODO: https://github.com/launchbadge/sqlx/issues/875
    // Could take group_id array and avoid the subquery
    let resources = sqlx::query_as!(
        Resource,
        "SELECT id, group_id, name
        FROM resources WHERE group_id
        IN (SELECT id FROM groups WHERE owner_id = ?)",
        user.id
    )
    .fetch_all(&mut **db)
    .await?;
    Ok(resources)
}