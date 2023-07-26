# App Effects

Alright folks, we've got our state management all set up. Now, the magic happens! We need to synchronize the values of that state when different parts of our app interact with our users.

Imagine our first call to the API to fetch our freshly minted films, or the moment when we open the Film Modal in edit mode. We need to pre-populate the form with the values of the film we're about to edit.

No sweat, we've got the `use_effect` hook to handle this. This useful hook allows us to execute a function when a value changes, or when the component is mounted or unmounted. Pretty cool, huh?

Now, let's break down the key parts of the `use_effect` hook:
- It should be nestled inside a closure function.
- If we're planning to use a `use_state` hook inside it, we need to `clone()` it or pass the ownership using `to_owned()` to the closure function.
- The parameters inside the `use_effect()` function include the Scope of our app (`cx`), the `dependencies` that will trigger the effect again, and a `future` that will spring into action when the effect is triggered.

Here's a quick look at how it works:

```rust
{
  let some_state = some_state.clone();
  use_effect(cx, change_dependency, |_| async move {
    // Do something with some_state or something else
  })
}
```

## Film Modal

We will begin by adapting our `FilmModal` component. This will be modified to pre-populate the form with the values of the film that is currently being edited. To accomplish this, we will use the `use_effect` hook.

`front/src/components/film_modal.rs`
```diff
...

pub fn FilmModal<'a>(cx: Scope<'a, FilmModalProps>) -> Element<'a> {
    let is_modal_visible = use_shared_state::<FilmModalVisibility>(cx).unwrap();
    let draft_film = use_state::<Film>(cx, || Film {
        title: "".to_string(),
        poster: "".to_string(),
        director: "".to_string(),
        year: 1900,
        id: Uuid::new_v4(),
        created_at: None,
        updated_at: None,
    });

+   {
+       let draft_film = draft_film.clone();
+       use_effect(cx, &cx.props.film, |film| async move {
+           match film {
+               Some(film) => draft_film.set(film),
+               None => draft_film.set(Film {
+                   title: "".to_string(),
+                   poster: "".to_string(),
+                   director: "".to_string(),
+                   year: 1900,
+                   id: Uuid::new_v4(),
+                   created_at: None,
+                   updated_at: None,
+               }),
+           }
+       });
+   }

    ...
}
```

In essence, we are initiating an effect when the `film` property changes. If the `film` property is `Some(film)`, we set the `draft_film` state to the value of the `film` property. If the `film` property is `None`, we set the `draft_film` state to a new `Film` initial object.

## App Component

Next, we will adapt our `App` component to fetch the films from the API when the app is mounted or when we need to force the API to update the list of films. We'll accomplish this by modifying `force_get_films`. As this state has no type or initial value, it is solely used to trigger the effect.

We will also add HTTP request configurations to enable these functions. We will use the `reqwest` crate for this purpose, which can be added to our `Cargo.toml` file or installed with the following command:

```bash
cargo add reqwest
```

To streamline future requests, we will create a `films_endpoint()` function to return the URL of our API endpoint.

First install some missing dependencies by updating our `Cargo.toml`.

`front/Cargo.toml`
```diff
+reqwest = { version = "0.11.18", features = ["json"] }
+web-sys = "0.3.64"
+serde = { version = "1.0.164", features = ["derive"] }
```

After that, here are the necessary modifications for the `App` component:

`front/src/main.rs`
```diff
...

+const API_ENDPOINT: &str = "api/v1";

+fn films_endpoint() -> String {
+   let window = web_sys::window().expect("no global `window` exists");
+   let location = window.location();
+   let host = location.host().expect("should have a host");
+   let protocol = location.protocol().expect("should have a protocol");
+   let endpoint = format!("{}//{}/{}", protocol, host, API_ENDPOINT);
+   format!("{}/films", endpoint)
+}

+async fn get_films() -> Vec<Film> {
+   log::info!("Fetching films from {}", films_endpoint());
+   reqwest::get(&films_endpoint())
+       .await
+       .unwrap()
+       .json::<Vec<Film>>()
+       .await
+       .unwrap()
+}

fn App(cx: Scope) -> Element {
    ...
    let force_get_films = use_state(cx, || ());

+   {
+       let films = films.clone();


+       use_effect(cx, force_get_films, |_| async move {
+           let existing_films = get_films().await;
+           if existing_films.is_empty() {
+               films.set(None);
+           } else {
+               films.set(Some(existing_films));
+           }
+       });
+   }
}
```

What we have done here is trigger an effect whenever there is a need to fetch films from our API. We then evaluate whether there are any films available. If there are, we set the `films` state to these existing films. If not, we set the `films` state to `None`. This allows us to enhance our `App` component with additional functionality.
