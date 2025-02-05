use diesel::prelude::*;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {

    let database_url = "postgres://budiman:loremipsum@localhost/go_maro";
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}