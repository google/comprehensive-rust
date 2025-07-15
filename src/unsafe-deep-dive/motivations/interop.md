---
minutes: 5
---

> TODO: Refactor this content into multiple slides as this slide is intended as
> an introduction to the motivations only, rather than to be an elaborate
> discussion of the whole problem.

# Interop

Language interoperability allows you to:

- Call functions written in other languages from Rust
- Write functions in Rust that are callable from other languages

However, this requires unsafe.

```rust,editable
unsafe extern "C" {
    safe fn random() -> libc::c_long;
}

fn main() {
    let a = random() as i64;
    println!("{a:?}");
}
```

<details>

The Rust compiler can't enforce any safety guarantees for programs that it
hasn't compiled, so it delegates that responsibility to you through the unsafe
keyword.

The code example we're seeing shows how to call the random function provided by
libc within Rust.

This uses Rust's _foreign function interface_.

This isn't the only style of interoperability, however it is the method that's
needed if you want to work between Rust and some other language in a zero cost
way. Another important strategy is message passing.

Message passing avoids unsafe, but serialization, allocation, data transfer and
parsing all take energy and time.

## Answers to questions

- _Where does "random" come from?_\
  libc is dynamically linked to Rust programs by default, allowing our code to
  rely on its symbols, including `random`, being available to our program.
- _What is the "safe" keyword?_\
  It allows callers to call the function without needing to wrap that call in
  `unsafe`. The [`safe` function qualifier] was introduced in the 2024 edition
  of Rust and can only be used within `extern` blocks. It was introduced because
  `unsafe` became a mandatory qualifier for `extern` blocks in that edition.
- _What is the [`std::ffi::c_long`] type?_\
  According to the C standard, an integer that's at least 32 bits wide. On
  today's systems, It's an `i32` on Windows and an `i64` on Linux.

[`safe` keyword]: https://doc.rust-lang.org/reference/safe-keyword.html
[`std::ffi::c_long`]: https://doc.rust-lang.org/std/ffi/type.c_long.html

## Consideration: type safety

Modify the code example to remove the need for type casting later. Discuss the
potential UB - long's width is defined by the target.

```rust
unsafe extern "C" {
    safe fn random() -> i64;
}

fn main() {
    let a = random();
    println!("{a:?}");
}
```

> Changes from the original:
>
> ```diff
> unsafe extern "C" {
> -    safe fn random() -> libc::c_long;
> +    safe fn random() -> i64;
> }
>
> fn main() {
> -    let a = random() as i64;
> +    let a = random();
>     println!("{a:?}");
> }
> ```

It's also possible to completely erase the type. Stress that the Rust compiler
will trust that the wrapper is telling the truth.

```rust
unsafe extern "C" {
    safe fn random() -> [u8; 64];
}

fn main() {
    let a = random();
    println!("{a:?}");
}
```

> Changes from the original:
>
> ```diff
> unsafe extern "C" {
> -    safe fn random() -> libc::c_long;
> +    safe fn random() -> [u8; 64];
> }
>
> fn main() {
> -    let a = random() as i64;
> -    println!("{a}");
> +    let a = random();
> +    println!("{a:?}");
> }
> ```

Mention that type safety is generally not a large concern in practice.
Auto-generated wrappers, i.e. those produced by bindgen and related tools, are
excellent at reading header files and producing values of the correct type.

## Consideration: Ownership and lifetime management

While libc's `random` function doesn't use pointers, may do. This creates the
possibility that interacting with another programming language introduce
unsoundness.

- both sides might attempt to free the memory (double free)
- both sides can attempt to write to the data

For example, some C libraries expose functions that write to static buffers that
are re-used between calls.

```rust
use std::ffi::{CStr, c_char};
use std::time::{SystemTime, UNIX_EPOCH};

unsafe extern "C" {
    /// Create a formatted time based on time `t`, including trailing newline.
    fn ctime(t: *const libc::time_t) -> *const c_char;
}

unsafe fn format_timestamp<'a>(t: u64) -> &'a str {
    let t = t as libc::time_t;

    unsafe {
        let fmt_ptr = ctime(&t);
        CStr::from_ptr(fmt_ptr).to_str().unwrap()
    }
}

fn main() {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let now = now.as_secs();
    let now_fmt = unsafe { format_timestamp(now) };
    print!("now (1): {}", now_fmt);

    let future = now + 60;
    let future_fmt = unsafe { format_timestamp(future) };
    print!("future:  {}", future_fmt);

    print!("now (2): {}", now_fmt);
}
```

Bonus points: can anyone spot the lifetime bug? `format_timestamp()` should
return a `&'static str`.

## Consideration: Representation mismatch

Different programming languages have made design decisions and this can create
impedance mismatches between different domains.

Consider string handling. C++ defines `std::string`, which has an incompatible
memory layout with Rust's `String` type. `String` also requires text to be
encoded as UTF-8, whereas `std::string` does not. In C, text is represented by a
null-terminated sequence of bytes (`char*`).

```rust
fn main() {
    let c_repr = b"Hello, C\0";
    let rust_repr = (b"Hello, Rust", 11);

    let c: &str = unsafe {
        let ptr = c_repr.as_ptr() as *const i8;
        std::ffi::CStr::from_ptr(ptr).to_str().unwrap()
    };
    println!("{c}");

    let rust: &str = unsafe {
        let ptr = rust_repr.0.as_ptr();
        let bytes = std::slice::from_raw_parts(ptr, rust_repr.1);
        std::str::from_utf8_unchecked(bytes)
    };
    println!("{rust}");
}
```

</details>
