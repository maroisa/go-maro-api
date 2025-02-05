use go_maro_api::*;
use actix_web::HttpResponse;
use diesel::prelude::*;

pub fn get(id: String) -> HttpResponse {
    use self::schema::links::dsl::*;

    let connection = &mut establish_connection();
    let results = links
        .filter(alias.eq(id))
        .load::<models::Link>(connection)
        .expect("Failed to load links");

    if results.len() <= 0 {
        return HttpResponse::BadRequest()
            .body("Requested resource not found")
    }

    HttpResponse::Ok()
        .json(results)
}