# Import a Javascript method

Methods from javascript can be imported directly as `extern "C"` bindings.

```rust
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    pub fn jsMethod();
}

#[wasm_bindgen]
pub fn add_and_log(a: i32, b: i32) {
    let result = a + b;
    // Doesn't show in the console
    println!("println! {}", result);
    log(&format!("log {}", result));
    alert(&format!("alert {}", result));
    jsMethod("Hi from Rust");
}
```

```javascript
import init, { add_and_log } from "/wasm/project.js";

export function jsMethod(value) {
  alert(`Hello from JS! Value: ${value}`);
}

window.jsMethod = jsMethod;

(async () => {
  await init();
  add_and_log(1, 2);
})();
```

<details>

Since Wasm runs in the browser, we will want to interact directly with Javascript APIs from Rust.
For instance `println!` will not log to the javascript console, so we need to use `console.log`.
Similarly, we want to be able to call `alert`. This works the same way as FFIs with C.

</details>