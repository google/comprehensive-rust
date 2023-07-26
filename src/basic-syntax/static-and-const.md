# Static and Constant Variables

Static and constant variables are two different ways to create values that
cannot be moved or reallocated during the execution of the program. 

## `const`

Constant variables are evaluated at compile time and their values are inlined
wherever they are used:

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

Only functions marked `const` can be called at compile time to generate `const` values. `const` functions can however be called at runtime.

## `static`

Static variables will live during the whole execution of the program, and therefore will not move:

```rust,editable
static BANNER: &str = "Welcome to RustOS 3.14";

fn main() {
    println!("{BANNER}");
}
```

As noted in the [Rust RFC Book][1], these are not inlined upon use and have an actual associated memory location.  This is useful for unsafe and embedded code, and the variable lives through the entirety of the program execution.


Static variables have more ownership constraints than data on the stack or the heap. For instance, they cannot be safely mutated. We will look at mutating static data in the [chapter on Unsafe Rust](../unsafe.md).


<details>

* Mention that `const` behaves semantically similar to C++'s `constexpr`.
* `static`, on the other hand, is much more similar to a `const` or mutable global variable in C++.
* It isn't super common that one would need a runtime evaluated constant, but it is helpful and safer than using a static.
* `thread_local` data can be created with the macro `std::thread_local`.

### Properties table: 

| Property | Static | Constant |
|---|---|---|
| Allocated at runtime | Yes | No |
| Lives for the entire duration of the program | Yes | No |
| Can be mutable | Yes (unsafe) | No |
| Evaluated at compile time | No | Yes |
| Inlined wherever it is used | No | Yes |


</details>

[1]: https://rust-lang.github.io/rfcs/0246-const-vs-static.html
