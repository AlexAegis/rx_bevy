# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## `rx_bevy` - [0.5.1](https://github.com/AlexAegis/rx_bevy/compare/v0.5.0...v0.5.1) - 2026-10-10

### Other
- update Cargo.toml dependencies

## `rx_bevy` - [0.5.0](https://github.com/AlexAegis/rx_bevy/compare/v0.4.0...v0.5.0) - 2026-10-09

### Fixed
- *(rx_bevy)* build the entity destination example without all features
- [**breaking**] typos

### Other
- [**breaking**] prepare bevy 0.20 upgrade

## `rx_core` - [0.3.0](https://github.com/AlexAegis/rx_bevy/compare/core-v0.2.2...core-v0.3.0) - 2026-10-09

### Added
- *(rx_core_subject_publish)* notify subscribers in subscription order
- *(rx_core_testing_mute_panic)* extracted and fixed mute_panic

### Fixed
- *(rx_core_notification_store)* drop oldest overflow behavior
- invoked work cancellation
- [**breaking**] typos
- handle zero interval in repeated work
- don't panic when the notification queue empties on the last round

### Other
- cover the deferred unsubscribe and the work invocation teardown
- *(rx_core)* label the merge examples merge_observable
- regenerate the stale example output blocks

## `rx_bevy` - [0.4.0](https://github.com/AlexAegis/rx_bevy/compare/v0.3.2...v0.4.0) - 2026-06-20

### Added
- *(rx_bevy)* [**breaking**] upgrade to bevy 0.19

## `rx_bevy` - [0.3.2](https://github.com/AlexAegis/rx_bevy/compare/v0.3.1...v0.3.2) - 2026-02-01

### Added
- *(rx_core_operator_throttle_time)* added the throttle_time operator
- *(rx_core_operator_debounce_time)* added the debounce_time operator

### Other
- release v0.3.1

## `rx_core` - [0.2.1](https://github.com/AlexAegis/rx_bevy/compare/core-v0.2.0...core-v0.2.1) - 2026-02-01

### Added
- *(rx_core_operator_throttle_time)* added the throttle_time operator
- *(rx_core_operator_debounce_time)* added the debounce_time operator

## `rx_bevy` - [0.3.1](https://github.com/AlexAegis/rx_bevy/compare/v0.3.0...v0.3.1) - 2026-01-24

### Fixed
- *(rx_bevy)* added missing observable_fn feature exports from prelude

### Other
- revert pub api change
- added missing entries for destinations
- added missing bevy destinations

## `rx_bevy` - [0.3.0](https://github.com/AlexAegis/rx_bevy/compare/v0.2.0...v0.3.0) - 2026-01-24

### Other
- update Cargo.toml dependencies

## `rx_core` - [0.2.0](https://github.com/AlexAegis/rx_bevy/compare/core-v0.1.2...core-v0.2.0) - 2026-01-24

### Added
- *(rx_bevy)* [**breaking**] upgrade to bevy 0.18

## `rx_bevy` - [0.2.0](https://github.com/AlexAegis/rx_bevy/compare/v0.1.1...v0.2.0) - 2026-01-24

### Added
- *(rx_bevy)* [**breaking**] upgrade to bevy 0.17

## `rx_bevy` - [0.1.1](https://github.com/AlexAegis/rx_bevy/compare/v0.1.0...v0.1.1) - 2026-01-22

### Added
- *(rx_core_operator_count)* added the count operator
- *(rx_core_operator_subscribe_on)* added the subscribe_on operator
- *(rx_core_operator_observe_on)* added the observe_on operator
- *(rx_core_operator_element_at)* added the element_at operator

## `rx_core` - [0.1.1](https://github.com/AlexAegis/rx_bevy/compare/core-v0.1.0...core-v0.1.1) - 2026-01-22

### Added
- *(rx_core_operator_count)* added the count operator
- *(rx_core_operator_subscribe_on)* added the subscribe_on operator
- *(rx_core_operator_observe_on)* added the observe_on operator
- *(rx_core_operator_element_at)* added the element_at operator

### Other
- simplified map example

## `rx_bevy` - [0.1.0](https://github.com/AlexAegis/rx_bevy/releases/tag/v0.1.0) - 2026-01-19

Initial Release!

## `rx_core` - [0.1.0](https://github.com/AlexAegis/rx_bevy/releases/tag/core-v0.1.0) - 2026-01-19

Initial Release!
