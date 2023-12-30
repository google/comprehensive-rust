# `Sync`

> A type `T` is [`Sync`][1] if it is safe to access a `T` value from multiple
> threads at the same time.

More precisely, the definition is:

> `T` is `Sync` if and only if `&T` is `Send`

[1]: https://doc.rust-lang.org/std/marker/trait.Sync.html

<details>

This statement is essentially a shorthand way of saying that if a type is
thread-safe for shared use, it is also thread-safe to pass references of it
across threads.

This is because if a type is Sync it means that it can be shared across multiple
threads without the risk of data races or other synchronization issues, so it is
safe to move it to another thread. A reference to the type is also safe to move
to another thread, because the data it references can be accessed from any
thread safely.

</details>
