---
minutes: 10
---

# Lifetime Annotations

A reference has a _lifetime_, which must not "outlive" the value it refers to.
This is verified by the borrow checker.

The lifetime can be implicit - this is what we have seen so far. Lifetimes can
also be explicit: `&'a Point`, `&'document str`. Lifetimes start with `'` and
`'a` is a typical default name. Read `&'a Point` as "a borrowed `Point` which is
valid for at least the lifetime `a`".

Only ownership, not lifetime annotations, control when values are destroyed and
determine the concrete lifetime of a given value. The borrow checker just
validates that borrows never extend beyond the concrete lifetime of the value.

Explicit lifetime annotations, like types, are required on function signatures
(but can be elided in common cases). These provide information for inference at
callsites and within the function body, helping the borrow checker to do its
job.

<!-- The multi-line formatting by rustfmt in left_most is apparently
     intentional: https://github.com/rust-lang/rustfmt/issues/1908 -->

```rust,editable,compile_fail
#[derive(Debug)]
struct Point(i32, i32);

fn left_most(p1: &Point, p2: &Point) -> &Point {
    if p1.0 < p2.0 { p1 } else { p2 }
}

fn main() {
    let p1: Point = Point(10, 10);
    let p2: Point = Point(20, 20);
    let p3 = left_most(&p1, &p2); // What is the lifetime of p3?
    dbg!(p3);
}
```

<details>

In this example, the compiler does not know what lifetime to infer for `p3`.
Looking inside the function body shows that it can only safely assume that
`p3`'s lifetime is the shorter of `p1` and `p2`. But just like types, Rust
requires explicit annotations of lifetimes on function arguments and return
values.

Add `'a` appropriately to `left_most`:

```rust,ignore
fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
```

This says there is some lifetime `'a` which both `p1` and `p2` outlive, and
which outlives the return value. The borrow checker verifies this within the
function body, and uses this information in `main` to determine a lifetime for
`p3`.

Try dropping `p2` in `main` before printing `p3`.

</details>
