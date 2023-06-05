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

## Building

### Linux target uses the mold linker:

```sh
# Ubuntu
sudo apt-get install mold
# Arch
sudo pacman -S mold
```

### For the Web

Requires trunk:

```sh
  cargo install trunk
```

Then:

```sh
  cd maprox-application
  WGPU_BACKENDS=gl cargo build --target wasm32-unknown-unknown
  trunk build
```

Output is placed in `maprox-application/dist`.
Omit `WGPU_BACKENDS=gl` if `WebGPU` is to be used.

