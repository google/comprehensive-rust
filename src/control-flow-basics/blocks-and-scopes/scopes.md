# Scopes and Shadowing

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

- Show that a variable's scope is limited by adding a `b` in the inner block in
  the last example, and then trying to access it outside that block.
- Shadowing is different from mutation, because after shadowing both variables'
  memory locations exist at the same time. Both are available under the same
  name, depending where you use it in the code.
- A shadowing variable can have a different type.
- Shadowing looks obscure at first, but is convenient for holding on to values
  after `.unwrap()`.

</details>
