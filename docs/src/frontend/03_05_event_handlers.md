# Event Handlers

Event handlers are crucial elements in an interactive application. These functions are invoked in response to certain user events like mouse clicks, keyboard input, or form submissions.

In the final section of this guide, we will introduce interactivity to our application by implementing creation, updating, and deletion of film actions. For this, we will be spawning `futures` using `cx.spawn` and `async move` closures. It is crucial to remember that `use_state` values should be cloned before being used in `async move` closures.

## delete_film Function

This function will be triggered when a user clicks the delete button of a film card. It will send a `DELETE` request to our API and subsequently call `force_get_films` to refresh the list of films. In the event of a successful operation, a message will be logged to the console. If an error occurs, the error will be logged instead.

```rust
let delete_film = move |filmId| {
    let force_get_films = force_get_films.clone();
    cx.spawn({
        async move {
            let response = reqwest::Client::new()
                .delete(&format!("{}/{}", &films_endpoint(), filmId))
                .send()
                .await;
            match response {
                Ok(_data) => {
                    log::info!("Film deleted");
                    force_get_films.set(());
                }
                Err(err) => {
                    log::info!("Error deleting film: {:?}", err);
                }
            }
        }
    });
};
```

## create_or_update_film Function

This function is invoked when the user clicks the create or update button of the film modal. It sends a `POST` or `PUT` request to our API, followed by a call to `force_get_films` to update the list of films. The decision to edit or create a film depends on whether the `selected_film` state is `Some(film)` or `None`.

In case of success, a console message is logged, the `selected_film` state is reset, and the modal is hidden. If an error occurs, the error is logged.

```rust
let create_or_update_film = move |film: Film| {
    let force_get_films = force_get_films.clone();
    let current_selected_film = selected_film.clone();
    let is_modal_visible = is_modal_visible.clone();

    cx.spawn({
        async move {
            let response = if current_selected_film.get().is_some() {
                reqwest::Client::new()
                    .put(&films_endpoint())
                    .json(&film)
                    .send()
                    .await
            } else {
                reqwest::Client::new()
                    .post(&films_endpoint())
                    .json(&film)
                    .send()
                    .await
            };
            match response {
                Ok(_data) => {
                    log::info!("Film created");
                    current_selected_film.set(None);
                    is_modal_visible.write().0 = false;
                    force_get_films.set(());
                }
                Err(err) => {
                    log::info!("Error creating film: {:?}", err);
                }
            }
        }
    });
};
```

## Final Adjustments

All the subsequent modifications will be implemented on our `App` component.

`front/src/main.rs`
```diff
...

fn App(cx: Scope) -> Element {
    ...
    {
        let films = films.clone();
        use_effect(cx, force_get_films, |_| async move {
            let existing_films = get_films().await;
            if existing_films.is_empty() {
                films.set(None);
            } else {
                films.set(Some(existing_films));


            }
        });
    }

+   let delete_film = move |filmId| {
+       let force_get_films = force_get_films.clone();
+       cx.spawn({
+           async move {
+               let response = reqwest::Client::new()
+                   .delete(&format!("{}/{}", &films_endpoint(), filmId))
+                   .send()
+                   .await;
+               match response {
+                   Ok(_data) => {
+                       log::info!("Film deleted");
+                       force_get_films.set(());
+                   }
+                   Err(err) => {
+                       log::info!("Error deleting film: {:?}", err);
+                   }
+               }
+           }
+       });
+   };

+   let create_or_update_film = move |film: Film| {
+       let force_get_films = force_get_films.clone();
+       let current_selected_film = selected_film.clone();
+       let is_modal_visible = is_modal_visible.clone();
+       cx.spawn({
+           async move {
+               let response = if current_selected_film.get().is_some() {
+                   reqwest::Client::new()
+                       .put(&films_endpoint())
+                       .json(&film)
+                       .send()
+                       .await
+               } else {
+                   reqwest::Client::new()
+                       .post(&films_endpoint())
+                       .json(&film)
+                       .send()
+                       .await
+               };
+               match response {
+                   Ok(_data) => {
+                       log::info!("Film created");
+                       current_selected_film.set(None);
+                       is_modal_visible.write().0 = false;
+                       force_get_films.set(());
+                   }
+                   Err(err) => {
+                       log::info!("Error creating film: {:?}", err);
+                   }
+               }
+           }
+       });
+   };

    cx.render(rsx! {
        ...
        section {
                class: "md:container md:mx-auto md:py-8 flex-1",
                   rsx!(
                if let Some(films) = films.get() {
                       ul {
                          class: "flex flex-row justify-center items-stretch gap-4 flex-wrap",
                          {films.iter().map(|film| {
                              rsx!(
                                   FilmCard {
                                       key: "{film.id}",
                                       film: film,
                                       on_edit: move |_| {
                                           selected_film.set(Some(film.clone()));
                                           is_modal_visible.write().0 = true
                                       },
-                                      on_delete: move |_| {}
+                                      on_delete: move |_| {
+                                           delete_film(film.id);
+                                      }
                                   }
                               )
                           })}
                       }
                   )
               }
            }
        FilmModal {
            film: selected_film.get().clone(),
-           on_create_or_update: move |new_film| {},
+           on_create_or_update: move |new_film| {
+               create_or_update_film(new_film);
+           },
            on_cancel: move |_| {
                selected_film.set(None);
                is_modal_visible.write().0 = false;
            }
        }
    })
}
```

Upon successful implementation of the above changes, the application should now have the capability to create, update, and delete films.