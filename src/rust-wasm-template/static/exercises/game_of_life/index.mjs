import init, {setup} from '/wasm/project.js';


(async () => { 
    // Run the init method to initiate the WebAssembly module.
    await init();
    setup();
    const canvas = document.getElementById('game-of-life-canvas');
    const ctx = canvas.getContext('2d');
    const game = new GameOfLife(ctx, canvas.width, canvas.height);
})();
