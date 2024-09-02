use actix_web::{web, App, HttpServer};

mod link;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(crate::link::linkage::index)
            .service(web::scope("/v0").configure(crate::link::linkage::scoped_config))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
