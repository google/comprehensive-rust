# Re-exposing Traits

Sometimes we use an enum to abstract over multiple types that implement the same
trait, and want the enum to also re-expose the trait's interface.

```rust,editable
trait Pet {
    fn talk(&self);
}

struct Dog {
    name: String,
}

struct Cat {
    age: u8,
}

impl Pet for Dog {
    fn talk(&self) {
        println!("Woof! My name is {}~!", self.name);
    }
}

impl Pet for Cat {
    fn talk(&self) {
        println!("Meow! I am {} years old", self.age);
    }
}

enum AnyPet {
    Dog(Dog),
    Cat(Cat),
}

impl Pet for AnyPet {
    fn talk(&self) {
        match self {
            Self::Dog(dog) => dog.talk(),
            Self::Cat(cat) => cat.talk(),
        }
    }
}

fn do_pet_stuff(pet: &impl Pet) {
    pet.talk();
}

fn main() {
    let cat = Cat {
        age: 19,
    };

    let dog = Dog {
        name: "Fido".into(),
    };

    do_pet_stuff(&cat);
    do_pet_stuff(&dog);
    do_pet_stuff(&AnyPet::Dog(dog));
    do_pet_stuff(&AnyPet::Cat(cat));
}
```

<details>

- One drawback of using an enum for dynamic polymorphism is that if our
  underlying types (`Cat` and `Dog` in this case) implement a trait (`Pet`), our
  wrapper enum doesn't automatically expose that same trait interface.

- We can generally implement the trait for the wrapper enum by matching on the
  enum and dispatching to the corresponding trait method on the underlying
  types.

- This is dynamic dispatch, but using the enum's discriminant instead of a
  vtable in order to lookup the correct function to call.

- This is an ergonomic drawback of an enum vs `dyn`: The implementation of `Pet`
  for `AnyPet` is pure boilerplate that we need to repeat each time we have a
  situation like this, whereas `dyn` gives us this behavior purely from the
  `Pet` impls on `Cat` and `Dog`.

- The advantage of this approach is that we retain the useful properties of an
  enum (e.g. the ability to pattern match on it) while also exposing a way to do
  dynamic dispatch through the trait's interface.

- This also enables us to use `AnyPet` with generic functions like
  `do_pet_stuff`, which we can also do with `dyn`.

</details>
