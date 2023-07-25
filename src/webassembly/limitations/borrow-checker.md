# Borrow Checker

When we export a Rust type to Javascript and the pass an instance of this type to a method that takes ownership of it, the javascript variable will be cleared and dereferencing it will throw a runtime error.
This essentially implements the borrow checker at Runtime in Javascript.

```rust
#[wasm_bindgen]
pub struct MultiCounter {
    // We use the counter from the previous slide
    counters: Vec<Counter>,
}

#[wasm_bindgen]
impl MultiCounter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> MultiCounter {
        MultiCounter { counters: Vec::new() }
    }

    pub fn increment(&mut self) {
        for counter in &mut self.counters {
            counter.increment();
        }
    }

    pub fn add_counter(&mut self, counter: Counter) {
        self.counter.push(counter);
    }
}
```

```javascript
import init, {set_panic_hook, Counter, MultiCounter} from '/wasm/project.js';

(async () => { 
    // Run the init method to initiate the WebAssembly module.
    await init();
    set_panic_hook();
    const wasmOutput = document.querySelector("#wasmoutput");
    const button = document.querySelector("#button");

    const counter = new Counter("ButtonCounter", 42);
    counter.increment();
    // Works fine
    wasmOutput.textContent = counter.count;

    const multiCounter = new MultiCounter();
    // Move counter into the MultiCounter
    multiCounter.add_counter(counter);
    // Error: Open console
    counter.increment();
})();
```

<details>

* `counter` is moved before the second call, so the code panics
* Ownership rules must be respected

</details>