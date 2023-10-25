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
* A limited number of traits are supported for `#[derive()]` on shared types.
  Corresponding functionality is also generated for the C++ code, e.g. if you
  derive `Hash` also generates an implementation of `std::hash` for the
  corresponding C++ type.

</details>
