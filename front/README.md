# Setup

1. Install the Dioxus CLI:

```bash
cargo install dioxus-cli
```

Make sure the `wasm32-unknown-unknown` target for rust is installed:

```bash
rustup target add wasm32-unknown-unknown
```

2. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
3. Install the tailwind css cli: https://tailwindcss.com/docs/installation
4. Initialize the tailwind css project:

```bash
npx tailwindcss init
```

Create frontend crate:

```bash
cargo new --bin demo
cd demo
```

Add Dioxus and the web renderer as dependencies (this will edit your `Cargo.toml`):

```bash
cargo add dioxus
cargo add dioxus-web
```

This should create a `tailwind.config.js` file in the root of the project.

5. Edit the `tailwind.config.js` file to include rust files:

```json
module.exports = {
    mode: "all",
    content: [
        // include all rust, html and css files in the src directory
        "./src/**/*.{rs,html,css}",
        // include all html files in the output (dist) directory
        "./dist/**/*.html",
    ],
    theme: {
        extend: {},
    },
    plugins: [],
}
```

6. Create a `input.css` file with the following content:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

7. Create a `Dioxus.toml` file with the following content that links to the `tailwind.css` file:

```toml
[application]

# App (Project) Name
name = "Rusty Films"

# Dioxus App Default Platform
# desktop, web, mobile, ssr
default_platform = "web"

# `build` & `serve` dist path
out_dir = "dist"

# resource (public) file folder
asset_dir = "public"

[web.app]

# HTML title tag content
title = "🦀 | Rusty Films"

[web.watcher]

# when watcher trigger, regenerate the `index.html`
reload_html = true

# which files or dirs will be watcher monitoring
watch_path = ["src", "public"]

# include `assets` in web platform
[web.resource]

# CSS style file
style = ["tailwind.css"]

# Javascript code file
script = []

[web.resource.dev]

# serve: [dev-server] only

# CSS style file
style = []

# Javascript code file
script = []
```

## Bonus Steps

8. Install the tailwind css vs code extension
9. Go to the settings for the extension and find the experimental regex support section. Edit the setting.json file to look like this:

```json
"tailwindCSS.experimental.classRegex": ["class: \"(.*)\""],
"tailwindCSS.includeLanguages": {
    "rust": "html"
},
```

# Development

1. Run the following command in the root of the project to start the tailwind css compiler:

```bash
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
```

## Web

- Run the following command in the root of the project to start the dioxus dev server:

```bash
dioxus serve --port 8000
```

- Open the browser to http://localhost:8000

# Usefull resources
- [Movie posters](https://www.movieposters.com/)
- [Tailwind](https://tailwindcss.com/docs/installation)
- [Tailwind Animated](https://www.tailwindcss-animated.com/)
