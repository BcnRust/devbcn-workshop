mod film_repository;
mod memory_film_repository;
mod postgres_film_repository;

pub use film_repository::FilmRepository;
pub use memory_film_repository::MemoryFilmRepository;
pub use postgres_film_repository::PostgresFilmRepository;

#[cfg(test)]
pub use film_repository::MockFilmRepository;
