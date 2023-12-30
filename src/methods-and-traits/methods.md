---
minutes: 10
---

# Methods

Rust allows you to associate functions with your new types. You do this with an
`impl` block:

```rust,editable
#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    // No receiver, a static method
    fn new(name: &str) -> Self {
        Self { name: String::from(name), laps: Vec::new() }
    }

    // Exclusive borrowed read-write access to self
    fn add_lap(&mut self, lap: i32) {
        self.laps.push(lap);
    }

    // Shared and read-only borrowed access to self
    fn print_laps(&self) {
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    // Exclusive ownership of self
    fn finish(self) {
        let total: i32 = self.laps.iter().sum();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

fn main() {
    let mut race = Race::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();
    // race.add_lap(42);
}
```

The `self` arguments specify the "receiver" - the object the method acts on.
There are several common receivers for a method:

- `&self`: borrows the object from the caller using a shared and immutable
  reference. The object can be used again afterwards.
- `&mut self`: borrows the object from the caller using a unique and mutable
  reference. The object can be used again afterwards.
- `self`: takes ownership of the object and moves it away from the caller. The
  method becomes the owner of the object. The object will be dropped
  (deallocated) when the method returns, unless its ownership is explicitly
  transmitted. Complete ownership does not automatically mean mutability.
- `mut self`: same as above, but the method can mutate the object.
- No receiver: this becomes a static method on the struct. Typically used to
  create constructors which are called `new` by convention.

<details>

Key Points:

- It can be helpful to introduce methods by comparing them to functions.
  - Methods are called on an instance of a type (such as a struct or enum), the
    first parameter represents the instance as `self`.
  - Developers may choose to use methods to take advantage of method receiver
    syntax and to help keep them more organized. By using methods we can keep
    all the implementation code in one predictable place.
- Point out the use of the keyword `self`, a method receiver.
  - Show that it is an abbreviated term for `self: Self` and perhaps show how
    the struct name could also be used.
  - Explain that `Self` is a type alias for the type the `impl` block is in and
    can be used elsewhere in the block.
  - Note how `self` is used like other structs and dot notation can be used to
    refer to individual fields.
  - This might be a good time to demonstrate how the `&self` differs from `self`
    by trying to run `finish` twice.
  - Beyond variants on `self`, there are also
    [special wrapper types](https://doc.rust-lang.org/reference/special-types-and-traits.html)
    allowed to be receiver types, such as `Box<Self>`.

</details>
