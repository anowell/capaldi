use crate::models::{user::User, *};
use crate::{Db, Result};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use rocket_db_pools::{sqlx, Connection};
use serde::{Deserialize, Serialize};
use sqlx::Connection as _;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Allocation {
    pub id: i64,
    pub start_date: NaiveDateTime,
    pub resource_id: i64,
    pub project_id: i64,
    pub component_id: i64,
    pub percent: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewResourceAllocation {
    pub project_id: i64,
    pub component_id: i64,
    pub percent: i64,
}

pub async fn get_group_allocations(
    db: &mut Connection<Db>,
    id: i64,
    start_at: Option<NaiveDate>,
) -> Result<Vec<Allocation>> {
    let start_at = start_at.unwrap_or(Utc::today().naive_local());
    let allocations = sqlx::query_as!(
        Allocation,
        "SELECT *
            FROM allocations WHERE resource_id IN
            (SELECT id FROM resources WHERE group_id = ?)
            AND start_date >= date(?)
            AND start_date < date(?, '+14 days')",
        id,
        start_at,
        start_at
    )
    .fetch_all(&mut **db)
    .await?;

    Ok(allocations)
}

pub async fn put_resource_allocations(
    db: &mut Connection<Db>,
    id: i64,
    start_at: NaiveDate,
    allocations: Vec<NewResourceAllocation>,
) -> Result<()> {

    let mut conn = db.begin().await?;

    sqlx::query!(
        "DELETE FROM allocations WHERE resource_id = ? AND start_date = date(?)",
        id,
        start_at
    )
    .execute(&mut conn)
    .await?;
    for alloc in allocations {
        sqlx::query!(
            "INSERT INTO
                allocations(resource_id, start_date, project_id, component_id, percent)
                VALUES(?, ?, ?, ?, ?)",
            id,
            start_at,
            alloc.project_id,
            alloc.component_id,
            alloc.percent
        )
        .execute(&mut conn)
        .await?;
    }

    conn.commit().await?;
    Ok(())
}
