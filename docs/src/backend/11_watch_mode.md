# Watch Mode

You may be thinking that it would be nice to have a way to **automatically restart the backend when you make changes to the code**. 

Well, you're in luck! Enter [cargo-watch](https://github.com/watchexec/cargo-watch).

If you don't have it already installed, you can do so by running:

```bash
cargo install cargo-watch
```

Next, in order to start the backend in watch mode, you can run:

```bash
cargo watch -x "shuttle run"
```

This will start the backend in watch mode. Now, whenever you make changes to the code, the backend will automatically restart.

```admonish example "Try it out"
Test it out by making a change to the `src/main.rs` file.
```
