---
minutes: 15
---

# Wrapping `abs(3)`

```rust,editable,ignore
fn abs(x: i32) -> i32;

fn main() {
    let x = -42;
    let abs_x = abs(x);
    println!("{x}, {abs_x}");
}
```

<details>

In this slide, we’re establishing a pattern for writing wrappers.

Find the external definition of a function’s signature Write a matching function
in Rust within an `extern` block Confirm which safety invariants need to be
upheld Decide whether it’s possible to mark the function as safe

Note that this doesn’t work _yet_.

Add the extern block:

```rust
unsafe extern "C" {
    fn abs(x: i32) -> i32;
}
```

Explain that many POSIX functions are available in Rust because cargo links
against the C standard library (libc) by default, which brings its symbols into
the program’s scope.

Show `man 3 abs` in the terminal or [a webpage][abs].

Explain that our function signature must match its definition:
`int abs(int j);`.

Update the code block to use the C types.

```rust
use std::ffi::c_int;

unsafe extern "C" {
    fn abs(x: c_int) -> c_int;
}
```

Discuss rationale: using `ffi::c_int` increases the portability of our code.
When the standard library is compiled for the target platform, the platform can
determine the widths. According to the C standard, a `c_int` may be defined as
an `i16` rather than the much more common `i32`.

(Optional) Show the [documentation for c_int][c_int] to reveal that it is a type
alias for `i32`.

Attempt to compile to trigger “error: extern blocks must be unsafe” error
message.

Add the unsafe keyword to the block:

```rust
use std::ffi::c_int;

unsafe extern "C" {
    fn abs(x: c_int) -> c_int;
}
```

Check that learners understand the significance of this change. We are required
to uphold type safety and other safety preconditions.

Recompile.

Add safe keyword to the abs function:

```rust
use std::ffi::c_int;

unsafe extern "C" {
    safe fn abs(x: c_int) -> c_int;
}
```

Explain that the `safe fn` marks `abs` as safe to call without an `unsafe`
block.

Completed program for reference:

```rust
use std::ffi::c_int;

unsafe extern "C" {
    safe fn abs(x: c_int) -> c_int;
}

fn main() {
    let x = -42;
    let abs_x = abs(x);
    println!("{x}, {abs_x}");
}
```

[abs]: https://www.man7.org/linux/man-pages/man3/abs.3.html
[c_int]: https://doc.rust-lang.org/std/ffi/type.c_int.html

</details>
