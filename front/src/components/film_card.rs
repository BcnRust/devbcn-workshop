use dioxus::prelude::*;

#[inline_props]
pub fn FilmCard<'a>(cx: Scope<'a>, poster: &'a str, title: &'a str) -> Element {
    cx.render(rsx!(
      li {
        img {
          src: "{poster}"
        },
        "{title}"
      }
    ))
}
