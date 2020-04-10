## Rust Wasm App

### Step 1
__commands__:

- `rustup target add wasm32-unknown-unknown` (wasm is not specific to any platform. eg, `darwin`)
- `cargo install wasm-gc`
- `cargo install https`
- `cargo new --lib rust-wasm-app`
- `cargo build --target wasm32-unknown-unknown --release`
- `wasm-gc target/wasm32-unknown-unknown/release/rust_wasm_app.wasm -o rust_wasm_app.gc.wasm`

__snippets__ for index.html:

```js
// Legacy way: fetching entire wasm before we initiate
fetch("rust_wasm_app.gc.wasm")
  .then(response => response.arrayBuffer())
  .then(result => WebAssembly.instantiate(result))
  .then(wasmModule => {
    const result = wasmModule.instance.exports.add_one(2);
    const text = document.createTextNode(result);
    document.body.appendChild(text);
  });
// New way: stream, compile and instantiate
WebAssembly.instantiateStreaming(fetch("rust_wasm_app.gc.wasm"))
  .then(wasmModule => {
    const result = wasmModule.instance.exports.add_one(3);
    const text = document.createTextNode(result);
    document.body.appendChild(text);
  });
```

### Step 2
Pass a JavaScript Function to WebAssembly and Invoke it from Rust

__snippets__ for index.html:

```js
const appendNumberToBody = (number) => {
  const text = document.createTextNode(number);
  document.body.appendChild(text;)
}

const importObject = {
  env: {
    appendNumberToBody: appendNumberToBody,
    alert: alert,
  }
};

WebAssembly.instantiateStreaming(fetch("rust_wasm_app.gc.wasm"), importObject)
  .then(wasmModule => {
    wasmModule.instance.exports.run();
  })
```

__snippets__ for `lib.rs`:

```rust
extern {
  fn appendNumberToBody(x: u32);
  fn alert(x: u32);
}

#[no_mangle]
pub extern fn run() {
  unsafe {
    appendNumberToBody(42);
    alert(4);
  }
}
```

Reference:
- https://egghead.io/courses/using-webassembly-with-rust
