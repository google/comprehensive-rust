# Async

Rust methods in WebAssembly can be declared async. 

```rust
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::Response;

#[wasm_bindgen]
pub async fn get_current_page() -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_str("")).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let text = JsFuture::from(resp.text()?).await?;
    Ok(text)
}
```

```javascript
import init, { get_current_page} from '/wasm/project.js';

(async () => { 
    await init();
    console.log(await get_current_page());
})();

```

<details>

- Async methods are scheduled on the Javascript event loop.
- Instead of `tokio::spawn`, `wasm_bindgen` provides `wasm_bindgen_futures::spawn_local`.
- We use `JsFuture::from` to convert Javascript futures to Rust futures that we can `.await`.

</details>