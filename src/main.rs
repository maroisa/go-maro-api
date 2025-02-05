use actix_web::{App, HttpServer, middleware::Logger};
use env_logger::Env;

mod routes;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(routes::index)
    })
    .bind(("127.0.0.1", 5433))?
    .run()
    .await
}
