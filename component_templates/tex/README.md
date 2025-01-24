# Tex

Tari Exchange Template. This component provides an ability to add and remove liquidity, and execute simple swaps
using Automated Market Makers mechanism.

## Build

```
cargo build-wasm -p tex --release
```

build-wasm is defined in `.cargo/config.toml`:

```
[alias]
build-wasm = "build --target=wasm32-unknown-unknown"
```

## Deploy

Open [UI](`http://localhost:8080/`) and scroll to the bottom of the page. Browse for WASM inside target folder and click deploy.
