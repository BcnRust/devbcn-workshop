use crate::{components::Button, models::ButtonType};
use dioxus::prelude::*;
use shared::models::Film;

#[inline_props]
pub fn FilmCard<'a>(
    cx: Scope<'a>,
    film: &'a Film,
    on_edit: EventHandler<'a, MouseEvent>,
    on_delete: EventHandler<'a, MouseEvent>,
) -> Element {
    cx.render(rsx!(
        li {
            class: "film-card md:basis-1/4 p-4 rounded box-border bg-neutral-100 drop-shadow-md transition-all ease-in-out hover:drop-shadow-xl flex-col flex justify-start items-stretch animate-fade animate-duration-500 animate-ease-in-out animate-normal animate-fill-both",
            header {
                img {
                    class: "max-h-80 w-auto mx-auto rounded",
                    src: "{film.poster}"
                },
            }
            section {
                class: "flex-1",
                h3 {
                    class: "text-lg font-bold my-3",
                    "{film.title}"
                }
                p {
                    "{film.director}"
                }
                p {
                    class: "text-sm text-gray-500",
                    "{film.year.to_string()}"
                }
            }
            footer {
                class: "flex justify-end space-x-2 mt-auto",
                Button {
                    button_type: ButtonType::Secondary,
                    onclick: move |event| on_delete.call(event),
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1.5",
                        view_box: "0 0 24 24",
                        class: "w-5 h-5",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0"
                        }
                    }
                }
                Button {
                    button_type: ButtonType::Primary,
                    onclick: move |event| on_edit.call(event),
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "1.5",
                        view_box: "0 0 24 24",
                        class: "w-5 h-5",
                        path {
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            d: "M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L6.832 19.82a4.5 4.5 0 01-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 011.13-1.897L16.863 4.487zm0 0L19.5 7.125"
                        }
                    }
                }
            }
        }
    ))
}
