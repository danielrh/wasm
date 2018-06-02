# Rust + WebAssembly demo

## Build

```
rustup target add wasm32-unknown-unknown
cargo build --target=wasm32-unknown-unknown --release
```

### Run


```
python server.py
```

Visit [localhost:8000]() to see the checkerboard rust icon
