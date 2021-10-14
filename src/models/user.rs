use chrono::{NaiveDate};
use serde::{Serialize, Deserialize};
use sqlx::types::chrono::NaiveDateTime;


#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
    // pub created_at: NaiveDateTime,
    // is_admin: bool,
    // is_deleted: bool,
    // deleted_at: Option<NaiveDateTime>,
}