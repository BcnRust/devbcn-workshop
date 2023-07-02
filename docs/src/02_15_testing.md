# Unit and Integration tests

Although `testing` is a little bit out of the scope of this workshop, we thought it would be interesting to show you how to write tests for your API.

These will be simple examples of how to test the `health` endpoint. 

```admonish info title="Learn more"
For more information about testing in [Actix Web](https://actix.rs), please refer to the [Actix Web documentation](https://actix.rs/docs/testing/).

For more information about testing in [Rust](https://www.rust-lang.org), please refer to the [Rust Book](https://doc.rust-lang.org/book/ch11-01-writing-tests.html).
```

## Unit tests

Unit tests are usually created in the same file containin the subject under test. In our case, we will create a unit test for the `health` endpoint in the `api > lib > src > health.rs` file.

The common practice is to create a new module called `tests`. But before that, we will need to add a `dev-dependency` to the `Cargo.toml` file of our library:

```toml
[dev-dependencies]
actix-rt = "2.0.0"
```

Now, let's add this to our `health.rs` file:

```rust
#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;

    use super::*;

    #[actix_rt::test]
    async fn health_check_works() {
        let res = health().await;
        
        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);
        
        let data = res
            .headers()
            .get("health-check")
            .and_then(|h| h.to_str().ok());

        assert_eq!(data, Some("v0.0.1"));
    }
}
```

A **few things to note** here:

- The `#[cfg(test)]` annotation tells the compiler to only compile the code in this module if we are running tests.
- The `#[actix_rt::test]` annotation tells the compiler to run this test in the `Actix` runtime (giving you async support).
- The ` use super::*;` statement imports all the items from the parent module even if they're not public (in this case, the `health` function).

### Running the tests

To run the tests, you can use the following command:

```bash
cargo test
# or, if you prefer to test in watch mode:
cargo watch -x test
```

> We introduced a bug in our test. Can you fix it?

~~~admonish tip title="Solution" collapsible=true
The name of the header is `version`, not `health-check`. So, the either we change the name of the header in the test or we change the name of the header in the `health` function. Your call ;D.
~~~

> Can you extract the version to a constant so we can reuse it in the test?

~~~admonish tip title="Solution" collapsible=true
Declare the constant in the `health.rs` file and then use it in the `health` function and in the test:

```rust 
const API_VERSION: &str = "v0.0.1";
``` 
~~~

## Integration tests

Next, we're going to create an integration test for the `health` endpoint. This test will run the whole application and make a request to the `health` endpoint.

The **convention** is to have a `tests` folder in the root of the crate under test.

Let's create this folder and add a new file called `health.rs` in it. The path of the file should be `api > lib > tests > health.rs`.

Copy this content into it:

```rust
use actix_web::{http::StatusCode, App};
use api_lib::health::{service, API_VERSION};

#[actix_rt::test]
async fn health_check_works() {
    let app = App::new().configure(service);
    let mut app = actix_web::test::init_service(app).await;
    let req = actix_web::test::TestRequest::get()
        .uri("/health")
        .to_request();

    let res = actix_web::test::call_service(&mut app, req).await;
    
    assert!(res.status().is_success());
    assert_eq!(res.status(), StatusCode::OK);
    let data = res.headers().get("version").and_then(|h| h.to_str().ok());
    assert_eq!(data, Some(API_VERSION));
}
```

> This code will fail, can you figure out why?

~~~admonish tip title="Solution" collapsible=true
We need to make the `API_VERSION` constant **public** so we can use it in the test. To do that, we need to add the `pub` keyword to the constant declaration
~~~

```admonish info title="Learn more"
For more information about `Integration Tests` check the links we provided above in the beginning of this section:
- [Actix Web Testing](https://actix.rs/docs/testing/)
- [Rust Book - Writing Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
```

Don't forget to **commit** your changes:

```bash
git add .
git commit -m "add unit and integration tests"
```
 