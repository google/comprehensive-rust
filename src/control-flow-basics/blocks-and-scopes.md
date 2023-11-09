---
minutes: 10
existing course material:
- basic-syntax/scopes-shadowing.md
- control-flow/blocks.md
---

<!-- NOTES:
Mutable and immutable variables, scopes, shadowing, block values, expression values (e.g., value of an if expression)
-->
# Blocks and Scopes

# Scopes and Shadowing

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

* Definition: Shadowing is different from mutation, because after shadowing both variable's memory locations exist at the same time. Both are available under the same name, depending where you use it in the code.
* A shadowing variable can have a different type.
* Shadowing looks obscure at first, but is convenient for holding on to values after `.unwrap()`.
* The following code demonstrates why the compiler can't simply reuse memory locations when shadowing an immutable variable in a scope, even if the type does not change.

<!-- mdbook-xgettext: skip -->
```rust,editable
fn main() {
    let a = 1;
    let b = &a;
    let a = a + 1;
    println!("{a} {b}");
}
```

</details>
# Blocks

A block in Rust contains a sequence of expressions.
Each block has a value and a type,
which are those of the last expression of the block:

<!-- mdbook-xgettext: skip -->
```rust,editable
fn main() {
    let x = {
        let y = 10;
        println!("y: {y}");
        let z = {
            let w = {
                3 + 4
            };
            println!("w: {w}");
            y * w
        };
        println!("z: {z}");
        z - y
    };
    println!("x: {x}");
}
```

If the last expression ends with `;`, then the resulting value and type is `()`.

The same rule is used for functions: the value of the function body is the
return value:

<!-- mdbook-xgettext: skip -->
```rust,editable
fn double(x: i32) -> i32 {
    x + x
}

fn main() {
    println!("double: {}", double(7));
}
```

<details>

Key Points:
* The point of this slide is to show that blocks have a type and value in Rust.
* You can show how the value of the block changes by changing the last line in the block. For instance, adding/removing a semicolon or using a `return`.

</details>
