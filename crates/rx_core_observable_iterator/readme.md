# rx_core_observable_iterator

This crate provides functionality to convert iterators into observables using the `IntoIteratorObservableExt` extension trait.

## Features

- **Extension Trait**: `IntoIteratorObservableExt` provides the `into_observable()` method for any type that implements `IntoIterator + Clone`
- **Universal Support**: Works with ranges, vectors, arrays, and any other iterator type
- **No Conflicts**: Uses an extension trait approach to avoid conflicts with the main `IntoObservable` trait

## Usage

```rust
use rx_bevy::prelude::*;

// Convert ranges into observables
(1..=5).into_observable::<()>().subscribe(PrintObserver::new("range"));

// Convert vectors into observables
vec![1, 2, 3].into_observable::<()>().subscribe(PrintObserver::new("vector"));

// Convert arrays into observables
[10, 20, 30].into_observable::<()>().subscribe(PrintObserver::new("array"));
```

## Examples

```sh
cargo run -p rx_core_observable_iterator --example into_observable_example
cargo run -p rx_core_observable_iterator --example iterator_example
```
