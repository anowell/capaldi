use crate::models::user::User;
use crate::models::*;
use crate::models::resource::ResourceView;
use crate::{Db, Result};
use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, sqlx::SqlitePool, Connection};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    pub id: i64,
    pub owner_id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GroupView {
    pub id: i64,
    pub name: String,
    pub resources: Vec<ResourceView>,
}

impl Group {
    pub fn with_resources(self, resources: Vec<ResourceView>) -> GroupView {
        let Group{id, name, ..} = self;
        GroupView { id, name, resources }
    }
}

pub async fn get_user_groups(user: &User, db: &mut Connection<Db>) -> Result<Vec<Group>> {
    let groups = sqlx::query_as!(Group, "SELECT * FROM groups WHERE owner_id = ?", user.id)
        .fetch_all(&mut **db)
        .await?;
    Ok(groups)
}

pub async fn get_group(user: &User, db: &mut Connection<Db>, id: i64) -> Result<Group> {
    let group = sqlx::query_as!(
        Group,
        "SELECT * from groups where id = ? AND owner_id = ?",
        id,
        user.id
    )
    .fetch_one(&mut **db)
    .await?;
    Ok(group)
}
