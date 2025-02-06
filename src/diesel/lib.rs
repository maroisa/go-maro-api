use actix_web::{HttpResponse, web::Form};

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_link(id: String) -> HttpResponse {
    use self::schema::links::dsl::*;

    let connection = &mut establish_connection();
    let results = links
        .filter(alias.eq(id))
        .load::<models::Link>(connection)
        .expect("Failed to load links");

    if results.len() <= 0 {
        return HttpResponse::NotFound()
            .body("Requested resource not found")
    }

    HttpResponse::Ok()
        .json(results)
}

pub fn create_link(source: String, alias: String) -> HttpResponse {
    use self::schema::links;

    let connection = &mut establish_connection();
    let new_post = models::NewLink { source, alias };

    diesel::insert_into(links::table)
        .values(&new_post)
        .returning(models::Link::as_returning())
        .get_result(connection)
        .expect("Error saving new post");

    HttpResponse::Ok()
        .json(new_post)
}