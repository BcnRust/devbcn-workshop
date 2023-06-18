use async_trait::async_trait;
use shared::models::Film;
use uuid::Uuid;

pub type FilmError = String;
pub type FilmResult<T> = Result<T, FilmError>;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait FilmRepository: Send + Sync + 'static {
    async fn get_film(&self, id: &Uuid) -> FilmResult<Film>;
    async fn create_film(&self, id: &Film) -> FilmResult<Film>;
    async fn update_film(&self, id: &Film) -> FilmResult<Film>;
    async fn delete_film(&self, id: &Uuid) -> FilmResult<Uuid>;
}
