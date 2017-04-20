# This project is available for adoption
If you have interest in maintaining this project, please contact me and I will initiate a transfer of ownership.

No further development efforts are expected (it's been three years!)

# MagickWand API bindings for Rust

This project aims to create Rust style safety for using the
popular ImageMagick library, specifically the MagickWand API.

Original API docs for MagickWand can be found [here](http://www.imagemagick.org/script/magick-wand.php).

The concepts used in this C API do not directly lend themselves
to translation to Rust, so the API will change as it is wrapped.


Initial generation of bindings was made using the excellent [rust-bindgen](https://github.com/crabtw/rust-bindgen) tool.

### Building

To build the library and samples, simply run `make`.

Samples will be built as binaries and placed in `./bin`.

Samples are based on the code samples for MagickWand, though
the particular idioms will be adapted to a more Rust friendly
style.

### Contributing

If you have need of ImageMagick, and you love Rust, feel 
free to contribute! As the project is in a very early phase
right now, the best place to help out is implementing the
sample code from the ImageMagick website, making it Rust 
friendly and updating the exposed rustwand API to match.

### Additional Information

I'll be blogging about the development of rustwand when
relevant at the [Scale It!](http://www.dahlgren.so) blog.

### NOTE

The library is currently **very** unstable. Once I figure out
a good way of handling the versioning of the underlying ImageMagick
and MagickWand APIs, things will be relatively safe.
