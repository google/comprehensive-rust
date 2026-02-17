---
minutes: 10
---

<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Named Structs

Like C and C++, Rust has support for custom structs:

<!-- dprint-ignore-start -->

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person) {
    println!("{} is {} years old", person.name, person.age);
}

fn main() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    describe(&peter);

    peter.age = 28;
    describe(&peter);

    let name = String::from("Avery");
    let age = 39;
    let avery = Person { name, age };
    describe(&avery);
}
```

<!-- dprint-ignore-end -->

<details>

Key Points:

- Structs work like in C or C++.
  - Like in C++, and unlike in C, no typedef is needed to define a type.
  - Unlike in C++, there is no inheritance between structs.
- This may be a good time to let people know there are different types of
  structs.
  - Zero-sized structs (e.g. `struct Foo;`) might be used when implementing a
    trait on some type but don’t have any data that you want to store in the
    value itself.
  - The next slide will introduce Tuple structs, used when the field names are
    not important.
- If you already have variables with the right names, then you can create the
  struct using a shorthand.
- Struct fields do not support default values. Default values are specified by
  implementing the `Default` trait which we will cover later.

## More to Explore

- You can also demonstrate the struct update syntax here:

  ```rust,ignore
  # // Copyright 2023 Google LLC
  # // SPDX-License-Identifier: Apache-2.0
  #
  let jackie = Person { name: String::from("Jackie"), ..avery };
  ```

- It allows us to copy the majority of the fields from the old struct without
  having to explicitly type it all out. It must always be the last element.

- It is mainly used in combination with the `Default` trait. We will talk about
  struct update syntax in more detail on the slide on the `Default` trait, so we
  don't need to talk about it here unless students ask about it.

</details>
