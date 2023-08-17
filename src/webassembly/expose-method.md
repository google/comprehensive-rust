# Expose a method

The first thing you will want to do with WebAssembly is expose your methods to Javascript.
This is straightforward using the `#[wasm_bindgen]` procedural macro.

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn set_panic_hook() {
    // Generates better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    console_error_panic_hook::set_once();
}

// Exposes the `add` method from the previous slide
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

<details>
 
* `set_panic_hook` is a convenient setup method that adds debug information to stack traces when a Wasm module panics. Don't use it in prod builds because it tends to bloat the bundle size.

</details>
