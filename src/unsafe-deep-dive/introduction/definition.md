---
minutes: 5
---

# Defining Unsafe Rust

<!-- mdbook-xgettext: skip -->

```bob
╭───────────────────────────────────────────────────────────╮
│╭─────────────────────────────────────────────────────────╮│
││                                                         ││
││  Safe                                                   ││
││  Rust                                                   ││
││                                                         ││
││                                                         ││
│╰─────────╮                                               ││
│          │                                               ││
│  Unsafe  │                                               ││
│   Rust   │                                               ││
│          ╰───────────────────────────────────────────────╯│
╰───────────────────────────────────────────────────────────╯
```

<details>

“Unsafe Rust is a superset of Safe Rust.”

“Unsafe Rust adds extra capabilities, such as allowing you to dereference raw
pointers and call functions that can break Rust’s safety guarantees if called
incorrectly.”

“These extra capabilities are referred to as _unsafe operations_.”

“Unsafe operations provide the foundation that the Rust standard library is
built on. For example, without the ability to dereference a raw pointer, it
would be impossible to implement `Vec` or `Box`.”

“The compiler will still assist you while writing Unsafe Rust. Borrow checking
and type safety still apply. Unsafe operations have their own rules, which we’ll
learn about in this class.”

The unsafe operations from the [Rust Reference] (Avoid spending too much time):

> The following language level features cannot be used in the safe subset of
> Rust:
>
> - Dereferencing a raw pointer.
> - Reading or writing a mutable or unsafe external static variable.
> - Accessing a field of a union, other than to assign to it.
> - Calling an `unsafe` function.
> - Calling a safe function marked with a `<target_feature>` from a function
>   that does not have a `<target_feature>` attribute enabling the same
>   features.
> - Implementing an unsafe trait.
> - Declaring an extern block.
> - Applying an unsafe attribute to an item.

[Rust Reference]: https://doc.rust-lang.org/reference/unsafety.html

</details>
