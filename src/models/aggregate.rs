use crate::{Db, Result};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use rocket_db_pools::{sqlx, Connection};
use serde::{Deserialize, Serialize};
use sqlx::Connection as _;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct ProjectAggregate {
    pub start_date: String, // NaiveDate ???
    pub project_id: i64,
    pub resources: f32,
}

pub async fn get_project_aggregates(
    db: &mut Connection<Db>,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
) -> Result<Vec<ProjectAggregate>> {
    let start = start.unwrap_or(Utc::today().naive_local());
    let end = end.unwrap_or_else(|| start.clone());
    log::info!("Query aggregates range: {} to {}", start, end);
    let aggregates = sqlx::query_as!(
        ProjectAggregate,
        r#"SELECT start_date, project_id, sum(percent)/100.0 as "resources!: _"
            FROM allocations
            WHERE start_date >= date(?)
            AND start_date <= date(?)
            GROUP BY start_date, project_id"#,
        start,
        end
    )
    .fetch_all(&mut **db)
    .await?;

    Ok(aggregates)
}