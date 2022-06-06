<div align="center">

  <h1><code>Numeric Oxide</code></h1>

  <strong>A numerical precision library written in Rust and compiled to WebAssembly for use in Javascript enviroments.</strong>

</div>

## About

This library allows for precise numeric operations in Javascript to be used in financial calculations or other situations where accuracy is key. The library is written in Rust, compiled to WebAssembly and uses [`rust_decimal`](https://crates.io/crates/rust_decimal) under the hood to perform calculations.

## 🔋 Usage

```js
import { oxidate } from 'numeric-oxide';

// Perform accurate calculations
oxidate("add(2.0003, 6.71)"); // => "8.71003"

oxidate("mult(2, add(6, 7.77))"); // => "27.54"

oxidate("div(9, 5)"); // => "1.8"

oxidate("mod(10, 3)"); // => "1"

```

The longest time-sink in calculations is passing between WebAssembly and Javascript, so, for optimum performance, it is best to chain calculations together into one string or to perform multiple calculations simultaneously with `oxidate_multiple`:

```js
import { oxidate_multiple } from 'numeric-oxide';

// Run multiple calculations together
oxidate_multiple([
  "add(2.0003, 6.71)",
  "mult(2, add(6, 7.77))",
  "div(9, 5)",
  "mod(10, 3)"
]); // => ["8.71003", "27.54", "1.8", "1"]

```

### 🛠️ Build with `wasm-pack build`

```sh
wasm-pack build
```

### 🔬 Test using `wasm-pack test`

```sh
cargo test && wasm-pack test --node
```
