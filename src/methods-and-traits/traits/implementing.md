# Implementing Traits

```rust,editable
trait Pet {
    fn talk(&self) -> String;

    fn greet(&self) {
        println!("Oh you're a cutie! What's your name? {}", self.talk());
    }
}

struct Dog {
    name: String,
    age: i8,
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }
}

fn main() {
    let fido = Dog { name: String::from("Fido"), age: 5 };
    dbg!(fido.talk());
    fido.greet();
}
```

<details>

- To implement `Trait` for `Type`, you use an `impl Trait for Type { .. }`
  block.

- Unlike Go interfaces, just having matching methods is not enough: a `Cat` type
  with a `talk()` method would not automatically satisfy `Pet` unless it is in
  an `impl Pet` block.

- Traits may provide default implementations of some methods. Default
  implementations can rely on all the methods of the trait. In this case,
  `greet` is provided, and relies on `talk`.

- Multiple `impl` blocks are allowed for a given type. This includes both
  inherent `impl` blocks and trait `impl` blocks. Likewise multiple traits can
  be implemented for a given type (and often types implement many traits!).
  `impl` blocks can even be spread across multiple modules/files.

</details>
