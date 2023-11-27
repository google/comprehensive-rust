# cxx error handling

cxx's support for `Result<T,E>` relies on C++ exceptions, so we can't use that
in Chromium. Alternatives:

* Where success can be represented as a simple Boolean, as done in our [QR code generator][1]:
  Return a Boolean representing success, and record results using out-parameters:
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
* Where success is more complex, provide a Rust
  object which can be queried for details of success or failure:
  ```rust,ignore
  #[cxx::bridge]
  mod ffi {
    extern "Rust" {
      type PngDecoder;
      fn create_png_decoder() -> Box<PngDecoder>;
      fn decode(self: &PngDecoder, png: &[u8]) -> bool; // whether successful
      fn get_err_code(self: &PngDecoder) -> u32; // or some more complex error type
      fn get_decoded_image(self: &PngDecoder) -> &[u8];
            // or some more complex success type
    }
  }
  ```


The best way to learn cxx is by doing, so, another exercise!

[0]: https://cxx.rs/binding/result.html
[1]: https://source.chromium.org/chromium/chromium/src/+/main:components/qr_code_generator/qr_code_generator_ffi_glue.rs;l=10