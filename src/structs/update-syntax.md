# Struct Update Syntax

Sometimes it is useful to create a new instance of a struct based on an old
instance with a few changes. Rust has a facility for this called the "Struct Update Syntax".

This pattern is often used when it comes to configuring some other struct since it enables us
to provide reasonable defaults and allows the user of our struct to change what they need in
an easy to read manner. Consider the following example for a graphical program:

```rust,editable
#[derive(Debug)]
struct DisplayConfiguration {
    width: Option<u32>,
    height: Option<u32>,
    vsync: bool,
}

impl DisplayConfiguration {
    pub fn new() -> Self {
        DisplayConfiguration {
            width: None,
            height: None,
            vsync: false
        }
    }

    pub fn with_resolution(self, width: u32, height: u32) -> Self {
        DisplayConfiguration {
            width: Some(width),
            height: Some(height),
            ..self
        }
    }

    pub fn with_vsync(self, vsync: bool) -> Self {
        DisplayConfiguration {
            vsync,
            ..self
        }
    }
}

fn main() {
    // Override resolution, but leave other fields as defaults:
    let config = DisplayConfiguration::new()
        .with_resolution(1920, 1080);
    // .. we could keep chaining such `with_x()` methods here.

    println!("Configuration is: {config:#?}");
}
```

In the above scanerio, each of the `with_*()` functions takes `self`, which means that it _moves_ 
the previous instance into the function (takes ownership of it) and creates a new instance, by copying
or moving the fields which were not specified from the old struct. The struct specified after the `..`
syntax must have the same type as the struct being constructed.

In many popular libraries, this process of configuration is implemented on an intermediate struct, which has
an ultimate `build()` function which returns the actual instance we go on to use.

Note that we're also using the field shorthand syntax in the function `with_vsync()`.

<details>

* Note that the update syntax isn't limited to `self`, as we've seen in one of the previous slides. The only rule
  is that the old struct must be the same type as the struct being constructed.

* The update expression must be the last expression in the struct initialization block.

</details>
