# Camera

In this exercise we will play with the camera in real time.

Serve the web server and navigate to [http://localhost:8000/exercises/camera/](http://localhost:8000/exercises/camera/).

You will edit [lib.rs](../../rust-wasm-template/lib.rs) as usual.

Here is the basic code you will need:

```rust
extern crate console_error_panic_hook;

use js_sys::Uint8ClampedArray;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn setup() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn edit_bitmap(image_data: &Uint8ClampedArray, width: u32, height: u32) {
}

```

We want to edit the function `edit_bitmap`. `image_data` is a reference to the [Uint8ClampedArray](https://rustwasm.github.io/wasm-bindgen/api/js_sys/struct.Uint8ClampedArray.html) being rendered on your screen. `Uint8ClampedArray` is a flat array of unsigned int of `8` bits.
It represents `srgb` image, which means that each pixel is represented as a vector of 4 elements [red, green, blue, illumination].
The image can be thought of as a matrix of dimension `(width, height, 4)`, it is row-major.

We will reuse our different implementations. So do not erase them when going from one exercise to the next.

First off let's implement some methods that modify the video live:

1. Paint the top half the image to black.
2. Paint the left half of the image to white.
3. Create an X-ray effect. This is done by mapping colors to their opposite, for instance `0<->255`, `100<->155`. Beware of illumination.
4. `(BONUS)` Now feel free to implement other transformations such as `greyscale` or adding `random noise`.
5. Let's now add functionalities to our page. In the `setup` function create a dropdown (`<select>`) that will change which transformation to apply to the image.
6. `(BONUS)` Track moving objects. This can be done by figuring out only the pixels that didn't change between multiple frames. For instance, you could compute the standard deviation of the pixel and black out below a threshold.
While this can be achieved without touching Javascript, I recommend editing it.
