# Compound Types

|        | Types               | Literals                 |
|--------|---------------------|--------------------------|
| Arrays | `[T; N]`            | `[20, 30, 40]`, `[0; 3]` |
| Tuples | `(T1, T2, T3, ...)` | `('x', 1.2, 0)`          |

Array assignment and access:

```rust,editable
fn main() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {:?}", a);
}
```

Tuple assignment and access:

```rust,editable
fn main() {
    let t: (i8, bool) = (7, true);
    println!("1st index: {}", t.0);
    println!("2nd index: {}", t.1);
}
```
