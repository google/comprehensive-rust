---
minutes: 5
---

# Static and Const

Static and constant variables are two different ways to create globally-scoped
values that cannot be moved or reallocated during the execution of the program.

## `const`

Constant variables are evaluated at compile time and their values are inlined
wherever they are used:

<!-- mdbook-xgettext: skip -->

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
    println!("digest: {digest:?}");
}
```

According to the [Rust RFC Book][1] these are inlined upon use.

Only functions marked `const` can be called at compile time to generate `const`
values. `const` functions can however be called at runtime.

## `static`

Static variables will live during the whole execution of the program, and
therefore will not move:

```rust,editable
static BANNER: &str = "Welcome to RustOS 3.14";

fn main() {
    println!("{BANNER}");
}
```

As noted in the [Rust RFC Book][1], these are not inlined upon use and have an
actual associated memory location. This is useful for unsafe and embedded code,
and the variable lives through the entirety of the program execution. When a
globally-scoped value does not have a reason to need object identity, `const` is
generally preferred.

<details>

- Mention that `const` behaves semantically similar to C++'s `constexpr`.
- `static`, on the other hand, is much more similar to a `const` or mutable
  global variable in C++.
- `static` provides object identity: an address in memory and state as required
  by types with interior mutability such as `Mutex<T>`.
- It isn't super common that one would need a runtime evaluated constant, but it
  is helpful and safer than using a static.

### Properties table:

| Property                                     | Static                            | Constant     |
| -------------------------------------------- | --------------------------------- | ------------ |
| Has an address in memory                     | Yes                               | No (inlined) |
| Lives for the entire duration of the program | Yes                               | No           |
| Can be mutable                               | Yes (unsafe)                      | No           |
| Evaluated at compile time                    | Yes (initialised at compile time) | Yes          |
| Inlined wherever it is used                  | No                                | Yes          |

# More to Explore

Because `static` variables are accessible from any thread, they must be `Sync`.
Interior mutability is possible through a
[`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html), atomic or
similar.

Thread-local data can be created with the macro `std::thread_local`.

</details>

[1]: https://rust-lang.github.io/rfcs/0246-const-vs-static.html
