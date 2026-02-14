---
minutes: 10
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# PhantomData 4/4: OwnedFd & BorrowedFd

`BorrowedFd` is a prime example of `PhantomData` in action.

<!--
  This code has to define a fake libc module even though libc works fine on
  rust playground because the CI does not currently support dependencies.

  TODO: Once we can use libc as a dependency in Rust tests, replace the
  faux libc code with appropriate imports & `O_WRONLY | O_CREAT` permissions.
-->

```rust,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
use std::marker::PhantomData;
use std::os::raw::c_int;

mod libc_ffi {
    use std::os::raw::{c_char, c_int};
    pub unsafe fn open(path: *const c_char, oflag: c_int) -> c_int {
        3
    }
    pub unsafe fn close(fd: c_int) {}
}

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
        unsafe { libc_ffi::close(self.fd) };
    }
}

struct BorrowedFd<'a> {
    fd: c_int,
    _phantom: PhantomData<&'a ()>,
}

fn main() {
    // Create a file with a raw syscall with write-only and create permissions.
    let fd = unsafe { libc_ffi::open(c"c_str.txt".as_ptr(), 065) };
    // Pass the ownership of an integer file descriptor to an `OwnedFd`.
    // `OwnedFd::drop()` closes the file descriptor.
    let owned_fd =
        OwnedFd::try_from_fd(fd).expect("Could not open file with syscall!");

    // Create a `BorrowedFd` from an `OwnedFd`.
    // `BorrowedFd::drop()` does not close the file because it doesn't own it!
    let borrowed_fd: BorrowedFd<'_> = owned_fd.as_fd();
    // std::mem::drop(owned_fd); // ❌🔨
    std::mem::drop(borrowed_fd);
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

  Demonstrate: Uncomment the `std::mem::drop(owned_fd)` line and try to compile
  to show that `borrowed_fd` relies on the lifetime of `owned_fd`.

  This has been encoded by the API designers to mean _that other value is what
  keeps the access to the file open_.

  Because Rust's borrow checker enforces this relationship where one value must
  last at least as long as another, users of this API do not need to worry about
  handling this correct file descriptor aliasing and closing logic themselves.

</details>
