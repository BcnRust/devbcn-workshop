use std::{collections::HashMap, sync::RwLock};

use async_trait::async_trait;
use chrono::Utc;
use shared::models::{CreateFilm, Film};
use uuid::Uuid;

use super::{FilmRepository, FilmResult};

pub struct MemoryFilmRepository {
    films: RwLock<HashMap<Uuid, Film>>,
}

impl MemoryFilmRepository {
    pub fn new() -> Self {
        Self {
            films: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MemoryFilmRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl FilmRepository for MemoryFilmRepository {
    async fn get_films(&self) -> FilmResult<Vec<Film>> {
        let result = self
            .films
            .read()
            .map(|films| films.clone().into_values().collect::<Vec<_>>())
            .map_err(|e| format!("An error happened while trying to read films: {}", e));

        if result.is_err() {
            tracing::error!("Couldn't retrive a films");
        }

        result
    }

    async fn get_film(&self, film_id: &uuid::Uuid) -> FilmResult<Film> {
        let result = self
            .films
            .read()
            .map_err(|e| format!("An error happened while trying to read films: {}", e))
            .and_then(|films| {
                films
                    .get(film_id)
                    .cloned()
                    .ok_or_else(|| format!("Couldn't find film: {}", film_id))
            });

        if result.is_err() {
            tracing::error!("Couldn't retrive a film with id {}", film_id);
        }

        result
    }

    async fn create_film(&self, create_film: &CreateFilm) -> FilmResult<Film> {
        match self.films.write() {
            Ok(mut films) => {
                let new_film = Film {
                    id: uuid::Uuid::new_v4(),
                    title: create_film.title.clone(),
                    director: create_film.director.clone(),
                    year: create_film.year,
                    poster: create_film.poster.clone(),
                    created_at: Some(Utc::now()),
                    updated_at: None,
                };
                films.insert(new_film.id, new_film.clone());
                tracing::trace!("Film with id {} correctly created", new_film.id);
                Ok(new_film)
            }
            Err(e) => {
                let err = format!("An error happened while trying to update film: {}", e);
                tracing::error!(err);
                Err(err)
            }
        }
    }

    async fn update_film(&self, film: &Film) -> FilmResult<Film> {
        match self.films.write() {
            Ok(mut films) => {
                let old_film = films.get_mut(&film.id);
                match old_film {
                    Some(old_film) => {
                        let mut updated_film = film.to_owned();
                        updated_film.created_at = old_film.created_at;
                        updated_film.updated_at = Some(Utc::now());
                        films.insert(film.id, updated_film.clone());
                        tracing::debug!("Film with id {} correctly updated", film.id);
                        Ok(updated_film)
                    }
                    None => {
                        let err = format!("Film with id {} does not exist", film.id);
                        tracing::error!(err);
                        Err(err)
                    }
                }
            }
            Err(e) => {
                let err = format!("An error happened while trying to update film: {}", e);
                tracing::error!(err);
                Err(err)
            }
        }
    }

    async fn delete_film(&self, film_id: &uuid::Uuid) -> FilmResult<Uuid> {
        match self.films.write() {
            Ok(mut films) => {
                films.remove(film_id);
                Ok(film_id.to_owned())
            }
            Err(e) => {
                let err = format!("An error happened while trying to delete film: {}", e);
                tracing::error!(err);
                Err(err)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MemoryFilmRepository;
    use crate::film_repository::FilmRepository;
    use shared::models::{CreateFilm, Film};
    use std::{collections::HashMap, sync::RwLock};

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
    async fn repo_must_be_empty_on_new() {
        let repo = MemoryFilmRepository::new();
        let result = repo.get_films().await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.len(), 0);
    }

    #[actix_rt::test]
    async fn repo_must_be_empty_on_default() {
        let repo = MemoryFilmRepository::default();
        let result = repo.get_films().await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.len(), 0);
    }

    #[actix_rt::test]
    async fn get_films_works() {
        let store = RwLock::new(HashMap::new());
        let film1 = create_test_film("1");
        let film2 = create_test_film("2");
        {
            let mut store = store.write().unwrap();
            store.insert(film1.id, film1.clone());
            store.insert(film2.id, film2.clone());
        }

        let repo = MemoryFilmRepository { films: store };
        let result = repo.get_films().await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.iter().any(|f| f.id == film1.id));
        assert!(result.iter().any(|f| f.id == film2.id));
    }

    #[actix_rt::test]
    async fn get_film_works() {
        let store = RwLock::new(HashMap::new());
        let film = create_test_film("1");
        store.write().unwrap().insert(film.id, film.clone());

        let repo = MemoryFilmRepository { films: store };
        let result = repo.get_film(&film.id).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), film);
    }

    #[actix_rt::test]
    async fn get_film_fails_if_file_is_not_present() {
        let film_update = create_test_film("2");

        let repo = MemoryFilmRepository::default();
        let result = repo.update_film(&film_update).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("does not exist"));
    }

    #[actix_rt::test]
    async fn create_film_works() {
        let store = RwLock::new(HashMap::new());
        let create_film = create_test_create_film("1");

        let repo = MemoryFilmRepository { films: store };
        let result = repo.create_film(&create_film).await;

        assert!(result.is_ok());
        let created_file = result.unwrap();
        assert_eq!(created_file.title, create_film.title);
        assert_eq!(created_file.director, create_film.director);
        assert_eq!(created_file.poster, create_film.poster);
        assert_eq!(created_file.year, create_film.year);
        assert!(created_file.created_at.is_some());
    }

    #[actix_rt::test]
    async fn update_film_works() {
        let store = RwLock::new(HashMap::new());
        let film = create_test_film("1");
        store.write().unwrap().insert(film.id, film.clone());

        let mut film_update = film.clone();
        film_update.title = "new-title".to_string();
        film_update.year = 2002;

        let repo = MemoryFilmRepository { films: store };
        let result = repo.update_film(&film_update).await;

        assert!(result.is_ok());
        let updated_file = result.unwrap();
        assert_eq!(updated_file.id, film.id);
        assert_ne!(updated_file.title, film.title);
        assert_eq!(updated_file.title, film_update.title);
        assert_eq!(updated_file.director, film.director);
        assert_eq!(updated_file.poster, film.poster);
        assert_ne!(updated_file.year, film.year);
        assert_eq!(updated_file.year, film_update.year);
        assert_eq!(updated_file.created_at, film.created_at);
        assert!(updated_file.updated_at.is_some());
        assert!(film.updated_at.is_none());
    }

    #[actix_rt::test]
    async fn update_film_fails_if_file_is_not_present() {
        let store = RwLock::new(HashMap::new());
        let film = create_test_film("1");
        store.write().unwrap().insert(film.id, film.clone());

        let film_update = create_test_film("2");

        let repo = MemoryFilmRepository { films: store };
        let result = repo.update_film(&film_update).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("does not exist"));
    }

    #[actix_rt::test]
    async fn delete_film_works() {
        let store = RwLock::new(HashMap::new());
        let film = create_test_film("1");
        store.write().unwrap().insert(film.id, film.clone());

        let repo = MemoryFilmRepository { films: store };
        let result = repo.delete_film(&film.id).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), film.id);
    }

    #[actix_rt::test]
    async fn delete_film_does_not_fail_if_film_is_not_present() {
        let repo = MemoryFilmRepository::default();
        let id = uuid::Uuid::new_v4();
        let result = repo.delete_film(&id).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), id);
    }
}
