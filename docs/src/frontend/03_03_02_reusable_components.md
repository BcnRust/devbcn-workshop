# Crafting Reusable Components

Let's turn up the heat in this section and start creating some more complex components for our app. Our assembly line will produce:

- A quick run-through on component props
- A Button that can be used anywhere in our app
- A Film Card to display details about a film
- A Film Modal for creating or updating films

## Props

Before we start building, let's break down how we're going to define props in our components. We'll be doing this using two methods: `struct` and `inline` Props. The main difference between them lies in their location. `struct` Props are defined outside in a struct with prop macros and we attach the generic to our `Scope` type. On the other hand, `inline` Props are tucked right into the component function params. If you're craving more details about this, you can have a peek at the [Dioxus Props documentation](https://dioxuslabs.com/docs/0.3/guide/en/describing_ui/component_props.html)

### Struct Props

These kinds of props are defined separately from the component function, and the generic type needs to be hooked onto the `Scope` type. We use the `#[derive(Props)]` macro to define the props:

```admonish tip title="Optional Props"
You can mark a prop as optional using `#[props(!optional)]`
```

```rust
#[derive(Props)]
pub struct FilmModalProps<'a> {
    on_create_or_update: EventHandler<'a, Film>,
    on_cancel: EventHandler<'a, MouseEvent>,
    #[props(!optional)]
    film: Option<Film>,
}

pub fn FilmModal<'a>(cx: Scope<'a, FilmModalProps>) -> Element<'a> {
    ...
}
```

### Inline Props

Inline props are defined within the component function params. A nice plus is that you can access the `prop` variable directly inside the component, while struct props need a bit of navigation like `cx.props.my_prop`.

For these props, we tag the component function with the `#[inline_props]` macro.

```rust
#[inline_props]
pub fn FilmCard<'a>(
    cx: Scope<'a>,
    film: &'a Film,
    on_edit: EventHandler<'a, MouseEvent>,
    on_delete: EventHandler<'a, MouseEvent>,
) -> Element {
    ...
}
```

Alright, now that we've got props figured out, let's start building some components!

```admonish info title="Props in RSX"
When you want to use props inside your components, here's how to do it: `"{cx.props.my_prop}"`, `"{my_prop}"`, or `"{prop.to_string()}"`. Make sure to keep the curly braces and the prop name as shown.
```

## Button

First up, we're creating a button. Since we'll be using this in various spots, it's a smart move to make it a reusable component.

`front/src/components/button.rs`
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

Button props are pretty straightforward.
- `button_type` prop that takes a `ButtonType` enum and assign the right Tailwind classes to the button.
- `onclick` prop that takes an `EventHandler` for the click event, and a
- `children` prop that takes an `Element` for the button text, icon or whatever `Element` desired.

Just like we did with the components, we're going to set up a models folder inside our frontend directory. Here, we'll create a `button.rs` file to hold our Button models. While we're at it, let's also create a `film.rs` file for our Film models. We'll need those soon!

```bash
└── src                # Source code
    ├── models         # Models folder
    │   ├── mod.rs     # Models module
    │   ├── button.rs  # Button models
    │   └── film.rs    # Film models
```

Here's what we're working with for these files:

`front/src/models/mod.rs`
```rust
mod button;
mod film;

pub use button::ButtonType;
pub use film::FilmModalVisibility;
```

`front/src/models/button.rs`
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

`front/src/models/film.rs`
```rust
pub struct FilmModalVisibility(pub bool);
```

But wait, what's that `impl` thing in `button.rs`? This is where Rust's implementation blocks come in. We're using `impl` to add methods to our `ButtonType` enum. Specifically, we're implementing the `Display` trait, which gives us a standard way to display our enum as a string. The `fmt` method determines how each variant of the enum should be formatted as a string. So, when we use `button_type.to_string()` in our Button component, it will return the right Tailwind classes based on the button type. Handy, right?

### Update components module
Add the `button` module to the `components` module.

`front/src/components/mod.rs`
```diff
+mod button;
mod footer;
mod header;

+pub use button::Button;
pub use footer::Footer;
pub use header::Header;
```

```admonish warning title="Add shared models dependency"
We should add `shared = { path = "../shared" }` inside our [dependencies] on front's `Cargo.toml` file.

```

## Film Card

Moving along, our next creation is the Film Card component. Its role is to present the specifics of a film in our list. Moreover, it will integrate a pair of Button components allowing us to edit and delete the film.

`front/src/components/film_card.rs`
```rust
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
```

This Film Card component is indeed more intricate than the Button component, due to its wider use of Tailwind classes and the incorporation of event handlers. Let's dissect this a bit:

- `on_edit` and `on_delete` are event handlers that we introduce into the component. They are responsible for managing the click events on the edit and delete buttons respectively.
- `film` is a reference to the film whose details we are exhibiting in the card.

## Film Modal

As the grand finale of our components building phase, we're constructing the Film Modal component. This vital piece will facilitate the creation or update of a film. Its appearance will be commanded by a button located in the app's header or the `edit` button inside the Film Card.

`front/src/components/film_modal.rs`
```rust
use dioxus::prelude::*;

use crate::components::Button;
use crate::models::{ButtonType};

#[derive(Props)]
pub struct FilmModalProps<'a> {
    on_create_or_update: EventHandler<'a, MouseEvent>,
    on_cancel: EventHandler<'a, MouseEvent>,
}

pub fn FilmModal<'a>(cx: Scope<'a, FilmModalProps>) -> Element<'a> {
    cx.render(rsx!(
        article {
            class: "z-50 w-full h-full fixed top-0 right-0 bg-gray-800 bg-opacity-50 flex flex-col justify-center items-center",
            section {
                class: "w-1/3 h-auto bg-white rounded-lg flex flex-col justify-center items-center box-border p-6",
                header {
                    class: "mb-4",
                    h2 {
                        class: "text-xl text-teal-950 font-semibold",
                        "🎬 Film"
                    }
                }
                form {
                    class: "w-full flex-1 flex flex-col justify-stretch items-start gap-y-2",
                    div {
                        class: "w-full",
                        label {
                            class: "text-sm font-semibold",
                            "Title"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "text",
                            placeholder: "Enter film title",
                        }
                    }
                    div {
                        class: "w-full",
                        label {
                            class: "text-sm font-semibold",
                            "Director"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "text",
                            placeholder: "Enter film director",
                        }
                    }
                    div {
                        class: "w-full",
                        label {
                            class: "text-sm font-semibold",
                            "Year"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "number",
                            placeholder: "Enter film year",
                        }
                    }
                    div {
                        class: "w-full",
                        label {
                            class: "text-sm font-semibold",
                            "Poster"
                        }
                        input {
                            class: "w-full border border-gray-300 rounded-lg p-2",
                            "type": "text",
                            placeholder: "Enter film poster URL",
                        }
                    }
                }
                footer {
                    class: "flex flex-row justify-center items-center mt-4 gap-x-2",
                    Button {
                        button_type: ButtonType::Secondary,
                        onclick: move |evt| {
                            cx.props.on_cancel.call(evt)
                        },
                        "Cancel"
                    }
                    Button {
                        button_type: ButtonType::Primary,
                        onclick: move |evt| {
                            cx.props.on_create_or_update.call(evt);
                        },
                        "Save film"
                    }
                }
            }

        }
    ))
}
```

At the moment, we're primarily focusing on establishing the basic structural framework of the modal. We'll instill the logic in the upcoming section. The current modal props comprise on_create_or_update and on_cancel. These event handlers are key to managing the click events associated with modal actions.

- `on_create_or_update`: This handler is in charge of creating or updating a film.
- `on_cancel`: This one takes responsibility for shutting down the modal and aborting any ongoing film modification or creation.

Let's update our `main.rs` file to include the Film Modal component. Film Card component will be added later.

`front/src/main.rs`
```diff
#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
+mod components;
+mod models;
...

-use components::{Footer, Header};
+use components::{FilmModal, Footer, Header};

...

fn App(cx: Scope) -> Element {
    ...
    cx.render(rsx! {
        main {
            ...
+           FilmModal {
+               on_create_or_update: move |_| {},
+               on_cancel: move |_| {}
+           }
        }
    })
}
```