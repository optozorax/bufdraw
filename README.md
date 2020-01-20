# bufdraw

Simple library to create games or programs with simple interface. You can draw to rgba buffer directly and it was drawed to screen. Based on [miniquad](https://github.com/not-fl3/miniquad), therefore supports all platforms miniquad supports: windows, linux, webassembly (wasm).

# example

```
cargo run --example simple
```

# web

Run example in browser:

```
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release --example simple
cp target/wasm32-unknown-unknown/release/examples/simple.wasm www/target.wasm
cargo install --git https://github.com/TheWaWaR/simple-http-server.git
simple-http-server www --nocache --ip 0.0.0.0
```

Visit localhost:8000 in browser

# known bugs

Memory leak because of not deallocating textures. Waiting for new version of miniquad.