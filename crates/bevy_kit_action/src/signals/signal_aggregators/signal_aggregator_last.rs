use crate::{Signal, SignalAggregator};

#[cfg(feature = "reflect")]
use bevy::prelude::*;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

/// Not sure if I should keep this aggregator, there isn't an inherent order
/// to signals routed to a single socket, but this is a simple way of selecting
/// a single one.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
	all(feature = "serialize", feature = "reflect"),
	reflect(Serialize, Deserialize)
)]
pub struct SignalAggregatorLast;

impl<S: Signal> SignalAggregator<S> for SignalAggregatorLast {
	fn combine(&self, signals: impl Iterator<Item = S>) -> S {
		signals.last().unwrap_or_default()
	}
}
