# rx_bevy

[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)

> Reactive Extensions for the [Bevy Game Engine][BevyWebsite]!

## Usage

## Development of this repository

This repository is using `cargo-make`, it will take care of installing all
required cargo extensions and rustup components used in this repository.

1. Run `scripts/setup.sh` (Or run `cargo install cargo-make`)
2. (Optional) Install the rest of the tooling/cargo extensions using
   `cargo make setup`

### Requirements

- [Git LFS](https://git-lfs.github.com/)
- [Latest Rust Stable](https://rustup.rs/)

### What is that `package.json` file doing here?

Remark, the markdown formatter I use is JS based, and my configuration is a
JavaScript package.

### `cargo-make` tasks

- `cargo make all` to run everything that could make ci fail (Everything below)
- `cargo make build` to build all crates
- `cargo make test` to test all crates
- `cargo make format` to format all crates
- `cargo make lint` to lint all crates using `clippy` and `rustfmt`
- `cargo make book-build` to build the documentation book

## Bevy Compatibility Table

| Bevy | rx_bevy |
| ---- | ------- |
| 0.17 | 0.2     |
| 0.16 | 0.1     |

[BevyWebsite]: https://bevyengine.org/
