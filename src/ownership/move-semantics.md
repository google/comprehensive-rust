# Move Semantics

An assignment will transfer _ownership_ between variables:

```rust,editable
fn main() {
    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");
}
```

* The assignment of `s1` to `s2` transfers ownership.
* When `s1` goes out of scope, nothing happens: it has no ownership.
* When `s2` goes out of scope, the string data is freed.
* There is always _exactly_ one variable binding which owns a value.

<details>

* Mention that this is the opposite of the defaults in C++, which copies by value unless you use `std::move` (and the move constructor is defined!).

* Data is not moved when you do `s2 = s1`, it is only the ownership that moves.

* Simple values (such as integers) can be marked `Copy` (see later slides).

* In Rust, clones are explicit (by using `clone`).

</details>
