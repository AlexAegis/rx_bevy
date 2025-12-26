# [xtask](https://github.com/AlexAegis/rx_bevy/tree/master/crates/xtask)

Utility binary for repository maintenance tasks.

## Lints

- **Codecov**: checks that every workspace crate appears in `codecov.yml` under
  `component_management.individual_components`.
  - `--ignore` to skip specific crate names

  ```sh
  cargo run -p xtask -- lint codecov
  cargo run -p xtask -- lint codecov --ignore some_crate --ignore other_crate
  ```

- **Docs**: validates that observable and operator crates have the matching
  Markdown stubs in the docs tree:
  - `rx_core_observable_*` → `docs/observable/{name}.md`
  - `rx_bevy_observable_*` → `docs/observable_bevy/{name}.md`
  - `rx_core_operator_*` → `docs/operator/{name}.md`
  - `rx_bevy_operator_*` → `docs/operator_bevy/{name}.md`

  ```sh
  cargo run -p xtask -- lint docs
  ```
