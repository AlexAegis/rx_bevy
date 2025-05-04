use bevy::math::Vec2;

use crate::SignalAggregator;

#[cfg(feature = "reflect")]
use bevy::prelude::*;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "reflect", derive(Reflect), reflect(Debug, Clone))]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
#[cfg_attr(
	all(feature = "serialize", feature = "reflect"),
	reflect(Serialize, Deserialize)
)]
pub enum SignalAggregatorVec2 {
	#[default]
	Add,
	Multiply,
	First,
	Last,
}

impl SignalAggregator<Vec2> for SignalAggregatorVec2 {
	fn combine(&self, mut signals: impl Iterator<Item = Vec2>) -> Vec2 {
		match &self {
			SignalAggregatorVec2::Add => signals.fold(Vec2::ZERO, |acc, v| acc + v),
			SignalAggregatorVec2::Multiply => signals.fold(Vec2::ONE, |acc, v| acc * v),
			SignalAggregatorVec2::First => signals.next().unwrap_or(Vec2::ZERO),
			SignalAggregatorVec2::Last => signals.last().unwrap_or(Vec2::ZERO),
		}
	}
}
