# Solutions for the Game Of Life exercise

1. Let's create a `GameOfLife` struct that segments the canvas into cells of dimensions `10x10` pixels. Cells can be
   either dead or alive.

```rust
struct GameOfLife {
    ctx: CanvasRenderingContext2d,
    width: i32,
    height: i32,
    // true == alive, false == dead
    cells: Vec<bool>,
}

impl GameOfLife {
    fn new(ctx: CanvasRenderingContext2d, width: i32, height: i32) -> Self {
        let mut cells = vec![false; (width * height) as usize / 10];
        cells.iter_mut().step_by(2).for_each(|cell| *cell = true);
        Self {
            ctx,
            width,
            height,
            cells,
        }
    }
}
```

2. Use the struct to paint every alive cells with and dead cells black. To test this, make every other cell alive.

```rust
    // In impl GameOfLife
    fn draw(&self) {
        self.ctx
            .clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
        self.ctx.set_fill_style(&JsValue::from_str("white"));
        for (i, cell) in self.cells.iter().enumerate() {
            if *cell {
                let x = (i % (self.width as usize / 10)) as f64 * 10.0;
                let y = (i / (self.width as usize / 10)) as f64 * 10.0;
                self.ctx.fill_rect(x, y, 10.0, 10.0);
            }
        }
    }
```

3. 4 times per second, randomly change every cell's status to dead or alive.

```rust
    #[wasm_bindgen]
    extern "C" {
        fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    }

    #[wasm_bindgen]
    pub struct GameOfLifeHandle {
        closure: Closure<dyn FnMut()>,
    }

    // In impl GameOfLife
    fn game_logic(&mut self) {
        for cell in &mut self.cells {
            let rand = js_sys::Math::random();
            if rand > 0.5 {
                *cell = true;
            } else {
                *cell = false;
            }
        }
    }

    pub fn start(mut self) -> GameOfLifeHandle {
        let closure = Closure::wrap(Box::new(move || {
            self.game_logic();
            self.draw();
        }) as Box<dyn FnMut()>);
        setInterval(&closure, 250);
        GameOfLifeHandle { closure }
    }
```

4. Using Javascript and Rust; Create a button that triggers the start of the game and allows stopping it.


```javascript
import init, {setup, GameOfLife} from '/wasm/project.js';


(async () => { 
    // Run the init method to initiate the WebAssembly module.
    await init();
    setup();
    const canvas = document.getElementById('game-of-life-canvas');
    const ctx = canvas.getContext('2d');
    const game = new GameOfLife(ctx, canvas.width, canvas.height);
    const startButton = document.createElement('button');
    document.body.appendChild(startButton);
    let handle = null;
    let playing = false;
    startButton.addEventListener('click', () => {
        if (handle === null) {
            handle = game.start();
            playing = true;
        } else {
            handle.stop_or_start();
            playing = !playing;
        }
        startButton.textContent = playing ? 'STOP' : 'START';
    });
})();

```

```rust
#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, millis: u32) -> f64;
    fn clearInterval(token: f64);
}

#[wasm_bindgen]
pub struct GameOfLifeHandle {
    closure: Closure<dyn FnMut()>,
    interval: Option<f64>,
}

#[wasm_bindgen]
impl GameOfLifeHandle {
    pub fn stop_or_start(&mut self) {
        if let Some(interval) = self.interval.take() {
            clearInterval(interval);
        } else {
            let interval = setInterval(&self.closure, 250);
            self.interval = Some(interval);
        }
    }
}
```

5. Implement the rules of game of life

```rust
    fn alive_neighbour_count(&self, idx: i32) -> usize {
        let mut alive_count = 0;
        let (width, height) = (self.width as i32 / 10, self.height as i32 / 10);
        let x = idx % width;
        let y = idx / width;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let x = x as i32 + i;
                let y = y as i32 + j;
                if x < 0 || x >= width || y < 0 || y >= height {
                    continue;
                }
                let idx = (x + y * self.width) as usize;
                if self.cells[idx] {
                    alive_count += 1;
                }
            }
        }
        alive_count
    }

    fn game_logic(&mut self) {
        let mut new_cells = self.cells.clone();
        for idx in 0..self.cells.len() {
            let alive_count = self.alive_neighbour_count(idx as i32);
            // Rule 1
            if self.cells[idx] && alive_count < 2 {
                new_cells[idx] = false;
            }
            // Rule 2
            if self.cells[idx] && (alive_count == 2 || alive_count == 3) {
                new_cells[idx] = true;
            }
            // Rule 3
            if self.cells[idx] && alive_count > 3 {
                new_cells[idx] = false;
            }
            // Rule 4
            if !self.cells[idx] && alive_count == 3 {
                new_cells[idx] = true;
            }
        }
        self.cells = new_cells;
    }
```