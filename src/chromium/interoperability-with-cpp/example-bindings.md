# Example bindings

cxx requires you to declare the whole C++/Rust boundary in one of your `.rs`
files. For instance:

```rust
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type MultiBuf;

        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }

    unsafe extern "C++" {
        include!("cxx-demo/include/blobstore.h");

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
        fn put(&self, parts: &mut MultiBuf) -> u64;
    }
}

// It's fine to put more Rust code here after your #[cxx::bridge]
```

<details>
Point out:

* Native support for C++'s `std::unique_ptr` in Rust
* Native support for Rust slices in C++
* Calls from C++ to Rust, and Rust types (in the top part)
* Calls from Rust to C++, and C++ types (in the bottom part)
* If the function definitions in C++ or Rust don't match the cxx::bridge,
  a compilation failure results.

**Common misconception**: It _looks_ like a C++ header is being parser by Rust,
but this is misleading. This header is never interpreted by Rust, but simply
`#include`d in the generated C++ code for the benefit of C++ compilers.
</details>