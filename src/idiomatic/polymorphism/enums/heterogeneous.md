# Heterogeneous Collections with Enums

Enums give us a way to create collections that can store different types of
element at runtime:

```rust,editable
struct Dog {
    name: String,
}

struct Cat {
    age: u8,
}

enum AnyPet {
    Dog(Dog),
    Cat(Cat),
}

fn main() {
    let pets = vec![
        AnyPet::Dog(Dog { name: "Fido".into() }),
        AnyPet::Cat(Cat { age: 19 }),
    ];
}
```

<details>

- A common situation where we might need dynamic polymorphism is when we want to
  store different types of value in the same collection.

- In the above example, we want to store both `Cat`s and `Dog`s in the same
  list. `Vec` doesn't support this directly: All elements of the `Vec` must be
  the same type.

- Wrapping our two different pet types into a single `AnyPet` enum gives us a
  unified type representation that can be stored in a `Vec`, while allowing
  individual elements of the `Vec` to be different types.

- This requires that we know all possible pet types up front, as we need to
  explicitly list them as different variants of the `AnyPet` enum. This works
  well for libraries or applications that define the full set of possible types,
  but does not allow downstream users to extend our list of pet types.

- Later we will see that we can do the same thing with `dyn`, which allows
  downstream extension at the cost of being harder to downcast and requiring
  dynamic dispatch.

</details>
