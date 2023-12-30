---
minutes: 5
---

# Compiler Lints and Clippy

The Rust compiler produces fantastic error messages, as well as helpful built-in
lints. [Clippy](https://doc.rust-lang.org/clippy/) provides even more lints,
organized into groups that can be enabled per-project.

```rust,editable,should_panic
#[deny(clippy::cast_possible_truncation)]
fn main() {
    let x = 3;
    while (x < 70000) {
        x *= 2;
    }
    println!("X probably fits in a u16, right? {}", x as u16);
}
```

<details>

Run the code sample and examine the error message. There are also lints visible
here, but those will not be shown once the code compiles. Switch to the
Playground site to show those lints.

After resolving the lints, run `clippy` on the playground site to show clippy
warnings. Clippy has extensive documentation of its lints, and adds new lints
(including default-deny lints) all the time.

Note that errors or warnings with `help: ...` can be fixed with `cargo fix` or
via your editor.

</details>
