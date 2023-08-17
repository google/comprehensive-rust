# Borrow Checker

When we export a Rust type to Javascript we need to beware about the borrow checker on the Javascript side.

```rust
#[wasm_bindgen]
pub struct RustType {}
#[wasm_bindgen]
impl RustType {
    #[wasm_bindgen]
    pub fn new() -> RustType {
        RustType {}
    }
}
#[wasm_bindgen]
pub fn takes_struct(s: RustType) -> RustType {
    s
}
```

```javascript
import init, {RustType, takes_struct} from '/wasm/project.js';

(async () => { 
    // Run the init method to initiate the WebAssembly module.
    await init();
    const rustType = RustType.new();
    const moved = takes_struct(rustType);
    console.log(moved);
    console.log(rustType);
})();
```

<details>

* `rustType` is moved so it points to null in the second log.
* Ownership rules must be respected even in Javascript.
* Integral types in JS that are automatically translated to Rust do not have those constraints. `(String, Vec, etc.)`

</details>