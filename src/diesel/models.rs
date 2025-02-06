use diesel::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Link {
    pub link_id: i32,
    pub created_at: chrono::NaiveDate,
    pub source: String,
    pub alias: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::links)]
pub struct NewLink {
    pub source: String,
    pub alias: String
}