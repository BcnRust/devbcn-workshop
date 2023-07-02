mod integration {

    use actix_web::{http::StatusCode, App};
    use api_lib::health::{service, API_VERSION};

    #[actix_rt::test]
    async fn health_check_works() {
        let app = App::new().configure(service);
        let mut app = actix_web::test::init_service(app).await;
        let req = actix_web::test::TestRequest::get()
            .uri("/health")
            .to_request();
        let res = actix_web::test::call_service(&mut app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);
        let data = res
            .headers()
            .get("health-check")
            .and_then(|h| h.to_str().ok());
        assert_eq!(data, Some(API_VERSION));
    }
}
