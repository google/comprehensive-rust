# CXX Error Handling: PNG Example

A prototype of a PNG decoder illustrates what can be done when the successful
result cannot be passed across the FFI boundary:

```rust,ignore
#[cxx::bridge(namespace = "gfx::rust_bindings")]
mod ffi {
    extern "Rust" {
        /// This returns an FFI-friendly equivalent of `Result<PngReader<'a>,
        /// ()>`.
        fn new_png_reader<'a>(input: &'a [u8]) -> Box<ResultOfPngReader<'a>>;

        /// C++ bindings for the `crate::png::ResultOfPngReader` type.
        type ResultOfPngReader<'a>;
        fn is_err(self: &ResultOfPngReader) -> bool;
        fn unwrap_as_mut<'a, 'b>(
            self: &'b mut ResultOfPngReader<'a>,
        ) -> &'b mut PngReader<'a>;

        /// C++ bindings for the `crate::png::PngReader` type.
        type PngReader<'a>;
        fn height(self: &PngReader) -> u32;
        fn width(self: &PngReader) -> u32;
        fn read_rgba8(self: &mut PngReader, output: &mut [u8]) -> bool;
    }
}
```

<details>

`PngReader` and `ResultOfPngReader` are Rust types --- objects of these types
cannot cross the FFI boundary without indirection of a `Box<T>`. We can't have
an `out_parameter: &mut PngReader`, because CXX doesn't allow C++ to store Rust
objects by value.

This example illustrates that even though CXX doesn't support arbitrary generics
nor templates, we can still pass them across the FFI boundary by manually
specializing / monomorphizing them into a non-generic type. In the example
`ResultOfPngReader` is a non-generic type that forwards into appropriate methods
of `Result<T, E>` (e.g. into `is_err`, `unwrap`, and/or `as_mut`).

</details>
