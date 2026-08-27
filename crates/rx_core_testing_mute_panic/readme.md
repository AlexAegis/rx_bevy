# [testing_mute_panic](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_testing_mute_panic)

[![crates.io](https://img.shields.io/crates/v/rx_core_testing_mute_panic.svg)](https://crates.io/crates/rx_core_testing_mute_panic)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_testing_mute_panic)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_testing_mute_panic)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

`mute_panic` silences the panic message while a closure runs, on the calling
thread only. A `#[should_panic]` test run with `--nocapture` prints the message
it expects.

## Example

```rust
use rx_core_testing_mute_panic::mute_panic;

#[test]
#[should_panic]
fn should_panic_when_asked_to() {
    mute_panic(|| panic!("expected"));
}
```

The panic hook is installed once and stays installed for the process. It cannot
be restored per call, because `set_hook` and `take_hook` both refuse to run on a
panicking thread.
