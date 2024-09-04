---
minutes: 3
---

# Functions

<!-- mdbook-xgettext: skip -->

```rust,editable
fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

fn main() {
    println!("gcd: {}", gcd(143, 52));
}
```

<details>

- Declaration parameters are followed by a type (the reverse of some programming
  languages), then a return type.
- The last expression in a function body (or any block) becomes the return
  value. Simply omit the `;` at the end of the expression. The `return` keyword
  can be used for early return, but the "bare value" form is idiomatic at the
  end of a function (refactor `gcd` to use a `return`).
- Some functions have no return value, and return the 'unit type', `()`. The
  compiler will infer this if the return type is omitted.
- Overloading is not supported -- each function has a single implementation.
  - Always takes a fixed number of parameters. Default arguments are not
    supported. Macros can be used to support variadic functions.
  - Always takes a single set of parameter types. These types can be generic,
    which will be covered later.

</details>
