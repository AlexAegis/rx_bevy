# Bevy Pipes

[![ci](https://github.com/AlexAegis/bevy_kit/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/bevy_kit/actions/workflows/ci.yml)

Signals across channels,defined by a pipe.
A pipe defines a set of channels between two signal-sockets.

It is a very generic library, it could be used to
map input actions, or to write logic with it, trigger events
based on signals and tresholds to actuate.

TODO:

- Since the user defined stuff is triggered using an observer, it runs in the same schedule as the library. (this also means a difference between pull events like eventreader and push events like observers, that you can chose when to listen to an event, if events are not per schedule idk)
  put trigger into the user schedule as the last step of the propagation process
- Rename connectors to pipes, and allow composable transformations on it.

## Requirements

- [Git LFS](https://git-lfs.github.com/)
- [Latest Rust Stable](https://rustup.rs/)
- [Mold](https://github.com/rui314/mold) (Only on Linux)
- [LLD](https://lld.llvm.org/) (Only on Windows)

## Development

This repository is using `cargo-make`, it will take care of installing all
required cargo extensions and rustup components used in this repository.

1. Run `scripts/setup.sh` (Or run `cargo install cargo-make`)
2. (Optional) Install the rest of the tooling/cargo extensions using
   `cargo make setup`

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
