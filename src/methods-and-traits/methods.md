---
minutes: 10
existing course material:
- methods.md
- methods/receiver.md
- methods/example.md
---

<!-- NOTES:
Methods, associated functions, constructors
-->
# Methods

# Methods

Rust allows you to associate functions with your new types. You do this with an
`impl` block:

```rust,editable
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn say_hello(&self) {
        println!("Hello, my name is {}", self.name);
    }
}

fn main() {
    let peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    peter.say_hello();
}
```

<details>

Key Points:
* It can be helpful to introduce methods by comparing them to functions.
  * Methods are called on an instance of a type (such as a struct or enum), the first parameter represents the instance as `self`.
  * Developers may choose to use methods to take advantage of method receiver syntax and to help keep them more organized. By using methods we can keep all the implementation code in one predictable place.
* Point out the use of the keyword `self`, a method receiver.
  * Show that it is an abbreviated term for `self: Self` and perhaps show how the struct name could also be used.
  * Explain that `Self` is a type alias for the type the `impl` block is in and can be used elsewhere in the block.
  * Note how `self` is used like other structs and dot notation can be used to refer to individual fields.
  * This might be a good time to demonstrate how the `&self` differs from `self` by modifying the code and trying to run say_hello twice.
* We describe the distinction between method receivers next.

</details>
# Method Receiver

The `&self` above indicates that the method borrows the object immutably. There
are other possible receivers for a method:

* `&self`: borrows the object from the caller using a shared and immutable
  reference. The object can be used again afterwards.
* `&mut self`: borrows the object from the caller using a unique and mutable
  reference. The object can be used again afterwards.
* `self`: takes ownership of the object and moves it away from the caller. The
  method becomes the owner of the object. The object will be dropped (deallocated)
  when the method returns, unless its ownership is explicitly
  transmitted. Complete ownership does not automatically mean mutability.
* `mut self`: same as above, but the method can mutate the object.
* No receiver: this becomes a static method on the struct. Typically used to
  create constructors which are called `new` by convention.

Beyond variants on `self`, there are also
[special wrapper types](https://doc.rust-lang.org/reference/special-types-and-traits.html)
allowed to be receiver types, such as `Box<Self>`.

<details>

Consider emphasizing "shared and immutable" and "unique and mutable". These constraints always come
together in Rust due to borrow checker rules, and `self` is no exception. It isn't possible to
reference a struct from multiple locations and call a mutating (`&mut self`) method on it.

</details>
# Example

```rust,editable
#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    fn new(name: &str) -> Race {  // No receiver, a static method
        Race { name: String::from(name), laps: Vec::new() }
    }

    fn add_lap(&mut self, lap: i32) {  // Exclusive borrowed read-write access to self
        self.laps.push(lap);
    }

    fn print_laps(&self) {  // Shared and read-only borrowed access to self
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    fn finish(self) {  // Exclusive ownership of self
        let total = self.laps.iter().sum::<i32>();
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

<details>

Key Points:
* All four methods here use a different method receiver.
  * You can point out how that changes what the function can do with the variable values and if/how it can be used again in `main`.
  * You can showcase the error that appears when trying to call `finish` twice.
* Note that although the method receivers are different, the non-static functions are called the same way in the main body. Rust enables automatic referencing and dereferencing when calling methods. Rust automatically adds in the `&`, `*`, `muts` so that that object matches the method signature.
* You might point out that `print_laps` is using a vector that is iterated over. We describe vectors in more detail in the afternoon.

</details>
