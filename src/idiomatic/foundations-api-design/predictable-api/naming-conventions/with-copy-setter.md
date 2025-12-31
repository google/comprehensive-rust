---
minutes: 2
---

# `with` as copy-and-set

`with` appears when a value is being copied, but also changed in a specific way.

`with` as in "like `<value>`, but with something different."

```rust,compile_fail
impl Path {
    // Simplified. "/home/me/mortgage.pdf".with_extension("mov") =>
    // "/home/me/mortgage.mov"
    fn with_extension(&self, ext: &OsStr) -> PathBuf;
}
```

<details>

- `with` can be used for methods that copy a value, but then change a specific
  part of that value.

  In the example here, `with_extension` copies the data of a `&Path` into a new
  `PathBuf`, but changes the extension to something else.

  The original `Path` is unchanged.

</details>
