mod routes;

use go_maro_api::get_pool;
use axum::{Router, routing};

#[tokio::main]
async fn main() {
    let pool = get_pool();

    let app = Router::new()
        .route("/{alias}", routing::get(routes::get_link))
        .route("/", routing::post(routes::create_link))
        .with_state(pool.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}