#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
mod components;
mod models;

use components::{FilmCard, FilmModal, Header};
use dioxus::prelude::*;
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
    let films = use_state::<Option<Vec<Film>>>(cx, || None);
    // let is_modal_visible = use_shared_state::<bool>(cx);
    let force_get_films = use_state(cx, || ());

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

    cx.render(rsx! {
        main {
            class: "relative",
            Header {}
            FilmModal {}
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
                                        film: film
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

// pub fn RightArrowIcon(cx: Scope) -> Element {
//     cx.render(rsx!(
//         svg {
//             fill: "none",
//             stroke: "currentColor",
//             stroke_linecap: "round",
//             stroke_linejoin: "round",
//             stroke_width: "2",
//             class: "w-4 h-4 ml-1",
//             view_box: "0 0 24 24",
//             path { d: "M5 12h14M12 5l7 7-7 7"}
//         }
//     ))
// }
