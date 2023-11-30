# cxx error handling - QR example

The QR code generator is an example where a boolean is used to communicate
success vs failure, and where the successful result can be passed across the FFI
boundary.

Notes:

* A Rust reference that points to uninitialized memory results in Undefined
  Behavior (unlike in C++, when only the act of dereferencing such memory
  results in UB).  Therefore, it is imporant that the C++ caller initializes
  `out_qr_size` before calling into Rust.
* `Pin<&mut SomeCppType>` behaves like `&mut SomeCppType`, but prevents
  moving. cxx uses this wrapper, because all Rust moves are `memcpy`s --- Rust
  doesn't know that C++ move constructors sometimes need to fix up internal
  pointers (such as may be used in small string optimization).

Example from [`qr_code_generator_ffi_glue.rs`][0]:

```rust,ignore
#[cxx::bridge(namespace = "qr_code_generator")]
mod ffi {
    extern "Rust" {
        fn generate_qr_code_using_rust(
            data: &[u8],
            min_version: i16,
            out_pixels: Pin<&mut CxxVector<u8>>,  // Output1
            out_qr_size: &mut usize,
        ) -> bool;
    }
}
```

<details>

Students may be curious about the semantics of the `out_qr_size` output.  This
is not the size of the vector, but the size of the QR code (and admittedly it is
a bit redundant - this is the square root of the size of the vector).

</details>

[0]: https://source.chromium.org/chromium/chromium/src/+/main:components/qr_code_generator/qr_code_generator_ffi_glue.rs;l=13-18;drc=7bf1b75b910ca430501b9c6a74c1d18a0223ecca

