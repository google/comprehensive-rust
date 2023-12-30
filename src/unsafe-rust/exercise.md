---
minutes: 30
---

# Safe FFI Wrapper

Rust has great support for calling functions through a _foreign function
interface_ (FFI). We will use this to build a safe wrapper for the `libc`
functions you would use from C to read the names of files in a directory.

You will want to consult the manual pages:

- [`opendir(3)`](https://man7.org/linux/man-pages/man3/opendir.3.html)
- [`readdir(3)`](https://man7.org/linux/man-pages/man3/readdir.3.html)
- [`closedir(3)`](https://man7.org/linux/man-pages/man3/closedir.3.html)

You will also want to browse the [`std::ffi`] module. There you find a number of
string types which you need for the exercise:

| Types                      | Encoding       | Use                            |
| -------------------------- | -------------- | ------------------------------ |
| [`str`] and [`String`]     | UTF-8          | Text processing in Rust        |
| [`CStr`] and [`CString`]   | NUL-terminated | Communicating with C functions |
| [`OsStr`] and [`OsString`] | OS-specific    | Communicating with the OS      |

You will convert between all these types:

- `&str` to `CString`: you need to allocate space for a trailing `\0` character,
- `CString` to `*const i8`: you need a pointer to call C functions,
- `*const i8` to `&CStr`: you need something which can find the trailing `\0`
  character,
- `&CStr` to `&[u8]`: a slice of bytes is the universal interface for "some
  unknown data",
- `&[u8]` to `&OsStr`: `&OsStr` is a step towards `OsString`, use
  [`OsStrExt`](https://doc.rust-lang.org/std/os/unix/ffi/trait.OsStrExt.html) to
  create it,
- `&OsStr` to `OsString`: you need to clone the data in `&OsStr` to be able to
  return it and call `readdir` again.

The [Nomicon] also has a very useful chapter about FFI.

[`std::ffi`]: https://doc.rust-lang.org/std/ffi/
[`str`]: https://doc.rust-lang.org/std/primitive.str.html
[`String`]: https://doc.rust-lang.org/std/string/struct.String.html
[`CStr`]: https://doc.rust-lang.org/std/ffi/struct.CStr.html
[`CString`]: https://doc.rust-lang.org/std/ffi/struct.CString.html
[`OsStr`]: https://doc.rust-lang.org/std/ffi/struct.OsStr.html
[`OsString`]: https://doc.rust-lang.org/std/ffi/struct.OsString.html
[Nomicon]: https://doc.rust-lang.org/nomicon/ffi.html

Copy the code below to <https://play.rust-lang.org/> and fill in the missing
functions and methods:

```rust,should_panic
// TODO: remove this when you're done with your implementation.
#![allow(unused_imports, unused_variables, dead_code)]

{{#include exercise.rs:ffi}}

{{#include exercise.rs:DirectoryIterator}}
        unimplemented!()
    }
}

{{#include exercise.rs:Iterator}}
        unimplemented!()
    }
}

{{#include exercise.rs:Drop}}
        unimplemented!()
    }
}

{{#include exercise.rs:main}}
```
