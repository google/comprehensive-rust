# C++ Bridge Declarations

```rust,ignore
#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("demo/include/blobstore.h");
        type BlobstoreClient;
        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
        fn put(&self, parts: &mut MultiBuf) -> u64;
    }

    unsafe extern "C++" {
        fn f();  // safe to call
    }
}
```

<details>

* The programmer does not need to promise that the signatures they have typed in
  are accurate; that would be unreasonable. CXX performs static assertions that
  the signatures exactly correspond with what is declared in C++.
* `unsafe extern` blocks allow you to declare C++ functions that are safe to
  call from Rust.

</details>
