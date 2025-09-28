---
minutes: 5
---

# Readability: Consistency and Shorthands

Be consistent in function & variable names, and use shorthands with care.

```rust,editable
// Step 1
fn do_thing() { /* Imagine something! */}
// Step 2
fn execute_the_other_thing() {}
// Step 3
fn anthr_thng_whch_shld_b_dn() {}

fn main() {
    do_thing();
    execute_the_other_thing();
    anthr_thng_whch_shld_b_dn();
}
```

<details>

- Aim to be consistent in how things are named and abbreviated.

- Shorthands should be used with care, and consistent across a codebase when
  used.

- This example shows three functions that all do "something." Yet each one of
  them has a different naming scheme.

- Ask: Imagine what the domain should be for these three functions.

- Ask: How should they be renamed?

</details>
