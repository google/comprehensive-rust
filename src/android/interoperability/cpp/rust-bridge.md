# Rust Bridge Declarations

```rust,ignore
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        type MyType; // Opaque type
        fn foo(&self); // Method on `MyType`
        fn bar() -> Box<MyType>; // Free function
    }
}

struct MyType(i32);

impl MyType {
    fn foo(&self) {
        println!("{}", self.0);
    }
}

fn bar() -> Box<MyType> {
    Box::new(MyType(123))
}
```

<details>

* Items declared in the `extern "Rust"` reference items that are in scope in the
  parent module.
* The CXX code generator uses your `extern "Rust"` section(s) to produce a C++
  header file containing the corresponding C++ declarations. The generated
  header has the same path as the Rust source file containing the bridge, except
  with a .rs.h file extension.

</details>
