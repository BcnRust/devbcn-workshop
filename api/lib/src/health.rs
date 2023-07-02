use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

pub const API_VERSION: &str = "v0.0.3";

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
    use actix_web::http::StatusCode;

    use super::*;

    #[actix_rt::test]
    async fn health_check_works() {
        let res = health_check().await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);
        let data = res
            .headers()
            .get("health-check")
            .and_then(|h| h.to_str().ok());
        assert_eq!(data, Some(API_VERSION));
    }
}
