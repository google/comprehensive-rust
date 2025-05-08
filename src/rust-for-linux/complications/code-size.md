---
minutes: 10
---

# Code Size

One pitfall when writing Rust code can be the multiplicative increase in
generated machine code when using generics.

For the Linux kernel, which must be suitable for space-limited embedded
environments, keeping code size low is a significant concern.

Experiments with Rust in the kernel so far have shown that Rust code can be of
similar code size to C, but may also be larger in some cases[^1].

## Assessing Bloat

Tools exist to help analyze different source code's contribution to the size of
compiled code, such as
[`cargo-bloat`](https://github.com/RazrFalcon/cargo-bloat).

## Shrinking Code Size

The reasons for code bloat vary and are not generally specific to Linux kernel
usage of Rust. The most common causes for code bloat are excessive use of
generics and forced inlining. In general, generics should be preferred over
trait objects when writing abstractions that are expected to "compile out" or
where generating separate code for different types is critical for performance
(e.g. inner loops or arithmetic on values of a generic type).

In other situations, trait objects should be preferred to allow reusing
definitions without machine-code duplication, which may closer mirror patterns
that would be most natural in C.

When accepting generic parameters that get converted to a concrete type before
use, follow the pattern of defining an inner monomorphic function that can be
shared[^2]:

```rust
pub fn read_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fn inner(path: &Path) -> io::Result<String> {
        let mut file = File::open(path)?;
        let size = file.metadata().map(|m| m.len() as usize).ok();
        let mut string = String::with_capacity(size.unwrap_or(0));
        io::default_read_to_string(&mut file, &mut string, size)?;
        Ok(string)
    }
    inner(path.as_ref())
}
```

[^1]: <https://www.usenix.org/system/files/atc24-li-hongyu.pdf>

[^2]: <https://github.com/rust-lang/rust/blob/ae612bedcbfc7098d1711eb35bc7ca994eb17a4c/library/std/src/fs.rs#L295-L304>
