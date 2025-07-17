# Keep unsafe narrow

Compare these two code examples:

```rust
fn main() {
    let raw = b"Crab";

    // SAFETY: `raw` has the static lifetime of valid UTF-8 data and therefore `ptr` is valid
    let crab = unsafe {
        let ptr = raw.as_ptr();
        let bytes = std::slice::from_raw_parts(ptr, 4);
        std::str::from_utf8_unchecked(bytes)
    };

    println!("{crab}");
}
```

```rust
fn main() {
    let raw = b"Crab";
    let ptr = raw.as_ptr();

    // SAFETY: `raw` has the static lifetime and therefore `ptr` is valid
    let bytes = unsafe { std::slice::from_raw_parts(ptr, 4) };

    // SAFETY: We created `raw` with valid UTF-8 data
    let crab = unsafe { std::str::from_utf8_unchecked(bytes) };

    println!("{crab}");
}
```

<details>

Unsafe blocks should have a narrow lens.

<!-- TODO(timclicks): fix clunky wording below -->

If an unsafe block has multiple safety conditions that can be assessed
independently, then it's likely that each of those conditions should be in its
own block.

</details>
