# cxx error handling - PNG example

A prototype of a PNG decoder illustrates what can be done when the successful
result cannot be passed across the FFI boundary.

Notes:

* cxx doesn't support
  arbitrary generics (like `Option<T>`, or `Result<T, E>`)
  nor templates (like `std::expected<T, E>`),
  but we can manually specialize / monomorphize
  into a non-generic `ResultOfPngReader` type
  that forwards into appropriate methods of `Result<T, E>`
  (e.g. into `is_err`, `unwrap`, and/or `as_mut`).
* `PngReader` and `ResultOfPngReader` are Rust types --- objects of these types
  cannot cross the FFI boundary without indirection of a `Box<T>`.  We can't
  have an `out_parameter: &mut PngReader`, because `cxx` is unable to replicate
  in C++ the memory layout of custom Rust types and is unable to map C++
  destructor nor move constructors into methods of the Rust type.

Example ([from a WIP prototype][0]):

```rust,ignore
#[cxx::bridge(namespace = "gfx::rust_bindings")]
mod ffi {
    extern "Rust" {
        /// C++ bindings for the `crate::png::new_png_reader` function.
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

If time allows, it may be worth mentioning that returning
`Box<ResultOfPngReader>` doesn't risk UB stemming from uninitialized memory
(unlike `out_parameter: &mut SomePrimitiveType`).  OTOH, this UB risk still
remains in the `read_rgba8` method.

It may be worth mentioning [the Crubit project][1] which is not available in
Chromium at this point, but which plans to support some of the FFI gaps
highlighted on this slide:

* Passing across the FFI boundary objects of most arbitrary, custom C++ and Rust types
  (replicating the memory layout, mapping C++ destructors to `drop`, making sure
  that C++ moved-away objects are compatible with semantics of Rust).

* Most arbitrary generic or template structs (automatically performing the
  monomorphisation / specialization behind the scene).

</details>

[0]: https://chromium-review.googlesource.com/c/chromium/src/+/4860211/12/ui/gfx/codec/rust_bindings/ffi.rs
[1]: https://github.com/google/crubit
