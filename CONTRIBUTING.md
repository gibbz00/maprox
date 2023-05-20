## Summary of dev packages than should be installed:

- The rust toolchain: `rustup`
- Linker (Linux only): `mold`

## Use pre-commit hooks for some local CI/CD checks

Reduces `git commit --amend && git push --force` abuse.

Do so by copying over a pre-commit script into .git/hooks: 

```sh
# in repo root directory
cp scripts/pre-commit.hook .git/hooks/pre-commit
```

## Using the mold linker (Linux only):

```sh
# Ubuntu
sudo apt-get install mold
# Arch
sudo pacman -S mold
```

