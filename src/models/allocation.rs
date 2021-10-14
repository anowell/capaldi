use crate::models::*;
use crate::{Result, Db};
use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Connection};
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Allocation {
    id: i64,
    start_date: NaiveDate,
    resource_id: i64,
    project_id: i64,
    percent: i64,
}