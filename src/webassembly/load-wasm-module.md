# Load a WASM module

## Commands to run:

You can compile the basic WASM library provided in [rust-wasm-template](https://github.com/google/comprehensive-rust/tree/main/src/rust-wasm-template) with this command:

```shell
cd src/rust-wasm-template/project

wasm-pack build --target web && cp -r pkg ../server
```

You can start the web server provided in [rust-wasm-template](https://github.com/google/comprehensive-rust/tree/main/src/rust-wasm-template) with this command:


```shell
cd src/rust-wasm-template/server

cargo run
```

Open the web page on port `8080`. HTML and JS files are provided at [rust-wasm-template/server](https://github.com/google/comprehensive-rust/tree/main/src/rust-wasm-template/server/files).

## Javascript

Once you have compiled your WebAssembly module, you want to call it from your Web application.

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
