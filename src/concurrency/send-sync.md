# `Send` and `Sync`

How does Rust know to forbid shared access across thread? The answer is in two traits:

* [`Send`][1]: a type `T` is `Send` if it is safe to move a `T` across a thread
  boundary.
* [`Sync`][2]: a type `T` is `Sync` if it is safe to move a `&T` across a thread
  boundary.

Both traits are not to be implemented. They are implemented automatically when 
the compiler determines it’s appropriate.

[1]: https://doc.rust-lang.org/std/marker/trait.Send.html
[2]: https://doc.rust-lang.org/std/marker/trait.Sync.html

<details>

* One can think of these traits as markers that the type has certain
  thread-safety properties.
* They can be used in the generic constraints as normal traits.
  
</details>
