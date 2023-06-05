## Summary of dev packages than should be installed:

- The rust toolchain: `rustup`
- Linker (Linux only): `mold`
- Trunk (WASM builds): `trunk`

## Use pre-commit hooks for some local CI/CD checks

Reduces `git commit --amend && git push --force` abuse.

Do so by copying over a pre-commit script into .git/hooks: 

```sh
# in repo root directory
cp scripts/pre-commit.hook .git/hooks/pre-commit
```

## Usage

### Native 

Make sure that the appropriate graphics driver APIs are installed, i.e. Vulkan, DirectX12 or Metal depending on the platform.
Might be possible to use OpenGL by prepending `WGPU_BACKENDS=gl` to the respective build/run commands.

The Linux target uses the mold linker, make sure it is installed.

```sh
# Ubuntu
sudo apt-get install mold
# Arch
sudo pacman -S mold
```

Building `maprox` can then simply be done with:

```sh
 cargo build -p maprox-application
```

Replace `build` with to run it directly.

### Web

Omit `WGPU_BACKENDS=gl` from the respective commands if `WebGPU` is to be used.

#### Building

Add the `wasm` target:

```sh
  rustup target add wasm32-unknown-unknown
```

Then:

```sh
  WGPU_BACKENDS=gl cargo build --target wasm32-unknown-unknown -p maprox-application
```

#### Running locally with `wasm-server-runner`

```sh
  cargo install wasm-server-runner
```

Then:

```sh
  WGPU_BACKENDS=gl cargo run --target wasm32-unknown-unknown -p maprox-application
```

#### Bundling

Requires trunk:

```sh
  cargo install trunk
```

Then:

```sh
  cd maprox-application
  trunk build
```

Output is placed in `maprox-application/dist`.
