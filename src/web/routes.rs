use go_maro_api::*;
use go_maro_api::models::NewLink;
use axum::{
    extract::{State, Path, Json, Form},
    response::IntoResponse,
    http::StatusCode
};
use diesel::result::Error;

pub async fn get_link(State(pool): State<PgPool>, Path(alias): Path<String>) -> impl IntoResponse {
    let mut conn = pool.get().expect("Failed to get pool");

    if alias.contains(char::is_whitespace){
        return (StatusCode::BAD_REQUEST, "Requested resource cannot contain any whitespace!").into_response()
    }

    if alias.len() > 32 {
        return (StatusCode::BAD_REQUEST, "Requested Resource's length invalid").into_response()
    }

    let results = db::select_link(&mut conn, &alias.to_lowercase());

    match results {
        Ok(item) => (StatusCode::OK, Json(item)).into_response(),
        Err(Error::NotFound) => (StatusCode::NOT_FOUND, String::from("Resource Not found")).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
pub async fn create_link(State(pool): State<PgPool>, Form(data): Form<NewLink>) -> impl IntoResponse {
    let mut conn = pool.get().expect("Failed to get pool");

    let new_link = NewLink {
        alias: data.alias.to_lowercase(),
        source: data.source
    };

    let validation = db::select_link(&mut conn, &new_link.alias);

    match validation {
        Ok(_) => return (StatusCode::BAD_REQUEST, "Alias already exists!").into_response(),
        Err(_) => ()
    };

    let results = db::insert_link(&mut conn, new_link);
    
    match results {
        Ok(item) => Json(item).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Something Wrong, Can't Proceed").into_response()
    }
}