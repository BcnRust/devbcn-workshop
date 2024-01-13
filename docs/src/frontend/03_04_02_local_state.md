# Local state

In the context of component state, we typically refer to the local state. Dioxus simplifies the management of a component's state with the `use_state` hook. Noteworthy characteristics of this hook include:

- State initialization is achieved by passing a closure that returns the initial state.
```rust
let mut count = use_state(cx, || 0);
```
- The `use_state` hook provides the current value via `get()` and enables its modification using `set()`.
- Each value update triggers a component re-render.

In the `main.rs` file, the `App` component needs to be updated to introduce some local state. This state will be situated at the top of our app and can be passed to components as props. Our app's local states will consist of:

- `films`: A list of films.
- `selected_film`: The film to be updated.
- `force_get_films`: A flag that will be employed to force a refetch of the films list from the API.

```admonish
We are going to apply dynamic rendering again, this time to render a list of Film Cards only if the films list is not empty.
```

`front/src/main.rs`
```diff
...
-use components::{FilmModal, Footer, Header};
+use components::{FilmCard, FilmModal, Footer, Header};
use dioxus::prelude::*;
use models::FilmModalVisibility;
+use shared::models::Film;

...

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || FilmModalVisibility(false));
    let is_modal_visible = use_shared_state::<FilmModalVisibility>(cx).unwrap();
+   let films = use_state::<Option<Vec<Film>>>(cx, || None);
+   let selected_film = use_state::<Option<Film>>(cx, || None);
+   let force_get_films = use_state(cx, || ());

...

    cx.render(rsx! {
        main {
        ...
            section {
                class: "md:container md:mx-auto md:py-8 flex-1",
+               if let Some(films) = films.get() {
+                  rsx!(
+                      ul {
+                         class: "flex flex-row justify-center items-stretch gap-4 flex-wrap",
+                         {films.iter().map(|film| {
+                             rsx!(
+                                  FilmCard {
+                                      key: "{film.id}",
+                                      film: film,
+                                      on_edit: move |_| {
+                                          selected_film.set(Some(film.clone()));
+                                          is_modal_visible.write().0 = true
+                                      },
+                                      on_delete: move |_| {}
+                                  }
+                              )
+                          })}
+                      }
+                  )
+              }
            }
        ...
        }
        FilmModal {
+           film: selected_film.get().clone(),
            on_create_or_update: move |new_film| {},
            on_cancel: move |_| {
+               selected_film.set(None);
+               is_modal_visible.write().0 = false;
            }
        }
    })
}
```

As you can observe, the Film Modal is opened when the `FilmCard` edit button is clicked. Additionally, the selected **film** is passed as a prop to the `FilmModal` component.

We will implement the delete film feature later.

The `FilmModal` component also undergoes an update in the `on_cancel` callback to clear the selected film and close the modal, in case we decide not to create or update a film.

```admonish tip title="Passing state as props"
We utilize the `clone` method to generate a copy of the selected film. This is because we're employing the same film object in the `FilmCard`. Check [Clone documentation](https://doc.rust-lang.org/rust-by-example/trait/clone.html) from Rust by Example to learn more about the `clone` method.
```

Finally, it's essential to modify the `FilmModal` component to:

- Accept the selected film as a prop.
- Add a `draft_film` local state to contain the film that will be created or updated.
- Refresh the `on_cancel` callback to clear the `draft_film` and close the modal.
- Update the

 `on_create_or_update` callback to create or update the `draft_film` and close the modal.
- Assign values and change handlers to the input fields.

`front/src/components/film_modal.rs`
```diff
use dioxus::prelude::*;
+use shared::models::Film;
+use uuid::Uuid;

use crate::components::Button;
use crate::models::{ButtonType, FilmModalVisibility};

#[derive(Props)]
pub struct FilmModalProps<'a> {
-   on_create_or_update: EventHandler<'a, MouseEvent>,
+   on_create_or_update: EventHandler<'a, Film>,
    on_cancel: EventHandler<'a, MouseEvent>,
+   #[props(!optional)]
+   film: Option<Film>,
}

pub fn FilmModal<'a>(cx: Scope<'a, FilmModalProps>) -> Element<'a> {
    let is_modal_visible = use_shared_state::<FilmModalVisibility>(cx).unwrap();
+   let draft_film = use_state::<Film>(cx, || Film {
+       title: "".to_string(),
+       poster: "".to_string(),
+       director: "".to_string(),
+       year: 1900,
+       id: Uuid::new_v4(),
+       created_at: None,
+       updated_at: None,
+   });

    if !is_modal_visible.read().0 {
        return None;
    }
    cx.render(rsx!(
        article {
            class: "z-50 w-full h-full fixed top-0 right-0 bg-gray-800 bg-opacity-50 flex flex-col justify-center items-center",
            section {
                class: "w-1/3 h-auto bg-white rounded-lg flex flex-col justify-center items-center box-border p-6",
                header {
                    class: "mb-4",
                    h2 {
                        class: "text-xl text-teal-950 font-semibold",
                        "🎬 Film"
                    }
                }
                form {
                    class: "w-full flex-1 flex flex-col justify-stretch items-start gap-y-2",
                    div {
                        class: "w-full",
                        label {
                            class: "text-sm font-semibold",
                            "Title"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "text",
                            placeholder: "Enter film title",
+                           value: "{draft_film.get().title}",
+                           oninput: move |evt| {
+                               draft_film.set(Film {
+                                   title: evt.value.clone(),
+                                   ..draft_film.get().clone()
+                               })
+                           }
                        }
                    }
                    div {
                        class: "w-full",
                        label {
                            class: "text-sm font-semibold",
                            "Director"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "text",
                            placeholder: "Enter film director",
+                           value: "{draft_film.get().director}",
+                           oninput: move |evt| {
+                               draft_film.set(Film {
+                                   director: evt.value.clone(),
+                                   ..draft_film.get().clone()
+                               })
+                           }
                        }
                    }
                    div {
                        class: "w-full",
                        label {
                            class: "text-sm font-semibold",
                            "Year"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "number",
                            placeholder: "Enter film year",
+                           value: "{draft_film.get().year.to_string()}",
+                           oninput: move |evt| {
+                               draft_film.set(Film {
+                                   year: evt.value.clone().parse::<u16>().unwrap_or(1900),
+                                   ..draft_film.get

().clone()
+                               })
+                           }
                        }
                    }
                    div {
                        class: "w-full",
                        label {
                            class: "text-sm font-semibold",
                            "Poster"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "text",
                            placeholder: "Enter film poster URL",
+                           value: "{draft_film.get().poster}",
+                           oninput: move |evt| {
+                               draft_film.set(Film {
+                                   poster: evt.value.clone(),
+                                   ..draft_film.get().clone()
+                               })
+                           }
                        }
                    }
                }
                footer {
                    class: "flex flex-row justify-center items-center mt-4 gap-x-2",
                    Button {
                        button_type: ButtonType::Secondary,
                        onclick: move |evt| {
+                           draft_film.set(Film {
+                               title: "".to_string(),
+                               poster: "".to_string(),
+                               director: "".to_string(),
+                               year: 1900,
+                               id: Uuid::new_v4(),
+                               created_at: None,
+                               updated_at: None,
+                           });
                            cx.props.on_cancel.call(evt)
                        },
                        "Cancel"
                    }
                    Button {
                        button_type: ButtonType::Primary,
                        onclick: move |evt| {
-                           cx.props.on_create_or_update.call(evt);
+                           cx.props.on_create_or_update.call(draft_film.get().clone());
+                           draft_film.set(Film {
+                               title: "".to_string(),
+                               poster: "".to_string(),
+                               director: "".to_string(),
+                               year: 1900,
+                               id: Uuid::new_v4(),
+                               created_at: None,
+                               updated_at: None,
+                           });
                        },
                        "Save film"
                    }
                }
            }

        }
    ))
}
```

Finally add `uuid` dependency to the `Cargo.toml` file.

`front/Cargo.toml`
```diff
...
[dependencies]
# shared
shared = { path = "../shared" }
# dioxus
dioxus = "0.4.3"
dioxus-web = "0.4.3"
wasm-logger = "0.2.0"
+uuid = { version = "1.3.4", features = ["serde", "v4", "js"] }
```
