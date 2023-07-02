# Using the configure method

You may have noticed that when our `health.rs` file contained two different endpoints, we had to add them as a `service` to the [ServiceConfig](https://docs.rs/actix-web/4.3.1/actix_web/web/struct.ServiceConfig.html) in the `actix_web` function. This is not a problem when we have a few endpoints, but it can become a problem when we have many endpoints.

In order to make our code cleaner, we can use the [configure](https://actix.rs/docs/application#configure) function.

Take a look at the [Actix Web docs](https://actix.rs/docs/application#configure) to see how to use it.

Indeed, if you take a look at the code we have in our `shuttle` crate, you will see that we are already using it:

```rust
let config = move |cfg: &mut ServiceConfig| {
    cfg.app_data(pool).service(health);
};
```

You could change this code to this and it should work the same:

```rust
let config = move |cfg: &mut ServiceConfig| {
    cfg.app_data(pool).configure(|c| {
        c.service(health);
    });
};
```

Try it out!

## Refactoring our code

Let's refactor our code to use the `configure` method both in the `health.rs` file and in the `main.rs` file.

In the `main.rs` file, we will be expecting to receive a `configure` function from the `health` module, so we will change the code to this:

```rust
let config = move |cfg: &mut ServiceConfig| {
    cfg.app_data(pool).configure(api_lib::health::service);
};
```

Note that it won't compile, because we haven't changed the `health.rs` file yet.

So, in the `health.rs` file, we need to export a function called `service` that receives a `ServiceConfig` and returns nothing.

```rust	
// add the use statement for ServiceConfig
use actix_web::{get, web::ServiceConfig, HttpResponse};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(health);
}
```

Now, we can run the code and it should work the same as before.

There are, though, a few things that we can change. 

Not sure if you notice it but we required the `pub` keyword to be in front of the `service` function. This is because we are calling the function from another module. If we were calling it from the same module, we wouldn't need the `pub` keyword.

But then, how is that we didn't need that for the `health` function as well? Well, that's because we are using the `#[get("/health")]` macro, which automatically adds the `pub` keyword to the function.

Let's opt out of using [macros](https://doc.rust-lang.org/reference/macros-by-example.html) and do it manually. 

We will leverage the [route method](https://docs.rs/actix-web/4.3.1/actix_web/web/struct.ServiceConfig.html#method.route) of the [ServiceConfig struct](https://docs.rs/actix-web/4.3.1/actix_web/web/struct.ServiceConfig.html). Check out the [docs](https://docs.rs/actix-web/4.3.1/actix_web/web/struct.ServiceConfig.html#method.route).

```rust
 use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(health));
}

async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", "v0.0.1"))
        .finish()
}
```

Everything should still work. Check it out and commit your changes.

```bash
git add .
git commit -m "use configure"
```

