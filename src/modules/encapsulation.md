---
minutes: 5
---

# Visibility and Encapsulation

Like with items in a module, struct fields are also private by default. Private
fields are likewise visible within the rest of the module (including child
modules). This allows us to encapsulate implementation details of struct,
controlling what data and functionality is visible externally.

```rust,editable
use outer::Foo;

mod outer {
    pub struct Foo {
        pub val: i32,
        is_big: bool,
    }

    impl Foo {
        pub fn new(val: i32) -> Self {
            Self { val, is_big: val > 100 }
        }
    }

    pub mod inner {
        use super::Foo;

        pub fn print_foo(foo: &Foo) {
            println!("Is {} big? {}", foo.val, foo.is_big);
        }
    }
}

fn main() {
    let foo = Foo::new(42);
    println!("foo.val = {}", foo.val);
    // let foo = Foo { val: 42, is_big: true };

    outer::inner::print_foo(&foo);
    // println!("Is {} big? {}", foo.val, foo.is_big);
}
```

<details>

- This slide demonstrates how privacy in structs is module-based. Students
  coming from object oriented languages may be used to types being the
  encapsulation boundary, so this demonstrates how Rust behaves differently
  while showing how we can still achieve encapsulation.

- Note how the `is_big` field is fully controlled by `Foo`, allowing `Foo` to
  control how it's initialized and enforce any invariants it needs to (e.g. that
  `is_big` is only `true` if `val > 100`).

- Point out how helper functions can be defined in the same module (including
  child modules) in order to get access to the type's private fields/methods.

- The first commented out line demonstrates that you cannot initialize a struct
  with private fields. The second one demonstrates that you also can't directly
  access private fields.

- Enums do not support privacy: Variants and data within those variants is
  always public.

## More to Explore

- If students want more information about privacy (or lack thereof) in enums,
  you can bring up `#[doc_hidden]` and `#[non_exhaustive]` and show how they're
  used to limit what can be done with an enum.

- Module privacy still applies when there are `impl` blocks in other modules
  [(example in the playground)][1].

</details>

[1]: https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=3e61f43c88de12bcdf69c1d6df9ab3da
