use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

const API_VERSION: &str = "v0.0.1";

pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("health-check", API_VERSION))
        .finish()
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, App};

    use super::*;

    #[actix_rt::test]
    async fn health_check_works() {
        let res = health_check().await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);
        let data = res
            .headers()
            .get("health-check")
            .map(|h| h.to_str().ok())
            .flatten();
        assert_eq!(data, Some(API_VERSION));
    }

    #[actix_rt::test]
    async fn health_check_integration_works() {
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
            .map(|h| h.to_str().ok())
            .flatten();
        assert_eq!(data, Some(API_VERSION));
    }
}
