---
minutes: 5
---

# What pinning is

- A pinned type cannot change its memory address (move)
- The pointed-to value cannot be moved by safe code

`Pin<Ptr>` makes use of the ownership system to control how the pinned value is
accessed. Rather than changing the language, Rust's ownership system is used to
enforce pinning. `Pin` owns its contents and nothing in its safe API triggers a
move.

This is explained in

<details>

Conceptually, pinning prevents the default movement behavior.

This appears to be a change in the language itself.

However, the `Pin` wrapper doesn't actually change anything fundamental about
the language.

`Pin` doesn't expose safe APIs that would allow a move. Thus, it can prevent
bitwise copy.

Unsafe APIs allow library authors to wrap types that do not implement `Unpin`,
but they must uphold the same guarantees.

The documentation of `Pin` uses the term "pointer types".

The term "pointer type" is much more broad than the pointer primitive type in
the language.

A "pointer type" wraps every type that implements `Deref` with a target that
implements `Unpin`.

Rust style note: This trait bound is enforced through trait bounds on the
`::new()` constructor, rather than on the type itself.

</details>
