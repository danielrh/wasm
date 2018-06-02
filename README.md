# Rust + WebAssembly demo

## Build

```
rustup target add wasm32-unknown-unknown
cargo build --target=wasm32-unknown-unknown --release
```

### Run

Run [some web server](https://gist.github.com/willurd/5720255) from the folder.

*Note: make sure to configure your webserver... eg for apache: add mime type to `/etc/apache2/mime.types`*

```
application/wasm                    wasm
```

Visit [localhost:8000]() to see the checkerboard rust icon
