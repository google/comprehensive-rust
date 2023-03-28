# Traits

Rust lets you abstract over types with traits. They're similar to interfaces:

```rust,editable
trait Pet {
    fn name(&self) -> String;
}

struct Dog {
    name: String,
}

struct Cat;

impl Pet for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Pet for Cat {
    fn name(&self) -> String {
        String::from("The cat") // No name, cats won't respond to it anyway.
    }
}

fn greet(pet: &impl Pet) {
    println!("Who's a cutie? {} is!", pet.name());
}

fn main() {
    let fido = Dog { name: "Fido".into() };
    greet(&fido);

    let captain_floof = Cat;
    greet(&captain_floof);
}
```

<details>

* Later sections will get into more detail on generic functions like `greet`.
  For now, students only need to know that `greet` will operate on a reference
  to anything that implements `Pet`.

</details>
