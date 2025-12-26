# [observable_join](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_observable_join)

[![crates.io](https://img.shields.io/crates/v/rx_core_observable_join.svg)](https://crates.io/crates/rx_core_observable_join)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_observable_join)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_observable_join)

This observable will only emit once both of it's input observables
have completed. After which it will emit a tuple of the last emissions
from each input observable, then complete.

Meaning if even one of the observables haven't emitted before all of them
had completed, only a complete notification will be observed!

If not all observables complete, nothing will be emitted even if all
input observables were primed.

## Example

```sh
cargo run -p rx_core_observable_join --features example --example join_example
```

```sh
cargo run -p rx_core_observable_join --features example --example join_subject_example
```
