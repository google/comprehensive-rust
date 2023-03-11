# Traits

Rust lets you abstract over types with traits. They're similar to interfaces:

```rust,editable
trait Greet {
    fn say_hello(&self);
}

struct Dog {
    name: String,
}

struct Cat;  // No name, cats won't respond to it anyway.

impl Greet for Dog {
    fn say_hello(&self) {
        println!("Wuf, my name is {}!", self.name);
    }
}

impl Greet for Cat {
    fn say_hello(&self) {
        println!("Miau!");
    }
}

fn main() {
    let pets: Vec<Box<dyn Greet>> = vec![
        Box::new(Dog { name: String::from("Fido") }),
        Box::new(Cat),
    ];
    for pet in pets {
        pet.say_hello();
    }
}
```

<details>

* Traits may specify pre-implemented (default) methods and methods that users are required to implement themselves. Methods with default implementations can rely on required methods.
* Types that implement a given trait may be of different sizes. This makes it impossible to have things like `Vec<Greet>` in the example above.
* `dyn Greet` is a way to tell the compiler about a dynamically sized type that implements `Greet`.
* In the example, `pets` holds Fat Pointers to objects that implement `Greet`. The Fat Pointer consists of two components, a pointer to the actual object and a pointer to the virtual method table for the `Greet` implementation of that particular object.
* Compare these outputs in the above example:
     ```rust,ignore
         println!("{} {}", std::mem::size_of::<Dog>(), std::mem::size_of::<Cat>());
         println!("{} {}", std::mem::size_of::<&Dog>(), std::mem::size_of::<&Cat>());
         println!("{}", std::mem::size_of::<&dyn Greet>());
         println!("{}", std::mem::size_of::<Box<dyn Greet>>());
     ```

</details>
