use actix_web::HttpResponse;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use self::schema::links::dsl::*;


pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn fetch_link(connection: &mut PgConnection, id: &String) -> Option<models::Link> {
    let results = links
        .filter(alias.eq(id))
        .load::<models::Link>(connection)
        .expect("Failed to load links");

    if results.len() <= 0 {
        return None
    }

    return Some(results.into_iter().nth(0).unwrap())
}

pub fn get_link(id: String) -> HttpResponse {
    let connection = &mut establish_connection();

    let results: Option<models::Link> = fetch_link(connection, &id);
    if results.is_none() {
        return HttpResponse::NotFound()
            .body("Resource not found")
    }

    HttpResponse::Ok()
        .json(results)
}

pub fn create_link(new_post: models::NewLink) -> HttpResponse {
    let connection = &mut establish_connection();

    let results = fetch_link(connection, &new_post.alias);

    if results.is_some() {
        return HttpResponse::BadRequest()
            .body("Alias already exists!")
    }

    diesel::insert_into(links)
        .values(&new_post)
        .returning(models::Link::as_returning())
        .get_result(connection)
        .expect("Error saving new post");

    HttpResponse::Ok()
        .json(new_post)
}