---
minutes: 20
---

# Exercise: Modules for the GUI Library

In this exercise, you will reorganize the GUI Library exercise from the "Methods
and Traits" segment of the course into a collection of modules. It is typical to
put each type or set of closely-related types into its own module, so each
widget type should get its own module.

If you no longer have your version, that's fine - refer back to the
[provided solution](../methods-and-traits/solution.html).

## Cargo Setup

The Rust playground only supports one file, so you will need to make a Cargo
project on your local filesystem:

```shell
cargo init gui-modules
cd gui-modules
cargo run
```

Edit `src/main.rs` to add `mod` statements, and add additional files in the
`src` directory.

<details>

Encourage students to divide the code in a way that feels natural for them, and
get accustomed to the required `mod`, `use`, and `pub` declarations. Afterward,
discuss what organizations are most idiomatic.

</details>
