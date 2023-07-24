# Import user-defined Javascript types

User-defined Javascript types can be imported by declaring the relevant methods as `extern "C"` just like 
other foreign functions.

For instance, let's declare a class `OutputBox`

```javascript
import init, {set_panic_hook, edit_box} from '/wasm/project.js';

class OutputBox {
    constructor(element) {
        this.element = element;
        this.lastText = null;
    }

    setText(text) {
        this.element.innerHTML = text;
    }

    get currentText() {
        return this.element.innerHTML;
    }
}

window.OutputBox = OutputBox;

(async () => { 
    // Run the init method to initiate the WebAssembly module.
    await init();
    set_panic_hook();
    const wasmoutput = document.querySelector('#wasmoutput');
    const outputBox = new OutputBox(wasmoutput);
    const input = document.createElement('input');
    document.body.appendChild(input);
    wasmoutput.onclick = () => {
        const inputValue = input.value;
        edit_box(outputBox, inputValue);
    };
})();
```

It can be imported as such in Rust

```rust
#[wasm_bindgen]
extern "C" {
    pub type OutputBox;

    #[wasm_bindgen(constructor)]
    pub fn new(text: i32) -> OutputBox;

    #[wasm_bindgen(method)]
    pub fn setText(this: &OutputBox, text: &str);

    // Has to return owned
    #[wasm_bindgen(method, getter)]
    pub fn lastText(this: &OutputBox) -> Option<String>;

    #[wasm_bindgen(method, setter)]
    pub fn set_lastText(this: &OutputBox, text: Option<String>);

    #[wasm_bindgen(method, getter)]
    pub fn currentText(this: &OutputBox) -> String;
}

#[wasm_bindgen]
pub fn edit_box(output_box: &OutputBox, text: &str) {
    match text {
        "reset" => output_box.set_lastText(None),
        "recover" => {
            if let Some(last_text) = output_box.lastText() {
                output_box.setText(&last_text);
            } else {
                output_box.setText("No last text");
            }
        }
        "save" => output_box.set_lastText(Some(output_box.currentText())),
        _ => output_box.setText(text),
    }
}
```

<details>

* Getters and Setters have to be declared with an added parameter in the proc macro.
* `null` and `undefined` can be both represented by `Option::None`

</details>