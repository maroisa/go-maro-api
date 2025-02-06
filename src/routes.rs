use actix_web::{web, get, post, Responder, HttpResponse};
use go_maro_api::models::NewLink;

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

    go_maro_api::get_link(id)
}

#[post("/")]
async fn post(req: web::Form<NewLink>) -> impl Responder {
    if req.source.contains(char::is_whitespace) || req.alias.contains(char::is_whitespace) {
        return HttpResponse::BadRequest()
            .body("Body contains Whitespace!")
    }

    let new_post = NewLink { 
        source: req.source.to_lowercase(), 
        alias: req.alias.to_lowercase() 
    };

    go_maro_api::create_link(new_post)
}