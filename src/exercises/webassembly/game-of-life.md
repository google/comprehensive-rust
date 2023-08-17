# Game of life exercise

The goal of this exercise is to implement [John Conway's Game Of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

Serve the web server and navigate to [http://localhost:8000/exercises/game_of_life/](http://localhost:8000/exercises/game_of_life/).

You will edit [lib.rs](../../rust-wasm-template/lib.rs) as usual.

Here is the basic code you will need:

```rust
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

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
```

1. Let's create a `GameOfLife` struct that segments the canvas into cells of dimensions `10x10` pixels. Cells can be
   either dead or alive.
2. Use the struct to paint every alive cells with and dead cells black. To test this, make every other cell alive.
3. Randomly change every cell's status to dead or alive, multiple times per second.
4. Using Javascript and Rust; Create a button that triggers the start of the game and allows stopping/restarting it.
5. Implement the rules of game of life:
    1. Any live cell with fewer than two live neighbours dies, as if by underpopulation.
    2. Any live cell with two or three live neighbours lives on to the next generation.
    3. Any live cell with more than three live neighbours dies, as if by overpopulation.
    4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
6. Create an event listener to manually kill or revive cells on click.