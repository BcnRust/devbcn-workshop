# Creating Reusable Components

In this section, we'll be stepping up our game a bit and creating the rest of the components for our app. The lineup includes:

- A Button that we can use all over the place
- A Film Card for showcasing films
- A Film Modal for creating or updating films

## Button

First up, we're tackling the button. Since we'll be using this in various spots, it's a smart move to make it a reusable component.

```rust
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
        class: "text-slate-200 inline-flex items-center border-0 py-1 px-3 focus:outline-none rounded mt-4 md:mt-0 {button_type.to_string()}",
        onclick: move |event| onclick.call(event),
        children
    }))
}
```

Notice that we're importing `models::ButtonType` here. This is an enum that helps us define the different button types we might use in our app. By using this, we can easily switch up the button styles based on our needs.

Just like we did with the components, we're going to set up a models folder inside our frontend directory. Here, we'll create a `button.rs` file to hold our Button models. While we're at it, let's also create a `film.rs` file for our Film models. We'll need those soon!

```bash
└── src                # Source code
    ├── models         # Models folder
    │   ├── mod.rs     # Models module
    │   ├── button.rs  # Button models
    │   └── film.rs    # Film models
```

Here's what we're working with for these files:

`mod.rs`
```rust
mod button;
mod film;

pub use button::ButtonType;
pub use film::FilmModalVisibility;
```

`button.rs`
```rust
use std::fmt;

pub enum ButtonType {
    Primary,
    Secondary,
}

impl fmt::Display for ButtonType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ButtonType::Primary => write!(f, "bg-blue-700 hover:bg-blue-800 active:bg-blue-900"),
            ButtonType::Secondary => write!(f, "bg-rose-700 hover:bg-rose-800 active:bg-rose-900"),
        }
    }
}
```

`film.rs`
```rust
pub struct FilmModalVisibility(pub bool);
```

But wait, what's that `impl` thing in `button.rs`? This is where Rust's implementation blocks come in. We're using `impl` to add methods to our `ButtonType` enum. Specifically, we're implementing the `Display` trait, which gives us a standard way to display our enum as a string. The `fmt` method determines how each variant of the enum should be formatted as a string. So, when we use `button_type.to_string()` in our Button component, it will return the right Tailwind classes based on the button type. Handy, right?