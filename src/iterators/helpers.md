---
minutes: 5
---

# `Iterator` Helper Methods

In addition to the `next` method that defines how an iterator behaves, the
`Iterator` trait provides 70+ helper methods that can be used to build
customized iterators.

```rust,editable
let result: i32 = (1..10) // Create a range from 1 to 9
    .filter(|&x| x % 2 == 0) // Keep only even numbers
    .map(|x| x * x) // Square each number
    .sum(); // Sum up all the squared numbers

println!("The sum of squares of even numbers from 1 to 9 is: {}", result);
```

<details>

- The `Iterator` trait implements many common functional programming operations
  over collections (e.g. `map`, `filter`, `reduce`, etc). This is the trait
  where you can find all the documentation about them.

- Many of these helper methods take the original iterator and produce a new
  iterator with different behavior. These are know as "iterator adapter
  methods".

- Some methods, like `sum` and `count`, consume the iterator and pull all of the
  elements out of it.

- These methods are designed to be chained together so that it's easy to build a
  custom iterator that does exactly what you need.

## More to Explore

- Rust's iterators are extremely efficient and highly optimizable. Even complex
  iterators made by combining many adapter methods will still result in code as
  efficient as equivalent imperative implementations.

</details>
