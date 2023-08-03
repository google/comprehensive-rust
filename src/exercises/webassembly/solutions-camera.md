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