use dioxus::prelude::*;
use shared::models::Film;
use uuid::Uuid;

use crate::components::Button;
use crate::models::{ButtonType, FilmModalVisibility};

#[inline_props]
pub fn FilmModal<'a>(
    cx: Scope<'a>,
    on_create: EventHandler<'a, Film>,
    film: Option<Film>,
) -> Element {
    let is_modal_visible = use_shared_state::<FilmModalVisibility>(cx).unwrap();
    let draft_film = use_state::<Film>(cx, || match film.clone() {
        Some(film) => film.clone(),
        None => Film {
            title: "".to_string(),
            poster: "".to_string(),
            director: "".to_string(),
            year: 1900,
            id: Uuid::new_v4(),
            created_at: None,
            updated_at: None,
        },
    });

    if !is_modal_visible.read().0 {
        return None;
    }
    cx.render(rsx!(
        article {
            class: "z-50 w-screen h-screen absolute top-0 right-0 bg-gray-800 bg-opacity-50 flex flex-col justify-center items-center",
            section {
                class: "w-1/3 h-1/2 bg-white rounded-lg flex flex-col justify-center items-center box-border p-6",
                header {
                    class: "mb-4",
                    h2 {
                        class: "text-xl font-semibold",
                        "🎬 New film"
                    }
                }
                form {
                    class: "w-full flex-1 flex flex-col justify-stretch items-start gap-y-2",
                    div {
                        label {
                            class: "text-sm font-semibold",
                            "Title"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "text",
                            placeholder: "Enter film title",
                            value: "{draft_film.get().title}",
                            oninput: move |evt| {
                                draft_film.set(Film {
                                    title: evt.value.clone(),
                                    ..draft_film.get().clone()
                                })
                            }
                        }
                    }
                    div {
                        label {
                            class: "text-sm font-semibold",
                            "Director"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "text",
                            placeholder: "Enter film director",
                            value: "{draft_film.get().director}",
                            oninput: move |evt| {
                                draft_film.set(Film {
                                    director: evt.value.clone(),
                                    ..draft_film.get().clone()
                                })
                            }
                        }
                    }
                    div {
                        label {
                            class: "text-sm font-semibold",
                            "Year"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "number",
                            placeholder: "Enter film year",
                            value: "{draft_film.get().year.to_string()}",
                            oninput: move |evt| {
                                draft_film.set(Film {
                                    year: evt.value.clone().parse::<u16>().unwrap(),
                                    ..draft_film.get().clone()
                                })
                            }
                        }
                    }
                    div {
                        label {
                            class: "text-sm font-semibold",
                            "Poster"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "text",
                            placeholder: "Enter film poster URL",
                            value: "{draft_film.get().poster}",
                            oninput: move |evt| {
                                draft_film.set(Film {
                                    poster: evt.value.clone(),
                                    ..draft_film.get().clone()
                                })
                            }
                        }
                    }
                }
                footer {
                    class: "flex flex-row justify-center items-center mt-4 gap-x-2",
                    Button {
                        button_type: ButtonType::Secondary,
                        onclick: move |_| {
                            is_modal_visible.write().0 = false;
                        },
                        "Cancel"
                    }
                    Button {
                        button_type: ButtonType::Primary,
                        onclick: move |_| {
                            on_create.call(draft_film.get().clone());
                            draft_film.set(Film {
                                title: "".to_string(),
                                poster: "".to_string(),
                                director: "".to_string(),
                                year: 1900,
                                id: Uuid::new_v4(),
                                created_at: None,
                                updated_at: None,
                            })
                        },
                        "Create film"
                    }
                }
            }

        }
    ))
}
