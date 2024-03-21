# aria-of-borrow

> It's a library AND a binary, but at what cost?

This is a simple toy project that demonstrates the various failure modes of trying to make a single cargo package a libary and a binary.

* [v0.1.0 is a package that can't be published because its binary's dependences are behind a `feature="cli"`](https://github.com/Gankra/aria-of-borrow/blob/v0.1.0/Cargo.toml)
* [v0.2.0 is a package that has `bin.required-features = ["cli"]`, resulting in `cargo install` failing to install](https://github.com/Gankra/aria-of-borrow/blob/v0.2.0/Cargo.toml)
* [v0.3.0 is a package that enables the cli by default with features.default = ["cli"]`, resulting in `cargo add` adding too much to library users](https://github.com/Gankra/aria-of-borrow/blob/v0.3.0/Cargo.toml)
