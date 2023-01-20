# The `Drop` Trait

Values which implement `Drop` can specify code to run when they go out of scope:

```rust,editable
struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

fn main() {
    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "b" };
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Exiting block A");
    }
    drop(a);
    println!("Exiting main");
}
```

<details>

* References in structs must have a lifetime annotation and the compiler will use it to verify assigments.
```rust,editable
struct Droppable<'a> {
    name: &'a String,
}

impl Drop for Droppable<'_> {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

fn main() {
    let a_str = String::from("a");
    let a = Droppable { name: &a_str};
    {
        let c_str = String::from("c");
        let mut c = Droppable { name: &c_str};
        let d_str = String::from("d");
        let mut d = Droppable { name: &d_str};
        d.name = &c_str;  // Allowed, c_str lives longer than d.
        //c.name = d.name;  // Fails! d_str gets deconstructed before c.
    }
    println!("Exiting main");
}
```

</details>
