---
minutes: 10
---

# Less powerful than it seems

The `unsafe` keyword does not allow you to break Rust.

```rust
use std::mem::transmute;

let orig = b"RUST";
let n: i32 = unsafe { transmute(orig) };

println!("{n}")
```

<details>

## Suggested outline

- Request that someone explains what `std::mem::transmute` does
- Discuss why it doesn't compile
- Fix the code

## Expected compiler output

```
   Compiling playground v0.0.1 (/playground)
error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
 --> src/main.rs:5:27
  |
5 |     let n: i32 = unsafe { transmute(orig) };
  |                           ^^^^^^^^^
  |
  = note: source type: `&[u8; 4]` (64 bits)
  = note: target type: `i32` (32 bits)
```

## Suggested change

```diff
- let n: i32 = unsafe { transmute(orig) };
+ let n: i64 = unsafe { transmute(orig) };
```

## Notes on less familiar Rust

- the `b` prefix on a string literal marks it as byte slice (`&[u8]`) rather
  than a string slice (`&str`)

</details>
