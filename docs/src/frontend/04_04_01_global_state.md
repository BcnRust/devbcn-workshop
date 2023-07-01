# Global state

Let's start simple. We will create a global state that will be responsible to store our Film Modal visibility.

For accomplish this we will use the something similar as React Context. This hook allows us to create a context that will be available for all the components that are inside the context provider. So we will create a `use_shared_state_provider` that will be placed inside our `App` component.

It should initialize the value using a closure.

`main.rs`
```diff
+mod models;

use components::{FilmModal, Footer, Header};
use dioxus::prelude::*;
+use models::FilmModalVisibility;
...

fn App(cx: Scope) -> Element {
+    use_shared_state_provider(cx, || FilmModalVisibility(false));

...

}
```

Now we can use the `use_shared_state` hook to get the state and update it. So let's also add this hook where we need to be able to read or update the Film Modal visibility.


`header.rs`
```diff
...

pub fn Header(cx: Scope) -> Element {
+   let is_modal_visible = use_shared_state::<FilmModalVisibility>(cx).unwrap();

...


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
```

`film_modal.rs`
```diff
...
pub fn FilmModal<'a>(cx: Scope<'a, FilmModalProps>) -> Element<'a> {
+   let is_modal_visible = use_shared_state::<FilmModalVisibility>(cx).unwrap();

...
+    if !is_modal_visible.read().0 {
+        return None;
+    }
...
}
```

In this case we also can see other concept of Dioxus, dynamic rendering. Basically we are rendering the component only if the condition is true.
> **Note:** Dynamic rendering is a technique that allows you to render different content depending on a condition. You can find more information on [Dioxus Dynamic Rendering documentation](https://dioxuslabs.com/docs/0.3/guide/en/interactivity/dynamic_rendering.html)

`main.rs`
```diff
...

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || FilmModalVisibility(false));
+   let is_modal_visible = use_shared_state::<FilmModalVisibility>(cx).unwrap();


    ...
    cx.render(rsx! {
        main {
            ...
            FilmModal {
               on_create_or_update: move |_| {},
               on_cancel: move |_| {
+                  is_modal_visible.write().0 = false;
                }
            }
        }
    })
}
```

