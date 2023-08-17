# Expose user-defined Rust types

Similarily to methods, types can be exposed from Rust to Javascript with the `#[wasm_bindgen]` macro.

```rust
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub struct Counter {
    name: String,
    pub count: u8,
}
#[wasm_bindgen]
impl Counter {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, count: u8) -> Counter {
        Counter { name, count }
    }
    pub fn increment(&mut self) {
        self.count += 1;
    }
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
```

Javascript to use the `Counter`.

```javascript
import init, { Counter } from "/wasm/project.js";

(async () => {
  // Run the init method to initiate the WebAssembly module.
  await init();
  const wasmOutput = document.querySelector("#wasmoutput");
  const button = document.createElement("button");
  button.textContent = "increment";
  document.body.appendChild(button);
  const counter = new Counter("ButtonCounter", 42);
  wasmOutput.textContent = counter.count;
  button.addEventListener("click", () => {
    // Calls the getter
    console.log("Button clicked!", counter.name);
    counter.increment();
    wasmOutput.textContent = counter.count;
  });
})();
```

<details>

- `pub` members must implement copy
- Type parameters and lifetime annotations are not supported yet
- Members that implement `Copy` can be public and directly accessed from Javascript.
</details>
