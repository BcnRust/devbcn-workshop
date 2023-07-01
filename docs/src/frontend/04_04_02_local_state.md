# Local state

When we are talking about local state is better to talk about **Component** state. Fortunately, Dioxus provides a simple way to manage the state of a component using `use_state` hook.

The particularity of this hook are the following:
- To initialize the state you must pass a closure that returns the initial state
```rust
let mut count = use_state(cx, || 0);
```
- `use_state` gives you the current value with `get()` and a way to update it with `set()`
- Is important to know that on every value update will trigger a component re-render

Let's start by updating our `main.rs`, concretely the `App` component. We will add some local state that will be placed at the top of our app, and can be passed to components as props.

```diff
...

fn App(cx: Scope) -> Element {
+    let films = use_state::<Option<Vec<Film>>>(cx, || None);
+    let selected_film = use_state::<Option<Film>>(cx, || None);
+    let force_get_films = use_state(cx, || ());

...

}
```