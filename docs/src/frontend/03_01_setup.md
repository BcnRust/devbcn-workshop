# Setup

This guide outlines the steps necessary to set up a frontend development environment using Dioxus and Tailwind.

## Dioxus Configuration

Dioxus, a Rust framework, allows you to build responsive web applications. To use Dioxus, you need to install the Dioxus Command Line Interface (CLI) and the Rust target `wasm32-unknown-unknown`.

### Step 1: Install the Dioxus CLI

Install the Dioxus CLI by running the following command:

```bash
cargo install dioxus-cli
```

### Step 2: Install the Rust Target

Ensure the `wasm32-unknown-unknown` target for Rust is installed by running:

```bash
rustup target add wasm32-unknown-unknown
```

### Step 3: Create a Frontend Crate

Create a new frontend crate from root of our project by executing:

```bash
cargo new --bin front
cd front
```

Update the project's workspace configuration by adding the following lines to the `Cargo.toml` file:

```diff
[workspace]
members = [
    "api/lib",
    "api/shuttle",
    "shared",
+   "front",
]

```

### Step 4: Add Dioxus and the Web Renderer as Dependencies

Add Dioxus and the web renderer as dependencies to your project, modify your `Cargo.toml` file as follows:


```rust
...

[dependencies]
# dioxus
dioxus = "0.4.3"
dioxus-web = "0.4.3"
```

## Tailwind Configuration

Tailwind CSS is a utility-first CSS framework that can be used with Dioxus to build custom designs.

### Step 1: Install Node Package Manager and Tailwind CSS CLI

Install [Node Package Manager (npm)](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm) and the [Tailwind CSS CLI](https://tailwindcss.com/docs/installation).

### Step 2: Initialize a Tailwind CSS Project

Initialize a new Tailwind CSS project using the following command:

```bash
cd front
npx tailwindcss init
```

This command creates a `tailwind.config.js` file in your project's root directory.

### Step 3: Modify the Tailwind Configuration File

Edit the `tailwind.config.js` file to include Rust, HTML, and CSS files from the `src` directory and HTML files from the `dist` directory:

```json
module.exports = {
    mode: "all",
    content: [
        // Include all Rust, HTML, and CSS files in the src directory
        "./src/**/*.{rs,html,css}",
        // Include all HTML files in the output (dist) directory
        "./dist/**/*.html",
    ],
    theme: {
        extend: {},
    },
    plugins: [
        require('tailwindcss-animated')
    ]
}
```

### Step 4: Create an Input CSS File

Create an `input.css` file at the root of `front` crate and populate it with the following content:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

### Step 5: Tailwind animations
Install npm package `tailwind-animated` for small animations:

```bash
npm install tailwindcss-animated --save-dev
```

## Linking Dioxus with Tailwind

To use Tailwind with Dioxus, create a `Dioxus.toml` file in your project's root directory. This file links to the `tailwind.css` file.

### Step 1: Create a `Dioxus.toml` File

The `Dioxus.toml` file, placed inside our `front` crate root,  should contain:

```toml
[application]

# App (Project) Name
name = "rusty-films"

# Dioxus App Default Platform
# desktop, web, mobile, ssr
default_platform = "web"

# `build` & `serve` dist path
out_dir = "dist"

# Resource (public) file folder
asset_dir = "public"

[web.app]

# HTML title tag content
title = "🦀 | Rusty Films"

[web.watcher]

# When watcher trigger, regenerate the `index.html`
reload_html = true

# Which files or dirs will be watcher monitoring
watch_path = ["src", "public"]

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

## Update .gitignore
Ignore node_modules folder in `.gitignore` file:

```diff
target/
Secrets*.toml
static/
+dist/
+node_modules/
```

## Additional Steps

### Step 1: Install the Tailwind CSS IntelliSense VSCode Extension

The [Tailwind CSS IntelliSense VSCode extension](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss) can help you write Tailwind classes and components more efficiently.

### Step 2: Enable Regex Support for the Tailwind CSS IntelliSense VSCode Extension

Navigate to the settings for the Tailwind CSS IntelliSense VSCode extension and locate the experimental regex support section. Edit the `setting.json` file to look like this:

```json
"tailwindCSS.experimental.classRegex": ["class: \"(.*)\""],
"tailwindCSS.includeLanguages": {
    "rust": "html"
},
```

This configuration enables the IntelliSense extension to recognize Tailwind classes in Rust files treated as HTML.

After completing these steps, your frontend development environment should be ready. You can now start building your web application using Dioxus and Tailwind CSS.
