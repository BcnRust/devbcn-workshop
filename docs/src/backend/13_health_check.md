# Creating a health check endpoint

We're going to get rid of the previous endpoints and create a health check endpoint. This endpoint will be used to check if the application is running and ready to receive requests.

This endpoint will be very basic and will just **return a 200 OK response** with custom header containing the version (this is just for fun, not really needed at all).

Armed with the knowledge we've gained so far, we should be able to handle this change. 


```admonish example title="Exercise: Create a health check endpoint"
- The route should be `/health` and use the `GET` method.
- The response should be a `200 OK` with a custom header named `version`containing the version (a `&str` containing "v0.0.1" for example).
- As a hint, you can check the code in [Actix Web docs](https://actix.rs/docs/server) to see how to return a simple `200 OK` response.
- You can also check out the [HttpResponse docs](https://docs.rs/actix-web/3.3.2/actix_web/struct.HttpResponse.html) to see how to add a header to the response.
```

> Can you do it?

~~~admonish tip title="Solution" collapsible=true
- Remove the previous endpoints.
- Create a new endpoint with the route `/health` and the method `GET`.

```rust
use actix_web::{get, HttpResponse};

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", "0.0.1"))
        .finish()
}
```

- Configure the services in your shuttle crate. Remove the previous services and add the new one.

Your `api > shuttle > src > main.rs` file should look like this:

```rust
use actix_web::web::ServiceConfig;
use api_lib::health::health;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // initialize the database if not already initialized
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let pool = actix_web::web::Data::new(pool);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(pool).service(health);
    };

    Ok(config.into())
}
```
~~~

Test that everything is working by running the following command:

```bash
curl -i http://localhost:8000/health
```

You should get something like this:

```text
HTTP/1.1 200 OK
content-length: 0
version: v0.0.1
date: Sun, 02 Jul 2023 10:35:15 GMT
```

Commit your changes.

```bash
git add .
git commit -m "add health check endpoint"
```
