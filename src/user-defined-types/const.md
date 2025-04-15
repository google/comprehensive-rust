---
minutes: 10
---

# `const`

Constants are evaluated at compile time and their values are inlined wherever
they are used:

<!-- mdbook-xgettext: skip -->

```rust,editable
const DIGEST_SIZE: usize = 3;
const FILL_VALUE: u8 = calculate_fill_value();

const fn calculate_fill_value() -> u8 {
    if DIGEST_SIZE < 10 { 42 } else { 13 }
}

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [FILL_VALUE; DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn main() {
    let digest = compute_digest("Hello");
    println!("digest: {digest:?}");
}
```

According to the [Rust RFC Book][1] these are inlined upon use.

Only functions marked `const` can be called at compile time to generate `const`
values. `const` functions can however be called at runtime.

<details>

- Mention that `const` behaves semantically similar to C++'s `constexpr`

</details>

[1]: https://rust-lang.github.io/rfcs/0246-const-vs-static.html
