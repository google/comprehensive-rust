---
minutes: 5
---

# Enums

The `enum` keyword allows the creation of a type which has a few different
variants:

```rust,editable
#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,                        // Simple variant
    Run(Direction),              // Tuple variant
    Teleport { x: u32, y: u32 }, // Struct variant
}

fn main() {
    let m: PlayerMove = PlayerMove::Run(Direction::Left);
    println!("On this turn: {:?}", m);
}
```

<details>

Key Points:

- Enumerations allow you to collect a set of values under one type.
- `Direction` is a type with variants. There are two values of `Direction`:
  `Direction::Left` and `Direction::Right`.
- `PlayerMove` is a type with three variants. In addition to the payloads, Rust
  will store a discriminant so that it knows at runtime which variant is in a
  `PlayerMove` value.
- This might be a good time to compare structs and enums:
  - In both, you can have a simple version without fields (unit struct) or one
    with different types of fields (variant payloads).
  - You could even implement the different variants of an enum with separate
    structs but then they wouldn’t be the same type as they would if they were
    all defined in an enum.
- Rust uses minimal space to store the discriminant.
  - If necessary, it stores an integer of the smallest required size
  - If the allowed variant values do not cover all bit patterns, it will use
    invalid bit patterns to encode the discriminant (the "niche optimization").
    For example, `Option<&u8>` stores either a pointer to an integer or `NULL`
    for the `None` variant.
  - You can control the discriminant if needed (e.g., for compatibility with C):

    <!-- mdbook-xgettext: skip -->
    ```rust,editable
    #[repr(u32)]
    enum Bar {
        A, // 0
        B = 10000,
        C, // 10001
    }

    fn main() {
        println!("A: {}", Bar::A as u32);
        println!("B: {}", Bar::B as u32);
        println!("C: {}", Bar::C as u32);
    }
    ```

    Without `repr`, the discriminant type takes 2 bytes, because 10001 fits 2
    bytes.

## More to Explore

Rust has several optimizations it can employ to make enums take up less space.

- Null pointer optimization: For
  [some types](https://doc.rust-lang.org/std/option/#representation), Rust
  guarantees that `size_of::<T>()` equals `size_of::<Option<T>>()`.

  Example code if you want to show how the bitwise representation _may_ look
  like in practice. It's important to note that the compiler provides no
  guarantees regarding this representation, therefore this is totally unsafe.

  <!-- mdbook-xgettext: skip -->
  ```rust,editable
  use std::mem::transmute;

  macro_rules! dbg_bits {
      ($e:expr, $bit_type:ty) => {
          println!("- {}: {:#x}", stringify!($e), transmute::<_, $bit_type>($e));
      };
  }

  fn main() {
      unsafe {
          println!("bool:");
          dbg_bits!(false, u8);
          dbg_bits!(true, u8);

          println!("Option<bool>:");
          dbg_bits!(None::<bool>, u8);
          dbg_bits!(Some(false), u8);
          dbg_bits!(Some(true), u8);

          println!("Option<Option<bool>>:");
          dbg_bits!(Some(Some(false)), u8);
          dbg_bits!(Some(Some(true)), u8);
          dbg_bits!(Some(None::<bool>), u8);
          dbg_bits!(None::<Option<bool>>, u8);

          println!("Option<&i32>:");
          dbg_bits!(None::<&i32>, usize);
          dbg_bits!(Some(&0i32), usize);
      }
  }
  ```

</details>
