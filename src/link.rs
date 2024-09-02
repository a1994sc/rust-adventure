pub mod linkage {
    use actix_web::{web, HttpResponse, Responder, Result};
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
        use super::*;
        use actix_web::{
            body::to_bytes,
            dev::Service,
            http::{self, header::ContentType},
            test, App,
        };

        #[actix_web::test]
        async fn test_index_get() {
            let app = test::init_service(App::new().service(index)).await;
            let req = test::TestRequest::default()
                .insert_header(ContentType::plaintext())
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_success());
        }

        #[actix_web::test]
        async fn test_linker_api_sep() {
            let response = [
                r#"{"a":0,"b":0}"#,
                r#"{"a":1,"b":0}"#,
                r#"{"a":0,"b":1}"#,
                r#"{"a":2,"b":0}"#,
                r#"{"a":1,"b":1}"#,
            ];

            let app =
                test::init_service(App::new().service(web::scope("/v0").configure(scoped_config)))
                    .await;

            for (i, el) in response.iter().enumerate() {
                let req = test::TestRequest::get()
                    .uri(&format!("/v0/sep?id={:?}", i))
                    .to_request();
                let resp = app.call(req).await.unwrap();

                assert_eq!(resp.status(), http::StatusCode::OK);
                let body_bytes = to_bytes(resp.into_body()).await.unwrap();
                assert_eq!(body_bytes, el);
            }
        }

        #[actix_web::test]
        async fn test_linker_api_pair() {
            let response = [
                [
                    r#"{"id":0}"#,
                    r#"{"id":2}"#,
                    r#"{"id":5}"#,
                    r#"{"id":9}"#,
                    r#"{"id":14}"#,
                ],
                [
                    r#"{"id":1}"#,
                    r#"{"id":4}"#,
                    r#"{"id":8}"#,
                    r#"{"id":13}"#,
                    r#"{"id":19}"#,
                ],
                [
                    r#"{"id":3}"#,
                    r#"{"id":7}"#,
                    r#"{"id":12}"#,
                    r#"{"id":18}"#,
                    r#"{"id":25}"#,
                ],
                [
                    r#"{"id":6}"#,
                    r#"{"id":11}"#,
                    r#"{"id":17}"#,
                    r#"{"id":24}"#,
                    r#"{"id":32}"#,
                ],
                [
                    r#"{"id":10}"#,
                    r#"{"id":16}"#,
                    r#"{"id":23}"#,
                    r#"{"id":31}"#,
                    r#"{"id":40}"#,
                ],
            ];

            let app =
                test::init_service(App::new().service(web::scope("/v0").configure(scoped_config)))
                    .await;

            for a in 0..5 {
                for b in 0..5 {
                    let req = test::TestRequest::get()
                        .uri(&format!("/v0/pair?a={:?}&b={:?}", a, b))
                        .to_request();
                    let resp = app.call(req).await.unwrap();

                    assert_eq!(resp.status(), http::StatusCode::OK);
                    let body_bytes = to_bytes(resp.into_body()).await.unwrap();
                    assert_eq!(body_bytes, response[a][b]);
                }
            }
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
