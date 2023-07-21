# WebAssembly basics

WebAssembly (WASM) is a binary instruction format designed to bring more languages to web browsers (historically, Javascript was the only available choice.) WASM is designed as a portable target for compilation of high-level languages like Rust, enabling it to be used in web applications. WASM has also grown beyond its scope because it can be run in a number of use cases beyond the web, including [nodejs](https://nodejs.dev/en/learn/nodejs-with-webassembly/) or the [Ethereum WebAsssembly Runtime](https://ewasm.readthedocs.io/en/mkdocs/README/)

Rust is often considered as one of the best languages to compile to WebAssembly due to its safety, performance, and rather expansive WASM ecosystem.

# Setting up the environment

WebAssembly needs to be run in a Browser or a VM, therefore we will use a different set up for this class. Please navigate to [rust-wasm-template](https://github.com/google/comprehensive-rust/tree/main/src/rust-wasm-template) to view the installation instructions. Feel free to either clone the repository to run it locally, or open a new [Codespace on Github](https://codespaces.new/google/comprehensive-rust)

## Install wasm-pack

Recommended:

```shell
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Alternatively, see the [installation page](https://rustwasm.github.io/wasm-pack/installer/).

## Run the local server

WebAssembly cannot be loaded from the `files://` protocol yet, so we need to spawn a Web server to serve the different files.

```shell
cd server

cargo run
```

- On a devcontainer, go to `PORTS` and open the link under `Local Address` for Port `8080`
- Locally, visit http://localhost:8080

## Build WASM and copy target to the correct path

This command needs to be re-run to view your latest changes.

```shell
cd project

wasm-pack build --target web && cp -r pkg ../server
```
