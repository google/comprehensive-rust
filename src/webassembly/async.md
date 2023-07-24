# Async

Rust methods in WebAssembly can be declared async. Once called, they will be scheduled on the browser's event loop.
An event handler can for instance be implemented with a tokio channel.

Instead of `tokio::spawn`, `wasm_bindgen` provides `wasm_bindgen_futures::spawn_local`.

Let's create a class that waits for messages on a channel to rotate an HTML element:

```rust
#[derive(Debug)]
enum RotateSide {
    Left,
    Right,
}

#[wasm_bindgen]
pub struct Rotator {
    sender: Sender<RotateSide>,
}

#[wasm_bindgen]
impl Rotator {
    #[wasm_bindgen(constructor)]
    pub fn new(element: web_sys::HtmlElement) -> Rotator {
        let (sender, mut receiver) = channel::<RotateSide>(1);
        spawn_local(async move {
            let mut rotation = 0;
            while let Some(rotate_side) = receiver.recv().await {
                match rotate_side {
                    RotateSide::Left => rotation -= 45,
                    RotateSide::Right => rotation += 45,
                }
                element.set_inner_html(&rotation.to_string());
                let style = element.style();
                style
                    .set_property("transform", &format!("rotate({rotation}deg)"))
                    .expect("Failed to rotate");
            }
        });
        Rotator { sender }
    }

    #[wasm_bindgen]
    pub async fn rotate(&self, msg: String) -> Result<(), JsValue> {
        let rotate_side = match msg.as_str() {
            "ArrowLeft" => RotateSide::Left,
            "ArrowRight" => RotateSide::Right,
            _ => return Ok(()),
        };
        self.sender
            .send(rotate_side)
            .await
            .map_err(|e| JsValue::from_str(&format!("Receiver dropped {:?}", e)))
    }
}
```

Let's call it from Javascript

```javascript
import init, {Rotator} from '/wasm/project.js';

(async () => { 
    // Run the init method to initiate the WebAssembly module.
    await init();
    const wasmoutput = document.querySelector('#wasmoutput');
    const rotator = new Rotator(wasmoutput);
    document.body.addEventListener('keydown', async (e) => {
        await rotator.rotate(e.key);
    });
})();

```
