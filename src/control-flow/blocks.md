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
