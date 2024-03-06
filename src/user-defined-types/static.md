---
minutes: 5
---

# Static and Const

Static and constant variables are two different ways to create globally-scoped
values that cannot be moved or reallocated during the execution of the program.

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

- `static`, on the other hand, is much more similar to mutable global variable
  in C++.
- `static` provides object identity: an address in memory and state as required
  by types with interior mutability such as `Mutex<T>`.

</details>

[1]: https://rust-lang.github.io/rfcs/0246-const-vs-static.html
