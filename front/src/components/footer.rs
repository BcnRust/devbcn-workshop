use dioxus::prelude::*;

pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx!(
        footer {
            class: "bg-blue-200 w-full h-16 p-2 box-border gap-6 flex flex-row justify-center items-center text-teal-950",
            a {
                class: "w-auto h-full",
                href: "https://www.devbcn.com/",
                target: "_blank",
                img {
                    class: "h-full w-auto",
                    alt: "DevBcn",
                    src: "devbcn.png",
                    "loading": "lazy"
                }
            }
            svg {
                fill: "none",
                view_box: "0 0 24 24",
                stroke_width: "1.5",
                stroke: "currentColor",
                class: "w-6 h-6",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M6 18L18 6M6 6l12 12"
                }
            }
            a {
                class: "w-auto h-full",
                href: "https://www.meetup.com/es-ES/bcnrust/",
                target: "_blank",
                img {
                    class: "h-full w-auto",
                    alt: "BcnRust",
                    src: "bcnrust.png",
                    "loading": "lazy"
                }
            }
        }
    ))
}
