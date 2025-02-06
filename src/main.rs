use actix_web::{App, HttpServer, middleware::Logger};
use env_logger::Env;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(routes::index)
            .service(routes::post)
    })
    .bind(("0.0.0.0", 5000))?
    .run()
    .await
}
