use diesel::PgConnection;
use diesel::r2d2::{Pool, ConnectionManager};
use dotenv::dotenv;
use std::env;

pub mod db;
pub mod models;
mod schema;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Failed to load environment: DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .test_on_check_out(true)
        .max_size(4)
        .build(manager)
        .expect("Failed to build Connection Pool")
}