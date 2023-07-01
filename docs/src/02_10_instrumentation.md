# Instrumentation

In order to instrument our backend, we are going to use the [tracing crate](https://docs.rs/tracing/latest/tracing/).

Let's add this dependency to the `Cargo.toml` file of our [Shuttle](https://shuttle.rs) crate:

```toml
tracing = "0.1"
```

Now, let's add some instrumentation to our `main.rs` file. Feel free to add as many logs as you want. For example:

```rust
#[get("/version")]
async fn version(db: actix_web::web::Data<sqlx::PgPool>) -> String {
    // NOTE: added line below:
    tracing::info!("Getting version");
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.get_ref())
        .await;

    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e),
    }
}
```

If you run the application now, and hit the `version` endpoint, you should see something like this in the logs of your terminal:

```bash
2023-07-01T19:47:32.836809924+02:00  INFO api_shuttle: Getting version
```

## Log level

By default, the log level is set to `info`. This means that all logs with a level of `info` or higher will be displayed. If you want to change the log level, you can do so by setting the `RUST_LOG` environment variable. For example, if you want to see all logs, you can set the log level to `trace`:

```bash
RUST_LOG=trace cargo shuttle run
```

```admonish info
For more information about [Telemetry and Shuttle](https://docs.shuttle.rs/introduction/telemetry), please refer to the [Shuttle documentation](https://docs.shuttle.rs/introduction/telemetry).
```

Let's commit our changes:

```bash
git add .
git commit -m "added instrumentation"
```
