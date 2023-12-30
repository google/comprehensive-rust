# C++ Bridge Declarations

```rust,ignore
#[cxx::bridge]
mod ffi {
{{#include ../../../../third_party/cxx/blobstore/src/main.rs:cpp_bridge}}
}
```

Results in (roughly) the following Rust:

```rust,ignore
#[repr(C)]
pub struct BlobstoreClient {
    _private: ::cxx::private::Opaque,
}

pub fn new_blobstore_client() -> ::cxx::UniquePtr<BlobstoreClient> {
    extern "C" {
        #[link_name = "org$blobstore$cxxbridge1$new_blobstore_client"]
        fn __new_blobstore_client() -> *mut BlobstoreClient;
    }
    unsafe { ::cxx::UniquePtr::from_raw(__new_blobstore_client()) }
}

impl BlobstoreClient {
    pub fn put(&self, parts: &mut MultiBuf) -> u64 {
        extern "C" {
            #[link_name = "org$blobstore$cxxbridge1$BlobstoreClient$put"]
            fn __put(
                _: &BlobstoreClient,
                parts: *mut ::cxx::core::ffi::c_void,
            ) -> u64;
        }
        unsafe {
            __put(self, parts as *mut MultiBuf as *mut ::cxx::core::ffi::c_void)
        }
    }
}

// ...
```

<details>

- The programmer does not need to promise that the signatures they have typed in
  are accurate. CXX performs static assertions that the signatures exactly
  correspond with what is declared in C++.
- `unsafe extern` blocks allow you to declare C++ functions that are safe to
  call from Rust.

</details>
