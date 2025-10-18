# rx_core_operator_finalize

## Example

```sh
cargo run -p rx_core_operator_finalize --features example --example finalize_operator_completion_example
```

```sh
cargo run -p rx_core_operator_finalize --features example --example finalize_operator_unsubscribe_example
```

````rs
use rx_bevy::prelude::*;

/// The finalize operators closure will only be called once per subscription!
///
/// Output:
///
/// ```sh
/// finalize_example - next: 12
/// finally!
/// finalize_example - completed
/// ```
fn main() {
 of(12)
  .finalize(|| println!("finally!"))
  .subscribe(PrintObserver::new("finalize_example"));
}
````
