use crate::models::{project::Project, *};
use crate::{Db, Result};
use futures::TryFutureExt;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;

#[rocket::get("/?<show_closed>")]
async fn list_projects(
    mut db: Connection<Db>,
    show_closed: Option<bool>,
) -> Result<Json<Vec<Project>>> {
    let show_closed = show_closed.unwrap_or(false);
    project::get_projects(&mut db, show_closed)
        .map_ok(Json)
        .await
}

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

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![list_projects]
}
