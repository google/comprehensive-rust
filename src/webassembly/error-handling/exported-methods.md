# Error handling

`Result<T, E>` is translated to javascript errors. Methods directly exposed and called from Javascript that return a `Result`
will automatically throw an error if the variant of the return value is `Result::Err`. `E` has to implement `Into<JsValue>`.

```rust
#[wasm_bindgen]
pub fn str_to_int(s: &str) -> Result<i32, JsValue> {
    s.parse::<i32>()
        .map_err(|_| JsValue::from_str("Failed to parse string"))
}
```

`Option` is converted to `undefined` in case of `None`. 

```rust
#[wasm_bindgen]
pub fn str_to_int(s: &str) -> Option<i32> {
    s.parse::<i32>().ok()
}

```

Javascript, click on the wasm output box to parse the string:

```javascript
import init, {str_to_int} from '/wasm/project.js';


(async () => { 
    // Run the init method to initiate the WebAssembly module.
    await init();
    const wasmoutput = document.querySelector('#wasmoutput');
    const input = document.createElement('input');
    input.type = 'text';
    document.body.appendChild(input);
    wasmoutput.onclick = () => {
        try {
            wasmoutput.innerHTML = str_to_int(input.value);
        } catch (e) {
            wasmoutput.innerHTML = e;
        }
    };
})();
```


<details>

* Click on the wasm output box to see the output
* `?` and other error handling tools are also supported

</details>
