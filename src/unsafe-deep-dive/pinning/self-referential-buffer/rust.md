---
minutes: 10
---

# Modeled in Rust

```rust,ignore
/// Raw pointers
pub struct SelfReferentialBuffer {
    data: [u8; 1024],
    cursor: *mut u8,
}

/// Integer offsets
pub struct SelfReferentialBuffer {
    data: [u8; 1024],
    cursor: usize,
}

/// Pinning
pub struct SelfReferentialBuffer {
    data: [u8; 1024],
    cursor: *mut u8,
    _pin: std::marker::PhantomPinned,
}
```

## Original C++ class definition for reference

```cpp,ignore
class SelfReferentialBuffer {
    char data[1024];
    char* cursor;
}
```

<details>

The next few slides show three approaches to creating a Rust type with the same
semantics as the original C++.

- Using raw pointers: matches C++ very closely, but using the resulting type is
  extremely hazardous
- Storing integer offsets: more natural in Rust, but references need to be
  created manually
- Pinning: allows raw pointers with fewer `unsafe` blocks

</details>
