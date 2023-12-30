# Handwritten FFI

We can declare external functions by hand:

```rust
extern "C" {
    fn abs(x: i32) -> i32;
}

fn main() {
    let x = -42;
    let abs_x = unsafe { abs(x) };
    println!("{x}, {abs_x}");
}
```

We already saw this in the
[Safe FFI Wrapper exercise](../../exercises/day-3/safe-ffi-wrapper.md).

> This assumes full knowledge of the target platform. Not recommended for
> production.
