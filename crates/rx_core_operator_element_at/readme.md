# [operator_element_at](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_element_at)

[![crates.io](https://img.shields.io/crates/v/rx_core_operator_element_at.svg)](https://crates.io/crates/rx_core_operator_element_at)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_operator_element_at)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_operator_element_at)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Emit the value at the given index then complete.

If the element at the specified index does not exist, because it had
completed before reaching that index, the operator will either error
with [ElementAtOperatorError::IndexOutOfRange] or emit a default value
if one was provided.

## See Also

- [FindOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_find) -
  Emit the first value that matches a predicate.
- [FilterOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_filter) -
  Keep values that satisfy a predicate.
- [FirstOperator](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_operator_first) -
  Emit only the first value, then complete.

## Example

```sh
cargo run -p rx_core --example operator_element_at_example
```

```rs
let _subscription = vec!["a", "b", "c", "d", "e"]
    .into_observable()
    .element_at(2)
    .subscribe(PrintObserver::new("element_at_operator"));
```

Output:

```txt
element_at_operator - next: "c"
element_at_operator - completed
element_at_operator - unsubscribed
```
