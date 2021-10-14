use crate::models::*;
use crate::{Result, Db};
use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection};
use serde::{Serialize, Deserialize};


// #[rocket::get("/projects?<only_active>")]
// async fn list_projects(db: Connection<Db>, only_active: Option<bool>) -> Result<Json<Vec<Project>>> {
//     let only_active = only_active.unwrap_or(true);
//     let projects: Vec<Project> = db.run(move |conn| {
//         use crate::schema::projects;
//         if only_active {
//             projects::table
//                 .filter(projects::is_closed.eq(false))
//                 .load::<Project>(conn)
//         } else {
//             projects::table.load::<Project>(conn)
//         }
//     }).await?;
//     Ok(Json(projects))
// }

// #[rocket::get("/projects/<id>")]
// async fn get_project(db: Db, id: i32) -> Result<Json<Project>> {
//     let project: Project = db.run(move |conn| {
//         use crate::schema::projects;
//         projects::table
//             .filter(projects::id.eq(id))
//             .first::<Project>(conn)
//     }).await?;
//     Ok(Json(project))
// }

// #[rocket::post("/projects")]
// // repopulate project list (active and closed)
// fn sync_projects() -> Json<Project> {
//     todo!("implement update_projects");
// }