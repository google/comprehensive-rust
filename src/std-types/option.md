---
minutes: 10
---

# Option

We have already seen some use of `Option<T>`. It stores either a value of type
`T` or nothing. For example,
[`String::find`](https://doc.rust-lang.org/stable/std/string/struct.String.html#method.find)
returns an `Option<usize>`.

```rust,editable,should_panic
fn main() {
    let name = "Löwe 老虎 Léopard Gepardi";
    let mut position: Option<usize> = name.find('é');
    println!("find returned {position:?}");
    assert_eq!(position.unwrap(), 14);
    position = name.find('Z');
    println!("find returned {position:?}");
    assert_eq!(position.expect("Character not found"), 0);
}
```

<details>

- `Option` is widely used, not just in the standard library.
- `unwrap` will return the value in an `Option`, or panic. `expect` is similar
  but takes an error message.
  - You can panic on None, but you can't "accidentally" forget to check for
    None.
  - It's common to `unwrap`/`expect` all over the place when hacking something
    together, but production code typically handles `None` in a nicer fashion.
- The niche optimization means that `Option<T>` often has the same size in
  memory as `T`.

</details>
