# Camera exercise solution

1. The first exercise is to turn the top half the image to black.
```rust
#[wasm_bindgen]
pub fn edit_bitmap(image_data: &Uint8ClampedArray, width: u32, height: u32) {
    for i in 0..image_data.length() / 2 {
        image_data.set_index(i, 0);
    }
}

```

2. The second exercise is to turn the left half of the image to white.
```rust
#[wasm_bindgen]
pub fn edit_bitmap(image_data: &Uint8ClampedArray, width: u32, height: u32) {
    for i in 0..height {
        for j in 0..(width * 4) / 2 {
            image_data.set_index(i * 4 * width + j, 255);
        }
    }
}
```

3. Create an X-ray effect. This is done by mapping colors to their opposite, for instance 0<->255, 100<->155. Beware of illumination.
```rust
#[wasm_bindgen]
pub fn edit_bitmap(image_data: &Uint8ClampedArray, width: u32, height: u32) {
    for i in 0..image_data.length() {
        if i % 4 == 3 {
            continue;
        }
        let pixel = image_data.get_index(i);
        image_data.set_index(i, 255 - pixel);
    }
}
```

4. You can play with the function, for instance you can implement greyscale, or add random noise.
5. In the `setup` function create a dropdown (`<select>`) that will change which transformation to apply to the image.
```rust
static METHODS: [&str; 3] = ["xray", "top-black", "left-white"];

#[wasm_bindgen]
pub fn setup() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let select = document.create_element("select")?;
    for option_value in METHODS {
        let option = document.create_element("option")?;
        option.set_inner_html(option_value);
        option.set_attribute("value", option_value)?;
        select.append_child(&option)?;
    }
    body.append_child(&select)?;
    Ok(())
}

#[wasm_bindgen]
pub fn edit_bitmap(
    image_data: &Uint8ClampedArray,
    width: u32,
    height: u32,
) -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let select = document
        .query_selector("select")?
        .expect("No select")
        .dyn_into::<web_sys::HtmlSelectElement>()
        .expect("Failed to cast <select>");
    let selected_option = select.value();
    match selected_option.as_str() {
        "xray" => xray(image_data, width, height),
        "top-black" => top_black(image_data, width, height),
        "left-white" => left_white(image_data, width, height),
        _ => (),
    };
    Ok(())
}
```

6. Track moving objects. This can be done by figuring out only the pixels that didn't change between multiple frames. For instance, you could compute the standard deviation of the pixel and black out below a threshold.
While this can be achieved without touching Javascript, I recommend editing it.
```rust
extern crate console_error_panic_hook;

use js_sys::Uint8ClampedArray;
use std::collections::VecDeque;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use wasm_bindgen::{prelude::*, Clamped};
use wasm_bindgen_futures::spawn_local;
use web_sys::ImageData;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub struct FrameSender {
    sdr: Sender<Uint8ClampedArray>,
}

#[wasm_bindgen]
impl FrameSender {
    pub fn send(&self, data: Uint8ClampedArray) -> Result<(), JsValue> {
        self.sdr.try_send(data).map_err(|e| e.to_string().into())
    }
}

async fn frame_processor(
    mut image_data: Receiver<Uint8ClampedArray>,
    canvas_ctx: web_sys::CanvasRenderingContext2d,
) {
    let mut deque: VecDeque<Vec<u8>> = VecDeque::new();
    let mut current_data;
    while let Some(image_data) = image_data.recv().await {
        deque.push_back(image_data.to_vec());
        if deque.len() < 10 {
            continue;
        }
        let first = deque.pop_front().unwrap();
        for (idx, v) in first.into_iter().enumerate() {
            let pixels: Vec<f32> = deque
                .iter()
                .map(|image| image[idx] as f32)
                .chain(Some(v as f32))
                .collect();
            let mean = pixels.iter().fold(0.0, |a, b| a + *b) / pixels.len() as f32;
            let variance = pixels.iter().fold(0.0, |a, b| a + (b - mean).powi(2))
                / pixels.len() as f32;
            if variance.sqrt() < 8.0 && idx % 4 != 3 {
                image_data.set_index(idx as u32, 0);
            }
        }
        current_data = image_data.to_vec();
        canvas_ctx
            .put_image_data(
                &ImageData::new_with_u8_clamped_array(Clamped(&current_data), 400)
                    .unwrap(),
                0.0,
                0.0,
            )
            .unwrap();
    }
}

#[wasm_bindgen]
pub fn setup(
    canvas_ctx: web_sys::CanvasRenderingContext2d,
) -> Result<FrameSender, JsValue> {
    console_error_panic_hook::set_once();

    let (sdr, rcv) = channel(2);
    let sender = FrameSender { sdr };
    spawn_local(async move {
        frame_processor(rcv, canvas_ctx).await;
    });
    Ok(sender)
}

```