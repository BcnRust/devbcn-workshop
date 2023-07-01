# Implementing Global State

To begin, let's create a global state responsible for managing the visibility of our Film Modal. 

We will utilize a functionality similar to React's Context. This approach allows us to establish a context that will be accessible to all components contained within the context provider. To this end, we will construct a `use_shared_state_provider` that will be located within our `App` component.

The value should be initialized using a closure.

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

Now, by leveraging the `use_shared_state` hook, we can both retrieve the state and modify it. Therefore, it is necessary to incorporate this hook in locations where we need to read or alter the Film Modal visibility.

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

The value can be updated using the `write` method, which returns a mutable reference to the value. Consequently, we can use the `=` operator to update the visibility of the Film Modal when the button is clicked.

`film_modal.rs`
```diff
...
-use crate::models::{ButtonType};
+use crate::models::{ButtonType, FilmModalVisibility};
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

This demonstrates an additional concept of Dioxus: dynamic rendering. Essentially, the component is only rendered if the condition is met.
> **Note:** Dynamic rendering is a technique that enables rendering different content based on a condition. Further information can be found in the [Dioxus Dynamic Rendering documentation](https://dioxuslabs.com/docs/0.3/guide/en/interactivity/dynamic_rendering.html)

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
In the same manner we open the modal by altering the value, we can also close it. Here, we close the modal when the cancel button is clicked, invoking the `write` method to update the value.