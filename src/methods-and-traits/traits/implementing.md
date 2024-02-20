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

</details>
