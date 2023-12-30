---
minutes: 10
---

# Blocks and Scopes

## Blocks

A block in Rust contains a sequence of expressions. Each block has a value and a
type, which are those of the last expression of the block:

```rust,editable
fn main() {
    let z = 13;
    let x = {
        let y = 10;
        println!("y: {y}");
        z - y
    };
    println!("x: {x}");
}
```

If the last expression ends with `;`, then the resulting value and type is `()`.

## Scopes and Shadowing

A variable's scope is limited to the enclosing block.

You can shadow variables, both those from outer scopes and variables from the
same scope:

```rust,editable
fn main() {
    let a = 10;
    println!("before: {a}");
    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");
}
```

<details>

- You can show how the value of the block changes by changing the last line in
  the block. For instance, adding/removing a semicolon or using a `return`.
- Show that a variable's scope is limited by adding a `b` in the inner block in
  the last example, and then trying to access it outside that block.
- Shadowing is different from mutation, because after shadowing both variable's
  memory locations exist at the same time. Both are available under the same
  name, depending where you use it in the code.
- A shadowing variable can have a different type.
- Shadowing looks obscure at first, but is convenient for holding on to values
  after `.unwrap()`.

</details>
