use dioxus::prelude::*;

use crate::components::Button;
use crate::models::{ButtonType, FilmModalVisibility};

// pub title: String,
//     pub director: String,
//     #[cfg_attr(feature = "backend", sqlx(try_from = "i16"))]
//     pub year: u16,
//     pub poster: String,

#[inline_props]
pub fn FilmModal(cx: Scope) -> Element {
    let is_modal_visible = use_shared_state::<FilmModalVisibility>(cx).unwrap();

    if !is_modal_visible.read().0 {
        return None;
    }
    cx.render(rsx!(
        article {
            class: "w-screen h-screen absolute top-0 right-0 bg-gray-800 bg-opacity-50 flex flex-col justify-center items-center",
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
                            placeholder: "Enter film title"
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
                            placeholder: "Enter film director"
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
                            placeholder: "Enter film year"
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
                            placeholder: "Enter film poster URL"
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
                            is_modal_visible.write().0 = false;
                        },
                        "Save film"
                    }
                }
            }

        }
    ))
}
