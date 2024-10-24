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
      .service(actix_lib_impl::code)
      .service(web::scope("/v0/linkage").configure(actix_lib_impl::scoped_api_config))
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await
}

mod actix_lib_impl {
  use actix_web::{http::StatusCode, web, HttpResponse, Responder, Result};
  use linkage_lib::linkage::*;
  use serde::{Deserialize, Serialize};

  #[derive(Deserialize, Serialize, Debug)]
  struct Message {
    pub msg: String,
  }

  #[actix_web::get("/sep")]
  pub async fn actix_separate(encode: web::Query<Encoded>) -> Result<impl Responder> {
    Ok(web::Json(separate(encode.0)))
  }

  #[actix_web::get("/pair")]
  pub async fn actix_pair(decode: web::Query<Decoded>) -> Result<impl Responder> {
    Ok(web::Json(pair_dec(decode.0)))
  }

  #[actix_web::get("/")]
  pub async fn index() -> Result<impl Responder> {
    Ok(web::Json(Message {
      msg: "data".to_string(),
    }))
  }

  #[actix_web::get("/healthz")]
  pub async fn healthz() -> HttpResponse {
    HttpResponse::Ok()
      .append_header(("version", "0.0.1"))
      .json(Message {
        msg: "ok".to_string(),
      })
  }

  #[actix_web::get("/code")]
  pub async fn code() -> HttpResponse {
    HttpResponse::Ok()
      .status(StatusCode::ALREADY_REPORTED)
      .finish()
  }

  pub fn scoped_api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(actix_pair).service(actix_separate);
  }
}
