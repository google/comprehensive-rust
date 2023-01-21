# Slices

A slice gives you a view into a larger collection:

```rust,editable
fn main() {
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");
}
```

* Slices borrow data from the sliced type.
* Question: What happens if you modify `a[3]`?

<details>

* We create a slice by borrowing `a` and specifying the starting and ending indexes in brackets.

* If the slice starts at index 0, Rust’s range syntax means we can drop the starting index. 
    
* The same is true for the last index, so `&a[2..a.len()]` and `&a[2..]` are equal.

* `s` is a reference to a slice of `i32`s. Notice that the type of `s` no longer mentions the array length. This allows us to performing computations on slices of different sizes.
 
* Slices always borrow from another object. In this example, `a` has to remain 'alive' so we can take a slice from it. 
    
* The question about modifying `a[3]` can spark an interesting discussion, but the answer is that for memory safety reasons
  you cannot do it through `a` after you created a slice. But you can read the data from both `a` and `s` safely. 
  More details will be explained in the borrow checker section.

</details>
