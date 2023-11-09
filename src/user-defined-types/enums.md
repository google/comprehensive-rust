---
minutes: 5
existing course material:
- enums.md
- enums/variant-payloads.md
- enums/sizes.md
---

<!-- NOTES:
Including enums with payloads
-->
# Enums

# Enums

The `enum` keyword allows the creation of a type which has a few
different variants:

```rust,editable
fn generate_random_number() -> i32 {
    // Implementation based on https://xkcd.com/221/
    4  // Chosen by fair dice roll. Guaranteed to be random.
}

#[derive(Debug)]
enum CoinFlip {
    Heads,
    Tails,
}

fn flip_coin() -> CoinFlip {
    let random_number = generate_random_number();
    if random_number % 2 == 0 {
        return CoinFlip::Heads;
    } else {
        return CoinFlip::Tails;
    }
}

fn main() {
    println!("You got: {:?}", flip_coin());
}
```

<details>

Key Points:

* Enumerations allow you to collect a set of values under one type
* This page offers an enum type `CoinFlip` with two variants `Heads` and `Tails`. You might note the namespace when using variants.
* This might be a good time to compare Structs and Enums:
  * In both, you can have a simple version without fields (unit struct) or one with different types of fields (variant payloads).
  * In both, associated functions are defined within an `impl` block.
  * You could even implement the different variants of an enum with separate structs but then they wouldn’t be the same type as they would if they were all defined in an enum.
</details>
# Variant Payloads

You can define richer enums where the variants carry data. You can then use the
`match` statement to extract the data from each variant:

```rust,editable
{{#include ../../third_party/rust-by-example/webevent.rs}}
```

<details>

* The values in the enum variants can only be accessed after being pattern matched. The pattern binds references to the fields in the "match arm" after the `=>`.
  * The expression is matched against the patterns from top to bottom. There is no fall-through like in C or C++.
  * The match expression has a value. The value is the last expression in the match arm which was executed.
  * Starting from the top we look for what pattern matches the value then run the code following the arrow. Once we find a match, we stop.
* Demonstrate what happens when the search is inexhaustive. Note the advantage the Rust compiler provides by confirming when all cases are handled.
* `match` inspects a hidden discriminant field in the `enum`.
* It is possible to retrieve the discriminant by calling `std::mem::discriminant()`
  * This is useful, for example, if implementing `PartialEq` for structs where comparing field values doesn't affect equality.
* `WebEvent::Click { ... }` is not exactly the same as `WebEvent::Click(Click)` with a top level `struct Click { ... }`. The inlined version cannot implement traits, for example.

</details>
# Enum Sizes

Rust enums are packed tightly, taking constraints due to alignment into account:

```rust,editable
use std::any::type_name;
use std::mem::{align_of, size_of};

fn dbg_size<T>() {
    println!("{}: size {} bytes, align: {} bytes",
        type_name::<T>(), size_of::<T>(), align_of::<T>());
}

enum Foo {
    A,
    B,
}

fn main() {
    dbg_size::<Foo>();
}
```

* See the [Rust Reference](https://doc.rust-lang.org/reference/type-layout.html).

<details>

Key Points:

 * Internally Rust is using a field (discriminant) to keep track of the enum variant.

 * You can control the discriminant if needed (e.g., for compatibility with C):

     <!-- mdbook-xgettext: skip -->
     ```rust,editable
     #[repr(u32)]
     enum Bar {
         A,  // 0
         B = 10000,
         C,  // 10001
     }

     fn main() {
         println!("A: {}", Bar::A as u32);
         println!("B: {}", Bar::B as u32);
         println!("C: {}", Bar::C as u32);
     }
     ```

    Without `repr`, the discriminant type takes 2 bytes, because 10001 fits 2
    bytes.


 * Try out other types such as

     * `dbg_size!(bool)`: size 1 bytes, align: 1 bytes,
     * `dbg_size!(Option<bool>)`: size 1 bytes, align: 1 bytes (niche optimization, see below),
     * `dbg_size!(&i32)`: size 8 bytes, align: 8 bytes (on a 64-bit machine),
     * `dbg_size!(Option<&i32>)`: size 8 bytes, align: 8 bytes (null pointer optimization, see below).

## More to Explore

Rust has several optimizations it can employ to make enums take up less space.

 * Niche optimization: Rust will merge unused bit patterns for the enum
   discriminant.

 * Null pointer optimization: For [some
   types](https://doc.rust-lang.org/std/option/#representation), Rust guarantees
   that `size_of::<T>()` equals `size_of::<Option<T>>()`.

     Example code if you want to show how the bitwise representation *may* look like in practice.
     It's important to note that the compiler provides no guarantees regarding this representation, therefore this is totally unsafe.

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

     More complex example if you want to discuss what happens when we chain more than 256 `Option`s together.

     <!-- mdbook-xgettext: skip -->
     ```rust,editable
     #![recursion_limit = "1000"]

     use std::mem::transmute;

     macro_rules! dbg_bits {
         ($e:expr, $bit_type:ty) => {
             println!("- {}: {:#x}", stringify!($e), transmute::<_, $bit_type>($e));
         };
     }

     // Macro to wrap a value in 2^n Some() where n is the number of "@" signs.
     // Increasing the recursion limit is required to evaluate this macro.
     macro_rules! many_options {
         ($value:expr) => { Some($value) };
         ($value:expr, @) => {
             Some(Some($value))
         };
         ($value:expr, @ $($more:tt)+) => {
             many_options!(many_options!($value, $($more)+), $($more)+)
         };
     }

     fn main() {
         // TOTALLY UNSAFE. Rust provides no guarantees about the bitwise
         // representation of types.
         unsafe {
             assert_eq!(many_options!(false), Some(false));
             assert_eq!(many_options!(false, @), Some(Some(false)));
             assert_eq!(many_options!(false, @@), Some(Some(Some(Some(false)))));

             println!("Bitwise representation of a chain of 128 Option's.");
             dbg_bits!(many_options!(false, @@@@@@@), u8);
             dbg_bits!(many_options!(true, @@@@@@@), u8);

             println!("Bitwise representation of a chain of 256 Option's.");
             dbg_bits!(many_options!(false, @@@@@@@@), u16);
             dbg_bits!(many_options!(true, @@@@@@@@), u16);

             println!("Bitwise representation of a chain of 257 Option's.");
             dbg_bits!(many_options!(Some(false), @@@@@@@@), u16);
             dbg_bits!(many_options!(Some(true), @@@@@@@@), u16);
             dbg_bits!(many_options!(None::<bool>, @@@@@@@@), u16);
         }
     }
     ```

</details>
