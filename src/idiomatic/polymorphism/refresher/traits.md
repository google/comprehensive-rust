---
minutes: 10
---

# Traits, Protocols, Interfaces

```rust
trait Receiver {
    fn send(&self, message: &str);
}

struct EmailAddress(String);

impl Receiver for EmailAddress {
    fn send(&self, message: &str) {
        println!("Email to {}: {}", self.0, message);
    }
}

struct ChatId {
    uuid: [u8; 16],
}

impl Receiver for ChatId {
    fn send(&self, message: &str) {
        println!("Chat message sent to {:?}: {}", self.uuid, message);
    }
}
```

<details>

- Rust's concept of polymorphism and generics is heavily built around traits.

- Traits are requirements on a type in a generic context.

- Requirements function much like a compile-time checked duck typing.

  Duck typing is a concept from the practice of dynamic, untyped languages like
  Python, "if it walks like a duck and quacks like a duck, it's a duck."

  That is, types with the methods and fields expected by a function are all
  valid inputs for that function. If a type implements methods, it is that type
  in a duck-typing context.

  Traits behave like a static duck typing mechanism, in that we specify behavior
  rather than type. But we get the compile-time checks on if that behavior does
  really exist.

- Alternatively: Traits are like collections of propositions, and implementing a
  trait for a type is a proof that the type can be used wherever the trait is
  asked for.

  Traits have required methods, implementing those methods is the proof that a
  type has the required behavior.

reference:

- https://doc.rust-lang.org/reference/items/traits.html

</details>
