# Static and Constant Variables

Global state is managed with static and constant variables.

## `const`

You can declare compile-time constants:

```rust,editable
const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn main() {
    let digest = compute_digest("Hello");
    println!("Digest: {digest:?}");
}
```

According the the [Rust RFC Book][1] these are inlined upon use.

## `static`

You can also declare static variables:

```rust,editable
static BANNER: &str = "Welcome to RustOS 3.14";

fn main() {
    println!("{BANNER}");
}
```

As noted in the [Rust RFC Book][1], these are not inlined upon use and have an actual associated memory location.  This is useful for unsafe and embedded code, and have a `'static` lifetime.


We will look at mutating static data in the [chapter on Unsafe Rust](../unsafe.md).

<details>

* Mention that `const` behaves semantically similar to C++'s `constexpr`.
* `static`, on the other hand, is much more similar to a `const` or mutable global variable in C++.
* It isn't super common that one would need a runtime evaluated constant, but it is helpful and safer than using a static.

</details>
