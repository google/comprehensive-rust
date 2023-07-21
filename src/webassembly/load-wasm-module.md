# Load a WASM module

Once you have compiled your WebAssembly module, you want to call it from your Web application.
This chapter will cover minimum amount of Javascript required to load a WASM module into a Web application.

```javascript
// Import the module and the exported method `add`
import init, {add} from '/wasm/project.js'; // A typescript version is also generated

// Async IIFE
(async () => { 
    // Run the init method to initiate the WebAssembly module.
    await init();
    console.log('WASM output', add(1, 2));
})();
```

Adding the Javascript module to an HTML file:

```html
<script type="module" src="/path/to/module.mjs"></script>
```

<details>

* This loads the compiled WebAssembly
* `init` installs the bytecode and compiles it
* `add` is an exported method
* For this class, we are compiling `wasm-pack` with the `--web` flag, complex applications will want to use a bundler,
see more information about build options on the [official documentation](https://rustwasm.github.io/docs/wasm-pack/commands/build.html)

</details>
