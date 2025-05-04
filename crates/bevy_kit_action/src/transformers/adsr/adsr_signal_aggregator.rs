#[cfg(feature = "reflect")]
use bevy::prelude::*;

use crate::SignalAggregator;

use super::AdsrSignal;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
	all(feature = "serialize", feature = "reflect"),
	reflect(Serialize, Deserialize)
)]
pub struct AdsrSignalAggregator;

impl SignalAggregator<AdsrSignal> for AdsrSignalAggregator {
	fn combine(&self, mut signals: impl Iterator<Item = AdsrSignal>) -> AdsrSignal {
		signals.next().unwrap_or_default()
	}
}
