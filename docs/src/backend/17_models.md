# Models

So now we have the films endpoints working, but they don't really do anything nor return any data.

In order to return data **we need to create a model for our films**.

As we want to **share** the model between the `api` and the `frontend` crates we will use the `shared` crate for this.

The `shared` crate is a `library` crate. This means that it can be used by other crates in the workspace.

Let's import the dependency in the `Cargo.toml` file of our `api-lib` crate:

```diff
[dependencies]
+ # shared
+ shared = { path = "../../shared" }
```

Verify that the project is still compiling.

## Creating the `Film` model

We are going to create a new module called `models` in the `shared` crate.

Create a **new file** called `models.rs` in the `shared > src` folder and add the following code:

```rust
pub struct Film {
    pub id: uuid::Uuid, // we will be using uuids as ids
    pub title: String,
    pub director: String,
    pub year: u16, // only positive numbers
    pub poster: String, // we will use the url of the poster here
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
```

We could make it more complicated but for the sake of simplicity we will just use a `struct` with a small amount of fields.

Now, remove everything from the `lib.rs` file in the `shared` crate and add the following code:

```rust
pub mod models;
```

Soon you will notice that the compiler will complain about the `chrono` and `uuid` dependencies.

Let's add them:

```diff
[dependencies]
+ uuid = { version = "1.3.4", features = ["serde", "v4", "js"] }
+ chrono = { version = "0.4", features = ["serde"] }
```

```admonish info
Most of the features you see are related to the fact that we want our API to be able to serialize and deserialize the models to and from JSON.
```

Compile the code and check that everything is fine.

## Creating a model for the post endpoint

In our `POST` endpoint we will receive a JSON object with the following structure:

```json
{
  "title": "The Lord of the Rings: The Fellowship of the Ring",
  "director": "Peter Jackson",
  "year": 2001,
  "poster": "https://www.imdb.com/title/tt0120737/mediaviewer/rm1340569600/",
}
```

We don't need to pass the `id` or the `created_at` and `updated_at` fields as they will be generated by the API, so let's create a new model for that.

```rust
pub struct CreateFilm {
    pub title: String,
    pub director: String,
    pub year: u16,
    pub poster: String,
}
```

Compile again just in case and commit your changes:

```bash
git add .
git commit -m "add models"
```
