# Components

Alright, let's roll up our sleeves and dive into building some reusable components for our app. We'll start with layout components and then craft some handy components that we can use all over our app.

When you're putting together a component, keep these points in mind:
- Always remember to import `dioxus::prelude::*`. This gives you all the macros and functions you need, right at your fingertips.
- Create a `pub fn` with your chosen component name.
- Your function should include a `cx: Scope` parameter.
- It should return an `Element` type.

The real meat of our component is in the `cx.render` function. This is where the `rsx!` macro comes into play to create the markup of the component. You can put together your markup using html tags, attributes, and text.

Inside html tags, you can go wild with any attributes you want. Dioxus has a ton of them ready for you to use. But if you can't find what you're looking for, no problem! You can add it yourself using "double quotes".

```rust
use dioxus::prelude::*;

pub fn MyComponent(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "my-component",
            "data-my-attribute": "my value",
            "My component"
        }
    ))
}
```
