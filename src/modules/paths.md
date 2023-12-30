---
minutes: 10
---

# use, super, self

A module can bring symbols from another module into scope with `use`. You will
typically see something like this at the top of each module:

```rust,editable
use std::collections::HashSet;
use std::process::abort;
```

## Paths

Paths are resolved as follows:

1. As a relative path:
   - `foo` or `self::foo` refers to `foo` in the current module,
   - `super::foo` refers to `foo` in the parent module.

2. As an absolute path:
   - `crate::foo` refers to `foo` in the root of the current crate,
   - `bar::foo` refers to `foo` in the `bar` crate.

<details>

- It is common to "re-export" symbols at a shorter path. For example, the
  top-level `lib.rs` in a crate might have

  ```rust,ignore
  mod storage;

  pub use storage::disk::DiskStorage;
  pub use storage::network::NetworkStorage;
  ```

  making `DiskStorage` and `NetworkStorage` available to other crates with a
  convenient, short path.

- For the most part, only items that appear in a module need to be `use`'d.
  However, a trait must be in scope to call any methods on that trait, even if a
  type implementing that trait is already in scope. For example, to use the
  `read_to_string` method on a type implementing the `Read` trait, you need to
  `use std::io::Read`.

- The `use` statement can have a wildcard: `use std::io::*`. This is discouraged
  because it is not clear which items are imported, and those might change over
  time.

</details>
