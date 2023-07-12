# Static and Constant Variables

Globally-scoped names for values can be given with static variables and constant definitions.

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

According to the [Rust RFC Book][1] these are inlined upon use.

## `static`

You can also declare static variables:

```rust,editable
static BANNER: &str = "Welcome to RustOS 3.14";

fn main() {
    println!("{BANNER}");
}
```

As noted in the [Rust RFC Book][1], these are not inlined upon use and have an actual associated memory location.  This is useful for unsafe and embedded code, and the variable lives through the entirety of the program execution.
When a globally-scoped value does not have a reason to need object identity, `const` is generally preferred.


We will look at [mutating static data](../unsafe/mutable-static-variables.md) in the chapter on Unsafe Rust.
Because `static` variables are accessible from any thread, mutable static variables require manual, unsafe, synchronization of accesses.

<details>

* Mention that `const` behaves semantically similar to C++'s `constexpr`.
* `static`, on the other hand, is much more similar to a `const` or mutable global variable in C++.
* `static` provides object identity: an address in memory and state as required by types with interior mutability such as `Mutex<T>`.
* It isn't super common that one would need a runtime evaluated constant, but it is helpful and safer than using a static.

</details>

[1]: https://rust-lang.github.io/rfcs/0246-const-vs-static.html
