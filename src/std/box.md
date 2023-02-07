# `Box`

[`Box`][1] is an owned pointer to data on the heap:

```rust,editable
fn main() {
    let five = Box::new(5);
    println!("five: {}", *five);
}
```


```bob
 Stack                     Heap
.- - - - - - -.     .- - - - - - -.
:             :     :             :
:    five     :     :             :
:   +-----+   :     :   +-----+   :
:   | o---|---+-----+-->|  5  |   :
:   +-----+   :     :   +-----+   :
:             :     :             :
:             :     :             :
`- - - - - - -'     `- - - - - - -'
```

`Box<T>` implements `Deref<Target = T>`, which means that you can [call methods
from `T` directly on a `Box<T>`][2].

[1]: https://doc.rust-lang.org/std/boxed/struct.Box.html
[2]: https://doc.rust-lang.org/std/ops/trait.Deref.html#more-on-deref-coercion

<details>

* `Box` is like `std::unique_ptr` in C++. 
* In the above example, you can even leave out the `*` in the `println!` statement thanks to `Deref`. 
* A `Box` can be useful when you
   * have a size that can’t be known at compile time,  but the rust compiler wants to know an exact size.
   * want to transfer owenership of a large amount of data. To avoid copying large amounts of data on the stack, instead store data on a heap in a box and only pointer data is copied.
</details>
