# Connecting to the Database

Now that we have everything we need, let's start by doing a simple *endpoint* that will get the database version. 

This will help us to get acquainted with [SQLx](https://github.com/launchbadge/sqlx).

## Creating the endpoint

> Can you create the endpoint yourself? 

Don't worry about how to retrieve the information from the database, we will do that in a moment. Just focus on creating and endpoint that will return a string. The string can be anything you want and the route should be `/version`.

If you're not sure about how to do it, expand the section below to see the solution.

~~~admonish tip title="Solution" collapsible=true
Open the `main.rs` file in the `api > shuttle > src` folder and add the following code:
```rust
#[get("/version")]
async fn version() -> &'static str {
    "version 0.0.0"
}
```
~~~

## Setting up the endpoint

You may have noticed that if you run the project and go to the `/version` route, you will get a `404` error. 

```bash
curl -i http://localhost:8000/version # HTTP/1.1 404 Not Found
```

This is because we haven't set up the endpoint yet.

> Can you guess how to do it?

~~~admonish tip title="Solution" collapsible=true
Add this to the line containing this piece of code `cfg.service(hello_world);` in the `main.rs` file in the `api > shuttle > src` folder: `.service(version)`.

The line should look like this

```rust
let config = move |cfg: &mut ServiceConfig| {
    // NOTE: this is the modified line
    cfg.service(hello_world).service(version);
};

```
~~~

Now, let's try it again:

```bash
curl -i http://localhost:8000/version # HTTP/1.1 200 OK version 0.0.0
```

Did it work? If so, congratulations! You have just created your first endpoint.

## Connecting to the database

Now that we have the endpoint, let's connect to the database and retrieve the version.

For that we will need to do a couple of things:

1. Pass the database connection pool to the endpoint.
1. Execute a query in the endpoint and return the result.

### Passing the database connection pool to the endpoint

In order to pass the connection pool to the endpoints we're going to leverage the [Application State Extractor](https://actix.rs/docs/extractors#application-state-extractor) from [Actix Web](https://actix.rs/).

``` admonish info title="State in Actix Web"
You can learn more about how to handle the [state in Actix Web](https://actix.rs/docs/application/#state) in the official documentation.
```

Ok, so just after the line where we initialized our database, **let's add the following code**:

```rust
let pool = actix_web::web::Data::new(pool);
```

```admonish info title="Variable shadowing" 
You may have noticed that we're using the same name for the variable that holds the connection pool and the one that holds the data. This is called [variable shadowing](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing).
```

Now, we need to change again the line we changed before when we added a new endpoint and use the [.app_data method](https://actix.rs/docs/application#state) like this:

```rust
let config = move |cfg: &mut ServiceConfig| {
    cfg.app_data(pool).service(hello_world).service(version);
};
```

### Executing a query in the endpoint and returning the result

Now let's change our `version` endpoint so we can get the connection pool from the state and execute a query. If you have taken a look to the [Application State Extractor documentation](https://actix.rs/docs/extractors#application-state-extractor) it should be pretty straightforward.

We have to add a parameter to the `version` function that will be our access to the database connection pool. We will call it `db` and it will be of type `actix_web::web::Data<sqlx::PgPool>`.

```rust
#[get("/version")]
async fn version(db: actix_web::web::Data<sqlx::PgPool>) -> &'static str {
    "version 0.0.0"
}
```

Now, we need to execute a query. For that, we will use the [sqlx::query_scalar](https://docs.rs/sqlx/latest/sqlx/query/struct.QueryScalar.html) function.

Let's change the `version` function to this:

```rust
#[get("/version")]
async fn version(db: actix_web::web::Data<sqlx::PgPool>) -> String {
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.get_ref())
        .await;

    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e),
    }
}
```

There are a couple of things going on here, so let's break it down.

First of all, it's worth noticing that **we changed the return type** of the function to `String`. This is for two different reasons:

1. We don't want our endpoint to fail. If the query fails, we will have to return an error message as a `String`.
1. We need that return to be a `String` because the version of the database will come to us as a `String`.

On the other hand, we have the [sqlx::query_scalar](https://docs.rs/sqlx/latest/sqlx/query/struct.QueryScalar.html) function. This function will execute a query and return a single value. In our case, the version of the database.

As you can see, the **query is pretty simple**. We're just selecting the version of the database. The most interesting part in there is that we need to use the `.get_ref()` method to get a **reference** to the inner `sqlx::PgPool` from the `actix_web::web::Data<sqlx::PgPool>`.

Finally, we have the [match expression](https://doc.rust-lang.org/reference/expressions/match-expr.html). The [sqlx::query_scalar](https://docs.rs/sqlx/latest/sqlx/query/struct.QueryScalar.html) function will return a [Result](https://doc.rust-lang.org/std/result/enum.Result.html) with either the version of the database or an error. With the [match expression](https://doc.rust-lang.org/reference/expressions/match-expr.html) we're covering both cases and we will make sure that we will always return a `String`.

```admonish tip
Note that most of the time we don't need the return keyword in Rust. The last expression in a function will be the return value.
```

```admonish example title="Try the error"
Introduce an error in the query and see what happens. Take some time to check out how the [format macro](https://doc.rust-lang.org/std/macro.format.html) works.
```

Note that even if you introduce an error in the query, the endpoint will not fail or even return a 500 error. This is because we're handling the error in the match expression.

We will see later how to handle errors in a more elegant way.

For now, let's commit our changes:

```bash
git add .
git commit -m "add version endpoint"
```
