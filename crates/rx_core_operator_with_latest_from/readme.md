# [operator_with_latest_from](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_with_latest_from)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_with_latest_from.svg)](https://crates.io/crates/rx_core_operator_with_latest_from)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_with_latest_from)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_with_latest_from)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

## Example

```sh
cargo run -p rx_core_operator_with_latest_from --example with_latest_from_operator_example
```

```rs
let mut source = PublishSubject::<usize, &'static str>::default();
let mut inner = PublishSubject::<&'static str, &'static str>::default();

let _subscription = source
    .clone()
    .with_latest_from(inner.clone())
    .subscribe(PrintObserver::new("with_latest_from_operator"));

source.next(1);
inner.next("hello");
source.next(2);
source.next(3);
source.next(4);
inner.next("bello");
source.next(5);
inner.error("error");
```

Output:

```txt
with_latest_from_operator - next: (2, "hello")
with_latest_from_operator - next: (3, "hello")
with_latest_from_operator - next: (4, "hello")
with_latest_from_operator - next: (5, "bello")
with_latest_from_operator - error: "error"
with_latest_from_operator - unsubscribed
```
