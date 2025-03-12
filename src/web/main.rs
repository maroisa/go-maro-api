use go_maro_api::get_pool;
use axum::{Router, routing};
use axum::http::Method;
use tower_http::{
    trace::{TraceLayer, DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse},
    cors::{CorsLayer, Any}
};

mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    
    let pool = get_pool();

    let app = Router::new()
        .route("/{alias}", routing::get(routes::get_link))
        .route("/", routing::post(routes::create_link))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(
                    DefaultOnRequest::new().level(tracing::Level::DEBUG)
                )
                .on_response(
                    DefaultOnResponse::new().level(tracing::Level::DEBUG)
                )
        )
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::PUT])
                .allow_origin(Any)
        )
        .with_state(pool.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}