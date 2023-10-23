# Shared Types

```rust,ignore
#[cxx::bridge]
mod ffi {
    #[derive(Clone, Debug, Hash)]
    struct PlayingCard {
        suit: Suit,
        value: u8,  // A=1, J=11, Q=12, K=13
    }

    enum Suit {
        Clubs,
        Diamonds,
        Hearts,
        Spades,
    }
}
```

<details>

* Only C-like (unit) enums are supported.
* On the Rust side, the code generated for shared enums is actually a struct
  wrapping a numeric value. This is because it is not UB in C++ for an enum
  class to hold a value different from all of the listed variants, and our Rust
  representation needs to have the same behavior.

</details>
