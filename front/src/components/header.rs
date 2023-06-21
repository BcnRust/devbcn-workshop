use dioxus::prelude::*;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx!(
      header {
        class: "text-gray-400 bg-amber-950 body-font",
        div { class: "container mx-auto flex flex-wrap p-5 flex-col md:flex-row items-center",
            a {
                class: "flex title-font font-medium items-center text-white mb-4 md:mb-0",
                img {
                    class: "w-20 h-20 bg-transparent p-2 animate-jump",
                    alt: "ferris",
                    src: "ferris.png"
                }
                span { class: "ml-3 text-xl", "Rusty films"}
            }
        }
      }
    ))
}
