# Testing

Rust community typically authors unit tests in a module placed in the same
source file as the code being tested. This was covered [earlier](../testing.md)
in the course and looks like this:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn my_test() {
        todo!()
    }
}
```

In Chromium we place unit tests in a separate source file and we continue to
follow this practice for Rust --- this makes tests consistently discoverable and
helps to avoid rebuilding `.rs` files a second time (in the `test`
configuration).

This results in the following options for testing Rust code in Chromium:

- Native Rust tests (i.e. `#[test]`). Discouraged outside of
  `//third_party/rust`.
- `gtest` tests authored in C++ and exercising Rust via FFI calls. Sufficient
  when Rust code is just a thin FFI layer and the existing unit tests provide
  sufficient coverage for the feature.
- `gtest` tests authored in Rust and using the crate under test through its
  public API (using `pub mod for_testing { ... }` if needed). This is the
  subject of the next few slides.

<details>

Mention that native Rust tests of third-party crates should eventually be
exercised by Chromium bots. (Such testing is needed rarely --- only after adding
or updating third-party crates.)

Some examples may help illustrate when C++ `gtest` vs Rust `gtest` should be
used:

- QR has very little functionality in the first-party Rust layer (it's just a
  thin FFI glue) and therefore uses the existing C++ unit tests for testing both
  the C++ and the Rust implementation (parameterizing the tests so they enable
  or disable Rust using a `ScopedFeatureList`).

- Hypothetical/WIP PNG integration may need to implement memory-safe
  implementation of pixel transformations that are provided by `libpng` but
  missing in the `png` crate - e.g. RGBA => BGRA, or gamma correction. Such
  functionality may benefit from separate tests authored in Rust.

</details>
