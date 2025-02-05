use diesel::prelude::*;
use serde::{Serialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Link {
    pub link_id: i32,
    pub created_at: chrono::NaiveDate,
    pub source: String,
    pub alias: String
}