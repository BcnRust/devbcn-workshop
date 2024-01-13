# Moving our endpoints to a library

The idea behind this section is to **move our endpoints** to a library so that we can **reuse** them in case we want to provide a different binary that doesn't rely on [Shuttle](https://shuttle.rs).

Imagine, for example, that you want to deploy your API to your cloud of choice. Most probably you'll want to use a container to do so. In that case, having our endpoints in a library will allow us to create a binary that works purely on [Actix Web](https://actix.rs).

## Adding a local dependency

Remember that we already created a `lib` crate in one of the previous sections? Well, we are going to use that crate to move our endpoints there.

But first of all, we need to add a dependency to our `api-shuttle` `Cargo.toml` file. We can do so by adding the following line:

```toml
[dependencies]
...
api-lib = { path = "../lib" }
```

`api-lib` is the name we gave to that library (you can check that in the `Cargo.toml` file in the `api > lib` folder).

Compile and check that you don't receive any compiler error:

```bash
# just compile
cargo build
# or compile in watch mode
cargo watch -x build
# or run the binary
cargo shuttle run
# or run the binary in watch mode
cargo watch -x "shuttle run"
```

As you can see, adding a local dependency is trivial. You just need to specify the path to the library.

## Moving the endpoints

Open the `api > lib > src` folder and create a new file called `health.rs`. This file will contain just one endpoint that will be used to check the health of the API, but for the sake of the example, we are going to **temporary move** our previous endpoints here.

Copy the following code in `api > shuttle > src > main.rs` to our recently created file `health.rs`:

```rust
#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/version")]
async fn version(db: actix_web::web::Data<sqlx::PgPool>) -> String {
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

```admonish 
If you are in **watch mode** or you try to **compile**, you will see that you don't get any kind of error. That's because the code in `health.rs` is **not being used yet**. 
```

Let's use it now. Open the `api > lib > src > lib.rs` file, remove all the content, and add the following line at the top of the file:

```rust
pub mod health;
```

A couple of things to take into account here:

- `lib.rs` files are the default entrypoint for library crates.
- The line we introduced in the `lib.rs` file is doing two things. 
  - First of all, it is declaring a new module called `health` (hence the compiler will care about our `health.rs` file's content).
  - Secondly, it is making that module public. This means that we can access everything that we export from that module.

Now, if you compile, you should be getting errors from the compiler complaining about dependencies. Let's add them to the `Cargo.toml` file:

```toml
[dependencies]
# actix
actix-web = "4.3.1"
# database
sqlx = { version = "0.7", default-features = false, features = [
    "tls-native-tls",
    "macros",
    "postgres",
    "uuid",
    "chrono",
    "json",
] }
# tracing
tracing = "0.1"
```

We will be adding more dependencies in the future, but for now, this is enough.

Finally, to make the compiler happy, let's import this in the top of the `health.rs` file:

```rust
use actix_web::get;
```

Everything should compile by now.

```admonish
Note that we're not using any [Shuttle](https://shuttle.rs) dependency in this crate.
```

## Using the endpoints

Now that we have our endpoints in a library, we can use them in our `main.rs` file. Let's do that.

Open the `api > shuttle > src > main.rs` file and remove the endpoints code that we copied before. Get rid of the unused `use` statements as well.

> Do you know what to do next?

~~~admonish tip title="Solution" collapsible=true
Yes, you only have to import the endpoints from the library. It's a one-liner:

```rust
use api_lib::health::{hello_world, version};
```
~~~

Is it working? It should!

```admonish example title="Actix Standalone"
If you want to try out the endpoints without using [Shuttle](https://shuttle.rs), you can create a new binary crate in the `api` folder and use the endpoints there. Check the [Actix Web documentation](https://actix.rs/) for more information.

This is out of the scope of this workshop because of time constraints but feel free to explore that option. You can also take a look at the [workshop's GitHub repository](https://github.com/BcnRust/devbcn-workshop) to see how to do it.
```

Commit your changes:

```bash
git add .
git commit -m "move endpoints to a library"
```
