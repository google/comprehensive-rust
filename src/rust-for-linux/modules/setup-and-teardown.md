---
minutes: 4
---

# Module Setup and Teardown

Our module implements the [`kernel::Module`](https://rust.docs.kernel.org/kernel/trait.Module.html) trait
to specify its entrypoint and perform any necessary set-up:

```rust
pub trait Module: Sized + Sync {
    fn init(name: &'static CStr, module: &'static ThisModule) -> Result<Self>;
}
```

If some setup fails (e.g. finding device tree nodes or acquiring needed resources),
the `init` method can return `Err`.

## `Drop` impl

By implementing `Drop` on our module struct, we can perform any necessary cleanup and teardown.

```rust
impl Drop for MyModule {
    fn drop(&mut self) {
        // ...
    }
}
```
