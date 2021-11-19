use crate::models::allocation::{Allocation, ResourceAllocation};
use crate::models::user::User;
use crate::models::*;
use crate::util::NaiveDateForm;
use crate::{Db, Result};
use chrono::{NaiveDate, Utc};
use futures::TryFutureExt;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use std::collections::HashMap;

#[rocket::get("/?<start>&<end>&<all>")]
async fn query_allocations(
    user: User,
    mut db: Connection<Db>,
    start: Option<NaiveDateForm>,
    end: Option<NaiveDateForm>,
    all: Option<bool>,
) -> Result<Json<HashMap<NaiveDate, Vec<Allocation>>>> {
    let allocations = match all.unwrap_or(false) {
        true => allocation::get_user_allocations(&user, &mut db, start.map(|d| d.0), end.map(|d| d.0))
        .await?,
        false => allocation::get_allocations(&mut db, start.map(|d| d.0), end.map(|d| d.0)).await?
    };

    let alloc_map = allocations
        .into_iter()
        .fold(HashMap::new(), |mut map, alloc| {
            if let Ok(date) = alloc.start_date.parse::<NaiveDate>() {
                map.entry(date).or_insert_with(Vec::new).push(alloc);
            } else {
                // TODO: remove after switching to mysql and using NaiveDate deserialization
                log::warn!(
                    "Found invalid allocation date: allocation={}, date={}",
                    alloc.id,
                    alloc.start_date
                );
            }
            map
        });

    Ok(Json(alloc_map))
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![query_allocations]
}
