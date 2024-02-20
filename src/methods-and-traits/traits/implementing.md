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

- Types must implement the trait with an `impl <trait> for <type> { .. }` block.

- Unlike Go interfaces, just having matching methods is not enough: a `Cat` type
  with a `talk()` method would not automatically satisfy `Pet` unless it is in
  an `impl Pet` block.

- Traits may specify pre-implemented (provided) methods and methods that users
  are required to implement themselves. Provided methods can rely on required
  methods. In this case, `greet` is provided, and relies on `talk`.

</details>
