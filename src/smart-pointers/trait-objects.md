---
minutes: 10
---

# Trait Objects

Trait objects allow for values of different types, for instance in a collection:

```rust,editable
struct Dog {
    name: String,
    age: i8,
}
struct Cat {
    lives: i8,
}

trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        String::from("Miau!")
    }
}

fn main() {
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Cat { lives: 9 }),
        Box::new(Dog { name: String::from("Fido"), age: 5 }),
    ];
    for pet in pets {
        println!("Hello, who are you? {}", pet.talk());
    }
}
```

Memory layout after allocating `pets`:

```bob
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - - - - - - - - - - - - -.
:                           :     :                                             :
:    pets                   :     :                     +----+----+----+----+   :
:   +-----------+-------+   :     :   +-----+-----+  .->| F  | i  | d  | o  |   :
:   | ptr       |   o---+---+-----+-->| o o | o o |  |  +----+----+----+----+   :
:   | len       |     2 |   :     :   +-|-|-+-|-|-+  `---------.                :
:   | capacity  |     2 |   :     :     | |   | |    data      |                :
:   +-----------+-------+   :     :     | |   | |   +-------+--|-------+        :
:                           :     :     | |   | '-->| name  |  o, 4, 4 |        :
:                           :     :     | |   |     | age   |        5 |        :
`- - - - - - - - - - - - - -'     :     | |   |     +-------+----------+        :
                                  :     | |   |                                 :
                                  :     | |   |      vtable                     :
                                  :     | |   |     +----------------------+    :
                                  :     | |   '---->| "<Dog as Pet>::talk" |    :
                                  :     | |         +----------------------+    :
                                  :     | |                                     :
                                  :     | |    data                             :
                                  :     | |   +-------+-------+                 :
                                  :     | '-->| lives |     9 |                 :
                                  :     |     +-------+-------+                 :
                                  :     |                                       :
                                  :     |      vtable                           :
                                  :     |     +----------------------+          :
                                  :     '---->| "<Cat as Pet>::talk" |          :
                                  :           +----------------------+          :
                                  :                                             :
                                  '- - - - - - - - - - - - - - - - - - - - - - -'
```

<details>

- Types that implement a given trait may be of different sizes. This makes it
  impossible to have things like `Vec<dyn Pet>` in the example above.
- `dyn Pet` is a way to tell the compiler about a dynamically sized type that
  implements `Pet`.
- In the example, `pets` is allocated on the stack and the vector data is on the
  heap. The two vector elements are _fat pointers_:
  - A fat pointer is a double-width pointer. It has two components: a pointer to
    the actual object and a pointer to the [virtual method table] (vtable) for
    the `Pet` implementation of that particular object.
  - The data for the `Dog` named Fido is the `name` and `age` fields. The `Cat`
    has a `lives` field.
- Compare these outputs in the above example:
  ```rust,ignore
  println!("{} {}", std::mem::size_of::<Dog>(), std::mem::size_of::<Cat>());
  println!("{} {}", std::mem::size_of::<&Dog>(), std::mem::size_of::<&Cat>());
  println!("{}", std::mem::size_of::<&dyn Pet>());
  println!("{}", std::mem::size_of::<Box<dyn Pet>>());
  ```

[virtual method table]: https://en.wikipedia.org/wiki/Virtual_method_table

</details>
