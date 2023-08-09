# rust-wasm-template

This repository contains the minimum amount of code needed to experiment with WebAssembly. Including a web server to serve the HTML and WASM as well as the javascript boilerplate needed to load WASM.

- `static` contains the static files including compiled webassembly.
- `src/lib.rs` contains the Rust code.

## Installation

### Rust

Recommended:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Alternatively, see the [installation page](https://www.rust-lang.org/tools/install).

### wasm-pack

Recommended:
```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Alternatively, see the [installation page](https://rustwasm.github.io/wasm-pack/installer/).

## Run the local server

If you have Python on your machine, you can simply

```
cd rust-wasm-template/static

python3 -m http.server
```

Otherwise a Rust alternative exists and you can install it with

```
cargo install cargo-server
```

And run

```
cd rust-wasm-template/static

cargo server
```

Visit http://localhost:8000

## Build WASM and copy target to the correct path

```
wasm-pack build --target web --out-dir static/wasm                                     
```

This command needs to be re-run to view your latest changes.
