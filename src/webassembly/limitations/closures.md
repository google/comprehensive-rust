# Closures

Closures can be returned to Rust and executed on the WASM runtime.

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
}

#[wasm_bindgen]
pub struct ClosureHandle {
    closure: Closure<dyn FnMut()>,
}

#[wasm_bindgen]
pub fn timeout_set_seconds(elem: web_sys::HtmlElement) -> ClosureHandle {
    let seconds = Rc::new(RefCell::new(0usize));
    elem.set_inner_html("0 seconds");
    let closure = Closure::wrap(Box::new(move || {
        let mut seconds = seconds.borrow_mut();
        *seconds += 1;
        elem.set_inner_html(&format!("{} seconds", seconds));
    }) as Box<dyn FnMut()>);
    setInterval(&closure, 1000);
    ClosureHandle { closure }
}
```

```javascript
import init, {set_panic_hook, timeout_set_seconds} from '/wasm/project.js';

(async () => { 
    // Run the init method to initiate the WebAssembly module.
    await init();
    const wasmOutput = document.querySelector("#wasmoutput");
    timeout_set_seconds(wasmOutput);
})();

```

<details>

* Since the function that creates the closure keeps its ownership, the closure would be dropped if we did't return it.
    * Returning ownership allows the JS runtime to manage the lifetime of the closure and to collect it when it can.
    * Try returning nothing from the method.
* Closures can only be passed by reference to WASM functions.
    * This is why we pass `&Closure` to `setInterval`.
    * This is also why we need to create `ClosureHandle` to return the closure.

</details>