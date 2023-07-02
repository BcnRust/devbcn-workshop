# State Management

In this part of our journey, we're going to dive into the lifeblood of the application — state management. We'll tackle this crucial aspect in two stages: local state management and global state management.

While we're only scratching the surface to get the application up and running, it's highly recommended that you refer to the [Dioxus Interactivity](https://dioxuslabs.com/docs/0.3/guide/en/interactivity/index.html) documentation. This way, you'll not only comprehend how it operates more fully, but also grasp the extensive capabilities the framework possesses.

For now, let's start with the basics. **Dioxus** as is very influenced by *React* and its ecosystem, so it's no surprise that it uses the same approach to state management, Hooks.
Hooks are Rust functions that take a reference to `ScopeState` (in a component, you can pass `cx`), and provide you with functionality and state. **Dioxus** allows hooks to maintain state across renders through a reference to `ScopeState`, which is why you must pass `&cx` to them.

```admonish tip  title="Rules of Hooks"
1. Hooks may be only used in components or other hooks
2. On every call to the component function
    1. The same hooks must be called
    2. In the same order
3. Hooks name's should start with `use_` so you don't accidentally confuse them with regular functions
```
