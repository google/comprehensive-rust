# Supertraits

A trait can require that types implementing it also implement other traits,
called _supertraits_. Here, any type implementing `Pet` must implement `Animal`.

```rust,editable
trait Animal {
    fn leg_count(&self) -> u32;
}

trait Pet: Animal {
    fn name(&self) -> String;
}

struct Dog(String);

impl Animal for Dog {
    fn leg_count(&self) -> u32 {
        4
    }
}

impl Pet for Dog {
    fn name(&self) -> String {
        self.0.clone()
    }
}

fn main() {
    let puppy = Dog(String::from("Rex"));
    println!("{} has {} legs", puppy.name(), puppy.leg_count());
}
```

<details>

This is sometimes called "trait inheritance" but students should not expect this
to behave like OO inheritance. It just specifies an additional requirement on
implementations of a trait.

</details>
