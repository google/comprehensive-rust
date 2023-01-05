# Modules

We have seen how `impl` blocks let us namespace functions to a type.

`mod` 역시 네임스페이스 타입과 함수를 제공합니다: 
> We have seen how `impl` blocks let us namespace functions to a type.
> Similarly, `mod` lets us namespace types and functions:

```rust,editable
mod foo {
    pub fn do_something() {
        println!("In the foo module");
    }
}

mod bar {
    pub fn do_something() {
        println!("In the bar module");
    }
}

fn main() {
    foo::do_something();
    bar::do_something();
}
```
