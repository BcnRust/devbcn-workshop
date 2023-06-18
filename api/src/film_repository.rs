use async_trait::async_trait;
use shared::models::Film;
use uuid::Uuid;

pub type FilmError = String;
pub type FilmResult<T> = Result<T, FilmError>;

#[async_trait]
pub trait FilmRepository: Send + Sync + 'static {
    async fn get_film(&self, id: &Uuid) -> FilmResult<Film>;
    async fn create_film(&self, id: &Film) -> FilmResult<Film>;
    async fn update_film(&self, id: &Film) -> FilmResult<Film>;
    async fn delete_film(&self, id: &Uuid) -> FilmResult<Uuid>;
}

pub struct PostgresFilmRepository {
    pool: sqlx::PgPool,
}

impl PostgresFilmRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FilmRepository for PostgresFilmRepository {
    async fn get_film(&self, film_id: &uuid::Uuid) -> FilmResult<Film> {
        sqlx::query_as::<_, Film>(
            r#"
        SELECT id, title, director, year, poster, created_at, updated_at
        FROM films
        WHERE id = $1
        "#,
        )
        .bind(film_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create_film(&self, film: &Film) -> FilmResult<Film> {
        sqlx::query_as::<_, Film>(
            r#"
        INSERT INTO films (id, title, director, year, poster)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, title, director, year, poster, created_at, updated_at
        "#,
        )
        .bind(film.id)
        .bind(&film.title)
        .bind(&film.director)
        .bind(film.year)
        .bind(&film.poster)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn update_film(&self, film: &Film) -> FilmResult<Film> {
        sqlx::query_as::<_, Film>(
            r#"
        UPDATE films
        SET title = $2, director = $3, year = $4, poster = $5
        WHERE id = $1
        RETURNING id, title, director, year, poster, created_at, updated_at
        "#,
        )
        .bind(film.id)
        .bind(&film.title)
        .bind(&film.director)
        .bind(film.year)
        .bind(&film.poster)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_film(&self, film_id: &uuid::Uuid) -> FilmResult<Uuid> {
        let result = sqlx::query_scalar::<_, Uuid>(
            r#"
        DELETE FROM films
        WHERE id = $1
        RETURNING id
        "#,
        )
        .bind(film_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string());
        result
    }
}