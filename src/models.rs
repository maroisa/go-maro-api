use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

#[derive(Selectable, Queryable, Deserialize, Serialize, Clone)]
#[diesel(table_name = crate::schema::links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Link {
    pub id: i32,
    pub created_at: NaiveDate,
    pub source: String,
    pub alias: String
}

#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::links)]
pub struct NewLink {
    pub source: String,
    pub alias: String
}