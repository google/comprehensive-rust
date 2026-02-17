---
minutes: 10
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Monomorphization and Binary Size

```rust
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
fn print_vec<T: std::fmt::Debug>(debug_vec: &Vec<T>) {
    for item in debug_vec {
        println!("{:?}", item);
    }
}

fn main() {
    let ints = vec![1u32, 2, 3];
    let floats = vec![1.1f32, 2.2, 3.3];

    // instance one, &Vec<u32> -> ()
    print_vec(&ints);
    // instance two, &Vec<f32> -> ()
    print_vec(&floats);
}
```

<details>

- Each instance of a function or type with generics gets transformed into a
  unique, concrete version of that function at compile time. Generics do not
  exist at runtime, only specific types.

- This comes with a strong baseline performance and capacity for optimization,
  but at a cost of binary size and compile time.

- There are plenty of ways to trim binary size and compilation times, but we're
  not covering them here.

- Pay for what you use: Binary size increase of monomorphization is only
  incurred for instances of a type or functions on a type used in the final
  program or dynamic library.

- When to care: Monomorphization impacts compile times and binary size. In
  circumstances like WebAssembly in-browser or embedded systems development, you
  may want to be mindful about designing with generics in mind.

</details>
