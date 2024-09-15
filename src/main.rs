extern crate linkage_lib;

use actix_web::{middleware, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // enable logging
            .wrap(middleware::Logger::default())
            .service(actix_lib_impl::index)
            .service(actix_lib_impl::healthz)
            .service(web::scope("/v0/linkage").configure(actix_lib_impl::scoped_api_config))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

mod actix_lib_impl {
    use actix_web::{web, HttpResponse, Responder, Result};
    use linkage_lib::linkage::*;

    #[actix_web::get("/sep")]
    pub async fn actix_separate(encode: web::Query<Encoded>) -> Result<impl Responder> {
        Ok(web::Json(separate(encode.0)))
    }

    #[actix_web::get("/pair")]
    pub async fn actix_pair(decode: web::Query<Decoded>) -> Result<impl Responder> {
        Ok(web::Json(pair(decode.0)))
    }

    #[actix_web::get("/")]
    pub async fn index() -> HttpResponse {
        HttpResponse::Ok().body("data")
    }

    #[actix_web::get("/healthz")]
    pub async fn healthz() -> HttpResponse {
        HttpResponse::Ok()
            .append_header(("version", "0.0.1"))
            .finish()
    }

    pub fn scoped_api_config(cfg: &mut web::ServiceConfig) {
        cfg.service(actix_pair).service(actix_separate);
    }
}
