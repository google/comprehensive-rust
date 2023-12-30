---
minutes: 30
---

# Exercise: GUI Library

Let us design a classical GUI library using our new knowledge of traits and
trait objects. We'll only implement the drawing of it (as text) for simplicity.

We will have a number of widgets in our library:

- `Window`: has a `title` and contains other widgets.
- `Button`: has a `label`. In reality, it would also take a callback function to
  allow the program to do something when the button is clicked but we won't
  include that since we're only drawing the GUI.
- `Label`: has a `label`.

The widgets will implement a `Widget` trait, see below.

Copy the code below to <https://play.rust-lang.org/>, fill in the missing
`draw_into` methods so that you implement the `Widget` trait:

```rust,compile_fail
// TODO: remove this when you're done with your implementation.
#![allow(unused_imports, unused_variables, dead_code)]

{{#include exercise.rs:setup}}

// TODO: Implement `Widget` for `Label`.

// TODO: Implement `Widget` for `Button`.

// TODO: Implement `Widget` for `Window`.

{{#include exercise.rs:main}}
```

The output of the above program can be something simple like this:

```text
========
Rust GUI Demo 1.23
========

This is a small text GUI demo.

| Click me! |
```

If you want to draw aligned text, you can use the
[fill/alignment](https://doc.rust-lang.org/std/fmt/index.html#fillalignment)
formatting operators. In particular, notice how you can pad with different
characters (here a `'/'`) and how you can control alignment:

```rust,editable
fn main() {
    let width = 10;
    println!("left aligned:  |{:/<width$}|", "foo");
    println!("centered:      |{:/^width$}|", "foo");
    println!("right aligned: |{:/>width$}|", "foo");
}
```

Using such alignment tricks, you can for example produce output like this:

```text
+--------------------------------+
|       Rust GUI Demo 1.23       |
+================================+
| This is a small text GUI demo. |
| +-----------+                  |
| | Click me! |                  |
| +-----------+                  |
+--------------------------------+
```
