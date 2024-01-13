# Implementing the FilmRepository trait

Cool, let's create a new file called `postgres_film_repository.rs` in the `film_repository` folder and add the new module to the `mod.rs` file in the same folder. This time don't use the `pub` keyword when declaring the module. 

The idea is that we will re-export the implementation as if it was coming from the `film_repository` module. This way, we can hide the implementation details from the rest of the application.

```rust
mod postgres_film_repository;
```

## Implementation

Let's open the recently created `postgres_film_repository.rs` file and add the following code:

```rust
pub struct PostgresFilmRepository {
    pool: sqlx::PgPool,
}
```

Note that this is a simple struct that holds a `sqlx::PgPool` instance. This is the connection pool we will use to connect to the database.

We don't need to expose it, hence the `pub` keyword is not used.

Now, let's add a `new` associated function to the struct that will make us easier to create new instances of this struct:

```rust
impl PostgresFilmRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}
```

```admonish
This sort of constructor pattern is very common in Rust and the convention is to use `new` as the name of the associated function. 
```

Next, let's implement the `FilmRepository` trait for this struct:

```rust
#[async_trait::async_trait]
impl FilmRepository for PostgresFilmRepository {
    async fn get_films(&self) -> FilmResult<Vec<Film>> {
        sqlx::query_as::<_, Film>(
            r#"
      SELECT id, title, director, year, poster, created_at, updated_at
      FROM films
      "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

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

    async fn create_film(&self, create_film: &CreateFilm) -> FilmResult<Film> {
        sqlx::query_as::<_, Film>(
            r#"
      INSERT INTO films (title, director, year, poster)
      VALUES ($1, $2, $3, $4)
      RETURNING id, title, director, year, poster, created_at, updated_at
      "#,
        )
        .bind(&create_film.title)
        .bind(&create_film.director)
        .bind(create_film.year as i16)
        .bind(&create_film.poster)
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
        .bind(film.year as i16)
        .bind(&film.poster)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_film(&self, film_id: &uuid::Uuid) -> FilmResult<uuid::Uuid> {
        sqlx::query_scalar::<_, uuid::Uuid>(
            r#"
      DELETE FROM films
      WHERE id = $1
      RETURNING id
      "#,
        )
        .bind(film_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
```

Don't forget to add the necessary imports:

```rust
use super::{FilmRepository, FilmResult};
use shared::models::{CreateFilm, Film};
```

Note that **this code won't compile**. Don't worry for the moment, we will fix it in a moment.

```admonish info
Take the time to review the code. Unfortunately, going deep into the details of [SQLx](https://github.com/launchbadge/sqlx) is **out of the scope of this tutorial**. However, if you are interested in learning more about it, you can check the [SQLx documentation](https://github.com/launchbadge/sqlx).
```

## Fixing the compilation error

If you check the compiler error you will see that it is complaining about the `Film` struct. It is telling us that it doesn't implement the [FromRow](https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html) trait. 

This is because we are using the [query_as](https://docs.rs/sqlx/latest/sqlx/fn.query_as.html) method from [SQLx](https://github.com/launchbadge/sqlx), which requires that the struct implements the [FromRow](https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html) trait.

```bash
4  |   pub struct Film {
   |   --------------- doesn't satisfy `Film: FromRow<'r, PgRow>`
   |
   = note: the following trait bounds were not satisfied:
           `Film: FromRow<'r, PgRow>`
```

Let's fix this by implementing the [FromRow](https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html) trait for the `Film` struct.

We must do this in the `shared` crate, because the `Film` struct is defined there.

Add the [SQLx](https://github.com/launchbadge/sqlx) dependency to the `Cargo.toml` file in the `shared` crate:

```toml
# database
sqlx = { version = "0.7", default-features = false, features = [
    "tls-native-tls",
    "macros",
    "postgres",
    "uuid",
    "chrono",
    "json",
] }
```

And then add the `sqlx::FromRow` trait into the `derive` attribute of the `Film` and `CreateFilm` structs.

Now we will hit another compiler error. [FromRow](https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html) doesn't work with `u16`.

Let's add a new annotation to the `year` field in both structs:

```rust
#[sqlx(try_from = "i16")]
```

~~~admonish tip title="This is how the models.rs file should look like" collapsible=true
```rust
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, sqlx::FromRow,
)]
pub struct Film {
    pub id: uuid::Uuid, // we will be using uuids as ids
    pub title: String,
    pub director: String,
    #[sqlx(try_from = "i16")]
    pub year: u16, // only positive numbers
    pub poster: String, // we will use the url of the poster here
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(
    Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, sqlx::FromRow,
)]
pub struct CreateFilm {
    pub title: String,
    pub director: String,
    #[sqlx(try_from = "i16")]
    pub year: u16,
    pub poster: String,
}
```
~~~

## Supporting WebAssmembly

We are almost done. The code is compiling but we still need to do some changes to make this `shared` crate work in the browser.

Our frontend will be compiled to [WebAssembly](https://webassembly.org/), so we need to make sure that the `shared` crate can be compiled to [WebAssembly](https://webassembly.org/).

The problem that we will face is that [SQLx](https://github.com/launchbadge/sqlx) doesn't support [WebAssembly](https://webassembly.org/) yet.

So, how to solve this? Enter [Cargo Features](https://doc.rust-lang.org/cargo/reference/features.html).

We will compile certain parts of the code only when a certain `feature` is enabled.

Note that this is how tests works. If you remember when we looked at that, each testing module is preceded by `#[cfg(test)]` annotation. This means that the code inside that module will only be compiled when the `test` feature is enabled.


### Adding the `backend` feature

The idea is that we will only use the [FromRow](https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html) trait when the `backend` feature is enabled.

This should be true for all the backend code (the `api-lib` crate) but not for the frontend code.

Let's add the `backend` feature to the `Cargo.toml` file in the `shared` crate:

```toml
[features]
backend = ["sqlx"]
```

Then modify the `sqlx` dependency to make it optional:

```toml
sqlx = { version = "0.6.3", default-features = false, features = [ "runtime-actix-native-tls", "macros", "postgres", "uuid", "chrono", "json" ], optional = true }
```

That's it. As the [SQLx](https://github.com/launchbadge/sqlx) dependency is now **optional**, it will only be used in case the `backend` feature is enabled.

### Using the `sqlx` feature

Modify the `models.rs` file in the `shared` crate to look like this:

```rust
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Film {
    pub id: uuid::Uuid,
    pub title: String,
    pub director: String,
    #[cfg_attr(feature = "backend", sqlx(try_from = "i16"))]
    pub year: u16,
    pub poster: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CreateFilm {
    pub title: String,
    pub director: String,
    #[cfg_attr(feature = "backend", sqlx(try_from = "i16"))]
    pub year: u16,
    pub poster: String,
}
```

```admonish info
Note that all the usages of the `sqlx` annotation are now conditioned to the `backend` feature.
```

But... the code doesn't compile!

Sure, no problem. We need to add the `backend` feature to the `Cargo.toml` file in the `api-lib` crate:

```diff
# shared
- shared = { path = "../../shared" }
+ shared = { path = "../../shared", features = ["backend"] }
```

We should be good by now but there's still a small detail to cover.

We want our `PostgresFilmRepository` struct to be available so we need expose it.

Head to the `mod.rs` file in `api > lib > src > film_repository` and add the following line:

```rust
pub use postgres_film_repository::PostgresFilmRepository;
```

```admonish example title="Memory repository (optional)"
Try to build a new module called `memory_film_repository` that implements the `FilmRepository` trait and uses an in-memory data structure to store the films.

You can also add tests to your implementation.

HINT: You can take a look at the [workshop GitHub repository](https://github.com/BcnRust/devbcn-workshop) if you get stuck.
```

Plenty of work in this section. Check that everything compiles and commit your changes:

```bash
git add .
git commit -m "add postgres film repository"
```
