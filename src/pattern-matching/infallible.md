---
minutes: 5
---

# Irrefutable Patterns

In day 1 we briefly saw how patterns can be used to _destructure_ compound
values. Let's review that and talk about a few other things patterns can
express:

```rust,editable
fn takes_tuple(tuple: (char, i32, bool)) {
    let a = tuple.0;
    let b = tuple.1;
    let c = tuple.2;

    // This does the same thing as above.
    let (a, b, c) = tuple;

    // Ignore the first element, only bind the second and third.
    let (_, b, c) = tuple;

    // Ignore everything but the last element.
    let (.., c) = tuple;
}

fn main() {
    takes_tuple(('a', 777, true));
}
```

<details>

- All of the demonstrated patterns are _irrefutable_, meaning that they will
  always match the value on the right hand side.

- Patterns are type-specific, including irrefutable patterns. Try adding or
  removing an element to the tuple and look at the resulting compiler errors.

- Variable names are patterns that always match and bind the matched
  value into a new variable with that name.

- `_` is a pattern that always matches any value, discarding the matched value.

- `..` allows you to ignore multiple values at once.

## More to Explore

- You can also demonstrate more advanced usages of `..`, such as ignoring the
  middle elements of a tuple.

  ```rust
  fn takes_tuple(tuple: (char, i32, bool, u8)) {
      let (first, .., last) = tuple;
  }
  ```

- All of these patterns work with arrays as well:

  ```rust
  fn takes_array(array: [u8; 5]) {
      let [first, .., last] = array;
  }
  ```

</details>
