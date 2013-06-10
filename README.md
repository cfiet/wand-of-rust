ImagickMagic's MagickWand API bindings for Rust

Initial generation of bindings was made using the excellent [rust-bindgen](https://github.com/crabtw/rust-bindgen) tool.

Currently I'm not really doing any wrapping, just translation. I'll be making the API safer for
Rust usage as I work through it.

### Building

To build the library and samples, simply run `make`.

Samples will be built as binaries named \*.sample in the samples directory.
