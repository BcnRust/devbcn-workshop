use dioxus::prelude::*;

use crate::components::Button;
use crate::models::{ButtonType, FilmModalVisibility};

pub fn Header(cx: Scope) -> Element {
    let is_modal_visible = use_shared_state::<FilmModalVisibility>(cx).unwrap();

    cx.render(rsx!(
      header {
        class: "sticky top-0 z-10 text-gray-400 bg-blue-300 body-font shadow-md",
        div { class: "container mx-auto flex flex-wrap p-0 flex-col md:flex-row justify-between items-center",
            a {
                class: "flex title-font font-medium items-center text-teal-950 mb-4 md:mb-0",
                img {
                    class: "bg-transparent p-2 animate-jump",
                    alt: "ferris",
                    src: "ferris.png",
                    "loading": "lazy"
                }
                span { class: "ml-3 text-2xl", "Rusty films"}
            }
            Button {
                button_type: ButtonType::Primary,
                onclick: move |_| {
                    is_modal_visible.write().0 = true;
                },
                "Add new film"
            }
        }
      }
    ))
}
