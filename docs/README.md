# Book

We're using [mdbook](https://rust-lang.github.io/mdBook/) in order to create our documentation.

If you want to `serve` or `build` the book locally, bear in mind that it will not work unless you copy the `README.md` from the root of this repository in the `docs/src` folder.

To avoid having to do this manually, and because we want to keep the root `README.md` file as the source of truth, we have created some [cargo-make](https://github.com/sagiegurari/cargo-make) tasks that will automate this process:

- `makers book-serve`
- `makers book-build`

## Preprocessors

We are using some mdbook plugins:

- [mdbook-mermaid](https://github.com/badboy/mdbook-mermaid)
- [mdbook-toc](https://github.com/badboy/mdbook-toc)
- [mdbook-admonish](https://github.com/tommilligan/mdbook-admonish)

If you use the `cargo-make` commands above, you don't need to worry about installing them, as they will be installed automatically.
