#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
mod components;

use components::{FilmCard, Header};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

// const HOST: &str = "https://devbcn.shuttleapp.rs/api/v1";
const HOST: &str = "http://localhost:5000/api/v1";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct Film {
    pub id: String,
    pub title: String,
    pub director: String,
    pub year: u16,
    pub poster: String,
}

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
    let films = use_state::<Option<Vec<Film>>>(cx, || None);
    let force_get_films = use_state(&cx, || ());

    {
        let films = films.clone();
        use_effect(&cx, force_get_films, |_| async move {
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
            Header {}
            section {
                class: "shadow-xl",
                if let Some(films) = films.get() {
                    rsx!(
                        ul {
                            {films.iter().map(|film| {
                                rsx!(
                                    FilmCard {
                                        key: "{film.id}",
                                        poster: "{film.poster}",
                                        title: "{film.title}"
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
