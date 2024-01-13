# Film Repository

Today, our API will work with a [Postgres](https://www.postgresql.org/) database. But this may change in the future. 

Even if that never happens (which is the most probable thing) we will still want to **decouple our API from the database** to make it easier to test and maintain.

To do that, we will leverage [traits](https://doc.rust-lang.org/book/ch10-02-traits.html) to define the behavior of our **film repository**.

This will also allow us to take a look at:
- [traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [async-trait](https://docs.rs/async-trait/latest/async_trait/)
- [Static vs Dynamic dispatch](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#trait-objects-perform-dynamic-dispatch)

## Defining the `FilmRepository` trait

We will define this [trait](https://doc.rust-lang.org/book/ch10-02-traits.html) in the `api-lib` crate although it could be its own crate if we wanted.

To keep it simple create a new `film_repository` folder in `api > lib > src` and add a `mod.rs` file with the following content:

```rust
pub type FilmError = String;
pub type FilmResult<T> = Result<T, FilmError>;

pub trait FilmRepository: Send + Sync + 'static {
    async fn get_films(&self) -> FilmResult<Vec<Film>>;
    async fn get_film(&self, id: &Uuid) -> FilmResult<Film>;
    async fn create_film(&self, id: &CreateFilm) -> FilmResult<Film>;
    async fn update_film(&self, id: &Film) -> FilmResult<Film>;
    async fn delete_film(&self, id: &Uuid) -> FilmResult<Uuid>;
}
```

Don't forget to add the module to the `lib.rs` file:

```rust
pub mod film_repository;
```

The code won't compile. But don't worry, we will fix that in a minute.

Let's review for a moment that piece of code:

1. We define two type aliases: `FilmError` and `FilmResult<T>`. This will allow us to easily change the `error` type if we need to and to avoid boilerplate when having to write the return of our functions.
1. The [Send & Sync traits](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html?highlight=sync#allowing-access-from-multiple-threads-with-sync) will allow us to share and send this the types implementiong this trait between threads.
1. The `'static` lifetime will make our life easier as we know that the repository will live for the entire duration of the program.
1. Finally, you see that we have defined 5 functions that will allow us to interact with our database. We will implement them in the next section.

Then, why does this code not compile?

The reason is that we are using the `async` keyword in our trait definition. This is not allowed by the Rust compiler.

To fix this, we will use the [async-trait](https://docs.rs/async-trait/latest/async_trait/) crate.

## async-trait

Let's bring this dependency into our `api-lib` crate by adding it to the `Cargo.toml` file. As we will be using the `uuid` crate in our repository, we will also add it to the `Cargo.toml` file:

```diff
[dependencies]
+ # utils
+ async-trait = "0.1.68"
+ uuid = { version = "1.3.4", features = ["serde", "v4", "js"] }
```

Now, let's mark our [trait](https://doc.rust-lang.org/book/ch10-02-traits.html) as `async` and add all the `use` statements we need:

```diff
+ use shared::models::{CreateFilm, Film};
+ use uuid::Uuid;

pub type FilmError = String;
pub type FilmResult<T> = Result<T, FilmError>;

+ #[async_trait::async_trait]
pub trait FilmRepository: Send + Sync + 'static {
    async fn get_films(&self) -> FilmResult<Vec<Film>>;
    async fn get_film(&self, id: &Uuid) -> FilmResult<Film>;
    async fn create_film(&self, id: &CreateFilm) -> FilmResult<Film>;
    async fn update_film(&self, id: &Film) -> FilmResult<Film>;
    async fn delete_film(&self, id: &Uuid) -> FilmResult<Uuid>;
}
```

Now, the code compiles. But we still need to implement the trait. We will do it in the next section.

## mod.rs

You probably noticed that we created file called `mod.rs` in the `film_repository` folder. 

So far, whenever we wanted to create a new module, we just used a file with the same name as the module. For example, we created a `film` module by creating a `film.rs` file. 

```admonish info
There are several ways to work with modules, you can learn more about it [here](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html).
```

This is the old way of doing things with modules but it's still valid and widely used in the Rust community.

Most of the time, you will do this if you plan to add more modules under the `film_repository` folder. For example, you could add a `memory_film_repository` module to implement a memory repository.

For now, let's commit our changes:

```bash
git add .
git commit -m "add film repository trait"
```
