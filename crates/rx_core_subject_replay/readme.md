# [subject_replay](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_replay)

[![crates.io](https://img.shields.io/crates/v/rx_core_subject_replay.svg)](https://crates.io/crates/rx_core_subject_replay)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_subject_replay)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_subject_replay)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/AlexAegis/rx_bevy?tab=MIT-1-ov-file)

Buffers the last N values and replays them to late subscribers.

## See Also

- [PublishSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_publish) -
  Forwards observed signals to active subscribers without replaying values, but
  terminal state is replayed.
- [AsyncSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_async) -
  Reduces observed values into one and emits it on completion, replaying the
  result to late subscribers.
- [BehaviorSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_behavior) -
  Always holds a value that is replayed to late subscribers.
- [ProvenanceSubject](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_subject_provenance) -
  BehaviorSubject that also stores an additional filtering value to track
  provenance.
