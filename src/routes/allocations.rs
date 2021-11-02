use crate::util::NaiveDateForm;
use crate::models::*;
use crate::models::allocation::{Allocation, ResourceAllocation};
use crate::models::user::User;
use crate::{Db, Result};
use futures::TryFutureExt;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use std::collections::HashMap;
use chrono::{NaiveDate, Utc};


#[rocket::get("/?<from>")]
async fn all_allocations(
    user: User,
    mut db: Connection<Db>,
    from: Option<NaiveDateForm>,
) -> Result<Json<HashMap<NaiveDate, HashMap<String, Vec<ResourceAllocation>>>>> {
    let allocations = allocation::get_user_allocations(&user, &mut db, from.map(|d| d.0)).await?;

    let alloc_map = allocations.into_iter().fold(HashMap::new(), |mut map, alloc| {
        let resource_alloc = ResourceAllocation {
            id: alloc.id,
            project_id: alloc.project_id,
            component_id: alloc.component_id,
            percent: alloc.percent,
        };
        if let Ok(date) = alloc.start_date.parse::<NaiveDate>() {
            let date_entry = map.entry(date).or_insert_with(HashMap::new);
            let res_entry = date_entry
                .entry(alloc.resource_id.to_string())
                .or_insert_with(Vec::new);
            res_entry.push(resource_alloc);
        } else {
            // TODO: remove after switching to mysql and using NaiveDate deserialization
            log::warn!("Found invalid allocation date: allocation={}, date={}", alloc.id, alloc.start_date);
        }
        map
    });

    Ok(Json(alloc_map))
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![all_allocations]
}
