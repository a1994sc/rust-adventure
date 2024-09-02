pub mod linkage {
    use actix_web::{web, Responder, Result, HttpResponse};
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    pub struct Encoded {
        id: u32,
    }

    impl PartialEq for Encoded {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    #[derive(Deserialize, Serialize)]
    pub struct Decoded {
        a: u32,
        b: u32,
    }

    impl PartialEq for Decoded {
        fn eq(&self, other: &Self) -> bool {
            self.a == other.a && self.b == other.b
        }
    }

    #[actix_web::get("/sep")]
    pub async fn separate(encode: web::Query<Encoded>) -> Result<impl Responder> {
        let pairf: f32 = encode.id as f32;
        let w: u32 = (((8.0 * pairf + 1.0).sqrt() - 1.0) / 2.0).floor() as u32;
        let t: u32 = (num::pow(w, 2) + w) / 2;
    
        Ok(web::Json(Decoded {
            a: (w - (encode.id - t)),
            b: (encode.id - t),
        }))
    }

    #[actix_web::get("/pair")]
    pub async fn pair(decode: web::Query<Decoded>) -> Result<impl Responder> {
        let x: u32 = decode.a;
        let y: u32 = decode.b;
        let sum: u32 = (x + y) * (x + y + 1);

        Ok(web::Json(Encoded { id: (sum / 2 + y) }))
    }

    pub fn scoped_config(cfg: &mut web::ServiceConfig) {
        cfg.service(pair).service(separate);
    }

    #[derive(Deserialize, Serialize)]
    pub struct Info {
        username: String,
    }
    
    #[actix_web::get("/")]
    pub async fn index() -> HttpResponse {
      HttpResponse::Ok().body("data")
    }

    #[cfg(test)]
    mod test {
      use actix_web::{http::header::ContentType, test, App};
      use super::*;

      #[actix_web::test]
      async fn test_index_get() {
          let app = test::init_service(App::new().service(index)).await;
          let req = test::TestRequest::default()
              .insert_header(ContentType::plaintext())
              .to_request();
          let resp = test::call_service(&app, req).await;
          assert!(resp.status().is_success());
      }

        // #[test]
        // fn pair() {
        //     super::pair(Decoded {
        //       a: 17,
        //       b: 9,
        //     })
        //     assert_eq!(super::pair(17, 9).into_inner().id, 360);
        //     assert_eq!(super::pair(9, 17).into_inner().id, 368);

        //     assert_eq!(super::pair(17, 9).into_inner(), super::Encoded { id: 360 });
        //     assert_eq!(super::pair(9, 17).into_inner(), super::Encoded { id: 368 });
        // }

        // #[test]
        // fn separate() {
        //     assert_eq!(super::separate(360).into_inner().a, 17);
        //     assert_eq!(super::separate(360).into_inner().b, 9);

        //     assert_eq!(super::separate(368).into_inner().a, 9);
        //     assert_eq!(super::separate(368).into_inner().b, 17);

        //     assert_eq!(
        //         super::separate(360).into_inner(),
        //         super::Decoded { a: 17, b: 9 }
        //     );
        //     assert_eq!(
        //         super::separate(368).into_inner(),
        //         super::Decoded { a: 9, b: 17 }
        //     );
        // }
    }
}
