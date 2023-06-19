use std::{collections::HashMap, sync::RwLock};

use async_trait::async_trait;
use chrono::Utc;
use shared::models::Film;
use uuid::Uuid;

use super::{film_repository::FilmResult, FilmRepository};

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
            .map_err(|e| format!("Couldn't find film: {}", e))
            .and_then(|films| {
                let r = films.clone().into_values().collect::<Vec<_>>();
                Ok(r)
            });

        if result.is_err() {
            tracing::error!("Couldn't retrive a films");
        }

        result
    }

    async fn get_film(&self, film_id: &uuid::Uuid) -> FilmResult<Film> {
        let result = self
            .films
            .read()
            .map_err(|e| format!("Couldn't find film: {}", e))
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

    async fn create_film(&self, film: &Film) -> FilmResult<Film> {
        if self.get_film(&film.id).await.is_ok() {
            let err = format!("Film with id {} already exists", film.id);
            tracing::error!(err);
            return Err(err);
        }
        let mut new_film = film.to_owned();
        new_film.created_at = Some(Utc::now());

        // TODO: (ROB) remove unwrap
        let mut films = self.films.write().unwrap();
        films.insert(film.id, new_film.clone());
        tracing::trace!("Film with id {} correctly created", film.id);
        Ok(new_film)
    }

    async fn update_film(&self, film: &Film) -> FilmResult<Film> {
        // TODO: (ROB) remove unwrap
        let mut films = self.films.write().unwrap();
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

    async fn delete_film(&self, film_id: &uuid::Uuid) -> FilmResult<Uuid> {
        // TODO: (ROB) remove unwrap
        let mut films = self.films.write().unwrap();
        films.remove(film_id);
        Ok(film_id.to_owned())
    }
}
