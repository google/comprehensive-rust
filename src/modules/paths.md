# Paths

Paths are resolved as follows:

1. As a relative path:
   * `foo` or `self::foo` refers to `foo` in the current module,
   * `super::foo` refers to `foo` in the parent module.

2. As an absolute path:
   * `crate::foo` refers to `foo` in the root of the current crate,
   * `bar::foo` refers to `foo` in the `bar` crate.

A module can bring symbols from another module into scope with `use`.
You will typically see something like this at the top of each module:

```rust,editable
use std::collections::HashSet;
use std::mem::transmute;
```
