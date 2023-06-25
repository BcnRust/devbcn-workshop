use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use shared::models::{CreateFilm, Film};
use uuid::Uuid;

use crate::film_repository::FilmRepository;

pub fn service<R: FilmRepository>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/films")
            // GET
            .route("", web::get().to(get_all::<R>))
            .route("/{film_id}", web::get().to(get::<R>))
            // POST
            .route("", web::post().to(post::<R>))
            // PUT
            .route("", web::put().to(put::<R>))
            // DELETE
            .route("/{film_id}", web::delete().to(delete::<R>)),
    );
}

async fn get_all<R: FilmRepository>(repo: web::Data<R>) -> HttpResponse {
    match repo.get_films().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn get<R: FilmRepository>(film_id: web::Path<Uuid>, repo: web::Data<R>) -> HttpResponse {
    match repo.get_film(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(_) => HttpResponse::NotFound().body("Not found"),
    }
}

async fn post<R: FilmRepository>(
    create_film: web::Json<CreateFilm>,
    repo: web::Data<R>,
) -> HttpResponse {
    match repo.create_film(&create_film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}

async fn put<R: FilmRepository>(film: web::Json<Film>, repo: web::Data<R>) -> HttpResponse {
    match repo.update_film(&film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn delete<R: FilmRepository>(film_id: web::Path<Uuid>, repo: web::Data<R>) -> HttpResponse {
    match repo.delete_film(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::film_repository::MockFilmRepository;
    use actix_web::body::to_bytes;
    use chrono::Utc;

    pub fn create_test_film(id: Uuid, title: String) -> Film {
        Film {
            id,
            title,
            director: "Director test name".to_string(),
            year: 2001,
            poster: "Poster test name".to_string(),
            created_at: Some(Utc::now()),
            updated_at: None,
        }
    }

    #[actix_rt::test]
    async fn get_all_works() {
        let film_id = uuid::Uuid::new_v4();
        let film_title1 = "Film test title1";
        let film_title2 = "Film test title2";

        let mut repo = MockFilmRepository::default();
        repo.expect_get_films().returning(move || {
            let film = create_test_film(film_id, film_title1.to_string());
            let film2 = create_test_film(film_id, film_title2.to_string());
            Ok(vec![film, film2])
        });

        let result = get_all(web::Data::new(repo)).await;

        let body = to_bytes(result.into_body()).await.unwrap();
        let films = serde_json::from_slice::<'_, Vec<Film>>(&body).unwrap();

        assert_eq!(films.len(), 2);
        assert_eq!(films[0].title, film_title1);
        assert_eq!(films[1].title, film_title2);
    }

    #[actix_rt::test]
    async fn get_works() {
        let film_id = uuid::Uuid::new_v4();
        let film_title = "Film test title";

        let mut repo = MockFilmRepository::default();
        repo.expect_get_film().returning(move |id| {
            let film = create_test_film(*id, film_title.to_string());
            Ok(film)
        });

        let result = get(web::Path::from(film_id), web::Data::new(repo)).await;

        let body = to_bytes(result.into_body()).await.unwrap();
        let film = serde_json::from_slice::<'_, Film>(&body).unwrap();

        assert_eq!(film.id, film_id);
        assert_eq!(film.title, film_title);
    }

    #[actix_rt::test]
    async fn create_works() {
        let film_id = uuid::Uuid::new_v4();
        let title = "Film test title";
        let create_film = CreateFilm {
            title: title.to_string(),
            director: "Director test name".to_string(),
            year: 2001,
            poster: "Poster test name".to_string(),
        };

        let mut repo = MockFilmRepository::default();
        repo.expect_create_film().returning(move |create_film| {
            Ok(Film {
                id: film_id,
                title: create_film.title.to_owned(),
                director: create_film.director.to_owned(),
                year: create_film.year,
                poster: create_film.poster.to_owned(),
                created_at: Some(Utc::now()),
                updated_at: None,
            })
        });

        let result = post(web::Json(create_film), web::Data::new(repo)).await;

        let body = to_bytes(result.into_body()).await.unwrap();
        let film = serde_json::from_slice::<'_, Film>(&body).unwrap();

        assert_eq!(film.id, film_id);
        assert_eq!(film.title, title);
    }

    #[actix_rt::test]
    async fn update_works() {
        let film_id = uuid::Uuid::new_v4();
        let film_title = "Film test title";
        let new_film = create_test_film(film_id, film_title.to_string());

        let mut repo = MockFilmRepository::default();
        repo.expect_update_film()
            .returning(|film| Ok(film.to_owned()));

        let result = put(web::Json(new_film), web::Data::new(repo)).await;

        let body = to_bytes(result.into_body()).await.unwrap();
        let film = serde_json::from_slice::<'_, Film>(&body).unwrap();

        assert_eq!(film.id, film_id);
        assert_eq!(film.title, film_title);
    }

    #[actix_rt::test]
    async fn delete_works() {
        let film_id = uuid::Uuid::new_v4();

        let mut repo = MockFilmRepository::default();
        repo.expect_delete_film().returning(|id| Ok(id.to_owned()));

        let result = delete(web::Path::from(film_id), web::Data::new(repo)).await;

        let body = to_bytes(result.into_body()).await.unwrap();
        let uuid = serde_json::from_slice::<'_, Uuid>(&body).unwrap();

        assert_eq!(uuid, film_id);
    }
}
