---
minutes: 10
---

# `From` and `Into`

Types implement [`From`][1] and [`Into`][2] to facilitate type conversions:

```rust,editable
fn main() {
    let s = String::from("hello");
    let addr = std::net::Ipv4Addr::from([127, 0, 0, 1]);
    let one = i16::from(true);
    let bigger = i32::from(123_i16);
    println!("{s}, {addr}, {one}, {bigger}");
}
```

[`Into`][2] is automatically implemented when [`From`][1] is implemented:

```rust,editable
fn main() {
    let s: String = "hello".into();
    let addr: std::net::Ipv4Addr = [127, 0, 0, 1].into();
    let one: i16 = true.into();
    let bigger: i32 = 123_i16.into();
    println!("{s}, {addr}, {one}, {bigger}");
}
```

<details>

- That's why it is common to only implement `From`, as your type will get `Into`
  implementation too.
- When declaring a function argument input type like "anything that can be
  converted into a `String`", the rule is opposite, you should use `Into`. Your
  function will accept types that implement `From` and those that _only_
  implement `Into`.

</details>

[1]: https://doc.rust-lang.org/std/convert/trait.From.html
[2]: https://doc.rust-lang.org/std/convert/trait.Into.html
