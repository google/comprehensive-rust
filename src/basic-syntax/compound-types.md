# Compound Types

|        | Types                         | Literals                          |
|--------|-------------------------------|-----------------------------------|
| Arrays | `[T; N]`                      | `[20, 30, 40]`, `[0; 3]`          |
| Tuples | `()`, `(T,)`, `(T1, T2)`, ... | `()`, `('x',)`, `('x', 1.2)`, ... |

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

<details>
    
Key points:
    
Arrays:
    
*Arrays have elements of the same type, T, and length, N, which is fixed. 

*We can use literals to assign values to arrays.

*Demonstrate out of bounds errors by setting n to different values 

```
a[n+15] = 11 // index out of bounds error since len is 10
```

*Efficient way to check n is in bounds:

```
assert!(n+20 < a.len()); // panics
```

*In the main function, the print statement contains the debug implementation {a :?}.

*Adding `#`, eg `{a:#?}`, invokes a "pretty printing" format, which can be easier to read.

Tuples:

*Like arrays, tuples have a fixed length.

*Tuples group together values of different types into a compound type. 

*Fields that can be accessed by the period and the index of the value, e.g. t.0, t.1.

</details>
