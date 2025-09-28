---
minutes: 10
---

# Clarity: Do Provide Context

Codebases are full of relationships between types, functions, inputs. Represent
them well!

```rust
pub struct MyType { pub my_number: u32 };

impl MyType {
    fn add_one(&mut self) { self.my_number += 1; }
    fn business_logic() {}
}

mod another_package {
    pub fn add_one(my_type: &mut super::MyType) { my_type.my_number += 1; }
}

fn add_one(my_type: &mut MyType) { my_type.my_number += 1; }

fn main() {
    let mut value = MyType { my_number: 39 };
    value.add_one();
    add_one(&mut value);
    another_package::add_one(&mut value);
}
```

<details>

- Context clues let a reader quickly understand details about what's going on.
  These can be anything from descriptive names, to if a function is a method, or
  where that function comes from.

- Descriptive names are key, but can be subjective in highly specialized
  business logic areas. Try to keep things

- Demo: Ask for suggestions for what the `MyType::business_logic` method does,
  then ask how we might rename the method.

- Ask: What is the difference in what you assume about the source of the
  function `add_one` when it's a method vs when it's a function that takes a
  value, or when it's a function from another module?

  - We know what it does, the name is descriptive enough.

  - Potentially different authors, different packages.

  While it makes sense to keep functions as methods a lot of the time, as
  there's usually an "authoritative" type, there's still plenty of reasons a
  function might not be a method.

  Note: Remember that a method is a relationship between a function and a type.

</details>
