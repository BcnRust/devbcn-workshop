# Serde

[Serde](https://serde.rs/) is a framework for **serializing and deserializing** Rust data structures efficiently and generically.

We are going to use it to add **serialization and deserialization** support to our models.

## Adding the dependency

Let's add the `serde` dependency to the `Cargo.toml` file of the `shared` crate:

```diff
[dependencies]
+ serde = { version = "1.0", features = ["derive"] }
```

Adding the `derive` feature will allow us to use the `#[derive(Serialize, Deserialize)]` macro on our models, which will automatically implement the `Serialize` and `Deserialize` [traits](https://doc.rust-lang.org/book/ch10-02-traits.html) for us.

As we will be working with `JSON` in our API, we need to bring in the `serde_json` crate as well in the `Cargo.toml` file of the `api-lib` crate:

```diff
[dependencies]
+ # serde
+ serde = "1.0"
+ serde_json = "1.0"
```

## Adding the `Serialize` and `Deserialize` traits to our models

Let's add the `Serialize` and `Deserialize` traits to our `Film` and `CreateFilm` models. 

For that, we are going to use the [derive macro](https://doc.rust-lang.org/rust-by-example/trait/derive.html):

```diff
+ use serde::{Deserialize, Serialize};

+ #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Film {
    pub id: uuid::Uuid, // we will be using uuids as ids
    pub title: String,
    pub director: String,
    pub year: u16,      // only positive numbers
    pub poster: String, // we will use the url of the poster here
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

+ #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CreateFilm {
    pub title: String,
    pub director: String,
    pub year: u16,
    pub poster: String,
}
```

```admonish info
Note that we added more [traits](https://doc.rust-lang.org/book/ch10-02-traits.html). It's a common practice for libraries to implement some of those [traits](https://doc.rust-lang.org/book/ch10-02-traits.html) to avoid issues when using them. See the [orphan rule](https://serde.rs/remote-derive.html) for more information.
```

Commit your changes:

```bash
git add .
git commit -m "add serde dependency and derive traits"
```
