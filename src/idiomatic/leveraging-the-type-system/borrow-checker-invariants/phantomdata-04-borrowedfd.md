---
minutes: 10
---

# PhantomData 4/4: OwnedFd & BorrowedFd

`BorrowedFd` is a prime example of `PhantomData` in action.

```rust,editable
use libc::{O_CREAT, O_WRONLY, close, open};
use std::marker::PhantomData;
use std::os::raw::c_int;

struct OwnedFd {
    fd: c_int,
}

impl OwnedFd {
    fn try_from_fd(fd: c_int) -> Option<Self> {
        if fd < 0 {
            return None;
        }
        Some(OwnedFd { fd })
    }

    fn as_fd<'a>(&'a self) -> BorrowedFd<'a> {
        BorrowedFd { fd: self.fd, _phantom: PhantomData }
    }
}

impl Drop for OwnedFd {
    fn drop(&mut self) {
        unsafe { close(self.fd) };
    }
}

struct BorrowedFd<'a> {
    fd: c_int,
    _phantom: PhantomData<&'a ()>,
}

fn main() {
    // A file we're creating with a raw syscall. Imagine this comes from an FFI.
    let fd = unsafe { open(c"c_str.txt".as_ptr(), O_WRONLY | O_CREAT) };
    // Creating an ownedfd from a file, when this drops the file descriptor
    // will be closed.
    let owned_fd =
        OwnedFd::try_from_fd(fd).expect("Could not open file with syscall!");

    // Creating a borrowedfd from an owned fd, when this drops the file
    // descriptor will still be open because it doesn't own it!
    let borrowed_fd: BorrowedFd<'_> = owned_fd.as_fd();
    std::mem::drop(borrowed_fd);
    // borrowed_fd is now dropped.
    let second_borrowed = owned_fd.as_fd();
    // owned_fd will be dropped here, and the file will be closed.
}
```

<details>

- A file descriptor represents a specific process's access to a file.

  Reminder: Device and OS-specific features are exposed as if they were files on
  unix-style systems.

- [`OwnedFd`](https://rust-lang.github.io/rfcs/3128-io-safety.html#ownedfd-and-borrowedfdfd)
  is an owned wrapper type for a file descriptor. It _owns_ the file descriptor,
  and closes it when dropped.

  Note: We have our own implementation of it here, draw attention to the
  explicit `Drop` implementation.

  `BorrowedFd` is its borrowed counterpart, it does not need to close the file
  when it is dropped.

  Note: We have not explicitly implemented `Drop` for `BorrowedFd`.

- `BorrowedFd` uses a lifetime captured with a `PhantomData` to enforce the
  invariant "if this file descriptor exists, the OS file descriptor is still
  open even though it is not responsible for closing that file descriptor."

  The lifetime parameter of `BorrowedFd` demands that there exists another value
  in your program that lasts as long as that specific `BorrowedFd` or outlives
  it (in this case an `OwnedFd`).

  This has been encoded by the API designers to mean _that other value is what
  keeps the access to the file open_.

  Because Rust's borrow checker enforces this relationship where one value must
  last at least as long as another, users of this API do not need to worry about
  handling this correct file descriptor aliasing and closing logic themselves.

</details>
