# Book

We're using [mdbook](https://rust-lang.github.io/mdBook/) in order to create our documentation.

If you want to `serve` or `build` the book locally, bear in mind that it will not work unless you copy the `README.md` from the root of this repository in the `docs/src` folder.

To avoid having to do this manually, and because we want to keep the root `README.md` file as the source of truth, we have created some [cargo-make](https://github.com/sagiegurari/cargo-make) tasks that will automate this process:

- `makers book-serve`
- `makers book-build`
