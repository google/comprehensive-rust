---
minutes: 10
---

# PhantomData 4/4: OwnedFd & BorrowedFd

`BorrowedFd` is a prime example of `PhantomData` in action.

```rust,editable
use std::fs::File;
use std::os::fd::{AsFd, BorrowedFd, OwnedFd};

fn main() {
    // A file we're creating.
    let file = File::create("my_file.txt");
    // Creating an ownedfd from a file, when this drops the file descriptor
    // will be closed.
    let owned_fd: OwnedFd = OwnedFd::from(file.unwrap());
    {
        // Creating a borrowedfd from an owned fd, when this drops the file
        // descriptor will still be open because it doesn't own it!
        let borrowed_fd: BorrowedFd<'_> = owned_fd.as_fd();
        // borrowed_fd will be dropped here, due to end of scope.
    }
    // owned_fd will be dropped here, and the file will be closed.
}
```

<details>

- A file descriptor represents a specific process's access to a file.

  Reminder: Device and OS-specific features are exposed as if they were files on
  unix-style systems.

- [`OwnedFd`](https://rust-lang.github.io/rfcs/3128-io-safety.html#ownedfd-and-borrowedfdfd)
  is a file descriptor representation that automatically closes the file when
  dropped.

  `BorrowedFd` is its borrowed counterpart, it does not need to close the file
  when it is dropped.

- `BorrowedFd` uses a lifetime captured with a `PhantomData` to enforce the
  invariant "if this file descriptor exists, the OS file descriptor is still
  open even though it is not responsible for closing that file descriptor."

  The lifetime parameter of `BorrowedFd` demands that there exists another value
  in your program that lasts as long as that specific `BorrowedFd` or outlives
  it (in this case an `OwnedFd`).

  This has been encoded by the API designers to mean _that other value is what
  keeps the access to the file open_.

  Because rust's borrow checker enforces this relationship where one value must
  last at least as long as another, users of this API do not need to worry about
  handling this correct file descriptor aliasing and closing logic themselves.

</details>
