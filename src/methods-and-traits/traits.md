---
minutes: 10
existing course material:
- traits.md
- traits/default-methods.md
---

<!-- NOTES:
Defining, implementing, and using traits, including provided methods
-->
# Traits

# Traits

Rust lets you abstract over types with traits. They're similar to interfaces:

```rust,editable
struct Dog { name: String, age: i8 }
struct Cat { lives: i8 } // No name needed, cats won't respond anyway.

trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Dog {
    fn talk(&self) -> String { format!("Woof, my name is {}!", self.name) }
}

impl Pet for Cat {
    fn talk(&self) -> String { String::from("Miau!") }
}

fn greet<P: Pet>(pet: &P) {
    println!("Oh you're a cutie! What's your name? {}", pet.talk());
}

fn main() {
    let captain_floof = Cat { lives: 9 };
    let fido = Dog { name: String::from("Fido"), age: 5 };

    greet(&captain_floof);
    greet(&fido);
}
```
# Default Methods

Traits can implement behavior in terms of other trait methods:

```rust,editable
trait Equals {
    fn equals(&self, other: &Self) -> bool;
    fn not_equals(&self, other: &Self) -> bool {
        !self.equals(other)
    }
}

#[derive(Debug)]
struct Centimeter(i16);

impl Equals for Centimeter {
    fn equals(&self, other: &Centimeter) -> bool {
        self.0 == other.0
    }
}

fn main() {
    let a = Centimeter(10);
    let b = Centimeter(20);
    println!("{a:?} equals {b:?}: {}", a.equals(&b));
    println!("{a:?} not_equals {b:?}: {}", a.not_equals(&b));
}
```

<details>

* Traits may specify pre-implemented (default) methods and methods that users are required to
  implement themselves. Methods with default implementations can rely on required methods.

* Move method `not_equals` to a new trait `NotEquals`.

* Make `Equals` a super trait for `NotEquals`.
    ```rust,editable,compile_fail
    trait NotEquals: Equals {
        fn not_equals(&self, other: &Self) -> bool {
            !self.equals(other)
        }
    }
    ```

* Provide a blanket implementation of `NotEquals` for `Equals`.
    ```rust,editable,compile_fail
    trait NotEquals {
        fn not_equals(&self, other: &Self) -> bool;
    }

    impl<T> NotEquals for T where T: Equals {
        fn not_equals(&self, other: &Self) -> bool {
            !self.equals(other)
        }
    }
    ```
  * With the blanket implementation, you no longer need `Equals` as a super trait for `NotEqual`.

</details>
