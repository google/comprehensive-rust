---
minutes: 10
---

# Tuples and Arrays

Tuples and arrays are the first "compound" types we have seen. All elements of
an array have the same type, while tuples can accommodate different types. Both
types have a size fixed at compile time.

|        | Types                         | Literals                          |
| ------ | ----------------------------- | --------------------------------- |
| Arrays | `[T; N]`                      | `[20, 30, 40]`, `[0; 3]`          |
| Tuples | `()`, `(T,)`, `(T1, T2)`, ... | `()`, `('x',)`, `('x', 1.2)`, ... |

```rust,editable
fn main() {
    let t: (i8, bool) = (7, true);
    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);
}
```

<details>

- Like arrays, tuples have a fixed length.

- Tuples group together values of different types into a compound type.

- Fields of a tuple can be accessed by the period and the index of the value,
  e.g. `t.0`, `t.1`.

- The empty tuple `()` is referred to as the "unit type" and signifies absence
  of a return value, akin to `void` in other languages.

</details>
