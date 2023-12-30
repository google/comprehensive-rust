# CXX Error Handling: QR Example

The QR code generator is [an example][0] where a boolean is used to communicate
success vs failure, and where the successful result can be passed across the FFI
boundary:

```rust,ignore
#[cxx::bridge(namespace = "qr_code_generator")]
mod ffi {
    extern "Rust" {
        fn generate_qr_code_using_rust(
            data: &[u8],
            min_version: i16,
            out_pixels: Pin<&mut CxxVector<u8>>,
            out_qr_size: &mut usize,
        ) -> bool;
    }
}
```

<details>

Students may be curious about the semantics of the `out_qr_size` output. This is
not the size of the vector, but the size of the QR code (and admittedly it is a
bit redundant - this is the square root of the size of the vector).

It may be worth pointing out the importance of initializing `out_qr_size` before
calling into the Rust function. Creation of a Rust reference that points to
uninitialized memory results in Undefined Behavior (unlike in C++, when only the
act of dereferencing such memory results in UB).

If students ask about `Pin`, then explain why CXX needs it for mutable
references to C++ data: the answer is that C++ data can’t be moved around like
Rust data, because it may contain self-referential pointers.

</details>

[0]: https://source.chromium.org/chromium/chromium/src/+/main:components/qr_code_generator/qr_code_generator_ffi_glue.rs;l=13-18;drc=7bf1b75b910ca430501b9c6a74c1d18a0223ecca
