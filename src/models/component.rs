use crate::models::*;
use crate::{Db, Result};
use rocket_db_pools::{sqlx, Connection};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Component {
    id: i64,
    name: String,
}

pub async fn get_components(db: &mut Connection<Db>) -> Result<Vec<Component>> {
    let projects = sqlx::query_as!(
        Component,
        "SELECT * FROM components",
    )
    .fetch_all(&mut **db)
    .await?;
    Ok(projects)
}

