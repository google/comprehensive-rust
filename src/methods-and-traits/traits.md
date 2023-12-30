---
minutes: 10
---

# Traits

Rust lets you abstract over types with traits. They're similar to interfaces:

```rust,editable
struct Dog {
    name: String,
    age: i8,
}
struct Cat {
    lives: i8,
}

trait Pet {
    fn talk(&self) -> String;

    fn greet(&self) {
        println!("Oh you're a cutie! What's your name? {}", self.talk());
    }
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        String::from("Miau!")
    }
}

fn main() {
    let captain_floof = Cat { lives: 9 };
    let fido = Dog { name: String::from("Fido"), age: 5 };

    captain_floof.greet();
    fido.greet();
}
```

<details>

- A trait defines a number of methods that types must have in order to implement
  the trait.

- Traits are implemented in an `impl <trait> for <type> { .. }` block.

- Traits may specify pre-implemented (provided) methods and methods that users
  are required to implement themselves. Provided methods can rely on required
  methods. In this case, `greet` is provided, and relies on `talk`.

</details>
