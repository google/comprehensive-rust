# What is Polymorphism?

In a dynamic language like Python, we don't have to do anything special to allow
a function to accept arguments of different types:

```python,editable
def print_value(val):
    print(val)

print_value(123)
print_value("hello")
print_value({})
```

But Rust's type system is extremely static, meaning that by default a function's
arguments are limited to exactly the type declared in the function signature:

```rust,editable
fn print_value(val: i32) {
    println!("{}", val);
}

fn main() {
    print_value(123);
    // print_value("hello"); // 🛠️❌ Mismatched types!
}
```

<details>

- If you are coming from a dynamic language like Python, the concept of
  polymorphism may be new to you because in dynamic languages everything is
  inherently polymorphic. But Rust is a very statically-typed language, meaning
  the compiler heavily restricts what types can be used where.

- Static typing is a powerful tool that allows the compiler to enforce correct
  usage of our APIs: If your function needs to be given an `i32` in order to
  function correctly, the compiler won't allow a user of that function to pass
  in a string.

- But static typing is restrictive in cases where we want to write code that is
  flexible in its handling of types. In the example on this slide, we might want
  our `print_value` function to be able to print both numbers and strings, but
  as written it can only accept `i32` values.

- Edit `print_value` to make it generic using `Display` to print different kinds
  of values:

  ```rust
  fn print_value(val: impl std::fmt::Display) {
      println!("{}", val);
  }
  ```

</details>
