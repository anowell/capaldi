use crate::models::aggregate::ProjectAggregate;
use crate::models::user::User;
use crate::models::*;
use crate::util::NaiveDateForm;
use crate::{Db, Result};
use chrono::{NaiveDate, Utc};
use futures::TryFutureExt;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;

// todo: add week grouping: weekly, biweekly (2wk), 4weeks, quarterly, and yearly
#[rocket::get("/projects?<start>&<end>")]
async fn project_aggregates(
    _user: User,
    mut db: Connection<Db>,
    start: Option<NaiveDateForm>,
    end: Option<NaiveDateForm>,
) -> Result<Json<Vec<ProjectAggregate>>> {
    aggregate::get_project_aggregates(&mut db, start.map(|d| d.0), end.map(|d| d.0))
        .map_ok(Json)
        .await
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![project_aggregates]
}
