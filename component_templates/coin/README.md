# Coin

Coins template, which can be used to create new coins for exchange.

To build:

```
cd package
cargo build-wasm
```

To test:
```
cargo test
```

build-wasm is defined in `.cargo/config.toml`:

```
[alias]
build-wasm = "build --target=wasm32-unknown-unknown"
```
