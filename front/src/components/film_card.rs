use dioxus::prelude::*;
use shared::models::Film;

#[inline_props]
pub fn FilmCard<'a>(cx: Scope<'a>, film: &'a Film) -> Element {
    cx.render(rsx!(
      li {
        class: "film-card flex-1 p-4 rounded box-border bg-neutral-100 drop-shadow-md",
        img {
          src: "{film.poster}"
        },
        p {
          "{film.title}"
        }
        p {
          "{film.director}"
        }
        p {
          "{film.year}"
        }
      }
    ))
}
