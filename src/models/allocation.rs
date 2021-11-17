use crate::models::{user::User, *};
use crate::{Db, Result};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use rocket_db_pools::{sqlx, Connection};
use serde::{Deserialize, Serialize};
use sqlx::Connection as _;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Allocation {
    pub id: i64,
    pub start_date: String, // NaiveDate ???
    pub resource_id: i64,
    pub project_id: i64,
    pub component_id: i64,
    pub percent: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceAllocation {
    pub id: i64,
    pub project_id: i64,
    pub component_id: i64,
    pub percent: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewResourceAllocation {
    pub project_id: i64,
    pub component_id: i64,
    pub percent: i64,
}

pub async fn get_user_allocations(
    user: &User,
    db: &mut Connection<Db>,
    start_at: Option<NaiveDate>,
) -> Result<Vec<Allocation>> {
    let start_at = start_at.unwrap_or(Utc::today().naive_local());
    log::info!("Query allocations for user={}, start_date: {}", user.id, start_at);
    let allocations = sqlx::query_as!(
        Allocation,
        "SELECT *
            FROM allocations WHERE resource_id IN
            (SELECT id FROM resources WHERE team_id IN (SELECT id FROM teams WHERE owner_id = ?))
            AND start_date >= date(?)
            AND start_date < date(?, '+14 days')",
        user.id,
        start_at,
        start_at
    )
    .fetch_all(&mut **db)
    .await?;

    Ok(allocations)
}

pub async fn get_team_allocations(
    db: &mut Connection<Db>,
    id: i64,
    start_at: Option<NaiveDate>,
) -> Result<Vec<Allocation>> {
    let start_at = start_at.unwrap_or(Utc::today().naive_local());
    let allocations = sqlx::query_as!(
        Allocation,
        "SELECT *
            FROM allocations WHERE resource_id IN
            (SELECT id FROM resources WHERE team_id = ?)
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

pub async fn delete_resource_allocations(
    db: &mut Connection<Db>,
    id: i64,
    start_at: NaiveDate,
) -> Result<()> {
    sqlx::query!(
        "DELETE FROM allocations WHERE resource_id = ? AND start_date = date(?)",
        id,
        start_at
    )
    .execute(&mut **db)
    .await?;

    Ok(())
}