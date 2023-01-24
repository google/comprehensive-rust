# Destructuring Arrays

You can destructure arrays, tuples, and slices by matching on their elements:

```rust,editable
{{#include ../../third_party/rust-by-example/destructuring-arrays.rs}}
```

<details>
  
* Create a new pattern using `_` to represent an element. 
* Add more values to the array. Some of the patterns will fail because they are expecting just 3 elements, go ahead and adjust them.
* Point out that how `..` will expand to account for different number of elements.
</details>
