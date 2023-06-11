# Prerequisites

In order to start the workshop there are a few things that we will have to **install or set up**.

## Rust

If you don't have [Rust](https://www.rust-lang.org) installed in your machine yet, please follow [these instructions](https://www.rust-lang.org/tools/install).

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

## cargo-make

Finally, let's install [cargo-make](https://github.com/sagiegurari/cargo-make):

```sh
cargo install cargo-make
```

We're going to leverage [cargo-make](https://github.com/sagiegurari/cargo-make) to **run all the commands** that we need to run in order to build and deploy our backend and frontend.
