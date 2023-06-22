use dioxus::prelude::*;

use crate::models::ButtonType;

#[inline_props]
pub fn Button<'a>(
    cx: Scope<'a>,
    button_type: ButtonType,
    onclick: EventHandler<'a, MouseEvent>,
    children: Element<'a>,
) -> Element {
    cx.render(rsx!(button {
        class: "{button_type.to_string()}",
        onclick: move |event| onclick.call(event),
        children
    }))
}
