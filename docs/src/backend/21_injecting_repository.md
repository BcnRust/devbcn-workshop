# Injecting the repository

Ok, so now we have our shared library working both for the `frontend` and the `backend`. We have our `FilmRepository` trait and even a [Postgres](https://www.postgresql.org/) implementation of it. Now we need to inject the repository into our handlers.

If you take a look again at the `main.rs` file of our `api-shuttle` crate, you will see that we were already sharing the `sqlx::PgPool` between the handlers. 

We will do the same with the `FilmRepository` trait.

## Creating a `PostgresFilmRepository` struct

Let's create a new instance of the `PostgresFilmRepository` struct in the `main.rs` file of our `api-shuttle` crate:

```diff
- let pool = actix_web::web::Data::new(pool);
+ let film_repository = api_lib::film_repository::PostgresFilmRepository::new(pool);
+ let film_repository = actix_web::web::Data::new(film_repository);

- cfg.app_data(pool)
+ cfg.app_data(film_repository)
```

Once you apply this change, everything should compile and work as before.

Commit your changes:

```bash
git add .
git commit -m "inject film repository"
```
