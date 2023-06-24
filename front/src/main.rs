#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
mod components;
mod models;

use components::{FilmCard, FilmModal, Header};
use dioxus::{html::s, prelude::*};
use models::FilmModalVisibility;
use shared::models::Film;

// const HOST: &str = "https://devbcn.shuttleapp.rs/api/v1";
const HOST: &str = "http://localhost:5000/api/v1";

fn films_endpoint() -> String {
    format!("{}/films", HOST)
}

async fn get_films() -> Vec<Film> {
    reqwest::get(&films_endpoint())
        .await
        .unwrap()
        .json::<Vec<Film>>()
        .await
        .unwrap()
}

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || FilmModalVisibility(false));
    let is_modal_visible = use_shared_state::<FilmModalVisibility>(cx).unwrap();
    let films = use_state::<Option<Vec<Film>>>(cx, || None);
    let selected_film = use_state::<Option<Film>>(cx, || None);
    let force_get_films = use_state(cx, || ());
    let test_film = Film {
        title: "Test".to_string(),
        poster: "https://picsum.photos/200/300".to_string(),
        director: "Test".to_string(),
        year: 2021,
        id: uuid::Uuid::new_v4(),
        created_at: None,
        updated_at: None,
    };

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

    let create_film = move |film: Film| {
        let force_get_films = force_get_films.clone();
        // let selected_film = selected_film.clone();
        let is_modal_visible = is_modal_visible.clone();

        cx.spawn({
            async move {
                let response = reqwest::Client::new()
                    .post(&films_endpoint())
                    .json(&film)
                    .send()
                    .await;
                match response {
                    Ok(_data) => {
                        log::info!("Film created");
                        // selected_film.set(None);
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

    cx.render(rsx! {
        main {
            class: "relative z-0",
            Header {}
            // if selected_film.get().is_some() {
            //     let selected = selected_film.get().clone();
            //     cx.render(rsx! {
            //         FilmModal {
            //                 film: &selected.unwrap_or(None)
            //         }
            //     })
            // } else {
            //     cx.render(rsx! {
            //         FilmModal {}
            //     })
            // }
            // FilmModal {
            //     film: test_film
            // }
            FilmModal {
                on_create: move |new_film| {
                    create_film(new_film);
                },
            }
            section {
                class: "md:container md:mx-auto p-8 box-border",
                if let Some(films) = films.get() {
                    rsx!(
                        ul {
                            class: "flex flex-row justify-center items-start",
                            {films.iter().map(|film| {
                                rsx!(
                                    FilmCard {
                                        key: "{film.id}",
                                        film: film,
                                        on_edit: move |_| {
                                            selected_film.set(Some(film.clone()));
                                            is_modal_visible.write().0 = true
                                        },
                                        on_delete: move |_| {
                                            delete_film(film.id)
                                        }
                                    }
                                )
                            })}
                        }
                    )
                }
            }
        }
    })
}
