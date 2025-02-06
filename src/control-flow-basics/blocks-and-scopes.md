---
minutes: 5
---

# Blocks and Scopes

A block in Rust contains a sequence of expressions, enclosed by braces `{}`.
Each block has a value and a type, which are those of the last expression of the
block:

```rust,editable
fn main() {
    let z = 13;
    let x = {
        let y = 10;
        println!("y: {y}");
        z - y
    };
    println!("x: {x}");
    // println!("y: {y}");
}
```

If the last expression ends with `;`, then the resulting value and type is `()`.

A variable's scope is limited to the enclosing block.

<details>

- You can show how the value of the block changes by changing the last line in
  the block. For instance, adding/removing a semicolon or using a `return`.

- Demonstrate that attempting to access `y` outside of its scope won't compile.

- Values are effectively "deallocated" when they go out of their scope, even if
  their data on the stack is still there.

</details>
