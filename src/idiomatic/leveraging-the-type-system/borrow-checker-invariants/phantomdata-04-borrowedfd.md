---
minutes: 5
---

# PhantomData 4/4: BorrowedFd

`BorrowedFd` is a prime example.

```rust
```

<details>

- [`BorrowedFd`](https://rust-lang.github.io/rfcs/3128-io-safety.html#ownedfd-and-borrowedfdfd)
  uses these captured lifetimes to enforce the invariant that "if this file
  descriptor exists, the OS file descriptor is still open."

  `BorrowedFd`'s lifetime parameter demands that there exists another value (in
  this case a file, in the Unix sense) in your program that lasts as long as the
  `BorrowedFd` or outlives it.

  This has been encoded by the API designer to mean _that other value is what
  keeps the access to the file open_.

  Because `BorrowedFd` has a lifetime parameter from that other value, users of
  the API can assume "this file descriptor existing means the file is open, and
  we don't need to manage or check that external state itself."

  Its counterpart `OwnedFd` is instead a file descriptor that closes that file
  on drop.

</details>
