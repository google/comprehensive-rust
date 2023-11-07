# Generated C++

```rust,ignore
#[cxx::bridge]
mod ffi {
{{#include ../../../../third_party/cxx/blobstore/src/main.rs:rust_bridge}}
}
```

Results in (roughly) the following C++:

```cpp
struct MultiBuf final : public ::rust::Opaque {
  ~MultiBuf() = delete;

private:
  friend ::rust::layout;
  struct layout {
    static ::std::size_t size() noexcept;
    static ::std::size_t align() noexcept;
  };
};

::rust::Slice<::std::uint8_t const> next_chunk(::org::blobstore::MultiBuf &buf) noexcept;
```
