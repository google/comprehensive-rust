# Destructuring Arrays

You can destructure arrays, tuples, and slices by matching on their elements:

```rust,editable
{{#include ../../third_party/rust-by-example/destructuring-arrays.rs}}
```


<details>

* Destructuring of slices of unknown length also works with patterns of fixed length.


```rust,editable
fn main() {
    inspect(&[0, -2, 3]);
    inspect(&[0, -2, 3, 4]);
}

#[rustfmt::skip]
fn inspect(slice: &[i32]) {
    println!("Tell me about {slice:?}");
    match slice {
        &[0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        &[1, ..]   => println!("First is 1 and the rest were ignored"),
        _          => println!("All elements were ignored"),
    }
}
```
  
* Create a new pattern using `_` to represent an element. 
* Add more values to the array.
* Point out that how `..` will expand to account for different number of elements.
* Show matching against the tail with patterns `[.., b]` and `[a@..,b]`

</details>
