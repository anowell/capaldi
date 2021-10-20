use crate::models::*;
use crate::{Db, Result};
use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection};
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Project {
    id: i64,
    name: String,
    category_id: i64,
    is_closed: bool,
    jira: Option<String>,
    release: Option<String>,
}

pub async fn get_projects(db: &mut Connection<Db>, show_closed: bool) -> Result<Vec<Project>> {
    let projects = sqlx::query_as!(
        Project,
        "SELECT * FROM projects WHERE is_closed = ?",
        show_closed
    )
    .fetch_all(&mut **db)
    .await?;
    Ok(projects)
}

async fn get_project(db: &mut Connection<Db>, id: i32) -> Result<Project> {
    let project = sqlx::query_as!(Project, "SELECT * FROM projects WHERE id = ?", id)
        .fetch_one(&mut **db)
        .await?;
    Ok(project)
}

// #[rocket::post("/projects")]
// // repopulate project list (active and closed)
// fn sync_projects() -> Json<Project> {
//     todo!("implement update_projects");
// }
