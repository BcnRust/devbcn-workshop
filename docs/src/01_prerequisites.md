# Prerequisites

In order to start the workshop there are a few things that we will have to **install or set up**.

## Rust

If you don't have [Rust](https://www.rust-lang.org) installed in your machine yet, please follow [these instructions](https://www.rust-lang.org/tools/install).

## Visual Studio Code

You can use whatever IDE you want but we're going to use [Visual Studio Code](https://code.visualstudio.com/) as our **code editor**.

If you're going to use [Visual Studio Code](https://code.visualstudio.com/) as well, please install the following extensions:

- [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
- [Crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates)
- [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml)
- [Rust Test Explorer](https://marketplace.visualstudio.com/items?itemName=swellaby.vscode-rust-test-adapter)
- [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client)

## Shuttle

This is the [tool and platform](https://shuttle.rs) that we're going to use to deploy our **backend** (api & database).

You can follow [this installation guide](https://docs.shuttle.rs/introduction/installation) or just do:

```sh
cargo install cargo-shuttle
```

## Dioxus

[Dioxus](https://dioxuslabs.com/) is the framework that we're going to use to build our **frontend**.

Be sure to install the [Dioxus CLI](https://github.com/DioxusLabs/cli):

```sh
cargo install dioxus-cli
```

After that, make sure the `wasm32-unknown-unknown` target for [Rust](https://www.rust-lang.org) is installed:

```sh
rustup target add wasm32-unknown-unknown
```

## Docker

We will also need to have [Docker](https://www.docker.com/) installed in order to **deploy locally** while we're developing the backend.

## DBeaver

We will use [DBeaver](https://dbeaver.io/) to **connect to the database** and run queries. Feel free to use any other tool that you prefer.


## cargo-watch

We will also use [cargo-watch](https://github.com/watchexec/cargo-watch) to **automatically recompile** our backend when we make changes to the code.

```sh
cargo install cargo-watch
```

## cargo-make

Finally, let's install [cargo-make](https://github.com/sagiegurari/cargo-make):

```sh
cargo install cargo-make
```

We're going to leverage [cargo-make](https://github.com/sagiegurari/cargo-make) to **run all the commands** that we need to run in order to build and deploy our backend and frontend.
