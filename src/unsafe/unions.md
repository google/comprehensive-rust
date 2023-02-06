# Unions

Unions are like enums, but you need to track the active field yourself:

```rust,editable
#[repr(C)]
union MyUnion {
    i: u8,
    b: bool,
}

fn main() {
    let u = MyUnion { i: 42 };
    println!("int: {}", unsafe { u.i });
    println!("bool: {}", unsafe { u.b });  // Undefined behavior!
}
```

<details>

Unions are very rarely needed in Rust as you can usually use an enum. They are occasionally needed
for interacting with C library APIs.

If you just want to reinterpret bytes as a different type, you probably want
[`std::mem::transmute`](https://doc.rust-lang.org/stable/std/mem/fn.transmute.html) or a safe
wrapper such as the [`zerocopy`](https://crates.io/crates/zerocopy) crate.

</details>
