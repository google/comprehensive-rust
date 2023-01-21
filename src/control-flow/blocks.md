# Blocks

A block in Rust has a value and a type: the value is the last expression of the
block:

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

The same rule is used for functions: the value of the function body is the
return value:

```rust,editable
fn double(x: i32) -> i32 {
    x + x
}

fn main() {
    println!("doubled: {}", double(7));
}
```

However if the last expression ends with `;`, then the resulting value and type is `()`.
