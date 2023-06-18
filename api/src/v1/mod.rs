use actix_web::web::{self, ServiceConfig};

use crate::film_repository::FilmRepository;

mod films;

pub fn service<R: FilmRepository>(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/v1").configure(films::service::<R>));
}
