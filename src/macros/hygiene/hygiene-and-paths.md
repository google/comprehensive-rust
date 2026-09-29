---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Referencing Paths Hygienically

Item and crate paths are unhygienic, so item paths within a macro definition
will refer to a different item than intended if their leading module or crate
name is defined differently at the call site than the macro author expected.

Within a single crate, using absolute paths (which start with `::` or a crate
name) is a start towards referencing items unambiguously. But the crate name
could be shadowed by a module in the current crate, or at the call site, the
crate could be imported with a different name. So even absolute paths are not a
fully robust means to refer to items from macros.

However, there is a way out: we can unambiguously refer to the macro's
**defining crate** only via the `$crate` metavariable.

This can be used to refer to local items from the same crate as the macro
without fear of interference, regardless of the macro call site.

```rust,compile_fail
// Macro-defining crate `custom_vec`
/// A custom Vec type with API similar to the standard one.
pub struct Vec<T>(...);

macro_rules! vec {
    ($elems: $expr) => {
        // Always refers to this crate's `Vec<T>` type, regardless of call-site
        // environment; `$crate` always refers to the macro-defining crate.
        let v = $crate::Vec::new(); for i in $elems { v.push(i); }; v
    };
}

// Main crate, which depends on `custom_vec` but aliases it to the name `cvec`.
type Vec<T> = std::vec::Vec<T>;

fn main() {
    // In this context, `::Vec` is a local type alias of the stdlib's `Vec`.
	let _ = ::Vec::<u8>::new();
	// But the macro expansion's `$crate` still refers to the defining crate,
	// without needing to worry about this crate having renamed its import, or
	// anything else about the local lexical environment.
    let n = cvec::vec!([50]);
}
```

<details>

- In the example, using the `crate` keyword or starting the path with `::Vec`
  would both resolve to the main crate's type alias for the stdlib's vec type.
- Because the main crate renames its import of the `custom_vec` crate to `cvec`,
  starting the path with `custom_vec` would fail to find the crate.
- The `$crate`-based path expands correctly to the crate that defined the macro,
  despite these hurdles.

</details>
