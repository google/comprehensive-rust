# web-sys and js_sys

You don't need to import all Javascript methods manually. [web_sys](https://rustwasm.github.io/wasm-bindgen/web-sys/index.html) amd [js_sys](https://docs.rs/js-sys/latest/js_sys/) import Javascript and DOM methods for us.

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_a_cat() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let image = document.create_element("img")?;
    // We need a random number to prevent the browser from caching the image
    let random_number = js_sys::Math::random() * 10_000.0;
    image.set_attribute("src", &format!("https://picsum.photos/300?random={random_number}"))?;
    body.append_child(&image)?;

    Ok(())
}
```

```javascript
import init, { add_a_cat } from "/wasm/project.js";

(async () => {
  // Run the init method to initiate the WebAssembly module.
  await init();
  const button = document.createElement("button");
  button.textContent = "Add a cat";
  document.body.appendChild(button);
  button.addEventListener("click", () => {
    add_a_cat();
  });
})();
```
