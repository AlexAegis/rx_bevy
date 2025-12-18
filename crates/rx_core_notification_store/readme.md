# [notification_store](https://github.com/AlexAegis/rx_bevy/tree/master/crates/rx_core_notification_store)

[![crates.io](https://img.shields.io/crates/v/rx_core_notification_store.svg)](https://crates.io/crates/rx_core_notification_store)
[![ci](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml/badge.svg)](https://github.com/AlexAegis/rx_bevy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/github/AlexAegis/rx_bevy/graph/badge.svg?token=hUtTGQaWMn&component=rx_core_notification_store)](https://app.codecov.io/github/AlexAegis/rx_bevy?components%5B0%5D=rx_core_notification_store)

This crate defines stateful storages of `SubscriberNotification`s for `rx_core`
crates. Used when the state of an upstream source must be tracked based
solely from the events received from it.

The `NotificationQueue` allows queuing up notifications for uses where they
might be consumed slower than they are received. The queue also deals with
overflow by dropping either the oldest value, or ignoring the next once a
limit is reached.

The queue only reflects the state of the front notification, and updates as they
are consumed.
