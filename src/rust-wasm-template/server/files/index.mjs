import init, {set_panic_hook, add} from '/wasm/project.js';

(async () => { 
    // Run the init method to initiate the WebAssembly module.
    await init();
    set_panic_hook();
    document.querySelector("#wasmoutput").innerHTML = add(1, 2);
})();
