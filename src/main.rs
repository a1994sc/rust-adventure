use actix_web::{middleware, web, App, HttpServer};

mod link;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // enable logging
            .wrap(middleware::Logger::default())
            .service(crate::link::linkage::index)
            .service(web::scope("/v0").configure(crate::link::linkage::scoped_config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
