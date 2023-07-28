# Lifetimes

A borrowed value has a _lifetime_:

* The lifetime can be implicit: `add(p1: &Point, p2: &Point) -> Point`.
* Lifetimes can also be explicit: `&'a Point`, `&'document str`.
* Read `&'a Point` as "a borrowed `Point` which is valid for at least the
  lifetime `a`".
* Lifetimes are always inferred by the compiler: you cannot assign a lifetime
  yourself.
  * Lifetime annotations create constraints; the compiler verifies that there is
    a valid solution.
* Lifetimes for function arguments and return values must be fully specified,
  but Rust allows lifetimes to be elided in most cases with [a few simple
  rules](https://doc.rust-lang.org/nomicon/lifetime-elision.html).
