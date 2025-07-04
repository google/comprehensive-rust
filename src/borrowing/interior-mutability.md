---
minutes: 10
---

# Interior Mutability

In some situations, it's necessary to modify data behind a shared (read-only)
reference. For example, a shared data structure might have an internal cache,
and wish to update that cache from read-only methods.

The "interior mutability" pattern allows exclusive (mutable) access behind a
shared reference. The standard library provides several ways to do this, all
while still ensuring safety, typically by performing a runtime check.

<details>

The main thing to take away from this slide is that Rust provides _safe_ ways to
modify data behind a shared reference. There are a variety of ways to ensure
that safety, and the next sub-slides present a few of them.

</details>
