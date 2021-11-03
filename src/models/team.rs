use crate::models::user::User;
use crate::models::*;
use crate::models::resource::ResourceView;
use crate::{Db, Result};
use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, sqlx::SqlitePool, Connection};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Team {
    pub id: i64,
    pub owner_id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeamView {
    pub id: i64,
    pub name: String,
    pub resources: Vec<ResourceView>,
}

impl Team {
    pub fn with_resources(self, resources: Vec<ResourceView>) -> TeamView {
        let Team{id, name, ..} = self;
        TeamView { id, name, resources }
    }
}

pub async fn get_user_teams(user: &User, db: &mut Connection<Db>) -> Result<Vec<Team>> {
    let teams = sqlx::query_as!(Team, "SELECT * FROM teams WHERE owner_id = ?", user.id)
        .fetch_all(&mut **db)
        .await?;
    Ok(teams)
}

pub async fn get_team(user: &User, db: &mut Connection<Db>, id: i64) -> Result<Team> {
    let team = sqlx::query_as!(
        Team,
        "SELECT * from teams where id = ? AND owner_id = ?",
        id,
        user.id
    )
    .fetch_one(&mut **db)
    .await?;
    Ok(team)
}
