---
minutes: 5
---

# Destructuring

Destructuring is a way of extracting data from a data structure by writing a
pattern that is matched up to the data structure, binding variables to
subcomponents of the data structure.

You can destructure tuples and arrays by matching on their elements:

## Tuples

```rust,editable
fn main() {
    describe_point((1, 0));
}

fn describe_point(point: (i32, i32)) {
    match point {
        (0, _) => println!("on Y axis"),
        (_, 0) => println!("on X axis"),
        (x, _) if x < 0 => println!("left of Y axis"),
        (_, y) if y < 0 => println!("below X axis"),
        _ => println!("first quadrant"),
    }
}
```

## Arrays

```rust,editable
{{#include ../../third_party/rust-by-example/destructuring-arrays.rs}}
```

<details>

- Create a new array pattern using `_` to represent an element.
- Add more values to the array.
- Point out that how `..` will expand to account for different number of
  elements.
- Show matching against the tail with patterns `[.., b]` and `[a@..,b]`

</details>
