# Enum Sizes

Rust enums are packed tightly, taking constraints due to alignment into account:

```rust,editable
use std::mem::{align_of, size_of};

macro_rules! dbg_size {
    ($t:ty) => {
        println!("{}: size {} bytes, align: {} bytes",
                 stringify!($t), size_of::<$t>(), align_of::<$t>());
    };
}

enum Foo {
    A,
    B,
}

#[repr(u32)]
enum Bar {
    A,  // 0
    B = 10000,
    C,  // 10001
}

fn main() {
    dbg_size!(Foo);
    dbg_size!(Bar);
    dbg_size!(bool);
    dbg_size!(Option<bool>);
    dbg_size!(&i32);
    dbg_size!(Option<&i32>);
}
```

* See the [Rust Reference](https://doc.rust-lang.org/reference/type-layout.html).

<details>
    
Key Points: 
 * Internally Rust is using a field (discriminant) to keep track of the enum variant.
 * `Bar` enum demonstrates that there is a way to control the discriminant value and type. If `repr` is removed, the discriminant type takes 2 bytes, becuase 10001 fits 2 bytes.
 * As a niche optimization an enum discriminant is merged with the pointer so that `Option<&Foo>` is the same size as `&Foo`.
 * `Option<bool>` is another example of tight packing.
 * For [some types](https://doc.rust-lang.org/std/option/#representation), Rust guarantees that `size_of::<T>()` equals `size_of::<Option<T>>()`.
 * Zero-sized types allow for efficient implementation of `HashSet` using `HashMap` with `()` as the value.

Example code if you want to show how the bitwise representation *may* look like in practice.
It's important to note that the compiler provides no guarantees regarding this representation, therefore this is totally unsafe.

```rust,editable
use std::mem::transmute;

macro_rules! dbg_bits {
    ($e:expr, $bit_type:ty) => {
        println!("- {}: {:#x}", stringify!($e), transmute::<_, $bit_type>($e));
    };
}

fn main() {
    // TOTALLY UNSAFE. Rust provides no guarantees about the bitwise
    // representation of types.
    unsafe {
        println!("Bitwise representation of bool");
        dbg_bits!(false, u8);
        dbg_bits!(true, u8);

        println!("Bitwise representation of Option<bool>");
        dbg_bits!(None::<bool>, u8);
        dbg_bits!(Some(false), u8);
        dbg_bits!(Some(true), u8);

        println!("Bitwise representation of Option<Option<bool>>");
        dbg_bits!(Some(Some(false)), u8);
        dbg_bits!(Some(Some(true)), u8);
        dbg_bits!(Some(None::<bool>), u8);
        dbg_bits!(None::<Option<bool>>, u8);

        println!("Bitwise representation of Option<&i32>");
        dbg_bits!(None::<&i32>, usize);
        dbg_bits!(Some(&0i32), usize);
    }
}
```

More complex example if you want to discuss what happens when we chain more than 256 `Option`s together.

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
