mod integration {

    use actix_web::{http::StatusCode, web, App};
    use api_lib::film_repository::{FilmRepository, MemoryFilmRepository};
    use shared::models::{CreateFilm, Film};

    fn create_test_film(id: &'static str) -> Film {
        Film {
            id: uuid::Uuid::new_v4(),
            title: format!("title-{}", id),
            director: format!("director-{}", id),
            poster: format!("poster-{}", id),
            year: 2001,
            created_at: Some(chrono::Utc::now()),
            updated_at: None,
        }
    }

    fn create_test_create_film(id: &'static str) -> CreateFilm {
        CreateFilm {
            title: format!("title-{}", id),
            director: format!("director-{}", id),
            poster: format!("poster-{}", id),
            year: 2001,
        }
    }

    #[actix_rt::test]
    async fn get_films_works() {
        let repo = MemoryFilmRepository::default();
        let create_film1 = create_test_create_film("1");
        let create_film2 = create_test_create_film("2");
        let _ = repo.create_film(&create_film1).await;
        let _ = repo.create_film(&create_film2).await;

        let repo = web::Data::new(repo);

        let app = App::new()
            .app_data(repo.clone())
            .configure(api_lib::health::service)
            .configure(api_lib::v1::service::<MemoryFilmRepository>);

        let mut app = actix_web::test::init_service(app).await;

        let req = actix_web::test::TestRequest::get()
            .uri("/v1/films")
            .to_request();

        let res = actix_web::test::call_service(&mut app, req).await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);

        let films: Vec<Film> = actix_web::test::read_body_json(res).await;

        assert_eq!(films.len(), 2);
    }

    #[actix_rt::test]
    async fn get_film_works() {
        let repo = MemoryFilmRepository::default();
        let create_film = create_test_create_film("1");
        let film = repo
            .create_film(&create_film)
            .await
            .expect("create film failed");

        let repo = web::Data::new(repo);

        let app = App::new()
            .app_data(repo.clone())
            .configure(api_lib::health::service)
            .configure(api_lib::v1::service::<MemoryFilmRepository>);

        let mut app = actix_web::test::init_service(app).await;

        let req = actix_web::test::TestRequest::get()
            .uri(&format!("/v1/films/{}", film.id))
            .to_request();

        let res = actix_web::test::call_service(&mut app, req).await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);

        let result: Film = actix_web::test::read_body_json(res).await;

        assert_eq!(result.id, film.id);
        assert_eq!(result.title, create_film.title);
        assert_eq!(result.director, create_film.director);
        assert_eq!(result.poster, create_film.poster);
        assert_eq!(result.year, create_film.year);
        assert!(result.created_at.is_some());
        assert!(result.updated_at.is_none());
    }

    #[actix_rt::test]
    async fn get_film_fails_if_file_is_not_present() {
        let repo = MemoryFilmRepository::default();
        let film = create_test_film("1");

        let repo = web::Data::new(repo);

        let app = App::new()
            .app_data(repo.clone())
            .configure(api_lib::health::service)
            .configure(api_lib::v1::service::<MemoryFilmRepository>);

        let mut app = actix_web::test::init_service(app).await;

        let req = actix_web::test::TestRequest::get()
            .uri(&format!("/v1/films/{}", film.id))
            .to_request();

        let res = actix_web::test::call_service(&mut app, req).await;

        assert!(res.status().is_client_error());
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn create_film_works() {
        let repo = MemoryFilmRepository::default();
        let create_film = create_test_create_film("1");

        let repo = web::Data::new(repo);

        let app = App::new()
            .app_data(repo.clone())
            .configure(api_lib::health::service)
            .configure(api_lib::v1::service::<MemoryFilmRepository>);

        let mut app = actix_web::test::init_service(app).await;

        let req = actix_web::test::TestRequest::post()
            .uri("/v1/films")
            .set_json(create_film.clone())
            .to_request();

        let res = actix_web::test::call_service(&mut app, req).await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);

        let created_file: Film = actix_web::test::read_body_json(res).await;

        assert_eq!(created_file.title, create_film.title);
        assert_eq!(created_file.director, create_film.director);
        assert_eq!(created_file.poster, create_film.poster);
        assert_eq!(created_file.year, create_film.year);
        assert!(created_file.created_at.is_some());
        assert!(created_file.updated_at.is_none());
    }

    #[actix_rt::test]
    async fn update_film_works() {
        let repo = MemoryFilmRepository::default();
        let create_film = create_test_create_film("1");
        let created_file = repo.create_film(&create_film).await.unwrap();

        let repo = web::Data::new(repo);

        let app = App::new()
            .app_data(repo.clone())
            .configure(api_lib::health::service)
            .configure(api_lib::v1::service::<MemoryFilmRepository>);

        let mut app = actix_web::test::init_service(app).await;

        let mut film_update = created_file.clone();
        film_update.title = "new-title".to_string();
        film_update.year = 2002;

        let req = actix_web::test::TestRequest::put()
            .uri("/v1/films")
            .set_json(&film_update)
            .to_request();

        let res = actix_web::test::call_service(&mut app, req).await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);

        let updated_file: Film = actix_web::test::read_body_json(res).await;

        assert_eq!(updated_file.id, created_file.id);
        assert_ne!(updated_file.title, created_file.title);
        assert_eq!(updated_file.title, film_update.title);
        assert_eq!(updated_file.director, created_file.director);
        assert_eq!(updated_file.poster, created_file.poster);
        assert_ne!(updated_file.year, created_file.year);
        assert_eq!(updated_file.year, film_update.year);
        assert_eq!(updated_file.created_at, created_file.created_at);
        assert!(updated_file.updated_at.is_some());
    }

    #[actix_rt::test]
    async fn update_film_fails_if_file_is_not_present() {
        let repo = MemoryFilmRepository::default();
        let film = create_test_film("1");

        let repo = web::Data::new(repo);

        let app = App::new()
            .app_data(repo.clone())
            .configure(api_lib::health::service)
            .configure(api_lib::v1::service::<MemoryFilmRepository>);

        let mut app = actix_web::test::init_service(app).await;

        let req = actix_web::test::TestRequest::put()
            .uri("/v1/films")
            .set_json(&film)
            .to_request();

        let res = actix_web::test::call_service(&mut app, req).await;

        assert!(res.status().is_client_error());
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn delete_film_works() {
        let repo = MemoryFilmRepository::default();
        let create_film = create_test_create_film("1");
        let film = repo
            .create_film(&create_film)
            .await
            .expect("create film failed");

        let repo = web::Data::new(repo);

        let app = App::new()
            .app_data(repo.clone())
            .configure(api_lib::health::service)
            .configure(api_lib::v1::service::<MemoryFilmRepository>);

        let mut app = actix_web::test::init_service(app).await;

        let req = actix_web::test::TestRequest::delete()
            .uri(&format!("/v1/films/{}", film.id))
            .set_json(create_film.clone())
            .to_request();

        let res = actix_web::test::call_service(&mut app, req).await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);

        let deleted_id: uuid::Uuid = actix_web::test::read_body_json(res).await;

        assert_eq!(deleted_id, film.id);
    }

    #[actix_rt::test]
    async fn delete_film_does_not_fail_if_film_is_not_present() {
        let repo = MemoryFilmRepository::default();
        let film = create_test_film("1");
        let repo = web::Data::new(repo);

        let app = App::new()
            .app_data(repo.clone())
            .configure(api_lib::health::service)
            .configure(api_lib::v1::service::<MemoryFilmRepository>);

        let mut app = actix_web::test::init_service(app).await;

        let req = actix_web::test::TestRequest::delete()
            .uri(&format!("/v1/films/{}", film.id))
            .set_json(film.clone())
            .to_request();

        let res = actix_web::test::call_service(&mut app, req).await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);

        let deleted_id: uuid::Uuid = actix_web::test::read_body_json(res).await;

        assert_eq!(deleted_id, film.id);
    }
}
