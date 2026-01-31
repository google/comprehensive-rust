---
minutes: 2
---

# `to`: Non-consuming Conversion

Prefix to a function that takes a borrowed value and creates an owned value

```rust,compile_fail
impl str {
    // &str is not consumed.
    fn to_owned(&str) -> String;

    fn to_uppercase(&self) -> String;
}

impl u32 {
    // take an owned self because `u32` implements `Copy`
    to_be(self) -> u32;
}
```

<details>
- Methods that create a new owned value without consuming `self`, and imply a type conversion, are named starting with `to`.

- This is not a borrow checker escape hatch, or an instance of unsafe code. A
  new value is created, the original data is left alone.

- Methods that start with "to" return a different type, and strongly imply a
  non-trivial type conversion, or even a data transformation. For example,
  `str::to_uppercase`.

- "to" methods take `&self`. However they can take `self` by value if the type
  implements `Copy`: this also ensures that the conversion method call does not
  consume `self`.

- If you simply want to define a method that takes `&self` and returns an owned
  value of the same type, implement the `Clone` trait.

Example: to_uppercase creates a version of a string with all uppercase letters.

- If you want to define a method that consumes the source value, use the "into"
  naming pattern.

- Also seen in functions that convert the endianness of primitives, or copy and
  expose the value of a newtype.

## More to Explore

- Ask the class: What's the difference between `to_owned` and `into_owned`?

  Answer: `to_owned` appears on reference values like `&str`, whereas
  `into_owned` appears on owned values that hold reference types, like `Cow`
  (copy-on-write).

  Types like `Cow` can be owned while containing references that are borrowed,
  so the owned value of `Cow` is consumed to create an owned value of the
  reference type it was holding onto.

</details>
