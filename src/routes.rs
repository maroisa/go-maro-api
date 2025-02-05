use actix_web::{get, web, Responder, HttpResponse};

use crate::db;

#[get("/{id}")]
async fn index(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner().to_lowercase();

    if id.len() > 10 {
        return HttpResponse::BadRequest()
            .body("Request Invalid")
    }

    if id.contains(char::is_whitespace){
        return HttpResponse::BadRequest()
            .body("Request Invalid")
    }

    db::get(id)
}