# Dereferencing Raw Pointers

Creating pointers is safe, but dereferencing them requires `unsafe`:

```rust,editable
fn main() {
    let mut num = 5;

    let r1 = &mut num as *mut i32;
    let r2 = &num as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        *r1 = 10;  // Data race if r1 is being written concurrently!
        println!("r2 is: {}", *r2);
    }
}
```
