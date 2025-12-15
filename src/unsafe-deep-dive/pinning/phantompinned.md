# PhantomPinned

Usage

```rust,editable
pub struct DynamicBuffer {
    data: Vec<u8>,
    cursor: NonNull<u8>,
    _pin: std::marker::PhantomPinned,
}
```

<details>

If a type contains a `PhantomPinned`, it will not implement `Unpin` by default.

This has the effect of enforcing pinning when `DynamicBuffer` is wrapped by
`Pin`.

<!-- TODO: Monitor issue https://github.com/rust-lang/rust/issues/125735 as this guidance will change at some point and future code will move to UnsafePinned -->

</details>
