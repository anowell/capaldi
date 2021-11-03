use crate::models::*;
use crate::models::user::User;
use crate::{Db, Result};
use rocket_db_pools::{sqlx, Connection};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Resource {
    pub id: i64,
    pub team_id: i64,
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

pub async fn create_resource(db: &mut Connection<Db>, team_id: i64, resource: NewResource) -> Result<()> {
    let _ = sqlx::query!(
        "INSERT INTO resources (team_id, name, role_id, is_fte)
        VALUES(?, ?, ?, ?)",
        team_id,
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
        "SELECT id, team_id, name
        FROM resources WHERE id = ? AND team_id
        IN (SELECT id FROM teams WHERE owner_id = ?)",
        id,
        user.id
    )
    .fetch_one(&mut **db)
    .await?;
    Ok(resource)
}

pub async fn get_resources(user: &User, db: &mut Connection<Db>) -> Result<Vec<Resource>> {
    // TODO: https://github.com/launchbadge/sqlx/issues/875
    // Could take team_id array and avoid the subquery
    let resources = sqlx::query_as!(
        Resource,
        "SELECT id, team_id, name
        FROM resources WHERE team_id
        IN (SELECT id FROM teams WHERE owner_id = ?)",
        user.id
    )
    .fetch_all(&mut **db)
    .await?;
    Ok(resources)
}