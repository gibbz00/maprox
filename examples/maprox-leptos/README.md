## Installing Additional Tools

By default, `cargo-leptos` uses `nightly` Rust, `cargo-generate`, and `sass`. If you run into any trouble, you may need to install one or more of these tools.

1. `rustup toolchain install nightly --allow-downgrade` - make sure you have Rust nightly
2. `rustup default nightly` - setup nightly as default, or you can use rust-toolchain file later on
3. `rustup target add wasm32-unknown-unknown` - add the ability to compile Rust to WebAssembly
4. `cargo install cargo-generate` - install `cargo-generate` binary (should be installed automatically in future)
5.  `cargo install wasm-server-runner` - serves the `maprox` application.
6. `cargo install matchbox_server` - Signalling server to bootstrap the `maprox_connection`.

## Run

1. Start the signalling server on port 3535:

```sh
  matchbox_server
```

This enables `leptos` and `maprox` to talk to each other through an established WebRTC connection.

2. Serve the `maprox` application on port 1334:

```sh
  WGPU_BACKENDS=gl cargo run --target wasm32-unknown-unknown -p maprox-application
```

Omit `WGPU_BACKENDS=gl` if `WebGPU` is to be used.

3. Serve a `leptos` project with that embeds the `maprox` application in an iframe on port 3000.

```bash
  cargo serve watch -p maprox-application
```

