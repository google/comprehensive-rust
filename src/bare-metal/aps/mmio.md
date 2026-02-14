<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Volatile memory access for MMIO

- Use [`pointer::read_volatile`] and [`pointer::write_volatile`].
- Never hold a reference to a location being accessed with these methods. Rust
  may read from (or write to, for `&mut`) a reference at any time.
- Use `&raw` to get fields of structs without creating an intermediate
  reference.

```rust,editable,ignore
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
const SOME_DEVICE_REGISTER: *mut u64 = 0x800_0000 as _;
// SAFETY: Some device is mapped at this address.
unsafe {
    SOME_DEVICE_REGISTER.write_volatile(0xff);
    SOME_DEVICE_REGISTER.write_volatile(0x80);
    assert_eq!(SOME_DEVICE_REGISTER.read_volatile(), 0xaa);
}
```

[`pointer::read_volatile`]: https://doc.rust-lang.org/stable/core/primitive.pointer.html#method.read_volatile
[`pointer::write_volatile`]: https://doc.rust-lang.org/stable/core/primitive.pointer.html#method.write_volatile
[`addr_of!`]: https://doc.rust-lang.org/stable/core/ptr/macro.addr_of.html

<details>

- Volatile access: read or write operations may have side-effects, so prevent
  the compiler or hardware from reordering, duplicating or eliding them.
  - If you write and then read, e.g. via a mutable reference, the compiler may
    assume that the value read is the same as the value just written, and not
    bother actually reading memory.
- Some existing crates for volatile access to hardware do hold references, but
  this is unsound. Whenever a reference exists, the compiler may choose to
  dereference it.
- Use `&raw` to get struct field pointers from a pointer to the struct.
- For compatibility with old versions of Rust you can use the [`addr_of!`] macro
  instead.

</details>
